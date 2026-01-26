use crate::{
    IdentGen,
    ast::child::Tokenize,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Lifetime,
    Result as SynResult,
    Token,
    parse::{
        Parse,
        ParseStream,
    },
};

pub(crate) struct CtrlExprBreak {
    break_token: Token![break],
    label: Option<Lifetime>,
    semi_token: Token![;],
}

impl Parse for CtrlExprBreak {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(Self {
            break_token: input.parse()?,
            label: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Tokenize for CtrlExprBreak {
    fn tokenize(
        &self,
        _ident_gen: &mut IdentGen,
        _parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        let CtrlExprBreak {
            label,
            break_token,
            semi_token,
        } = self;
        quote! { #break_token #label #semi_token }
    }
}
