use std::{
    ffi::{
        CStr,
        CString,
    },
    marker::PhantomData,
    ptr,
};

use rvvm_sys::{
    rvvm_mmio_dev_t,
    rvvm_mmio_type_t,
};

use super::mmio::Device;

#[repr(transparent)]
pub struct DeviceType<T: 'static> {
    pub(crate) inner: rvvm_mmio_type_t,
    _phantom: PhantomData<T>,
}

impl<T: 'static> DeviceType<T> {
    unsafe extern "C" fn drop_glue(dev: *mut rvvm_mmio_dev_t) {
        ptr::drop_in_place(dev as *mut Device<T>);
    }

    pub fn new(name: &'static CStr) -> Self {
        let type_ = rvvm_mmio_type_t {
            name: name.as_ptr(),

            // TODO: Implement custom handlers
            remove: Some(Self::drop_glue),
            update: None,
            reset: None,
        };

        Self {
            inner: type_,
            _phantom: PhantomData,
        }
    }
}
