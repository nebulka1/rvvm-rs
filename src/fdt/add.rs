use std::ffi::CStr;

use rvvm_sys::{
    fdt_node,
    fdt_node_add_prop_u32,
    fdt_node_add_prop_u64,
};

mod details {
    use crate::fdt::NodeBuf;

    pub trait Sealed {}

    impl Sealed for u32 {}
    impl Sealed for u64 {}

    impl Sealed for NodeBuf {}
}

pub trait FdtNodeAddExt: details::Sealed {
    /// # Safety
    ///
    /// Unsafe due to raw pointer logic
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node);
}

impl FdtNodeAddExt for u64 {
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node) {
        fdt_node_add_prop_u64(ptr, name.as_ptr(), *self)
    }
}

impl FdtNodeAddExt for u32 {
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node) {
        fdt_node_add_prop_u32(ptr, name.as_ptr(), *self)
    }
}
