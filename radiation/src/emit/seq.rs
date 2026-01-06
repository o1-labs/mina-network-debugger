// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use alloc::{string::String, boxed::Box, vec::Vec};

use super::{
    core::{RadiationBuffer, Emit},
    DynSized, Collection,
};

impl<W> Emit<W> for usize
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        (*self as u32).emit(buffer);
    }
}

impl<W> Emit<W> for str
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        let bytes = self.as_bytes();
        bytes.len().emit(buffer);
        buffer.extend(bytes);
    }
}

impl<W> Emit<W> for String
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        (**self).emit(buffer);
    }
}

impl<T, W> Emit<W> for DynSized<T>
where
    T: Emit<W>,
    W: for<'a> Extend<&'a u8> + RadiationBuffer,
{
    fn emit(&self, buffer: &mut W) {
        let pos = buffer.pos();
        0usize.emit(buffer);
        self.0.emit(buffer);
        let len = buffer.pos() - pos - 4;
        buffer.write_at(pos, &(len as u32).to_be_bytes());
    }
}

#[cfg(not(feature = "nightly"))]
impl<T, W> Emit<W> for Box<[T]>
where
    T: Emit<W>,
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        for i in self.iter() {
            i.emit(buffer);
        }
    }
}

#[cfg(feature = "nightly")]
impl<T, W> Emit<W> for Box<[T]>
where
    T: Emit<W>,
    W: for<'a> Extend<&'a u8>,
{
    default fn emit(&self, buffer: &mut W) {
        for i in self.iter() {
            i.emit(buffer);
        }
    }
}

// TODO: proper specialization
#[cfg(feature = "nightly")]
impl Emit<Vec<u8>> for Box<[u8]> {
    fn emit(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(self);
    }
}

#[cfg(not(feature = "nightly"))]
impl<T, W> Emit<W> for Vec<T>
where
    T: Emit<W>,
    W: for<'a> Extend<&'a u8> + RadiationBuffer,
{
    fn emit(&self, buffer: &mut W) {
        let pos = buffer.pos();
        0usize.emit(buffer);
        for v in self {
            v.emit(buffer);
        }
        let len = buffer.pos() - pos - 4;
        buffer.write_at(pos, &(len as u32).to_be_bytes());
    }
}

#[cfg(feature = "nightly")]
impl<T, W> Emit<W> for Vec<T>
where
    T: Emit<W>,
    W: for<'a> Extend<&'a u8> + RadiationBuffer,
{
    default fn emit(&self, buffer: &mut W) {
        let pos = buffer.pos();
        0usize.emit(buffer);
        for v in self {
            v.emit(buffer);
        }
        let len = buffer.pos() - pos - 4;
        buffer.write_at(pos, &(len as u32).to_be_bytes());
    }
}

// TODO: proper specialization
#[cfg(feature = "nightly")]
impl Emit<Vec<u8>> for Vec<u8> {
    fn emit(&self, buffer: &mut Vec<u8>) {
        let pos = buffer.pos();
        0usize.emit(buffer);
        buffer.extend_from_slice(self);
        let len = buffer.pos() - pos - 4;
        buffer.write_at(pos, &(len as u32).to_be_bytes());
    }
}

impl<C, W> Emit<W> for Collection<C>
where
    C: IntoIterator + Clone,
    C::Item: Emit<W>,
    W: for<'a> Extend<&'a u8> + RadiationBuffer,
{
    fn emit(&self, buffer: &mut W) {
        for v in C::clone(&self.0) {
            v.emit(buffer);
        }
    }
}
