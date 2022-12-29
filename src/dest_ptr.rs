use std::{
    ffi::c_void,
    marker::PhantomData,
};

pub struct DestPtr<'a> {
    pub(crate) ptr: *mut c_void,
    pub(crate) size: u8,
    pub(crate) offset: usize,

    pub(crate) phantom: PhantomData<&'a ()>,
}

impl<'a> DestPtr<'a> {
    /// # Safety
    pub unsafe fn new(ptr: *mut c_void, size: u8, offset: usize) -> Self {
        Self {
            ptr,
            size,
            offset,
            phantom: PhantomData,
        }
    }
}
