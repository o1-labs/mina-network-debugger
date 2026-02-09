use core::{ptr, ffi::c_void};

use super::helpers;

#[allow(dead_code)]
pub struct ArrayPerCpu<const VALUE_SIZE: usize, const MAX_ENTRIES: usize> {
    ty__: *mut [u32; 6],
    key_size: *mut [u32; 4],
    value_size: *mut [u32; VALUE_SIZE],
    max_entries: *mut [u32; MAX_ENTRIES],
}

impl<const V: usize, const M: usize> ArrayPerCpu<V, M> {
    pub const fn new() -> Self {
        ArrayPerCpu {
            ty__: ptr::null_mut(),
            key_size: ptr::null_mut(),
            value_size: ptr::null_mut(),
            max_entries: ptr::null_mut(),
        }
    }
}

pub struct ArrayPerCpuRef<const V: usize> {
    inner: usize,
}

impl<const V: usize> ArrayPerCpuRef<V> {
    #[inline(always)]
    pub fn new<const M: usize>(inner: &mut ArrayPerCpu<V, M>) -> Self {
        ArrayPerCpuRef {
            inner: inner as *mut _ as usize,
        }
    }

    #[inline(always)]
    fn inner(&self) -> *mut c_void {
        self.inner as *mut _
    }

    #[inline(always)]
    pub fn get(&self, index: u32) -> Option<&[u8; V]> {
        let key = index.to_ne_bytes();
        unsafe {
            let v = helpers::map_lookup_elem(self.inner(), key.as_ptr() as _);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const [u8; V]))
            }
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: u32) -> Option<&mut [u8; V]> {
        let key = index.to_ne_bytes();
        unsafe {
            let v = helpers::map_lookup_elem(self.inner(), key.as_ptr() as _);
            if v.is_null() {
                None
            } else {
                Some(&mut *(v as *mut [u8; V]))
            }
        }
    }
}

#[allow(dead_code)]
pub struct HashMap<const KEY_SIZE: usize, const VALUE_SIZE: usize, const MAX_ENTRIES: usize> {
    ty__: *mut [u32; 1],
    key_size: *mut [u32; KEY_SIZE],
    value_size: *mut [u32; VALUE_SIZE],
    max_entries: *mut [u32; MAX_ENTRIES],
}

impl<const K: usize, const V: usize, const M: usize> HashMap<K, V, M> {
    pub const fn new() -> Self {
        HashMap {
            ty__: ptr::null_mut(),
            key_size: ptr::null_mut(),
            value_size: ptr::null_mut(),
            max_entries: ptr::null_mut(),
        }
    }
}

pub struct HashMapRef<const K: usize, const V: usize> {
    inner: usize,
}

impl<const K: usize, const V: usize> HashMapRef<K, V> {
    #[inline(always)]
    pub fn new<const M: usize>(inner: &mut HashMap<K, V, M>) -> Self {
        HashMapRef {
            inner: inner as *mut _ as usize,
        }
    }

    #[inline(always)]
    fn inner(&self) -> *mut c_void {
        self.inner as *mut _
    }

    #[inline(always)]
    pub fn insert_unsafe<T>(&mut self, key: [u8; K], value: T) -> Result<(), i32> {
        let value = &value as *const T as _;
        let key = &key as *const [u8] as *const u8 as *const _;
        let c = unsafe { helpers::map_update_elem(self.inner(), key, value, 0) };
        if c >= 0 {
            Ok(())
        } else {
            Err(c as _)
        }
    }

    #[inline(always)]
    pub fn get_unsafe<T>(&self, key: &[u8; K]) -> Option<&T> {
        let key = key as *const [u8] as *const u8 as *const _;
        unsafe {
            let v = helpers::map_lookup_elem(self.inner(), key);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const T))
            }
        }
    }

    #[inline(always)]
    pub fn get_mut_unsafe<T>(&self, key: &[u8; K]) -> Option<&mut T> {
        let key = key as *const [u8] as *const u8 as *const _;
        unsafe {
            let v = helpers::map_lookup_elem(self.inner(), key);
            if v.is_null() {
                None
            } else {
                Some(&mut *(v as *mut T))
            }
        }
    }

    #[inline(always)]
    pub fn insert(&mut self, key: [u8; K], value: [u8; V]) -> Result<(), i32> {
        let c = unsafe {
            helpers::map_update_elem(self.inner(), key.as_ptr() as _, value.as_ptr() as _, 0)
        };
        if c >= 0 {
            Ok(())
        } else {
            Err(c as _)
        }
    }

    #[inline(always)]
    pub fn get(&self, key: &[u8; K]) -> Option<&[u8; V]> {
        unsafe {
            let v = helpers::map_lookup_elem(self.inner(), key.as_ptr() as _);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const [u8; V]))
            }
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, key: &[u8; K]) -> Option<&mut [u8; V]> {
        unsafe {
            let v = helpers::map_lookup_elem(self.inner(), key.as_ptr() as _);
            if v.is_null() {
                None
            } else {
                Some(&mut *(v as *mut [u8; V]))
            }
        }
    }

    #[inline(always)]
    pub fn remove(&mut self, key: &[u8; K]) -> Result<Option<[u8; V]>, i32> {
        match self.get(key) {
            Some(v) => {
                let c = unsafe { helpers::map_delete_elem(self.inner(), key.as_ptr() as _) };
                if c >= 0 {
                    Ok(Some(*v))
                } else {
                    Err(c as _)
                }
            },
            None => Ok(None),
        }
    }

    #[inline(always)]
    pub fn remove_unsafe<T>(&mut self, key: &[u8; K]) -> Result<Option<T>, i32>
    where
        T: Copy,
    {
        match self.get_unsafe(key) {
            Some(v) => {
                let c = unsafe { helpers::map_delete_elem(self.inner(), key.as_ptr() as _) };
                if c >= 0 {
                    Ok(Some(*v))
                } else {
                    Err(c as _)
                }
            },
            None => Ok(None),
        }
    }
}
