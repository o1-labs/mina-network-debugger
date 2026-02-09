// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use super::{DynSized, Collection};

mod limit;
pub use self::limit::{Limit, LimitDescriptor, LimitError};

mod error;
pub use self::error::{ParseError, ParseErrorKind};

mod core;
pub use self::core::{Absorb, AbsorbExt};

mod primitives;

mod atomics;

mod seq;

/// implementations for some standard types
#[cfg(feature = "std")]
mod types;
