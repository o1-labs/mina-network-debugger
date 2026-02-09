// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use core::marker::PhantomData;
use alloc::boxed::Box;

use super::core::Emit;

impl<T, W> Emit<W> for PhantomData<T>
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, _: &mut W) {}
}

impl<W> Emit<W> for ()
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, _: &mut W) {}
}

impl<W> Emit<W> for bool
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        if *self {
            buffer.extend(Some(&0xff));
        } else {
            buffer.extend(Some(&0x00));
        }
    }
}

impl<W> Emit<W> for i8
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(Some(&(*self as u8)));
    }
}

impl<W> Emit<W> for u8
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(Some(self));
    }
}

impl<W> Emit<W> for i16
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(&self.to_be_bytes());
    }
}

impl<W> Emit<W> for u16
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(&self.to_be_bytes());
    }
}

impl<W> Emit<W> for i32
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(&self.to_be_bytes());
    }
}

impl<W> Emit<W> for u32
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(&self.to_be_bytes());
    }
}

impl<W> Emit<W> for i64
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(&self.to_be_bytes());
    }
}

impl<W> Emit<W> for u64
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(&self.to_be_bytes());
    }
}

impl<W> Emit<W> for f32
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(&self.to_be_bytes());
    }
}

impl<W> Emit<W> for f64
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(&self.to_be_bytes());
    }
}

impl<A, B, W> Emit<W> for (A, B)
where
    A: Emit<W>,
    B: Emit<W>,
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        let (a, b) = self;
        a.emit(buffer);
        b.emit(buffer);
    }
}

impl<const S: usize, W> Emit<W> for [u8; S]
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        buffer.extend(self);
    }
}

impl<T, W> Emit<W> for Option<T>
where
    T: Emit<W>,
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        match self {
            None => false.emit(buffer),
            Some(v) => {
                true.emit(buffer);
                v.emit(buffer);
            }
        }
    }
}

impl<T, E, W> Emit<W> for Result<T, E>
where
    T: Emit<W>,
    E: Emit<W>,
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        match self {
            Ok(v) => {
                true.emit(buffer);
                v.emit(buffer);
            }
            Err(v) => {
                false.emit(buffer);
                v.emit(buffer);
            }
        }
    }
}

impl<T, W> Emit<W> for Box<T>
where
    T: Emit<W>,
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        (**self).emit(buffer);
    }
}
