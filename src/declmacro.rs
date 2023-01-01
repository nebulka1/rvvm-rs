/// Create C string from string literal
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

#[macro_export]
#[doc(hidden)]
macro_rules! __ts_handler {
    (
        name = $name:ident,
        raw = $raw:ty
    ) => {
        $crate::__paste::paste! {
            pub type [< Raw $name >] = $raw;

            /// Type-safe wrapper around the C-ABI handler
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $name<T> {
                pub(crate) inner: Option<$raw>,
                _phantom: core::marker::PhantomData<T>,
            }

            impl<T> $name<T> {
                /// Create type-safe wrapper from the underlying raw handler
                ///
                /// # Safety
                ///
                /// This function is unsafe due to possibility of
                /// violating the type-safety
                pub const unsafe fn new(inner: $raw) -> Self {
                    Self {
                        inner: Some(inner),
                        _phantom: core::marker::PhantomData,
                    }
                }

                /// Creates empty handle
                pub const fn none() -> Self {
                    Self {
                        inner: None,
                        _phantom: core::marker::PhantomData,
                    }
                }
            }
        }
    };
}
