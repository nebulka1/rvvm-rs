use std::ptr::NonNull;

use rvvm_sys::{
    rvvm_create_machine,
    rvvm_free_machine,
    rvvm_machine_t,
};

use crate::error::InstanceCreateError;

pub struct Instance {
    inner: NonNull<rvvm_machine_t>,
}

impl Instance {
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
