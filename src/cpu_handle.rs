use std::{
    ffi::c_void,
    ptr::NonNull,
};

use rvvm_sys::rvvm_read_cpu_reg;

pub struct CpuReg(pub usize); // TODO: wrap RVVM_REGID_* constants

pub struct CpuHandle {
    // TODO: indicate that this is rvvm_cpu_handle_t
    pub(crate) ptr: NonNull<c_void>,
}

impl CpuHandle {
    /// Run a userland thread until a trap happens. Returns
    /// trap cause.
    pub fn run(&self) -> u64 {
        unsafe { rvvm_sys::rvvm_run_user_thread(self.ptr.as_ptr()) }
    }

    pub fn read_cpu_reg(&self, reg: CpuReg) -> u64 {
        unsafe { rvvm_read_cpu_reg(self.ptr.as_ptr(), reg.0) }
    }

    pub fn write_cpu_reg(&self, reg: CpuReg, value: u64) {
        unsafe {
            rvvm_sys::rvvm_write_cpu_reg(self.ptr.as_ptr(), reg.0, value)
        }
    }
}

impl Drop for CpuHandle {
    fn drop(&mut self) {
        unsafe {
            rvvm_sys::rvvm_free_user_thread(self.ptr.as_ptr());
        }
    }
}
