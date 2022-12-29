use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::quote;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn rw_handler(args: TokenStream, stream: TokenStream) -> TokenStream {
    let args = arg_parser::parse_args(args);
    let item = syn::parse_macro_input!(stream as syn::ItemFn);

    let vis = item.vis;
    let sig = item.sig;

    if let Some(abi) = sig.abi {
        abort! {
            abi, "Abi selection is not supported";
            help = "Remove exact ABI specification from signature"
        }
    }
    if let Some(asyncness) = sig.asyncness {
        abort! {
            asyncness, "Async fns are not supported";
            help = "Remove async from signature"
        }
    }
    if let Some(c) = sig.constness {
        abort! {
            c, "RW Handle can't be const";
            help = "Remove const from signature"
        }
    }

    let generics = sig.generics;
    let fn_name = sig.ident;
    let params = sig.inputs;
    let ret = sig.output;
    let block = item.block;

    let ty = args.data_type;

    quote! {
        #vis unsafe extern "C" fn #fn_name<'a>(
            dev: ::rvvm::utils::TypeSafetyWrapper<
                *mut ::rvvm::ffi::rvvm_mmio_dev_t, #ty
            >,

            dest: *mut ::core::ffi::c_void,
            offset: usize,
            size: u8
        ) -> bool {
            let dev = dev.__unwrap();

            fn inner #generics(
                #params
            ) #ret {
                #block
            }

            let ptr: ::rvvm::dest_ptr::DestPtr<'a> =
                ::rvvm::dest_ptr::DestPtr::new(
                    dest,
                    size,
                    offset
                );
            let desc: ::rvvm::dev::mmio::DeviceDescriptor<'a, #ty> =
                ::rvvm::dev::mmio::DeviceDescriptor::from_raw_mmio(dev);
            inner(
                desc,
                ptr
            ).is_ok()
        }
    }
    .into()
}

mod arg_parser;
