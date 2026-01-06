use core::{
    mem,
    ffi::{c_void, c_long},
};

#[allow(non_camel_case_types)]
type uint64_t = u64;
#[allow(non_camel_case_types)]
type uint32_t = u32;

#[inline(always)]
pub unsafe fn map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void {
    let f: unsafe extern "C" fn(*mut c_void, *const c_void) -> *mut c_void = mem::transmute(1usize);

    f(map, key)
}

#[inline(always)]
pub unsafe fn map_update_elem(
    map: *mut c_void,
    key: *const c_void,
    value: *const c_void,
    flags: uint64_t,
) -> c_long {
    let f: unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, uint64_t) -> c_long =
        mem::transmute(2usize);

    f(map, key, value, flags)
}

#[inline(always)]
pub unsafe fn map_delete_elem(map: *mut c_void, key: *const c_void) -> c_long {
    let f: unsafe extern "C" fn(*mut c_void, *const c_void) -> c_long = mem::transmute(3usize);

    f(map, key)
}

#[inline(always)]
pub unsafe fn ktime_get_ns() -> uint64_t {
    let f: unsafe extern "C" fn() -> uint64_t = mem::transmute(5usize);

    f()
}

#[inline(always)]
pub unsafe fn ktime_get_boot_ns() -> uint64_t {
    let f: unsafe extern "C" fn() -> uint64_t = mem::transmute(125usize);

    f()
}

#[inline(always)]
pub unsafe fn get_current_pid_tgid() -> uint64_t {
    let f: unsafe extern "C" fn() -> uint64_t = mem::transmute(14usize);

    f()
}

#[inline(always)]
pub unsafe fn get_current_comm(buf: *mut c_void, size_of_buf: uint32_t) -> c_long {
    let f: unsafe extern "C" fn(*mut c_void, uint32_t) -> c_long = mem::transmute(16usize);

    f(buf, size_of_buf)
}

#[inline(always)]
pub unsafe fn get_stack(
    ctx: *mut c_void,
    buf: *mut c_void,
    size: uint32_t,
    flags: uint64_t,
) -> c_long {
    let f: unsafe extern "C" fn(*mut c_void, *mut c_void, uint32_t, uint64_t) -> c_long =
        mem::transmute(67usize);

    f(ctx, buf, size, flags)
}

#[inline(always)]
pub unsafe fn probe_read_user(
    dst: *mut c_void,
    size: uint32_t,
    unsafe_ptr: *const c_void,
) -> c_long {
    let f: unsafe extern "C" fn(*mut c_void, uint32_t, *const c_void) -> c_long =
        mem::transmute(112usize);

    f(dst, size, unsafe_ptr)
}

#[inline(always)]
pub unsafe fn probe_read_kernel(
    dst: *mut c_void,
    size: uint32_t,
    unsafe_ptr: *const c_void,
) -> c_long {
    let f: unsafe extern "C" fn(*mut c_void, uint32_t, *const c_void) -> c_long =
        mem::transmute(113usize);

    f(dst, size, unsafe_ptr)
}

#[inline(always)]
pub unsafe fn probe_read_user_str(
    dst: *mut c_void,
    size: uint32_t,
    unsafe_ptr: *const c_void,
) -> c_long {
    let f: unsafe extern "C" fn(*mut c_void, uint32_t, *const c_void) -> c_long =
        mem::transmute(114usize);

    f(dst, size, unsafe_ptr)
}

#[inline(always)]
pub unsafe fn probe_read_kernel_str(
    dst: *mut c_void,
    size: uint32_t,
    unsafe_ptr: *const c_void,
) -> c_long {
    let f: unsafe extern "C" fn(*mut c_void, uint32_t, *const c_void) -> c_long =
        mem::transmute(115usize);

    f(dst, size, unsafe_ptr)
}

#[inline(always)]
pub unsafe fn ringbuf_output(
    ringbuf: *mut c_void,
    data: *mut c_void,
    size: uint64_t,
    flags: uint64_t,
) -> c_long {
    let f: unsafe extern "C" fn(*mut c_void, *mut c_void, uint64_t, uint64_t) -> c_long =
        mem::transmute(130usize);

    f(ringbuf, data, size, flags)
}

#[inline(always)]
pub unsafe fn ringbuf_reserve(
    ringbuf: *mut c_void,
    size: uint64_t,
    flags: uint64_t,
) -> *mut c_void {
    let f: unsafe extern "C" fn(*mut c_void, uint64_t, uint64_t) -> *mut c_void =
        mem::transmute(131usize);

    f(ringbuf, size, flags)
}

#[inline(always)]
pub unsafe fn ringbuf_submit(data: *mut c_void, flags: uint64_t) {
    let f: unsafe extern "C" fn(*mut c_void, uint64_t) = mem::transmute(132usize);

    f(data, flags)
}

#[inline(always)]
pub unsafe fn ringbuf_discard(data: *mut c_void, flags: uint64_t) {
    let f: unsafe extern "C" fn(*mut c_void, uint64_t) = mem::transmute(133usize);

    f(data, flags)
}
