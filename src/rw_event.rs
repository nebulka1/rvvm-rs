use std::ffi::c_void;

pub struct RwEvent<'a> {
    pub region: &'a mut [u8],
    pub affected: u8,
    pub offset: usize,
}

impl<'a> RwEvent<'a> {
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
