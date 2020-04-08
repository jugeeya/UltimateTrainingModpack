use quote::{ToTokens, quote};
use proc_macro::TokenStream;
use syn::{parse_macro_input, token, Token, Ident, Path, AttrStyle, punctuated::Punctuated};
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::parse::Parser;


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
    let mod_fn = parse_macro_input!(input as syn::ItemFn);
    let mut output = TokenStream2::new();

    mod_fn.to_tokens(&mut output);

    let ident = quote::format_ident!(
        "{}_skyline_internal_hook_info",
        mod_fn.sig.ident
    );

    quote!(
        #[allow(non_upper_case_globals)]
        const #ident: ::skyline::hooks::HookInfo = ::skyline::hooks::HookInfo {
            name: None,
            offset: None,
            symbol: None,
            inline: false
        };
    ).to_tokens(&mut output);

    output.into()
}

fn parse_hook_fns(input: TokenStream) -> syn::Result<Vec<Path>> {
    Ok(
        Punctuated::<Path, Token![,]>::parse_terminated
            .parse(input)?
            .into_iter()
            .collect()
    )
}

fn concat_path(path: &Path) -> Path {
    let mut path = path.clone();
    
    let last = path.segments.iter_mut().last().unwrap();

    last.ident = quote::format_ident!("{}_skyline_internal_hook_info", last.ident);

    path
}

#[proc_macro]
pub fn hooks(tokens: TokenStream) -> TokenStream {
    parse_hook_fns(tokens)
        .map(|hook_fns|{
            let hook_fn_infos = hook_fns.iter().map(concat_path);
            quote!{
                ::skyline::hooks::Hooks(::skyline::alloc::vec![
                    #(
                        ::skyline::new_hook!(
                            #hook_fns,
                            #hook_fn_infos
                        )
                    ),*
                ])
            }
        })
        .unwrap_or_else(|e|{
            e.to_compile_error()
        })
        .into()
}
