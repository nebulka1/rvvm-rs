use rvvm_sys::rvvm_mmio_handle_t;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceHandle(pub(crate) rvvm_mmio_handle_t);
