use core::ffi::c_void;

use super::helpers;

// TODO: cross-platform
pub struct Context {
    inner: *const c_void,
}

impl Context {
    #[inline(always)]
    pub unsafe fn cast(ctx: *const c_void) -> Self {
        Context { inner: ctx }
    }

    #[inline(always)]
    pub fn read_here<T>(&self, offset: usize) -> T {
        unsafe { (self.inner.add(offset) as *const T).read() }
    }

    #[inline(always)]
    pub fn read_into(&self, offset: usize, slice: &mut [u8]) {
        unsafe {
            helpers::probe_read_kernel(
                slice.as_mut_ptr() as _,
                slice.len() as u32,
                self.inner.add(offset) as *const _,
            );
        }
    }

    #[inline(always)]
    pub fn read<T>(&self, offset: usize) -> T {
        use core::mem;

        let mut value = mem::MaybeUninit::uninit();
        unsafe {
            helpers::probe_read_kernel(
                value.as_mut_ptr() as _,
                mem::size_of::<T>() as u32,
                self.inner.add(offset) as *const _,
            );
            value.assume_init()
        }
    }

    #[inline(always)]
    pub fn get_user_stack(&self, buf: &mut [u8]) -> Result<usize, i32> {
        let c = unsafe {
            helpers::get_stack(self.inner as _, buf.as_mut_ptr() as _, buf.len() as _, 256)
        };
        if c < 0 {
            Err(c as _)
        } else {
            Ok(c as _)
        }
    }
}
