use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort;
use syn::{
    AttributeArgs,
    DeriveInput,
    NestedMeta,
    Path,
};

// Integral types's layout is specified
fn is_integral(pat: &Path) -> bool {
    ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
        .into_iter()
        .any(|val| pat.is_ident(val))
}

fn is_repr_valid(meta: AttributeArgs) -> bool {
    for nested in meta {
        match nested {
            lit @ NestedMeta::Lit(..) => abort! {
                lit, "Literals are not supported"
            },

            NestedMeta::Meta(meta) => {
                let pat = meta.path();

                if is_integral(pat) || pat.is_ident("C") {
                    return true;
                } else if pat.is_ident("align") {
                    continue;
                } else if pat.is_ident("Rust") {
                    abort! {
                        meta, "#[repr(Rust)] is not supported";
                        note = "#[repr(Rust)] types's layout is not specified"
                    }
                } else if pat.is_ident("packed") {
                    abort! {
                        meta, "#[repr(packed)] is not supported"
                    }
                }
            }
        }
    }

    false
}

pub fn known_layout_impl(stream: TokenStream) -> TokenStream {
    let derive = syn::parse_macro_input!(stream as DeriveInput);
    let Some(repr_attr) = derive
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("repr"))
        .cloned()
        .next()
    else {
        abort! {
            derive, "#[repr(C)] attribute must be present"
        }
    };

    let Ok(args) = repr_attr.parse_args::<TokenStream2>()
        .and_then(
            |args| syn::parse_macro_input::parse::<AttributeArgs>(args.into())
        ) else {
            abort! {
                repr_attr, "Failed to parse #[repr(..)] attribute"
            }
        };

    let ident = derive.ident;
    if is_repr_valid(args) {
        quote::quote! {
            unsafe impl ::rvvm::types::KnownLayout for #ident {}
        }
        .into()
    } else {
        abort! {
            repr_attr, "Type layout is not precisely specified";
            note = "specify the #[repr(C)] attribute (alignment also supported)"
        }
    }
}
