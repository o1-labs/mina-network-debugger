use std::{time::Duration, mem::ManuallyDrop};
use super::{skeleton::MapRef, kind};

pub struct RingBufferRef(MapRef);

impl kind::AppItem for RingBufferRef {
    const MAP: usize = 1;
    const PROG: usize = 0;

    fn named(name: &'static str) -> Self {
        RingBufferRef(MapRef::named(name))
    }

    fn kind_mut(&mut self) -> kind::AppItemKindMut<'_> {
        kind::AppItemKindMut::Map(&mut self.0)
    }

    fn kind(&self) -> kind::AppItemKind<'_> {
        kind::AppItemKind::Map(&self.0)
    }
}

pub struct RingBufferRegistry {
    inner: *mut libbpf_sys::ring_buffer,
}

impl Default for RingBufferRegistry {
    fn default() -> Self {
        use std::ptr;

        RingBufferRegistry {
            inner: ptr::null_mut(),
        }
    }
}

impl Drop for RingBufferRegistry {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { libbpf_sys::ring_buffer__free(self.inner) };
        }
    }
}

impl RingBufferRegistry {
    unsafe extern "C" fn cb<F>(ctx: *mut cty::c_void, data: *mut cty::c_void, size: u64) -> i32
    where
        F: FnMut(ManuallyDrop<Box<[u8]>>),
    {
        use std::slice;

        let s = slice::from_raw_parts_mut(data as *mut u8, size as _);
        (*(ctx as *mut F))(ManuallyDrop::new(Box::from_raw(s as *mut [u8])));
        0
    }

    pub fn add<F>(&mut self, rb: &RingBufferRef, cb: &mut F) -> Result<(), i32>
    where
        F: FnMut(ManuallyDrop<Box<[u8]>>),
    {
        self.add_fd(rb.0.fd(), cb)
    }

    pub fn add_fd<F>(&mut self, map_fd: i32, cb: &mut F) -> Result<(), i32>
    where
        F: FnMut(ManuallyDrop<Box<[u8]>>),
    {
        use std::ptr;

        let ctx = cb as *mut F as *mut _;
        if self.inner.is_null() {
            self.inner = unsafe {
                libbpf_sys::ring_buffer__new(map_fd, Some(Self::cb::<F>), ctx, ptr::null_mut())
            };
            Ok(())
        } else {
            let c = unsafe {
                libbpf_sys::ring_buffer__add(self.inner, map_fd, Some(Self::cb::<F>), ctx)
            };
            if c != 0 {
                Err(c)
            } else {
                Ok(())
            }
        }
    }

    pub fn poll(&self, timeout: Duration) -> Result<usize, i32> {
        if self.inner.is_null() {
            return Ok(0);
        }

        let c = unsafe { libbpf_sys::ring_buffer__poll(self.inner, timeout.as_millis() as i32) };

        if c < 0 {
            Err(c)
        } else {
            Ok(c as _)
        }
    }

    pub fn consume(&self) -> Result<usize, i32> {
        if self.inner.is_null() {
            return Ok(0);
        }

        let c = unsafe { libbpf_sys::ring_buffer__consume(self.inner) };

        if c < 0 {
            Err(c)
        } else {
            Ok(c as _)
        }
    }

    pub fn epoll_fd(&self) -> Option<i32> {
        if self.inner.is_null() {
            None
        } else {
            Some(unsafe { libbpf_sys::ring_buffer__epoll_fd(self.inner) })
        }
    }
}
