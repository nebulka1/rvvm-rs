use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{
    abort,
    proc_macro_error,
};
use quote::quote;
use syn::{
    punctuated::Punctuated,
    Fields,
    TypeTuple,
};

fn unit() -> syn::Type {
    syn::Type::Tuple(TypeTuple {
        paren_token: syn::token::Paren {
            span: Span::call_site(),
        },
        elems: Punctuated::new(),
    })
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn device(_attrs: TokenStream, stream: TokenStream) -> TokenStream {
    let dev = syn::parse_macro_input!(stream as syn::ItemStruct);

    let ident = dev.ident;
    let vis = dev.vis;

    let ty: syn::Type = match dev.fields {
        Fields::Unit => unit(),
        Fields::Unnamed(unnamed) => {
            let mut it = unnamed.unnamed.iter();
            let first = it.next();
            let second = it.next();

            match (first, second) {
                (Some(field), None) => field.ty.clone(),
                (None, None) => unit(),

                _ => abort! {
                    unnamed, "Struct must be tuple-like with single \
                              field or unit-like or tuple-like with no fields"
                },
            }
        }

        fields @ Fields::Named(_) => abort! {
            fields, "Device struct can only be created with the single unnamed field or without fields";
            help = "Create unit or tuple-like structure with the single field"
        },
    };

    quote! {
        #[repr(transparent)]
        #vis struct #ident {
            inner: ::rvvm::types::RawDevice<#ty>,
        }

        unsafe impl ::rvvm::dev::mmio::DeviceData for #ident {
            type Ty = #ty;

            fn data(&self) -> &Self::Ty {
                unsafe { self.inner.data() }
            }

            fn data_mut(&mut self) -> &mut Self::Ty {
                unsafe { self.inner.data_mut() }
            }
        }

        impl ::rvvm::dev::mmio::DeviceExt for #ident {
            type DataTy = #ty;

            fn new(
                address: u64,
                size: usize,
                op_size_range: ::core::ops::RangeInclusive<u8>,
                data: Self::DataTy,
            ) -> Self {
                type Box<T> = ::std::boxed::Box<T>;

                let min_op_size = *op_size_range.start();
                let max_op_size = *op_size_range.end();

                let data_boxed: Box<Self::DataTy> = Box::new(data);
                let data_voidptr: *mut ::core::ffi::c_void =
                    unsafe { Box::into_raw(data_boxed) } as *mut () as *mut _;

                Self {
                    inner: unsafe { ::rvvm::types::RawDevice::<Self::DataTy>::new(
                        ::rvvm::ffi::rvvm_mmio_dev_t {
                            addr: address,
                            size,

                            min_op_size, max_op_size,

                            read: None,
                            write: None,

                            data: data_voidptr,

                            machine: ::core::ptr::null_mut(),
                            type_: ::core::ptr::null_mut(),
                        }
                    ) }
                }
            }
        }
    }
    .into()
}
