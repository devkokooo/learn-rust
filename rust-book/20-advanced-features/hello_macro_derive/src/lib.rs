use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    ItemFn, Ident, LitStr, Result, Token, parse_macro_input
};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl ::hello_macro::HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}

struct RouteArgs {
    method: Ident,
    _comma: Token![,],
    path: LitStr,
}

impl Parse for RouteArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            method: input.parse()?,
            _comma: input.parse()?,
            path: input.parse()?,
        })
    }
}

// attribute-like macros
#[proc_macro_attribute]
/// example attribute-like macro for a hypothetical route attribute
/// for an HTTP server to handle routing
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as RouteArgs);
    let function = parse_macro_input!(item as ItemFn);
    
    let method = args.method;
    let path = args.path;
    let fn_name = &function.sig.ident;

    // emit the original function unchanged, plus registration metadata
    quote! {
        #function

        fn __dispatch_route() {
            #fn_name();
        }

        const _: (&str, &str) = (stringify!(#method), #path);
    }.into()
}

// function-like macros
#[proc_macro]
/// example function-like macro for sql!
pub fn sql(input: TokenStream) -> TokenStream {
    input.into()
}
