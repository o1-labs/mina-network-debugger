// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

pub trait RadiationBuffer {
    fn pos(&self) -> usize;
    fn write_at(&mut self, pos: usize, data: &[u8]);
}

impl RadiationBuffer for Vec<u8> {
    fn pos(&self) -> usize {
        self.len()
    }

    fn write_at(&mut self, pos: usize, data: &[u8]) {
        self[pos..(pos + data.len())].clone_from_slice(data);
    }
}

pub trait Emit<W>
where
    W: for<'a> Extend<&'a u8>,
{
    #[must_use]
    fn chain(&self, mut buffer: W) -> W {
        self.emit(&mut buffer);
        buffer
    }

    fn emit(&self, buffer: &mut W);
}

#[allow(dead_code)]
pub struct CsBuffer<const SIZE: usize> {
    pos: usize,
    bytes: [u8; SIZE],
}

impl<const SIZE: usize> RadiationBuffer for CsBuffer<SIZE> {
    fn pos(&self) -> usize {
        self.pos
    }

    fn write_at(&mut self, pos: usize, data: &[u8]) {
        self.bytes[pos..(pos + data.len())].clone_from_slice(data);
    }
}

#[allow(dead_code)]
trait SpecExtend<'a, I>
where
    I: IntoIterator<Item = &'a u8>,
{
    fn spec_extend(&mut self, iter: I);
}

#[cfg(not(feature = "nightly"))]
impl<'a, I, const SIZE: usize> SpecExtend<'a, I> for CsBuffer<SIZE>
where
    I: Iterator<Item = &'a u8>,
{
    fn spec_extend(&mut self, iter: I) {
        for b in iter {
            self.bytes[self.pos] = *b;
            self.pos += 1;
        }
    }
}

#[cfg(feature = "nightly")]
impl<'a, I, const SIZE: usize> SpecExtend<'a, I> for CsBuffer<SIZE>
where
    I: Iterator<Item = &'a u8>,
{
    default fn spec_extend(&mut self, iter: I) {
        for b in iter {
            self.bytes[self.pos] = *b;
            self.pos += 1;
        }
    }
}

#[cfg(feature = "nightly")]
impl<'a, const SIZE: usize> SpecExtend<'a, <&'a [u8] as IntoIterator>::IntoIter>
    for CsBuffer<SIZE>
{
    fn spec_extend(&mut self, iter: <&'a [u8] as IntoIterator>::IntoIter) {
        let new_pos = self.pos + iter.as_slice().len();
        self.bytes[self.pos..new_pos].clone_from_slice(iter.as_slice());
        self.pos = new_pos;
    }
}

impl<'a, const SIZE: usize> Extend<&'a u8> for CsBuffer<SIZE> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a u8>,
    {
        SpecExtend::<T::IntoIter>::spec_extend(self, iter.into_iter());
    }
}
