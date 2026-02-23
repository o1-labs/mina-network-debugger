// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use core::sync::atomic::{AtomicI64, AtomicU64, AtomicUsize};

use nom::{combinator, IResult};

use super::{core::Absorb, error::ParseError, limit::Limit};

impl<'pa> Absorb<'pa> for AtomicUsize {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(Absorb::absorb::<L>, Self::new)(input)
    }
}

impl<'pa> Absorb<'pa> for AtomicU64 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(Absorb::absorb::<L>, Self::new)(input)
    }
}

impl<'pa> Absorb<'pa> for AtomicI64 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(Absorb::absorb::<L>, Self::new)(input)
    }
}
