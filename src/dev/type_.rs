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
use crate::prelude::{
    RemoveHandler,
    TypeHandler,
};

/// Type that describes the device's type and
/// its lifetime-related things, like remove/update/reset
/// callbacks
#[repr(transparent)]
pub struct DeviceType<T: Send + Sync> {
    inner: rvvm_mmio_type_t,
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
    /// underlying `rvvm_mmio_type_t` with custom handlers.
    ///
    /// # Panics
    ///
    /// Panics if `name` contains nul-byte character
    pub fn custom(
        name: impl AsRef<str>,
        remove: RemoveHandler<T>,
        update: TypeHandler<T>,
        reset: TypeHandler<T>,
    ) -> Self {
        let name = CString::new(name.as_ref())
            .expect("Name contains nul-byte character");
        let type_ = rvvm_mmio_type_t {
            name: name.into_raw(),

            remove: if remove.inner.is_none() {
                Some(Self::drop_glue)
            } else {
                remove.inner
            },
            update: update.inner,
            reset: reset.inner,
        };

        Self {
            inner: type_,
            _phantom: PhantomData,
        }
    }

    /// Same as `DeviceType::custom`, but leaves update and
    /// reset handlers empty. `remove` handler is set always
    /// and it contains `DeviceType::drop_glue`.
    ///
    /// For more detailed information and **panics** see
    /// `DeviceType::custom`
    pub fn new(name: impl AsRef<str>) -> Self {
        Self::custom(
            name,
            // SAFETY: Self::drop_glue is internally
            // implemented fn
            unsafe { RemoveHandler::new(Self::drop_glue) },
            TypeHandler::none(),
            TypeHandler::none(),
        )
    }
}

impl<T: Send + Sync> Drop for DeviceType<T> {
    fn drop(&mut self) {
        union U {
            i: *const i8,
            o: *mut i8,
        }
        // SAFETY: safe, since `self.inner.name` is previously
        // obtained through the `CString::into_raw`
        let _ = unsafe { CString::from_raw(U { i: self.inner.name }.o) };
    }
}