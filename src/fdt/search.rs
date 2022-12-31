use std::ffi::{
    CStr,
    CString,
};

use rvvm_sys::{
    fdt_node,
    fdt_node_find,
    fdt_node_find_reg,
    fdt_node_find_reg_any,
};

pub struct Region<'a>(&'a CStr, u64);
pub struct AnyRegion<'a>(&'a CStr);
pub struct Name<'a>(&'a CStr);

mod details {
    use super::{
        AnyRegion,
        Name,
        Region,
    };
    pub trait Sealed {}

    impl Sealed for Region<'_> {}
    impl Sealed for AnyRegion<'_> {}
    impl Sealed for Name<'_> {}

    impl<T: AsRef<str>> Sealed for T {}
}

#[allow(private_in_public)]
pub trait FdtFindExt: details::Sealed {
    /// Internal trait for fancy search interface
    ///
    /// # Safety
    ///
    /// Unsafe due to logic through the raw pointer
    unsafe fn find_child_ptr(&self, ptr: *mut fdt_node) -> *mut fdt_node;
}

impl<T> FdtFindExt for T
where
    T: AsRef<str>,
{
    unsafe fn find_child_ptr(&self, ptr: *mut fdt_node) -> *mut fdt_node {
        let s = CString::new(self.as_ref())
            .expect("String contains nul-byte terminator");
        let name = Name(&s);

        FdtFindExt::find_child_ptr(&name, ptr)
    }
}

impl FdtFindExt for AnyRegion<'_> {
    unsafe fn find_child_ptr(&self, ptr: *mut fdt_node) -> *mut fdt_node {
        fdt_node_find_reg_any(ptr, self.0.as_ptr())
    }
}

impl FdtFindExt for Region<'_> {
    unsafe fn find_child_ptr(&self, ptr: *mut fdt_node) -> *mut fdt_node {
        fdt_node_find_reg(ptr, self.0.as_ptr(), self.1)
    }
}

impl FdtFindExt for Name<'_> {
    unsafe fn find_child_ptr(&self, ptr: *mut fdt_node) -> *mut fdt_node {
        fdt_node_find(ptr, self.0.as_ptr())
    }
}
