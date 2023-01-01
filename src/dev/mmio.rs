use std::{
    marker::PhantomData,
    ops::RangeInclusive,
    ptr,
};

use rvvm_sys::rvvm_mmio_dev_t;

use super::type_::DeviceType;
use crate::{
    builders::dev::mmio::DeviceBuilder,
    internal_utils::{
        allocate_boxed_voidptr,
        free_boxed_voidptr,
    },
    types::RwHandler,
};

#[repr(transparent)]
pub struct Device<T: Send + Sync> {
    pub(crate) inner: rvvm_mmio_dev_t,

    _phantom: PhantomData<T>,
}

impl<T: Send + Sync> Device<T> {
    pub const fn size(&self) -> usize {
        self.inner.size
    }
}

impl<T: Send + Sync> Device<T> {
    pub fn builder() -> DeviceBuilder<T> {
        DeviceBuilder::default()
    }

    pub fn new(
        address: u64,
        size: usize,

        data: T,
        dev_ty: DeviceType<T>,

        read: RwHandler<T>,
        write: RwHandler<T>,

        op_sizes: impl Into<RangeInclusive<u8>>,
    ) -> Self {
        let op_sizes = op_sizes.into();

        let dev = rvvm_mmio_dev_t {
            min_op_size: *op_sizes.start(),
            max_op_size: *op_sizes.end(),

            addr: address,
            size,

            // SAFETY: `Device<T>` will be managed by RVVM or cleared by
            // dropping
            data: unsafe { allocate_boxed_voidptr(data) },

            machine: ptr::null_mut(),

            // SAFETY: `DeviceType<'static, T>` has same in-memory
            // representation as the `rvvm_mmio_type_t`, so this cast is
            // safe
            type_: unsafe { allocate_boxed_voidptr(dev_ty) as *mut _ },

            // TODO: Implement read-write handlers
            read: read.inner,
            write: write.inner,
        };

        Self {
            inner: dev,
            _phantom: PhantomData,
        }
    }
}

impl<T: Send + Sync> Drop for Device<T> {
    fn drop(&mut self) {
        // Deallocate data
        // SAFETY: data is allocated through the
        // `allocate_boxed_voidptr::<T>`
        unsafe { free_boxed_voidptr::<T>(self.inner.data) }

        // Deallocate type
        // SAFETY: data is allocated through the
        // `allocate_boxed_voidptr::<T>`
        unsafe {
            free_boxed_voidptr::<DeviceType<T>>(self.inner.type_ as *mut _)
        }
    }
}
