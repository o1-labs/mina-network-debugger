use super::{skeleton::MapRef, kind};

pub struct ArrayPerCpuRef<const V: usize>(MapRef);

impl<const V: usize> kind::AppItem for ArrayPerCpuRef<V> {
    const MAP: usize = 1;
    const PROG: usize = 0;

    fn named(name: &'static str) -> Self {
        ArrayPerCpuRef(MapRef::named(name))
    }

    fn kind_mut(&mut self) -> kind::AppItemKindMut<'_> {
        kind::AppItemKindMut::Map(&mut self.0)
    }

    fn kind(&self) -> kind::AppItemKind<'_> {
        kind::AppItemKind::Map(&self.0)
    }
}

#[derive(Clone, Copy)]
pub struct HashMapRef<const K: usize, const V: usize>(MapRef);

impl<const K: usize, const V: usize> kind::AppItem for HashMapRef<K, V> {
    const MAP: usize = 1;
    const PROG: usize = 0;

    fn named(name: &'static str) -> Self {
        HashMapRef(MapRef::named(name))
    }

    fn kind_mut(&mut self) -> kind::AppItemKindMut<'_> {
        kind::AppItemKindMut::Map(&mut self.0)
    }

    fn kind(&self) -> kind::AppItemKind<'_> {
        kind::AppItemKind::Map(&self.0)
    }
}

impl<const K: usize, const V: usize> HashMapRef<K, V> {
    pub fn insert(&mut self, key: [u8; K], value: [u8; V]) -> Result<(), i32> {
        let c = unsafe {
            libbpf_sys::bpf_map_update_elem(self.0.fd(), key.as_ptr() as _, value.as_ptr() as _, 0)
        };
        if c >= 0 {
            Ok(())
        } else {
            Err(c as _)
        }
    }

    pub fn get(&self, key: &[u8; K]) -> Option<[u8; V]> {
        let mut value = [0; V];
        let c = unsafe {
            libbpf_sys::bpf_map_lookup_elem(self.0.fd(), key.as_ptr() as _, value.as_mut_ptr() as _)
        };
        if c >= 0 {
            Some(value)
        } else {
            None
        }
    }

    pub fn remove(&self, key: &[u8; K]) -> Result<(), i32> {
        let c = unsafe { libbpf_sys::bpf_map_delete_elem(self.0.fd(), key.as_ptr() as _) };
        if c >= 0 {
            Ok(())
        } else {
            Err(c)
        }
    }
}
