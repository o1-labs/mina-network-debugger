use core::ffi::c_void;

#[repr(i32)]
pub enum Action {
    Aborted = 0,
    Drop = 1,
    Pass = 2,
    Tx = 3,
    Redirect = 4,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
}

impl Context {
    #[inline(always)]
    pub unsafe fn cast(ctx: *const c_void) -> Self {
        *(ctx as *const Context)
    }
}
