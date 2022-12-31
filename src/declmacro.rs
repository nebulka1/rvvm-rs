#[macro_export]
macro_rules! c_str {
    ($lit:literal) => {
        // SAFETY: Safe, since we just concatenated the
        // nul-terminator
        unsafe {
            std::ffi::CStr::from_bytes_with_nul_unchecked(
                concat!($lit, "\0").as_bytes(),
            )
        }
    };
}
