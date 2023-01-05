use std::{
    ffi::c_void,
    ptr::NonNull,
};

use rvvm_sys::rvvm_read_cpu_reg;

use crate::reg::Register;

pub struct CpuHandle {
    // TODO: indicate that this is rvvm_cpu_handle_t
    pub(crate) ptr: NonNull<c_void>,
}

impl CpuHandle {
    /// Run a userland thread until a trap happens. Returns
    /// trap cause.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the machine interacts
    /// with host process memory directly.
    pub unsafe fn run(&mut self) -> u64 {
        unsafe { rvvm_sys::rvvm_run_user_thread(self.ptr.as_ptr()) }
    }

    pub fn read_cpu_reg(&self, reg: Register) -> u64 {
        unsafe { rvvm_read_cpu_reg(self.ptr.as_ptr(), reg as usize) }
    }

    pub fn write_cpu_reg(&mut self, reg: Register, value: u64) {
        unsafe {
            rvvm_sys::rvvm_write_cpu_reg(
                self.ptr.as_ptr(),
                reg as usize,
                value,
            )
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

#[cfg(test)]
mod tests {
    use crate::{
        prelude::{
            Instance,
            Userland,
        },
        reg::Register,
    };

    #[test]
    fn user_thread() {
        let mut instance = Instance::<Userland>::new(true);
        let mut thread = instance.create_user_thread();
        let program: [u32; 2] = [
            0x07b00513u32.to_le(), // li x10, 123
            0x00100073u32.to_le(), // ebreak
        ];
        thread.write_cpu_reg(Register::PC, program.as_ptr() as u64);
        let rc = unsafe { thread.run() };
        assert_eq!(rc, 3); // 3 == TRAP_BREAKPOINT
        assert_eq!(
            thread.read_cpu_reg(Register::PC),
            program.as_ptr() as u64 + 8
        );
        assert_eq!(thread.read_cpu_reg(Register::X10), 123);
    }
}
