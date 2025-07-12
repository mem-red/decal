extern crate proc_macro;
mod ast;
mod ident_gen;

use ast::tree::DecalTree;
use ident_gen::IdentGen;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn decal(input: TokenStream) -> TokenStream {
    let DecalTree { root } = parse_macro_input!(input as DecalTree);
    let mut ident_gen = IdentGen::new();
    let mut root_found = false;
    let expanded = root.to_tokens(&mut ident_gen, None, &mut root_found);
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn decal_fragment(input: TokenStream) -> TokenStream {
    let DecalTree { root } = parse_macro_input!(input as DecalTree);
    let mut ident_gen = IdentGen::new();
    let expanded = root.to_tokens_partial(&mut ident_gen, None);
    TokenStream::from(expanded)
}
