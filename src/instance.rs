use std::{
    mem,
    ptr::NonNull,
    slice,
};

use rvvm_sys::{
    rvvm_attach_mmio,
    rvvm_create_machine,
    rvvm_free_machine,
    rvvm_get_fdt_root,
    rvvm_get_fdt_soc,
    rvvm_get_mmio,
    rvvm_machine_powered_on,
    rvvm_machine_t,
    rvvm_pause_machine,
    rvvm_read_ram,
    rvvm_start_machine,
    rvvm_write_ram,
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
        MemoryAccessError,
    },
    fdt::*,
    types::DeviceHandle,
};

pub struct Instance {
    ptr: NonNull<rvvm_machine_t>,
}

impl Instance {
    /// Writes `data` to the machine's RAM
    ///
    /// - Returns `Ok` if data was successfully written
    /// - Returns `MemoryAccessError` otherwise
    pub fn write_ram(
        &mut self,
        dst: u64,
        data: &[u8],
    ) -> Result<(), MemoryAccessError> {
        let result = unsafe {
            rvvm_write_ram(
                self.ptr.as_ptr(),
                dst,
                data.as_ptr() as *mut _,
                data.len(),
            )
        };

        if result {
            Ok(())
        } else {
            Err(MemoryAccessError::OutOfBounds)
        }
    }

    /// Read from machine's memory to slice
    ///
    /// Same as `Instance::read_ram_to_uninit`, but reads
    /// into initialized slice.
    /// See `Instance::read_ram_to_uninit` for more detailed
    /// information
    pub fn read_ram_to<'a>(
        &self,
        src: u64,
        dest: &'a mut [u8],
    ) -> Result<(), MemoryAccessError> {
        unsafe {
            self.read_ram_to_uninit(
                src,
                slice::from_raw_parts_mut::<'a, mem::MaybeUninit<u8>>(
                    dest.as_ptr() as *mut _,
                    dest.len(),
                ),
            )
        }
    }

    /// Read from machine's memory to uninitialized slice
    ///
    /// - Returns `Ok` if memory is successfully read
    /// - Returns `MemoryAccessError` otherwise
    pub fn read_ram_to_uninit(
        &self,
        src: u64,
        dest: &mut [mem::MaybeUninit<u8>],
    ) -> Result<(), MemoryAccessError> {
        let result = unsafe {
            rvvm_read_ram(
                self.ptr.as_ptr(),
                dest.as_ptr() as *mut _,
                src,
                dest.len(),
            )
        };

        if result {
            Ok(())
        } else {
            Err(MemoryAccessError::OutOfBounds)
        }
    }
}

impl Instance {
    /// Get mutable reference to the root FDT
    pub fn fdt_root_mut<'a>(&'a mut self) -> &'a mut Node {
        unsafe {
            Node::from_ptr_mut::<'a>(rvvm_get_fdt_root(self.ptr.as_ptr()))
        }
    }

    /// Get immutable reference to the root FDT
    pub fn fdt_root<'a>(&'a self) -> &'a Node {
        unsafe {
            Node::from_ptr::<'a>(rvvm_get_fdt_root(self.ptr.as_ptr()))
        }
    }

    /// Get mutable reference to the SoC's FDT
    pub fn fdt_soc_mut<'a>(&'a mut self) -> &'a mut Node {
        unsafe {
            Node::from_ptr_mut::<'a>(rvvm_get_fdt_soc(self.ptr.as_ptr()))
        }
    }

    /// Get immutable reference to the SoC's FDT
    pub fn fdt_soc<'a>(&'a self) -> &'a Node {
        unsafe {
            Node::from_ptr::<'a>(rvvm_get_fdt_soc(self.ptr.as_ptr()))
        }
    }
}

impl Instance {
    /// Get mutable reference to the RVVM's mmio device
    pub fn get_device_mut<T: Send + Sync>(
        &mut self,
        handle: DeviceHandle<T>,
    ) -> Option<&mut Device<T>> {
        let dev = unsafe { rvvm_get_mmio(self.ptr.as_ptr(), handle.id) };

        if dev.is_null() {
            None
        } else {
            // SAFETY: dev != null and mutable reference can't be
            // obtained twice because `&mut Device<T>` lifetime is
            // bounded by the `self`
            Some(unsafe { &mut *(dev as *mut Device<T>) })
        }
    }

    /// Get immutable reference to the RVVM's mmio device
    pub fn get_device<T: Send + Sync>(
        &self,
        handle: DeviceHandle<T>,
    ) -> Option<&Device<T>> {
        // SAFETY: self.ptr is valid rvvm machine ptr
        let dev = unsafe { rvvm_get_mmio(self.ptr.as_ptr(), handle.id) };

        if dev.is_null() {
            None
        } else {
            // SAFETY: dev != null
            Some(unsafe { &*(dev as *const Device<T>) })
        }
    }
}

impl Instance {
    pub fn powered_on(&self) -> bool {
        // SAFETY: `self.ptr` is obtained from `rvvm_create_machine`
        unsafe { rvvm_machine_powered_on(self.ptr.as_ptr()) }
    }

    /// Spawns CPU threads and continues machine execution
    pub fn start(&mut self) {
        // SAFETY: `self.ptr` is obtained from `rvvm_create_machine`
        unsafe { rvvm_start_machine(self.ptr.as_ptr()) }
    }

    /// Stops the CPUs, the machine is frozen upon return
    pub fn pause(&mut self) {
        // SAFETY: `self.ptr` is obtained from `rvvm_create_machine`
        unsafe {
            rvvm_pause_machine(self.ptr.as_ptr());
        }
    }
}

impl Instance {
    /// Tries to attach `Device<T>` to the paused virtual
    /// machine.
    ///
    /// Possible results:
    /// - `Ok(handle)`, returns typed device handle inside
    ///   the virtual machine
    /// - `Err(e)`, returns `DeviceAttachError` enum
    pub fn try_attach_device<T: Send + Sync>(
        &mut self,
        mut device: Device<T>,
    ) -> Result<DeviceHandle<T>, DeviceAttachError> {
        device.inner.machine = self.ptr.as_ptr();

        let handle = unsafe {
            rvvm_attach_mmio(
                self.ptr.as_ptr(),
                &device as *const Device<_> as *const _,
            )
        };

        std::mem::forget(device);

        match handle {
            RVVM_VM_IS_RUNNING_ERR => Err(DeviceAttachError::VmIsRunning),
            RVVM_INVALID_MMIO => Err(DeviceAttachError::RegionIsOccupied),

            h @ 0.. => Ok(DeviceHandle::new(h)),
            _ => unreachable!(),
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
