// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![cfg_attr(feature = "nightly", feature(min_specialization))]
#![cfg_attr(not(any(feature = "codec", feature = "std")), no_std)]

#[macro_use]
extern crate alloc;

extern crate self as radiation;

#[cfg(feature = "derive")]
pub use radiation_macros::{Absorb, Emit, Limit};

pub use nom;

mod absorb;
pub use self::absorb::{
    Absorb, AbsorbExt, Limit, LimitDescriptor, LimitError, ParseError, ParseErrorKind,
};

mod emit;
pub use self::emit::{Emit, RadiationBuffer};

#[cfg(all(test, feature = "derive"))]
mod tests;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DynSized<T>(pub T);

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Collection<C>(pub C);

#[cfg(feature = "codec")]
pub mod codec;

#[cfg(feature = "dilithium")]
mod dil;
