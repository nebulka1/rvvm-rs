use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::quote;
use syn::{
    parse_macro_input,
    AttributeArgs,
    Ident,
    ItemFn,
    Type,
};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    pub ty: Type,
}

/// Creates type-safe RW Device Handle
#[proc_macro_attribute]
#[proc_macro_error]
pub fn on_rw(attributes: TokenStream, stream: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attributes as AttributeArgs);
    let mut item = parse_macro_input!(stream as ItemFn);

    let args = match MacroArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    if let Some(const_) = item.sig.constness {
        abort! {
            const_, "Handler can't be const";
            help = "Remove const from function signature"
        }
    }
    if let Some(abi) = item.sig.abi {
        abort! {
            abi, "Explicit ABI specification is not allowed";
            help = "Remove explicit ABI from function signature"
        }
    }

    let ident = item.sig.ident;
    item.sig.ident = Ident::new("__inner_fn", ident.span());

    let ty = args.ty;

    let output = quote! {
        #[allow(non_upper_case_globals)]
        static #ident : rvvm::types::RwHandler<#ty> = {
            #item

            unsafe extern "C" fn __bind_fn(
                dev: *mut rvvm::ffi::rvvm_mmio_dev_t,
                dest: *mut std::ffi::c_void,
                offset: usize,
                size: u8,
            ) -> bool {
                let dev = &*(dev as *mut Device<#ty>);

                let slice: &'static mut [u8] = std::slice::from_raw_parts_mut(
                    dest as *mut u8,
                    dev.size()
                );

                if let Err(e) = __inner_fn(dev, slice, offset, size) {
                    eprintln!("Device {} returned an error: {:?}", stringify!(#ident), e);
                    false
                } else {
                    true
                }
            }

            unsafe { rvvm::types::RwHandler::<#ty>::new(__bind_fn) }
        };
    };

    output.into()
}
