use std::{
    ffi::c_void,
    marker::PhantomData,
    mem,
    mem::ManuallyDrop,
    ops::{
        Deref,
        DerefMut,
        RangeInclusive,
    },
    ptr,
    ptr::NonNull,
};

use rvvm_sys::{
    rvvm_machine_t,
    rvvm_mmio_dev_t,
};

use super::type_::MmioType;
use crate::utils::{
    self,
    cold_path,
};

type RwCallback<T> = unsafe extern "C" fn(
    dev: utils::TypeSafetyWrapper<*mut rvvm_mmio_dev_t, T>,
    dest: *mut c_void,
    offset: usize,
    size: u8,
) -> bool;

#[repr(transparent)]
pub struct MmioHandler<T>(pub(crate) Option<RwCallback<T>>);

/// Descriptor of an MMIO device
pub struct DeviceDescriptor<'a, T = ()> {
    pub(crate) inner: NonNull<rvvm_mmio_dev_t>,

    /// HACK: since MmioDeviceDescriptor can be created
    /// either by Rust or C, this is used to prevent
    /// memory corruption.
    needs_free: bool,
    phantom: PhantomData<&'a T>,
}

// TODO: add more proper resource management.
// Currently DeviceDescriptorGlue is used to release
// resources owned by virtual machine itself. Some of these
// (probably) can be moved inside descriptor glue instead?
pub struct DeviceDescriptorGlue<'a, T> {
    inner: DeviceDescriptor<'a, T>,
}

impl<'a, T> DeviceDescriptor<'a, T>
where
    T: Send + Sync,
{
    pub fn data(&self) -> &T {
        // SAFETY: this is safe since T is `Send + Sync` and
        // inner != null
        unsafe { &*(self.inner.as_ref().data as *const T) }
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        address: u64,
        size: usize,
        op_size_range: RangeInclusive<u8>,

        read: MmioHandler<T>,
        write: MmioHandler<T>,

        data: T,
        mmio_type: MmioType<'a, T>,
    ) -> DeviceDescriptorGlue<'a, T> {
        let allocated_data = 'b: {
            if std::mem::size_of::<T>() == 0 {
                break 'b ptr::null_mut();
            }

            let mem = unsafe { utils::allocate_libc(data) };
            mem.as_ptr() as *mut std::ffi::c_void
        };

        let type_ptr = Box::into_raw(Box::new(mmio_type.inner));
        let allocated = unsafe {
            utils::allocate_libc(rvvm_mmio_dev_t {
                addr: address,
                size,
                data: allocated_data,
                machine: ptr::null_mut(),
                type_: type_ptr,

                // SAFETY: this is safe since TypeSafetyWrapper is marked
                // as repr(transparent), so it has no
                // effect on underlying data representation
                read: mem::transmute(read.0),
                write: mem::transmute(write.0),

                min_op_size: *op_size_range.start(),
                max_op_size: *op_size_range.end(),
            })
        };
        DeviceDescriptorGlue {
            inner: Self {
                inner: allocated,
                needs_free: true,
                phantom: PhantomData,
            },
        }
    }

    /// # Safety
    ///
    /// This function is extremely unsafe due to lack of
    /// rvvm_mmio_dev_t proper allocation check and due to
    /// possibility to produce an unbounded lifetime.
    ///
    /// So, result is considered UB if ptr was not owned by
    /// RVVM internals.
    pub unsafe fn from_raw_mmio(ptr: *mut rvvm_mmio_dev_t) -> Self {
        Self {
            inner: NonNull::new(ptr)
                .expect("Got nullptr inside from_raw_mmio"),
            needs_free: false,
            phantom: PhantomData,
        }
    }

    pub(crate) fn write_machine(
        &mut self,
        machine: NonNull<rvvm_machine_t>,
    ) {
        // SAFETY: this is safe since inner is properly allocated
        // and not null
        unsafe { self.inner.as_mut().machine = machine.as_ptr() };
    }
}

impl<'a, T> DeviceDescriptorGlue<'a, T> {
    pub(crate) unsafe fn move_out(
        this: ManuallyDrop<Self>,
    ) -> DeviceDescriptor<'a, T> {
        unsafe { std::ptr::read(&this.inner) }
    }
}

impl<'a, T> Drop for DeviceDescriptorGlue<'a, T> {
    fn drop(&mut self) {
        let dev = self.inner.inner.as_ptr();

        // SAFETY: this is safe since `inner.inner` is
        // NonNull and well-allocated. guarantees
        // by `DeviceDescriptor<T>`
        unsafe { utils::free_and_drop_dev_internals::<T>(dev) }
    }
}

impl<'a, T> Drop for DeviceDescriptor<'a, T> {
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

impl<'a, T> Deref for DeviceDescriptorGlue<'a, T> {
    type Target = DeviceDescriptor<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for DeviceDescriptorGlue<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

//

impl<T> MmioHandler<T> {
    pub const fn empty() -> Self {
        Self(None)
    }

    pub const fn new(handler: RwCallback<T>) -> Self {
        Self(Some(handler))
    }
}

impl<T> From<Option<RwCallback<T>>> for MmioHandler<T> {
    fn from(value: Option<RwCallback<T>>) -> Self {
        Self(value)
    }
}

impl<T> From<RwCallback<T>> for MmioHandler<T> {
    fn from(value: RwCallback<T>) -> Self {
        Self::new(value)
    }
}
