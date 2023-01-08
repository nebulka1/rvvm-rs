use std::{
    fmt::Debug,
    marker::PhantomData,
};

use rvvm_sys::rvvm_mmio_dev_t;

#[repr(transparent)]
pub struct RawDevice<T: Send + Sync> {
    inner: rvvm_mmio_dev_t,
    phantom: PhantomData<T>,
}

impl<T: Send + Sync> RawDevice<T> {
    pub fn data(&self) -> &T {
        unsafe { &*(self.inner.data as *const () as *const T) }
    }

    pub fn data_mut(&mut self) -> &mut T {
        unsafe { &mut *(self.inner.data as *mut () as *mut T) }
    }

    /// Create `UntypedDevice` from the underlying ffi
    /// `rvvm_mmio_dev_t` type
    ///
    /// # Safety
    ///
    /// This function is unsafe due to internal usage of it
    pub const unsafe fn new(inner: rvvm_mmio_dev_t) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<T: Send + Sync> Drop for RawDevice<T> {
    fn drop(&mut self) {
        let _ =
            unsafe { Box::from_raw(self.inner.data as *mut () as *mut T) };
    }
}

pub struct DeviceHandle<T> {
    pub(crate) inner: i32,
    phantom: PhantomData<T>,
}

impl<T> DeviceHandle<T> {
    /// Create handle that points to device with data `T`
    ///
    /// # Safety
    ///
    /// Unsafe due to possibility of violate the type safety
    pub const unsafe fn from_raw(raw: i32) -> Self {
        Self {
            inner: raw,
            phantom: PhantomData,
        }
    }

    pub const fn raw(self) -> i32 {
        self.inner
    }
}

impl<T> Debug for DeviceHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DeviceHandle")
            .field(&self.inner)
            .finish()
    }
}

impl<T> Copy for DeviceHandle<T> {}
impl<T> Clone for DeviceHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}
