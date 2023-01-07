use std::{
    ffi::{
        c_void,
        CString,
    },
    marker::PhantomData,
    mem,
    path::Path,
    ptr::NonNull,
    slice,
};

use rvvm_sys::{
    rvvm_attach_mmio,
    rvvm_create_machine,
    rvvm_create_user_thread,
    rvvm_create_userland,
    rvvm_dump_dtb,
    rvvm_free_machine,
    rvvm_get_fdt_root,
    rvvm_get_fdt_soc,
    rvvm_load_bootrom,
    rvvm_load_dtb,
    rvvm_load_kernel,
    rvvm_machine_powered_on,
    rvvm_machine_t,
    rvvm_mmio_dev_t,
    rvvm_mmio_type_t,
    rvvm_pause_machine,
    rvvm_read_ram,
    rvvm_start_machine,
    rvvm_write_ram,
    RVVM_DEFAULT_MEMBASE,
};

use crate::{
    builders::instance::InstanceBuilder,
    cpu_handle::CpuHandle,
    dev::mmio::Device,
    error::{
        DeviceAttachError,
        DtbDumpError,
        InstanceCreateError,
        InstancePauseError,
        InstanceStartError,
        MemoryAccessError,
    },
    fdt::Node,
    types::DeviceHandle,
};

/// Marker that indicates that the machine is running in the
/// standard context.
pub enum Machine {}

/// Marker that indicates that the machine is running in the
/// userland context.
pub enum Userland {}

/// Trait that parameterizes over the machine context
/// markers [`Machine`] and [`Userland`].
pub trait InstanceKind: private::Sealed {}

impl InstanceKind for Machine {}
impl private::Sealed for Machine {}

impl InstanceKind for Userland {}
impl private::Sealed for Userland {}

pub struct Instance<K: InstanceKind = Machine> {
    ptr: NonNull<rvvm_machine_t>,
    _kind: PhantomData<K>,
}

