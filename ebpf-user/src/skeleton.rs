use std::ptr;
use super::kind;

#[derive(Clone, Copy)]
pub struct MapRef {
    name: &'static str,
    inner: *mut libbpf_sys::bpf_map,
}

impl kind::AppItem for MapRef {
    const MAP: usize = 1;
    const PROG: usize = 0;

    fn named(name: &'static str) -> Self {
        MapRef {
            name,
            inner: ptr::null_mut(),
        }
    }

    fn kind_mut(&mut self) -> kind::AppItemKindMut<'_> {
        kind::AppItemKindMut::Map(self)
    }

    fn kind(&self) -> kind::AppItemKind<'_> {
        kind::AppItemKind::Map(self)
    }
}

impl MapRef {
    pub fn fd(&self) -> i32 {
        unsafe { libbpf_sys::bpf_map__fd(self.inner) }
    }
}

pub struct ProgRef {
    name: &'static str,
    prog: *mut libbpf_sys::bpf_program,
    link: *mut libbpf_sys::bpf_link,
}

impl kind::AppItem for ProgRef {
    const MAP: usize = 0;
    const PROG: usize = 1;

    fn named(name: &'static str) -> Self {
        ProgRef {
            name,
            prog: ptr::null_mut(),
            link: ptr::null_mut(),
        }
    }

    fn kind_mut(&mut self) -> kind::AppItemKindMut<'_> {
        kind::AppItemKindMut::Prog(self)
    }

    fn kind(&self) -> kind::AppItemKind<'_> {
        kind::AppItemKind::Prog(self)
    }
}

pub struct Skeleton<App>
where
    App: BpfApp,
{
    inner: libbpf_sys::bpf_object_skeleton,
    obj: Box<*mut libbpf_sys::bpf_object>,
    pub app: Box<App>,
}

pub struct SkeletonEmpty {
    inner: libbpf_sys::bpf_object_skeleton,
    _obj: Box<*mut libbpf_sys::bpf_object>,
}

pub trait BpfApp {
    const MAP_CNT: usize;
    const PROG_CNT: usize;

    fn instance() -> Self;
    fn as_mut_map(&mut self, index: usize) -> Option<&mut MapRef>;
    fn as_mut_prog(&mut self, index: usize) -> Option<&mut ProgRef>;
}

impl<App> Skeleton<App>
where
    App: BpfApp,
{
    pub fn open(name: &'static str, code: &'static [u8]) -> Result<Self, i32> {
        use std::{
            mem, slice,
            alloc::{Layout, alloc_zeroed},
        };

        let mut app = Box::new(App::instance());
        let mut obj = Box::<*mut libbpf_sys::bpf_object>::new(ptr::null_mut());

        let map_l = Layout::array::<libbpf_sys::bpf_map_skeleton>(App::MAP_CNT).unwrap();
        let prog_l = Layout::array::<libbpf_sys::bpf_prog_skeleton>(App::PROG_CNT).unwrap();

        let mut s = libbpf_sys::bpf_object_skeleton {
            sz: mem::size_of::<libbpf_sys::bpf_object_skeleton>() as _,
            name: name.as_ptr() as _,
            data: code.as_ptr() as *const _ as *mut _,
            data_sz: code.len() as _,
            obj: obj.as_mut(),
            map_cnt: App::MAP_CNT as _,
            map_skel_sz: mem::size_of::<libbpf_sys::bpf_map_skeleton>() as _,
            maps: unsafe { alloc_zeroed(map_l) } as *mut libbpf_sys::bpf_map_skeleton,
            prog_cnt: App::PROG_CNT as _,
            prog_skel_sz: mem::size_of::<libbpf_sys::bpf_prog_skeleton>() as _,
            progs: unsafe { alloc_zeroed(prog_l) } as *mut libbpf_sys::bpf_prog_skeleton,
        };

        let maps = unsafe { slice::from_raw_parts_mut(s.maps, App::MAP_CNT) };
        for (i, s_map) in maps.iter_mut().enumerate() {
            let map = app.as_mut_map(i).unwrap();
            s_map.name = map.name.as_ptr() as _;
            s_map.map = &mut map.inner;
            s_map.mmaped = ptr::null_mut();
        }

        let progs = unsafe { slice::from_raw_parts_mut(s.progs, App::PROG_CNT) };
        for (i, s_prog) in progs.iter_mut().enumerate() {
            let prog = app.as_mut_prog(i).unwrap();
            s_prog.name = prog.name.as_ptr() as _;
            s_prog.prog = &mut prog.prog;
            s_prog.link = &mut prog.link;
        }

        let c = unsafe { libbpf_sys::bpf_object__open_skeleton(&mut s, ptr::null()) };

        if c == 0 {
            Ok(Skeleton { inner: s, obj, app })
        } else {
            Err(c)
        }
    }

    pub fn load(&mut self) -> Result<(), i32> {
        let c = unsafe { libbpf_sys::bpf_object__load_skeleton(&mut self.inner) };

        let _ = self.obj.as_mut();
        if c == 0 {
            Ok(())
        } else {
            Err(c)
        }
    }

    // TODO: create enum for flags
    pub fn attach_xdp(&mut self, prog_name: &str, if_index: i32, flags: u32) -> Result<(), i32> {
        for index in 0..App::PROG_CNT {
            let xdp = self.app.as_mut_prog(index).unwrap();
            if xdp.name.starts_with(prog_name) {
                errno::set_errno(errno::Errno(0));
                let fd = unsafe { libbpf_sys::bpf_program__fd(xdp.prog) };
                unsafe { libbpf_sys::bpf_xdp_attach(if_index as _, fd, flags, ptr::null()) };
                let error = errno::errno().0;
                if error == 0 {
                    return Ok(());
                } else {
                    return Err(error);
                }
            }
        }

        Err(1234) // no such program
    }

    pub fn attach(mut self) -> Result<(SkeletonEmpty, Box<App>), i32> {
        let c = unsafe { libbpf_sys::bpf_object__attach_skeleton(&mut self.inner) };

        if c == 0 {
            Ok((
                SkeletonEmpty {
                    inner: self.inner,
                    _obj: self.obj,
                },
                self.app,
            ))
        } else {
            Err(c)
        }
    }
}

impl Drop for SkeletonEmpty {
    fn drop(&mut self) {
        let s = Box::new(self.inner);
        unsafe { libbpf_sys::bpf_object__destroy_skeleton(Box::leak(s)) }
    }
}
