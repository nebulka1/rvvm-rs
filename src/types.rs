use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceHandle<T> {
    pub(crate) id: i32,
    phantom: PhantomData<T>,
}

impl<T> DeviceHandle<T> {
    pub(crate) const fn new(id: i32) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }
}

crate::__ts_handler! {
    name = TypeHandler,
    raw = unsafe extern "C" fn(dev: *mut rvvm_sys::rvvm_mmio_dev_t)
}

crate::__ts_handler! {
    name = RemoveHandler,
    raw = RawTypeHandler
}

crate::__ts_handler! {
    name = RwHandler,
    raw = unsafe extern "C" fn(
        dev: *mut rvvm_sys::rvvm_mmio_dev_t,
        dest: *mut std::ffi::c_void,
        offset: usize,
        size: u8
    ) -> bool
}