impl Instance<Machine> {
    /// Try attach device to the RVVM.
    ///
    /// # Panics
    ///
    /// - if `Dev::HAS_READ` or `Dev::HAS_WRITE` are not
    ///   true (FIXME)
    /// - if `Dev::name()` returned string with the nul-byte
    ///   terminator
    pub fn try_attach_device<Ty, Dev>(
        &mut self,
        dev: Dev,
    ) -> Result<DeviceHandle<Ty>, DeviceAttachError>
    where
        Ty: Send + Sync,
        Dev: Device<Ty>,
    {
        // Dev is meant to be `repr(transparent)` to the
        // `rvvm_mmio_dev_t`, and `Device<Ty>` also
        // implements unsafe trait `DeviceData<Ty = Ty>`,
        // so, we can assume that `Dev` and the
        // `rvvm_mmio_dev_t` is same in the representation

        if !(Dev::HAS_READ && Dev::HAS_WRITE) {
            panic!(
                "FIXME: Validation of the data representation (Either \
                 repr(C) struct or smth that is represented as a simple \
                 u8 buffer)"
            );
        }

        union CopyCast<Src, Dst: Copy> {
            src: mem::ManuallyDrop<Src>,
            dst: Dst,
        }

        fn no_drop<T>(src: T) -> mem::ManuallyDrop<T> {
            mem::ManuallyDrop::new(src)
        }

        let mut underlying = unsafe {
            CopyCast::<Dev, rvvm_mmio_dev_t> { src: no_drop(dev) }.dst
        };

        unsafe extern "C" fn rw_handler<Ty, Dev, const CALL_READ: bool>(
            dev: *mut rvvm_mmio_dev_t,
            dest: *mut c_void,
            offset: usize,
            size: u8,
        ) -> bool
        where
            Ty: Send + Sync,
            Dev: Device<Ty>,
        {
            let region_size = { (*dev).size };
            let region =
                slice::from_raw_parts_mut(dest as *mut u8, region_size);

            let device = &*(dev as *const () as *const Dev);

            if CALL_READ {
                device.read(region, size, offset)
            } else {
                device.write(region, size, offset)
            }
            .is_ok()
        }

        unsafe extern "C" fn type_handler<
            Ty: Send + Sync,
            Dev: Device<Ty>,
            const CALL_RESET: bool,
        >(
            dev: *mut rvvm_mmio_dev_t,
        ) {
            let dev = &mut *(dev as *mut () as *mut Dev);
            if CALL_RESET {
                dev.reset()
            } else {
                dev.update()
            }
        }

        unsafe extern "C" fn drop_glue<Ty, Dev>(dev: *mut rvvm_mmio_dev_t)
        where
            Ty: Send + Sync,
            Dev: Device<Ty>,
        {
            {
                let dev = &mut *dev;

                {
                    let ty = dev.type_ as *mut rvvm_mmio_type_t;
                    let _ = CString::from_raw((*ty).name as *mut u8);
                    let _ =
                        Box::from_raw(dev.type_ as *mut rvvm_mmio_type_t);
                }
            }

            std::ptr::drop_in_place::<Dev>(dev as *mut () as *mut Dev);
        }

        underlying.read = Some(rw_handler::<_, Dev, true>);
        underlying.write = Some(rw_handler::<_, Dev, false>);
        underlying.machine = self.ptr.as_ptr();

        let machine_type = Box::new(rvvm_mmio_type_t {
            remove: Some(drop_glue::<_, Dev>),
            update: Some(type_handler::<_, Dev, false>),
            reset: Some(type_handler::<_, Dev, true>),
            name: CString::new(Dev::name())
                .expect("Name contains nul-byte terminator")
                .into_raw(),
        });

        underlying.type_ = Box::into_raw(machine_type);
        let handle = unsafe {
            rvvm_attach_mmio(self.ptr.as_ptr(), &underlying as *const _)
        };

        match handle {
            h @ 0.. => Ok(unsafe { DeviceHandle::<Ty>::from_raw(h) }),
            _ => Err(DeviceAttachError::RegionIsOccupied),
        }
    }
}

// FIXME: add correct error reporting
// mainly this could be fixed by RVVM, I'll make PR to ti
impl Instance<Machine> {
    pub fn try_dump_dtb(
        &mut self,
        dest: impl AsRef<Path>,
    ) -> Result<(), DtbDumpError> {
        self.loader_dumper_impl(
            DtbDumpError::FailedToOpenFile,
            dest,
            rvvm_dump_dtb,
        )
    }

