use std::{
    mem::ManuallyDrop,
    ptr::NonNull,
};

use rvvm_sys::{
    rvvm_attach_mmio,
    rvvm_create_machine,
    rvvm_free_machine,
    rvvm_machine_t,
    RVVM_DEFAULT_MEMBASE,
};

use crate::{
    dev::{
        handle::DeviceHandle,
        mmio::DeviceDescriptorGlue,
    },
    error::{
        DeviceAttachError,
        InstanceCreateError,
    },
};

pub struct Instance {
    inner: NonNull<rvvm_machine_t>,
}

impl Instance {
    pub fn attach_device<T>(
        &mut self,
        device: DeviceDescriptorGlue<'_, T>,
    ) -> Result<DeviceHandle, DeviceAttachError> {
        // This is required to transfer ownership of the device to
        // the RVVM internals

        let mut device = ManuallyDrop::new(device);
        device.write_machine(self.inner);
        let result = unsafe {
            rvvm_attach_mmio(self.inner.as_ptr(), device.inner.as_ptr())
        };

        if result >= 0 {
            // SAFETY: this is safe since DeviceDescriptor's resources
            // now is owned by RVVM internals.
            unsafe { DeviceDescriptorGlue::move_out(device) };
            Ok(DeviceHandle(result))
        } else {
            // required to drop if device if error
            let _ = ManuallyDrop::into_inner(device);
            Err(DeviceAttachError::DeviceIsOverlapped)
        }
    }
}

impl Instance {
    pub const DEFAULT_BASE_ADDRESS: u64 = RVVM_DEFAULT_MEMBASE as u64;

    pub fn new(
        base_address: u64,
        ram_size: usize,
        threads: usize,
        rv64: bool,
    ) -> Result<Self, InstanceCreateError> {
        let machine = NonNull::new(unsafe {
            rvvm_create_machine(base_address, ram_size, threads, rv64)
        });

        machine
            .map(|ptr| Self { inner: ptr })
            .ok_or(InstanceCreateError::FailedToAllocate)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        // SAFETY: This is safe since pointer is guaranteed to be
        // non-null & we're allocated this resource internally
        // through `rvvm_create_machine`
        unsafe { rvvm_free_machine(self.inner.as_ptr()) }
    }
}
