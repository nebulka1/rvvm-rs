use std::{
    ffi::CString,
    marker::PhantomData,
    ptr,
};

use rvvm_sys::{
    rvvm_mmio_dev_t,
    rvvm_mmio_type_t,
};

use super::mmio::Device;

/// Type that describes the device's type and
/// its lifetime-related things, like remove/update/reset
/// callbacks
#[repr(transparent)]
pub struct DeviceType<T: Send + Sync> {
    _inner: rvvm_mmio_type_t,
    _phantom: PhantomData<T>,
}

impl<T: Send + Sync> DeviceType<T> {
    /// Simple drop glue needed for the `type->remove`
    /// function
    ///
    /// # Safety
    ///
    /// This method is unsafe due to lack of pointer
    /// validity checks, internally it calls to the
    /// `std::ptr::drop_in_place` with the `dev as *mut
    /// Device<T>` argument.
    pub unsafe extern "C" fn drop_glue(dev: *mut rvvm_mmio_dev_t) {
        ptr::drop_in_place(dev as *mut Device<T>);
    }

    /// Create type-safe wrapper `DeviceType<T>` around the
    /// underlying `rvvm_mmio_type_t`
    ///
    /// # Panics
    ///
    /// Panics if `name` contains nul-byte character
    pub fn new(name: impl AsRef<str>) -> Self {
        let name = CString::new(name.as_ref())
            .expect("Name contains nul-byte character");
        let type_ = rvvm_mmio_type_t {
            name: name.as_ptr(),

            // TODO: Implement custom handlers
            remove: Some(Self::drop_glue),
            update: None,
            reset: None,
        };

        std::mem::forget(name);

        Self {
            _inner: type_,
            _phantom: PhantomData,
        }
    }
}
