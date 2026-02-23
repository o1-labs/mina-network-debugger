use core::sync::atomic::{AtomicI64, AtomicU64, AtomicUsize, Ordering};

use super::core::Emit;

impl<W> Emit<W> for AtomicUsize
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        self.load(Ordering::Relaxed).emit(buffer)
    }
}

impl<W> Emit<W> for AtomicU64
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        self.load(Ordering::Relaxed).emit(buffer)
    }
}

impl<W> Emit<W> for AtomicI64
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        self.load(Ordering::Relaxed).emit(buffer)
    }
}
