use proc_macro::TokenStream;
use proc_macro_error::{
    abort,
    OptionExt,
    ResultExt,
};
use syn::{
    Lit,
    NestedMeta,
};

pub struct Args {
    pub data_type: syn::Type,
}

pub fn parse_args(stream: TokenStream) -> Args {
    let mut data_type: Option<syn::Type> = None;
    let args: syn::AttributeArgs = syn::parse_macro_input::parse(stream)
        .expect_or_abort("Invalid attribute args");

    for arg in args {
        match arg {
            NestedMeta::Meta(meta) => abort! {
                meta, "Meta-symbols are not supported";
                help = "Consider specifying string literal"
            },

            NestedMeta::Lit(lit) => match lit {
                Lit::Str(litstr) => {
                    let ty: syn::Type = litstr
                        .parse()
                        .expect_or_abort("Input must be valid type");
                    data_type = Some(ty);
                }
                _ => abort! {
                    lit, "Literal is not supported";
                    help = "Consideer specifying string literal"
                },
            },
        }
    }

    Args {
        data_type: data_type
            .expect_or_abort("Consider specifying data type"),
    }
}
