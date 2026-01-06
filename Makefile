# Mina Network Debugger Makefile

# Docker configuration
DOCKER_ORG ?= openmina
DOCKER_TAG ?= latest
GIT_COMMIT := $(shell git rev-parse --short=8 HEAD)

# Rust toolchain
NIGHTLY_VERSION := $(shell cat rust-toolchain.toml 2>/dev/null | grep channel | cut -d'"' -f2 || echo "nightly")

# BPF linker source
BPF_LINKER_SRC ?= https://github.com/aya-rs/bpf-linker

# Cap'n Proto version
CAPNP_VERSION ?= 0.10.2

.PHONY: help
help: ## Show this help message
	@echo "Mina Network Debugger - Makefile"
	@echo ""
	@echo "Common Variables:"
	@echo "  DOCKER_ORG=<org>     Docker organization (default: openmina)"
	@echo "  DOCKER_TAG=<tag>     Docker image tag (default: latest)"
	@echo ""
	@echo "Available targets:"
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}'

# =============================================================================
# Setup targets
# =============================================================================

.PHONY: setup
setup: setup-rust setup-bpf-linker setup-capnproto ## Install all development dependencies

.PHONY: setup-deps
setup-deps: ## Install system dependencies (Ubuntu/Debian)
	@echo "Installing system dependencies..."
	sudo apt-get update
	sudo apt-get install -y git curl libelf-dev protobuf-compiler clang libssl-dev pkg-config libbpf-dev make libbz2-dev

.PHONY: setup-rust
setup-rust: ## Install Rust and required components
	@echo "Installing Rust..."
	@if ! command -v rustup &> /dev/null; then \
		curl https://sh.rustup.rs -sSf | sh -s -- -y; \
		echo "Please run 'source ~/.cargo/env' or restart your shell"; \
	fi
	@echo "Installing nightly toolchain and components..."
	rustup component add rust-src
	rustup component add rustfmt
	rustup component add clippy

.PHONY: setup-bpf-linker
setup-bpf-linker: ## Install BPF linker for eBPF compilation
	@echo "Installing bpf-linker..."
	cargo install bpf-linker --git $(BPF_LINKER_SRC)

.PHONY: setup-capnproto
setup-capnproto: ## Install Cap'n Proto compiler
	@echo "Installing Cap'n Proto $(CAPNP_VERSION)..."
	@if command -v capnp &> /dev/null; then \
		echo "Cap'n Proto already installed: $$(capnp --version)"; \
	else \
		curl -sSL https://capnproto.org/capnproto-c++-$(CAPNP_VERSION).tar.gz | tar -zxf - \
		&& cd capnproto-c++-$(CAPNP_VERSION) \
		&& ./configure \
		&& make -j$$(nproc) check \
		&& sudo make install \
		&& cd .. \
		&& rm -rf capnproto-c++-$(CAPNP_VERSION); \
	fi

# =============================================================================
# Build targets
# =============================================================================

.PHONY: build
build: ## Build the project in debug mode
	cargo build --bin bpf-recorder

.PHONY: build-release
build-release: ## Build the project in release mode
	cargo build --bin bpf-recorder --release

.PHONY: build-kern
build-kern: ## Build the BPF kernel module (requires nightly)
	CARGO_TARGET_DIR=target/bpf cargo rustc \
		--package=bpf-recorder \
		--bin=bpf-recorder-kern \
		--features=kern \
		--no-default-features \
		--target=bpfel-unknown-none \
		-Z build-std=core \
		--release \
		-- -Cdebuginfo=2 -Clink-arg=--disable-memory-builtins -Clink-arg=--btf

.PHONY: build-all
build-all: build-kern build-release ## Build both kernel module and userspace binary

.PHONY: build-aggregator
build-aggregator: ## Build the aggregator binary
	cargo build --bin mina-aggregator --release

.PHONY: build-topology-tool
build-topology-tool: ## Build the topology tool binary
	cargo build --bin topology-tool --release

.PHONY: build-test-binary
build-test-binary: ## Build the integration test binary
	cargo build --bin coda-libp2p_helper-test

# =============================================================================
# Test targets
# =============================================================================

.PHONY: test
test: ## Run unit tests
	cargo test

.PHONY: test-release
test-release: ## Run unit tests in release mode
	cargo test --release

# =============================================================================
# Code quality targets
# =============================================================================

.PHONY: format
format: ## Format code using rustfmt
	cargo fmt

.PHONY: check-format
check-format: ## Check code formatting
	cargo fmt -- --check

.PHONY: lint
lint: ## Run clippy linter
	cargo clippy --all-targets -- -D warnings

.PHONY: check
check: ## Check code for compilation errors
	cargo check --all-targets

# =============================================================================
# Docker targets
# =============================================================================

.PHONY: docker-build
docker-build: ## Build the Docker image
	docker build -t $(DOCKER_ORG)/mina-network-debugger:$(DOCKER_TAG) .

.PHONY: docker-build-commit
docker-build-commit: ## Build Docker image tagged with git commit
	docker build -t $(DOCKER_ORG)/mina-network-debugger:$(GIT_COMMIT) .

.PHONY: docker-push
docker-push: ## Push Docker image to registry
	docker push $(DOCKER_ORG)/mina-network-debugger:$(DOCKER_TAG)

.PHONY: docker-run
docker-run: ## Run the debugger in Docker (requires privileged mode)
	docker run --rm --privileged \
		-v /sys/kernel/debug:/sys/kernel/debug:rw \
		-v /proc:/proc:ro \
		-e RUST_LOG=info \
		-p 8000:8000 \
		$(DOCKER_ORG)/mina-network-debugger:$(DOCKER_TAG)

# =============================================================================
# Run targets
# =============================================================================

.PHONY: run
run: build-release ## Build and run the debugger (requires sudo)
	@echo "Running debugger (requires sudo)..."
	sudo -E RUST_LOG=info ./target/release/bpf-recorder

.PHONY: run-test
run-test: build-release build-test-binary ## Run the integration test
	@echo "Starting debugger in test mode..."
	@echo "Run in another terminal: BPF_ALIAS=test PATH=$$(pwd)/target/debug coda-libp2p_helper-test"
	sudo -E RUST_LOG=info TEST=1 TERMINATE=1 ./target/release/bpf-recorder

# =============================================================================
# Utility targets
# =============================================================================

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

.PHONY: clean-db
clean-db: ## Clean the database directory
	rm -rf target/db
