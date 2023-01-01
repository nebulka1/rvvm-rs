use std::ptr::NonNull;

use rvvm_sys::{
    rvvm_attach_mmio,
    rvvm_create_machine,
    rvvm_free_machine,
    rvvm_machine_t,
    RVVM_DEFAULT_MEMBASE,
    RVVM_INVALID_MMIO,
    RVVM_VM_IS_RUNNING_ERR,
};

use crate::{
    builders::instance::InstanceBuilder,
    dev::mmio::Device,
    error::{
        DeviceAttachError,
        InstanceCreateError,
    },
    types::DeviceHandle,
};

pub struct Instance {
    ptr: NonNull<rvvm_machine_t>,
}

impl Instance {
    pub fn attach_device<T>(
        &mut self,
        mut device: Device<T>,
    ) -> Result<DeviceHandle, DeviceAttachError> {
        device.inner.machine = self.ptr.as_ptr();

        let handle = unsafe {
            rvvm_attach_mmio(
                self.ptr.as_ptr(),
                &device as *const Device<_> as *const _,
            )
        };

        std::mem::forget(device);

        if handle == RVVM_VM_IS_RUNNING_ERR {
            Err(DeviceAttachError::VmIsRunning)
        } else if handle == RVVM_INVALID_MMIO {
            Err(DeviceAttachError::RegionIsOccupied)
        } else if handle >= 0 {
            Ok(DeviceHandle(handle))
        } else {
            unreachable!()
        }
    }
}

impl Instance {
    pub const DEFAULT_MEMBASE: u64 = RVVM_DEFAULT_MEMBASE as _;

    /// Creates the `InstanceBuilder` for the builder
    /// pattern.
    pub fn builder() -> InstanceBuilder {
        Default::default()
    }

    /// Tries to create the Instance. Returns
    /// `InstanceCreateError` if
    /// `rvvm::ffi::rvvm_create_machine` returned null
    /// pointer.
    pub fn try_new(
        harts: usize,

        mem_base: u64,
        mem_size: usize,

        rv64: bool,
    ) -> Result<Self, InstanceCreateError> {
        NonNull::new(unsafe {
            rvvm_create_machine(mem_base, mem_size, harts, rv64)
        })
        .map(|ptr| Self { ptr })
        .ok_or(InstanceCreateError::FailedToAllocate)
    }

    /// Creates virtual machine instance.
    ///
    /// # Panics
    ///
    /// Panics if underlying call to the `try_new` returned
    /// an `Err`. See `Instance::try_new` for more detailed
    /// description.
    pub fn new(
        harts: usize,

        mem_base: u64,
        mem_size: usize,

        rv64: bool,
    ) -> Self {
        Self::try_new(harts, mem_base, mem_size, rv64)
            .expect("Failed to allocate memory for the machine")
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        // SAFETY: `self.ptr` is allocated through the
        // `rvvm_create_machine`
        unsafe { rvvm_free_machine(self.ptr.as_ptr()) }
    }
}
