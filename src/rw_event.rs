use std::{
    ffi::c_void,
    ptr,
};

use crate::{
    error::MemError,
    utils::cold_path,
};

pub struct RwEvent<'a> {
    pub region: &'a mut [u8],
    pub affected: u8,
    pub offset: usize,
}

impl<'a> RwEvent<'a> {
    pub fn try_write(
        &mut self,
        offset: usize,
        buf: &[u8],
    ) -> Result<(), MemError> {
        let start = self
            .region
            .get_mut(offset..)
            .ok_or(MemError::InvalidMemoryRegion)?;
        if start.len() < buf.len() {
            cold_path();
            return Err(MemError::TooLongBuffer);
        }

        // SAFETY: this is safe since we're checked bounds above
        unsafe {
            ptr::copy_nonoverlapping(
                buf.as_ptr(),
                start.as_mut_ptr(),
                buf.len(),
            )
        }

        Ok(())
    }

    /// # Safety
    ///
    /// This function is unsafe due to possibility of
    /// producing unbounded lifetimes
    pub unsafe fn new(
        ptr: *mut c_void,
        size: u8,
        offset: usize,
        region_size: usize,
    ) -> Self {
        Self {
            region: std::slice::from_raw_parts_mut(
                ptr as *mut u8,
                region_size,
            ),
            affected: size,
            offset,
        }
    }
}
