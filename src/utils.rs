use std::{
    ffi::CStr,
    marker::PhantomData,
    ptr::NonNull,
};

use crate::error::CCharsCreateFailure;

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

#[cold]
pub(crate) const fn cold_path() {}
