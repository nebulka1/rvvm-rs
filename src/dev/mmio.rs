use std::{
    ffi::c_void,
    ops::RangeInclusive,
    ptr,
    ptr::NonNull,
};

use rvvm_sys::rvvm_mmio_dev_t;

use crate::utils::cold_path;

type RwCallback = unsafe extern "C" fn(
    dev: *mut rvvm_mmio_dev_t,
    dest: *mut c_void,
    offset: usize,
    size: u8,
) -> bool;

// FIXME: Make Option<RwCallback> available
pub struct MmioHandler(pub(crate) RwCallback);

/// Descriptor of an MMIO device
pub struct MmioDeviceDescriptor {
    pub(crate) inner: NonNull<rvvm_mmio_dev_t>,

    /// HACK: since MmioDeviceDescriptor can be created
    /// either by Rust or C, this is used to prevent
    /// memory corruption.
    needs_free: bool,
}

impl MmioDeviceDescriptor {
    pub fn new(
        address: u64,
        size: usize,
        op_size_range: RangeInclusive<u8>,
    ) -> Self {
        let allocated = Box::new(rvvm_mmio_dev_t {
            addr: address,
            size,
            data: ptr::null_mut(),
            machine: ptr::null_mut(),
            type_: ptr::null_mut(),
            read: None,
            write: None,
            min_op_size: *op_size_range.start(),
            max_op_size: *op_size_range.end(),
        });
        Self {
            inner: NonNull::new(Box::into_raw(allocated))
                .expect("FIXME: Failed to create box pointer"),
            needs_free: true,
        }
    }

    pub(crate) fn from_raw_mmio(ptr: *mut rvvm_mmio_dev_t) -> Self {
        Self {
            inner: NonNull::new(ptr)
                .expect("Got nullptr inside from_raw_mmio"),
            needs_free: false,
        }
    }
}

impl Drop for MmioDeviceDescriptor {
    fn drop(&mut self) {
        if self.needs_free {
            cold_path();

            // SAFETY: this is safe since data is marked needs free on
            // the Rust side see doc-comment on the
            // `needs_free` field
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
            self.inner = NonNull::dangling();
            // self.inner is no longer valid
        }
    }
}

//

impl MmioHandler {
    pub const fn empty() -> Self {
        // FIXME: Implement
        unimplemented!()
    }

    pub const fn new(handler: RwCallback) -> Self {
        Self(handler)
    }
}

impl From<RwCallback> for MmioHandler {
    fn from(value: RwCallback) -> Self {
        Self::new(value)
    }
}
