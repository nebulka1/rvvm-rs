#![allow(dead_code)] // TODO: remove this code?

use std::ffi::c_void;

pub unsafe fn allocate_boxed_voidptr<T>(value: T) -> *mut c_void {
    let data = Box::new(value);
    let ptr = Box::into_raw(data);

    ptr as *mut c_void
}

pub unsafe fn free_boxed_voidptr<T>(boxed: *mut c_void) {
    let _ = Box::from_raw(boxed as *mut T); // deallocate
                                            // the memory
}
