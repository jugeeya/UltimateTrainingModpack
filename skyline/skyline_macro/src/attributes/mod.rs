use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub struct Attrs {
    name: String,

}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let meta: syn::MetaNameValue = match input.parse() {
            Ok(x) => x,
            Err(_) => return Ok(Attrs { name: "skyline_rust_plugin".into() })
        };

        if meta.path.get_ident().unwrap().to_string() == "name" {
            match meta.lit {
                syn::Lit::Str(string) => {
                    Ok(Attrs {
                        name: string.value()
                    })
                }
                _ => panic!("Invalid literal, must be a string")
            }
        } else {
            panic!("Attributes other than 'name' not allowed");
        }
    }
}

impl ToTokens for Attrs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name[..];
        quote::quote!(
            ::skyline::set_module_name!(#name);
        ).to_tokens(tokens);
    }
}
