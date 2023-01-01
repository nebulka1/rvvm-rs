#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceHandle(pub(crate) i32);

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
