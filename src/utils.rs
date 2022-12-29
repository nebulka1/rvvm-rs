use std::{
    ffi::CStr,
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::NonNull,
};

use rvvm_sys::rvvm_mmio_dev_t;

use crate::error::CCharsCreateFailure;

#[repr(transparent)]
pub struct TypeSafetyWrapper<T, R> {
    pub(crate) inner: T,
    pub(crate) phantom: PhantomData<R>,
}

impl<T, R> TypeSafetyWrapper<T, R> {
    /// # Safety
    ///
    /// This function is internal, so its use outside is
    /// considered unsafe
    pub unsafe fn __unwrap(self) -> T {
        self.inner
    }
}

#[macro_export]
macro_rules! c_str {
    ($lit:literal) => {{
        let concatenated = concat!($lit, "\0");
        // SAFETY: this is safe since requirements are due to line
        // above
        unsafe {
            $crate::utils::CChars::new_unchecked(
                concatenated.as_bytes() as &[u8]
            )
        }
    }};
}

/// Simple container for nul-terminated C-Strings.
pub struct CChars<'a> {
    ptr: NonNull<std::ffi::c_char>,
    length: usize,
    phantom: PhantomData<&'a ()>,
}

impl<'a> CChars<'a> {
    pub fn as_c_str(&self) -> &CStr {
        // SAFETY: this is safe since structure can't be created
        // with invalid ptr
        unsafe {
            CStr::from_bytes_with_nul_unchecked(
                std::slice::from_raw_parts(
                    self.ptr.as_ptr() as *const _,
                    self.length,
                ),
            )
        }
    }

    pub const fn ptr(&self) -> *const std::ffi::c_char {
        self.ptr.as_ptr() as *const _
    }

    pub fn new(bytes: &'a [u8]) -> Self {
        Self::try_new(bytes).expect("Failed to create CChars instance")
    }

    pub const fn try_new(
        bytes: &'a [u8],
    ) -> Result<Self, CCharsCreateFailure> {
        // Required due to const context
        'block: {
            let mut pos = 0;
            while pos < bytes.len() {
                if bytes[pos] == 0 {
                    break 'block;
                }

                pos += 1;
            }

            return Err(CCharsCreateFailure::NoNulTerminator);
        };

        // SAFETY: this is safe since we're checked requirements
        // above
        Ok(unsafe { Self::new_unchecked(bytes) })
    }

    /// # Safety
    /// This operation is unsafe due to lack of
    /// nul-terminator check.
    ///
    /// Result is considered UB if `bytes` not contains \0
    pub const unsafe fn new_unchecked(bytes: &'a [u8]) -> Self {
        Self {
            ptr: NonNull::new_unchecked(bytes.as_ptr() as *mut i8),
            length: bytes.len(),
            phantom: PhantomData,
        }
    }
}

/// # Safety
///
/// If ptr points to invalid allocated data action
/// considered UB. For example: ptr is not allocated through
/// `libc::malloc`
pub(crate) unsafe fn free_and_drop_voidptr<T>(
    data: *mut std::ffi::c_void,
) {
    unsafe {
        std::ptr::drop_in_place::<T>(data as *mut T);
        if data.is_null() {
            libc::free(data);
        }
    }
}

/// # Safety
///
/// If data points to invalidly allocated data, then
/// behavior is undefined
pub(crate) unsafe fn free_and_drop_boxed<T>(data: *mut T) {
    let _ = unsafe { Box::<T>::from_raw(data) };
}

/// # Safety
///
/// This function is considered unsafe because it must be
/// used only by internals. Allocating non-repr(C) and ZST
/// T's are considered UB;
pub(crate) unsafe fn allocate_libc<T>(data: T) -> NonNull<T> {
    let mem = libc::malloc(std::mem::size_of::<T>());
    if mem.is_null() {
        cold_path();
        panic!(
            "Failed to allocate memory for {}",
            std::any::type_name::<T>()
        );
    }

    {
        let memref = &mut *(mem as *mut MaybeUninit<T>);
        memref.write(data);
    }

    // SAFETY: this is safe since we're checked null value above
    NonNull::new_unchecked(mem as *mut _)
}

/// # Safety
///
/// Considered UB if:
///
/// - `dev` is invalidly allocated or nullptr
/// - `dev->data` is invalidly allocated
/// - `dev->type_` is invalidly allocated or nullptr
pub(crate) unsafe fn free_and_drop_dev_internals<T>(
    dev: *mut rvvm_mmio_dev_t,
) {
    let dev = &mut *dev;

    free_and_drop_voidptr::<T>(dev.data);
    free_and_drop_boxed(dev.type_);
}

#[cold]
pub(crate) const fn cold_path() {}
