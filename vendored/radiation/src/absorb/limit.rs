// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use core::fmt;

pub struct LimitError {
    minimum: usize,
    maximum: usize,
    actual: usize,
}

impl fmt::Debug for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let LimitError {
            minimum,
            maximum,
            actual,
        } = self;
        if *actual > *maximum {
            write!(f, "falls outside the allowed maximum, {actual} > {maximum}")
        } else if *actual < *minimum {
            write!(f, "falls below the allowed minimum, {actual} < {minimum}")
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

pub trait Limit {
    type Inner: Limit;

    type Next: Limit;

    const LOWER: usize;

    const UPPER: usize;

    const DESCRIPTION: &'static str;

    fn check(size: usize) -> Result<usize, LimitError> {
        if size <= Self::UPPER && size >= Self::LOWER {
            Ok(size)
        } else {
            Err(LimitError {
                minimum: Self::LOWER,
                maximum: Self::UPPER,
                actual: size,
            })
        }
    }
}

impl Limit for () {
    type Inner = ();

    type Next = ();

    const LOWER: usize = 0;

    const UPPER: usize = usize::MAX;

    const DESCRIPTION: &'static str = "unlimited";

    fn check(size: usize) -> Result<usize, LimitError> {
        Ok(size)
    }
}

pub struct LimitDescriptor<L, N, const LOWER: usize, const UPPER: usize>(L, N);

impl<L, N, const LOWER: usize, const UPPER: usize> Limit for LimitDescriptor<L, N, LOWER, UPPER>
where
    L: Limit,
    N: Limit,
{
    type Inner = L;

    type Next = N;

    const LOWER: usize = LOWER;

    const UPPER: usize = UPPER;

    const DESCRIPTION: &'static str = "wait for issue 44580 (adt_const_params)";
}
