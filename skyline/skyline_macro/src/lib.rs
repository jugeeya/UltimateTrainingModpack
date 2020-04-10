use quote::{ToTokens, quote};
use proc_macro::TokenStream;
use syn::{parse_macro_input, token, Ident, AttrStyle, Lit, spanned::Spanned};
use proc_macro2::{Span, TokenStream as TokenStream2};

mod attributes;

fn new_attr(attr_name: &str) -> syn::Attribute {
    syn::Attribute {
        pound_token: token::Pound { spans: [Span::call_site()] },
        style: AttrStyle::Outer,
        bracket_token: token::Bracket { span: Span::call_site() },
        path: Ident::new(attr_name, Span::call_site()).into(),
        tokens: TokenStream2::new()
    }
}

#[proc_macro_attribute]
pub fn main(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut main_function = parse_macro_input!(item as syn::ItemFn);

    let attr_code = parse_macro_input!(attrs as attributes::Attrs);

    main_function.attrs.push(
        new_attr("no_mangle")
    );
    
    main_function.sig.ident = Ident::new("main", Span::call_site());

    let mut output = TokenStream2::new();

    quote!(
        #attr_code
        use skyline::prelude::*;
        ::skyline::setup!();
    ).to_tokens(&mut output);
    main_function.to_tokens(&mut output);

    output.into()
}

#[proc_macro_attribute]
pub fn hook(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut mod_fn = parse_macro_input!(input as syn::ItemFn);
    let mut output = TokenStream2::new();

    // #[no_mangle]
    mod_fn.attrs.push(
        new_attr("no_mangle")
    );

    // extern "C"
    mod_fn.sig.abi = Some(syn::Abi {
        extern_token: syn::token::Extern { span: Span::call_site() },
        name: Some(syn::LitStr::new("C", Span::call_site()))
    });

    mod_fn.to_tokens(&mut output);

    let mod_fn = mod_fn.sig.ident;

    let _info = quote::format_ident!(
        "{}_skyline_internal_hook_info",
        mod_fn
    );

    let _hook = quote::format_ident!(
        "{}_skyline_internal_hook",
        mod_fn
    );

    quote!(
        /*#[allow(non_upper_case_globals)]
        static #info: ::skyline::hooks::HookInfo = ::skyline::hooks::HookInfo {
            name: None,
            fn_name: stringify!(#mod_fn),
            offset: None,
            symbol: None,
            inline: false
        };
        #[allow(non_upper_case_globals)]
        #[link_section = ".rodata.hooks"]
        static #hook: ::skyline::hooks::Hook = ::skyline::hooks::Hook{
            ptr: #mod_fn as *const (),
            info: &#info
        };*/
    ).to_tokens(&mut output);

    output.into()
}

fn lit_to_bytes(lit: &Lit) -> Option<Vec<u8>> {
    match lit {
        Lit::Str(lit_str) => {
            Some(lit_str.value().into_bytes())
        }
        Lit::ByteStr(lit_str) => {
            Some(lit_str.value())
        }
        _ => {
            None
        }
    }
}

#[proc_macro]
pub fn crc32(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Lit);

    match lit_to_bytes(&expr) {
        Some(bytes) => {
            let crc = crc::crc32::checksum_ieee(&bytes);
            
            TokenStream::from(quote! {
                (#crc)
            })
        }
        None => {
            let span = expr.span();
            TokenStream::from(quote::quote_spanned!{span =>
                compile_error!("Invalid literal");
            })
        }
    }
    
}

#[proc_macro]
pub fn to_null_term_bytes(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Lit);

    match lit_to_bytes(&expr) {
        Some(mut bytes) => {
            bytes.push(0);

            let bytes = syn::LitByteStr::new(&bytes, expr.span());

            TokenStream::from(quote! {
                (#bytes)
            })
        }
        None => {
            let span = expr.span();
            TokenStream::from(quote::quote_spanned!{span =>
                compile_error!("Invalid literal");
            })
        }
    }
}
