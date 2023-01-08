use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_derive(KnownLayout)]
pub fn known_layout_macro(stream: TokenStream) -> TokenStream {
    known_layout::known_layout_impl(stream)
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn device(attrs: TokenStream, stream: TokenStream) -> TokenStream {
    device::device_impl(attrs, stream)
}

mod device;
mod known_layout;
