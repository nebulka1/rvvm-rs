use rvvm_sys::{
    rvvm_mmio_dev_t,
    rvvm_mmio_type_t,
};

type Handler = Option<unsafe extern "C" fn(dev: *mut rvvm_mmio_dev_t)>;
pub struct MmioType {
    pub(crate) inner: rvvm_mmio_dev_t,
}

impl MmioType {
    fn validate(&self) -> bool {
        todo!()
    }
}