    /// Load device tree binary into machine's RAM.
    ///
    /// - Returns `Ok` if load was successful
    /// - Returns `MemoryAccessError` otherwise
    ///
    /// # Panics
    ///
    /// Panics if path has nul-byte character or if path is
    /// not a valid utf8 sequence.
    pub fn try_load_dtb(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<(), MemoryAccessError> {
        self.loader_dumper_impl(
            MemoryAccessError::OutOfBounds,
            path,
            rvvm_load_dtb,
        )
    }

    /// Load kernel binary into machine's RAM.
    ///
    /// - Returns `Ok` if load was successful
    /// - Returns `MemoryAccessError` otherwise
    /// # Panics
    ///
    /// Panics if path has nul-byte character or if path is
    /// not a valid utf8 sequence.
    pub fn try_load_kernel(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<(), MemoryAccessError> {
        self.loader_dumper_impl(
            MemoryAccessError::OutOfBounds,
            path,
            rvvm_load_kernel,
        )
    }

    /// Load bootrom binary into machine's RAM.
    ///
    /// - Returns `Ok` if load was successful
    /// - Returns `MemoryAccessError` otherwise
    /// # Panics
    ///
    /// Panics if path has nul-byte character or if path is
    /// not a valid utf8 sequence.
    pub fn try_load_bootrom(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<(), MemoryAccessError> {
        self.loader_dumper_impl(
            MemoryAccessError::OutOfBounds,
            path,
            rvvm_load_bootrom,
        )
    }

    fn loader_dumper_impl<E>(
        &mut self,
        e: E,
        path: impl AsRef<Path>,
        fn_: unsafe extern "C" fn(
            *mut rvvm_machine_t,
            *const std::ffi::c_char,
        ) -> bool,
    ) -> Result<(), E> {
        let path = CString::new(
            path.as_ref()
                .to_str()
                .expect("path is not a valid utf8 sequence"),
        )
        .expect("Path contains nul-byte character");

        if unsafe { fn_(self.ptr.as_ptr(), path.as_ptr()) } {
            Ok(())
        } else {
            Err(e)
        }
    }
}

impl<K: InstanceKind> Instance<K> {
    /// Writes `data` to the machine's RAM
    ///
    /// - Returns `Ok` if data was successfully written
    /// - Returns `MemoryAccessError` otherwise
    pub fn write_ram(
        &mut self,
        dst: u64,
        data: &[u8],
    ) -> Result<(), MemoryAccessError> {
        Self::bool_to_memacc(unsafe {
            rvvm_write_ram(
                self.ptr.as_ptr(),
                dst,
                data.as_ptr() as *mut _,
                data.len(),
            )
        })
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
        Self::bool_to_memacc(unsafe {
            rvvm_read_ram(
                self.ptr.as_ptr(),
                dest.as_ptr() as *mut _,
                src,
                dest.len(),
            )
        })
    }

    fn bool_to_memacc(b: bool) -> Result<(), MemoryAccessError> {
        if b {
            Ok(())
        } else {
            Err(MemoryAccessError::OutOfBounds)
        }
    }
}

impl Instance<Machine> {
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

impl Instance<Machine> {
    pub fn powered_on(&self) -> bool {
        // SAFETY: `self.ptr` is obtained from `rvvm_create_machine`
        unsafe { rvvm_machine_powered_on(self.ptr.as_ptr()) }
    }

    /// Spawns CPU threads and continues machine execution
    pub fn start(&mut self) -> Result<(), InstanceStartError> {
        // SAFETY: `self.ptr` is obtained from `rvvm_create_machine`
        let result = unsafe { rvvm_start_machine(self.ptr.as_ptr()) };
        if result {
            Ok(())
        } else {
            Err(InstanceStartError::AlreadyRunning)
        }
    }

    /// Stops the CPUs, the machine is frozen upon return
    pub fn pause(&mut self) -> Result<(), InstancePauseError> {
        // SAFETY: `self.ptr` is obtained from `rvvm_create_machine`
        let result = unsafe { rvvm_pause_machine(self.ptr.as_ptr()) };

        if result {
            Ok(())
        } else {
            Err(InstancePauseError::NotRunning)
        }
    }
}

impl Instance<Machine> {
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
        .map(|ptr| Self {
            ptr,
            _kind: PhantomData,
        })
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

//
// Userland
//

impl Instance<Userland> {
    pub fn try_new(rv64: bool) -> Result<Self, InstanceCreateError> {
        NonNull::new(unsafe { rvvm_create_userland(rv64) })
            .map(|ptr| Self {
                ptr,
                _kind: PhantomData,
            })
            .ok_or(InstanceCreateError::FailedToAllocate)
    }

    pub fn new(rv64: bool) -> Self {
        Self::try_new(rv64)
            .expect("Failed to allocate memory for the machine")
    }
}

impl Instance<Userland> {
    pub fn create_user_thread(&mut self) -> CpuHandle {
        CpuHandle::new(unsafe {
            rvvm_create_user_thread(self.ptr.as_ptr())
        })
        .expect("Failed to create user thread")
    }
}

impl<K: InstanceKind> Drop for Instance<K> {
    fn drop(&mut self) {
        // SAFETY: `self.ptr` is allocated through the
        // `rvvm_create_machine`
        unsafe { rvvm_free_machine(self.ptr.as_ptr()) }
    }
}

mod private {
    pub trait Sealed {}
}
