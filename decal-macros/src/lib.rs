extern crate proc_macro;
mod ast;
mod ident_gen;

use ast::{child::Tokenize, ctrl_expr::TokenGenMode, tree::DecalTree};
use ident_gen::IdentGen;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn decal(input: TokenStream) -> TokenStream {
    let mut root_found = false;
    let mut mode = TokenGenMode::Full {
        root_found: &mut root_found,
    };
    decal_impl(input, &mut mode)
}

#[proc_macro]
pub fn fragment(input: TokenStream) -> TokenStream {
    let mut mode = TokenGenMode::Partial;
    decal_impl(input, &mut mode)
}

fn decal_impl(input: TokenStream, mode: &mut TokenGenMode) -> TokenStream {
    let DecalTree { root } = parse_macro_input!(input as DecalTree);
    let mut ident_gen = IdentGen::new();
    let expanded = root.tokenize(mode, &mut ident_gen, None);
    TokenStream::from(expanded)
}
