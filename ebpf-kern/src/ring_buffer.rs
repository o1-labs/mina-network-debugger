use core::{
    ptr,
    ffi::{c_void, c_int},
};

use super::helpers;

#[allow(dead_code)]
pub struct RingBuffer<const MAX_ENTRIES: usize> {
    ty__: *mut [u32; 27],
    max_entries: *mut [u32; MAX_ENTRIES],
}

impl<const MAX_ENTRIES: usize> RingBuffer<MAX_ENTRIES> {
    pub const fn new() -> Self {
        RingBuffer {
            ty__: ptr::null_mut(),
            max_entries: ptr::null_mut(),
        }
    }
}

pub struct RingBufferRef {
    inner: usize,
}

pub struct RingBufferData {
    inner: &'static mut [u8],
}

impl AsMut<[u8]> for RingBufferData {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner
    }
}

impl AsRef<[u8]> for RingBufferData {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.inner
    }
}

impl RingBufferRef {
    pub fn new<const MAX_ENTRIES: usize>(inner: &mut RingBuffer<MAX_ENTRIES>) -> Self {
        RingBufferRef {
            inner: inner as *mut _ as usize,
        }
    }

    #[inline(always)]
    fn inner(&mut self) -> *mut c_void {
        self.inner as *mut _
    }

    #[inline(always)]
    pub fn output(&mut self, data: &[u8]) -> Result<(), i32> {
        let code = unsafe {
            helpers::ringbuf_output(self.inner(), data.as_ptr() as *mut _, data.len() as _, 0)
        };
        if code == 0 {
            Ok(())
        } else {
            Err(code as _)
        }
    }

    #[inline(always)]
    pub fn reserve(&mut self, size: usize) -> Result<RingBufferData, c_int> {
        use core::slice;

        let data_ptr = unsafe { helpers::ringbuf_reserve(self.inner(), size as _, 0) };
        if data_ptr.is_null() {
            Err(-90)
        } else {
            Ok(RingBufferData {
                inner: unsafe { slice::from_raw_parts_mut(data_ptr as *mut _, size) },
                // it is really unsafe, the code become invalid if compiler change
                // the metadata format
                // unsafe { mem::transmute((data_ptr, size)) },
            })
        }
    }
}

impl RingBufferData {
    #[inline(always)]
    pub fn submit(self) {
        unsafe { helpers::ringbuf_submit(self.inner.as_mut_ptr() as *mut _, 0) };
    }

    #[inline(always)]
    pub fn discard(self) {
        unsafe { helpers::ringbuf_discard(self.inner.as_mut_ptr() as *mut _, 0) };
    }
}
