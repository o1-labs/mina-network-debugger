// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use core::marker::PhantomData;
use alloc::boxed::Box;

use nom::{IResult, combinator, number, branch, bytes::complete, sequence};

use super::{core::Absorb, error::ParseError, limit::Limit};

impl<'pa, T> Absorb<'pa> for PhantomData<T> {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::success(PhantomData)(input)
    }
}

impl<'pa> Absorb<'pa> for () {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::success(())(input)
    }
}

impl<'pa> Absorb<'pa> for bool {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        branch::alt((
            combinator::map(complete::tag(&[0x00][..]), |_| false),
            combinator::map(complete::tag(&[0xff][..]), |_| true),
        ))(input)
    }
}

impl<'pa> Absorb<'pa> for i8 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::i8(input)
    }
}

impl<'pa> Absorb<'pa> for u8 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::u8(input)
    }
}

impl<'pa> Absorb<'pa> for i16 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::be_i16(input)
    }
}

impl<'pa> Absorb<'pa> for u16 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::be_u16(input)
    }
}

impl<'pa> Absorb<'pa> for i32 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::be_i32(input)
    }
}

impl<'pa> Absorb<'pa> for u32 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::be_u32(input)
    }
}

impl<'pa> Absorb<'pa> for i64 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::be_i64(input)
    }
}

impl<'pa> Absorb<'pa> for u64 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::be_u64(input)
    }
}

impl<'pa> Absorb<'pa> for f32 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::be_f32(input)
    }
}

impl<'pa> Absorb<'pa> for f64 {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        number::complete::be_f64(input)
    }
}

impl<'pa, A, B> Absorb<'pa> for (A, B)
where
    A: Absorb<'pa>,
    B: Absorb<'pa>,
{
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        sequence::pair(A::absorb::<L>, B::absorb::<L>)(input)
    }
}

impl<'pa, const S: usize> Absorb<'pa> for [u8; S] {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(complete::take(S), |input| {
            <[u8; S]>::try_from(input).expect("impossible to fail here")
        })(input)
    }
}

impl<'pa, const S: usize> Absorb<'pa> for &'pa [u8; S] {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(complete::take(S), |input| {
            <&'pa [u8; S]>::try_from(input).expect("impossible to fail here")
        })(input)
    }
}

impl<'pa, T> Absorb<'pa> for Option<T>
where
    T: Absorb<'pa> + Clone,
{
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        branch::alt((
            sequence::preceded(
                complete::tag(0x00_u8.to_be_bytes()),
                combinator::success(None),
            ),
            sequence::preceded(
                complete::tag(0xff_u8.to_be_bytes()),
                combinator::map(T::absorb::<L>, Some),
            ),
        ))(input)
    }
}

impl<'pa, T, E> Absorb<'pa> for Result<T, E>
where
    T: Absorb<'pa>,
    E: Absorb<'pa>,
{
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        branch::alt((
            sequence::preceded(
                complete::tag(0xff_u8.to_be_bytes()),
                combinator::map(T::absorb::<L>, Ok),
            ),
            sequence::preceded(
                complete::tag(0xfe_u8.to_be_bytes()),
                combinator::map(E::absorb::<L>, Err),
            ),
        ))(input)
    }
}

impl<'pa, T> Absorb<'pa> for Box<T>
where
    T: Absorb<'pa>,
{
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(T::absorb::<L>, Box::new)(input)
    }
}
