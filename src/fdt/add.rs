use std::ffi::CStr;

use rvvm_sys::{
    fdt_node,
    fdt_node_add_prop,
    fdt_node_add_prop_cells,
    fdt_node_add_prop_str,
    fdt_node_add_prop_u32,
    fdt_node_add_prop_u64,
};

mod details {
    use std::ffi::CStr;

    pub trait Sealed {}

    impl Sealed for u32 {}
    impl Sealed for u64 {}

    impl Sealed for &'_ CStr {}
    impl Sealed for &'_ [u32] {}
    impl Sealed for &'_ [u8] {}

    impl<const N: usize> Sealed for [u32; N] {}
    impl<const N: usize> Sealed for [u8; N] {}
}

pub trait FdtNodeAddPropExt: details::Sealed {
    /// # Safety
    ///
    /// Unsafe due to raw pointer logic
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node);
}

impl<const N: usize> FdtNodeAddPropExt for [u8; N] {
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node) {
        let b: &[u8] = self;
        b.fdt_node_add(name, ptr)
    }
}

impl<const N: usize> FdtNodeAddPropExt for [u32; N] {
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node) {
        let b: &[u32] = self;
        b.fdt_node_add(name, ptr)
    }
}

impl FdtNodeAddPropExt for &'_ [u8] {
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node) {
        fdt_node_add_prop(
            ptr,
            name.as_ptr(),
            self.as_ptr() as *const _,
            self.len() as u32,
        )
    }
}

impl FdtNodeAddPropExt for &'_ [u32] {
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node) {
        union U {
            i: *const u32,
            o: *mut u32,
        }

        fdt_node_add_prop_cells(
            ptr,
            name.as_ptr(),
            // SAFETY: this is safe, since fdtlib doesen't mutate slice's
            // contents
            U { i: self.as_ptr() }.o,
            self.len() as u32,
        )
    }
}

impl FdtNodeAddPropExt for &'_ CStr {
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node) {
        fdt_node_add_prop_str(ptr, name.as_ptr(), self.as_ptr())
    }
}

impl FdtNodeAddPropExt for u64 {
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node) {
        fdt_node_add_prop_u64(ptr, name.as_ptr(), *self)
    }
}

impl FdtNodeAddPropExt for u32 {
    unsafe fn fdt_node_add(&self, name: &CStr, ptr: *mut fdt_node) {
        fdt_node_add_prop_u32(ptr, name.as_ptr(), *self)
    }
}
