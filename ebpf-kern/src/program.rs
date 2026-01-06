use core::{
    marker::PhantomData,
    ffi::{c_void, c_int},
};

pub struct ProgRef {
    _private: PhantomData<fn(ctx: *const c_void) -> c_int>,
}

impl ProgRef {
    #[inline(always)]
    pub fn new(function: fn(ctx: *const c_void) -> c_int) -> Self {
        let _ = function;
        ProgRef {
            _private: PhantomData,
        }
    }
}
