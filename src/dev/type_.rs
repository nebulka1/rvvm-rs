use std::marker::PhantomData;

use rvvm_sys::{
    rvvm_mmio_dev_t,
    rvvm_mmio_type_t,
};

use crate::utils::{
    self,
    CChars,
};

type Handler = unsafe extern "C" fn(dev: *mut rvvm_mmio_dev_t);
pub struct MmioType<'a, T> {
    pub(crate) inner: rvvm_mmio_type_t,

    name: CChars<'a>,
    phantom: PhantomData<T>,
}

impl<'a, T> MmioType<'a, T> {
    pub const fn name(&self) -> &CChars<'a> {
        &self.name
    }

    /// # Safety
    ///
    /// This function is internal and extremely unsafe,
    /// calling `drop_glue` with:
    ///
    /// - Invalidly allocated `dev`
    /// - Invalidly allocated `dev->data`
    /// - Invalidly allocated `dev->type_`
    ///
    /// are considered UB.
    unsafe extern "C" fn drop_glue(dev: *mut rvvm_mmio_dev_t) {
        utils::free_and_drop_dev_internals::<T>(dev);
    }

    pub fn new(
        name: CChars<'a>,

        // FIXME: Add more guards to Handler,
        // current behavior is unsound
        remove: Option<Handler>,
        update: Option<Handler>,
        reset: Option<Handler>,
    ) -> Self {
        Self {
            inner: rvvm_mmio_type_t {
                remove: Some(remove.unwrap_or(Self::drop_glue)),
                update,
                reset,
                name: name.ptr(),
            },
            name,
            phantom: PhantomData,
        }
    }
}
