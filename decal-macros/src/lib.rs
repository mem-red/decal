extern crate proc_macro;
mod ast;
mod ident_gen;
mod text;

use crate::text::text_impl;
use ast::{
    child::Tokenize,
    tree::SceneTree,
};
use ident_gen::IdentGen;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn decal(input: TokenStream) -> TokenStream {
    decal_impl(input)
}

#[proc_macro]
pub fn text(input: TokenStream) -> TokenStream {
    text_impl(input)
}

fn decal_impl(input: TokenStream) -> TokenStream {
    let SceneTree { root } = parse_macro_input!(input as SceneTree);
    let mut ident_gen = IdentGen::new();
    TokenStream::from(root.tokenize(&mut ident_gen, None))
}
