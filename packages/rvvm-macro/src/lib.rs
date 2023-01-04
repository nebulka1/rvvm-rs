use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{
    Span,
    TokenStream as TokenStream2,
};
use proc_macro_error::*;
use quote::quote;
use syn::{
    AttributeArgs,
    Ident,
    ItemFn,
    Signature,
    Type,
};

macro_rules! parse_fn {
    ($attrs:expr, $stream:expr) => {
        match parse_fn($attrs, $stream) {
            Ok(tpl) => tpl,
            Err(e) => return e,
        }
    };
}

#[derive(Debug, FromMeta)]
struct MacroArgs {
    pub ty: Type,
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn type_handler(
    attrs: TokenStream,
    stream: TokenStream,
) -> TokenStream {
    let (args, mut item) = parse_fn!(attrs, stream);

    let ident = replace_ident(&mut item.sig, "__ty_handler");
    let ty = args.ty;

    quote! {
        #[allow(non_upper_case_globals)]
        static #ident : rvvm::types::TypeHandler<#ty> = {
            #item

            unsafe extern "C" fn __bind_ty(dev: *mut rvvm::ffi::rvvm_mmio_dev_t) {
                let dev = &*(dev as *mut rvvm::dev::mmio::Device<#ty>);
                __ty_handler(dev);
            }

            unsafe { rvvm::types::TypeHandler::<#ty>::new(__bind_ty) }
        };
    }.into()
}

/// Creates `DeviceType<T>` remove handler
#[proc_macro_attribute]
#[proc_macro_error]
pub fn on_remove(attrs: TokenStream, stream: TokenStream) -> TokenStream {
    let (args, mut item) = parse_fn!(attrs, stream);

    let ident = replace_ident(&mut item.sig, "__rm_fn");
    let ty = args.ty;

    quote! {
        #[allow(non_upper_case_globals)]
        static #ident : rvvm::types::RemoveHandler<#ty> = {
            #item

            unsafe extern "C" fn __bind_fn(dev: *mut rvvm::ffi::rvvm_mmio_dev_t) {
                {
                    let dev = &mut *(dev as *mut rvvm::dev::mmio::Device<#ty>);
                    __rm_fn(dev);
                }

                unsafe { rvvm::dev::type_::DeviceType::<#ty>::drop_glue(dev) };
            }

            unsafe { rvvm::types::RemoveHandler::<#ty>::new(__bind_fn) }
        };
    }.into()
}

/// Creates type-safe RW Device Handle
#[proc_macro_attribute]
#[proc_macro_error]
pub fn on_rw(attributes: TokenStream, stream: TokenStream) -> TokenStream {
    let (args, mut item) = parse_fn!(attributes, stream);

    let ident = replace_ident(&mut item.sig, "__inner_fn");
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

fn parse_fn(
    attrs: TokenStream,
    stream: TokenStream,
) -> Result<(MacroArgs, ItemFn), TokenStream> {
    let args = syn::parse_macro_input::parse::<AttributeArgs>(attrs)
        .expect_or_abort("Failed to parse attribute args");
    let item = syn::parse_macro_input::parse::<ItemFn>(stream)
        .expect_or_abort("Failed to parse function");

    let args = match MacroArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return Err(e.write_errors().into());
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
    if let Some(async_) = item.sig.asyncness {
        abort! {
            async_, "Async handlers are not supported";
            help = "Remove async from function signature declaration"
        }
    }

    Ok((args, item))
}

fn replace_ident(sig: &mut Signature, with: &str) -> Ident {
    let prev = sig.ident.clone();
    sig.ident = Ident::new(with, prev.span());

    prev
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn device(
    _attributes: TokenStream,
    input: TokenStream,
) -> TokenStream {
    let input = syn::parse_macro_input::parse::<syn::DeriveInput>(input)
        .expect_or_abort("Failed to parse derive input");

    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields:
                syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        }) => unnamed,
        _ => abort! {
            input, "Device can only be derived for structs with a single unnamed field"
        },
    };
    let field_ty = match fields.into_iter().collect::<Vec<_>>().as_slice()
    {
        [field] => field.ty.clone(),
        _ => abort! {
            input, "Device can only be derived for structs with a single unnamed field"
        },
    };

    let ident = &input.ident;

    quote! {
        #[repr(transparent)]
        struct #ident {
            dev: rvvm_sys::rvvm_mmio_dev_t,
        }

        unsafe impl DeviceData for Uart {
            type Ty = #field_ty;

            fn data(&self) -> &Self::Ty {
                unsafe { &*(self.dev.data as *const Self::Ty) }
            }

            fn data_mut(&mut self) -> &mut Self::Ty {
                unsafe { &mut *(self.dev.data as *mut Self::Ty) }
            }
        }
    }
    .into()
}

#[proc_macro_derive(DeviceTypeExt)]
#[proc_macro_error]
pub fn device_type_ext(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input::parse::<syn::DeriveInput>(input)
        .expect_or_abort("Failed to parse derive input");
    let ty = input.ident;

    let remove_impl = device_impl_func(&ty, "remove");
    let update_impl = device_impl_func(&ty, "update");
    let reset_impl = device_impl_func(&ty, "reset");
    quote! {
        unsafe impl DeviceTypeExt for #ty {
            fn new() -> Self {
                const q: rvvm::ffi::rvvm_mmio_type_t = {
                    #remove_impl
                    #update_impl
                    #reset_impl
                    rvvm::ffi::rvvm_mmio_type_t {
                        remove: Some(remove),
                        update: Some(update),
                        reset: Some(reset),
                        name: 0 as *const std::os::raw::c_char, // TODO
                    }
                };
                Self(q)
            }
        }
    }
    .into()
}

fn device_impl_func(ty: &syn::Ident, func: &str) -> TokenStream2 {
    let func = Ident::new(func, Span::call_site());
    quote! {
        unsafe extern "C" fn #func(dev: *mut rvvm::ffi::rvvm_mmio_dev_t) {
            let dev = &mut *(dev as *mut <#ty as rvvm::dev::type_::DeviceType>::Device);
            <#ty as rvvm::dev::type_::DeviceType>::#func(dev);
        }
    }
}
