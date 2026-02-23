// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use nom::{Err, IResult};

use super::{error::ParseError, limit::Limit};

pub trait Absorb<'pa>
where
    Self: Sized,
{
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit;
}

pub trait AbsorbExt<'pa>
where
    Self: Sized,
{
    fn absorb_ext(input: &'pa [u8]) -> Result<Self, Err<ParseError<&'pa [u8]>>>;
}

impl<'pa, T> AbsorbExt<'pa> for T
where
    T: Absorb<'pa>,
{
    fn absorb_ext(input: &'pa [u8]) -> Result<Self, Err<ParseError<&'pa [u8]>>> {
        T::absorb::<()>(input).map(|(_, t)| t)
    }
}
