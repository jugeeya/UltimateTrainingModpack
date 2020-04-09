use quote::{ToTokens, quote};
use proc_macro::TokenStream;
use syn::{parse_macro_input, token, Ident, AttrStyle};
use proc_macro2::{Span, TokenStream as TokenStream2};


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
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut main_function = parse_macro_input!(item as syn::ItemFn);

    main_function.attrs.push(
        new_attr("no_mangle")
    );
    
    main_function.sig.ident = Ident::new("main", Span::call_site());

    let mut output = TokenStream2::new();

    quote!(
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
