# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build the main bpf-recorder binary (requires Linux with eBPF support)
cargo build --bin bpf-recorder --release

# Build with BPF kernel module (requires nightly-2022-10-10 toolchain)
cargo build --bin bpf-recorder-kern --release --features kern

# Build the integration test binary
cargo build --bin coda-libp2p_helper-test

# Run unit tests
cargo test

# Run the debugger (requires sudo on Linux)
sudo -E RUST_LOG=info ./target/release/bpf-recorder

# Run in test mode
sudo -E RUST_LOG=info TEST=1 TERMINATE=1 ./target/release/bpf-recorder
```

## Architecture

This is a Mina blockchain network debugger that uses eBPF to trace network traffic from the libp2p_helper subprocess. It consists of two main parts:

### Core Components

- **bpf-recorder**: Main binary that runs the eBPF module and userspace application. The kernel module (built with `kern` feature) and userspace (built with `user` feature) share the same `main.rs` entry point.

- **mina-recorder**: Core library containing:
  - `recorder.rs`: `P2pRecorder` state machine managing debuggee processes and TCP connections
  - `connection/`: Protocol stack implementation (pnet → multistream_select → noise → mux → mina_protocol)
  - `database/`: RocksDB-based storage for messages, connections, and IPC events
  - `server.rs`: HTTP/HTTPS server exposing REST API for the frontend
  - `decode/`: Transforms binary wire data to JSON (meshsub, kademlia, rpc, noise, yamux)
  - `key_recover.rs`: Recovers ephemeral encryption keys by intercepting them at startup

### Protocol Stack (nested state machines)

The connection state machine processes data through layers:
```
pnet::State → multistream_select::State → noise::State → mux::State → mina_protocol::State
```

Each layer implements the `HandleData` trait with `on_data()` method to process bytes and pass to inner layers.

### Supporting Crates

- **bpf-ring-buffer**: Shared memory ring buffer between kernel and userspace
- **mina-aggregator**: Aggregates data from multiple debugger instances
- **mina-ipc**: Decodes capnp-encoded IPC between mina daemon and libp2p_helper
- **simulator**: Network simulation tooling
- **topology-tool**: Analyzes network topology from debugger data
- **tester**: Integration test binary (`coda-libp2p_helper-test`)

## Environment Variables

- `SERVER_PORT`: HTTP server port (default: 8000)
- `DB_PATH`: RocksDB storage path (default: target/db)
- `DRY`: Disable BPF for database inspection only
- `HTTPS_KEY_PATH`, `HTTPS_CERT_PATH`: Enable HTTPS
- `BPF_ALIAS`: Set on Mina app (not debugger) as `${CHAIN_ID}-${EXTERNAL_IP}`
- `TEST`: Enable test mode
- `TERMINATE`: Auto-terminate after test completes

## Build Prerequisites

Requires Linux with:
- Rust nightly-2022-10-10 (for BPF kernel module)
- bpf-linker (from https://github.com/vlad9486/bpf-linker)
- capnproto, libelf-dev, libbpf-dev, protobuf-compiler
