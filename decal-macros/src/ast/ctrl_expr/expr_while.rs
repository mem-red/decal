use super::TokenGenMode;
use crate::{
    IdentGen,
    ast::child::{NodeChild, Tokenize, parse_children},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Expr, Label, Result as SynResult, Token, braced,
    parse::{Parse, ParseStream},
};

pub(crate) struct CtrlExprWhile {
    label: Option<Label>,
    while_token: Token![while],
    cond: Box<Expr>,
    body: Vec<NodeChild>,
}

impl Parse for CtrlExprWhile {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let label: Option<Label> = input.parse()?;
        let while_token: Token![while] = input.parse()?;
        let cond = Expr::parse_without_eager_brace(input)?;

        let content;
        braced!(content in input);
        let body = parse_children(&content)?;

        Ok(Self {
            label,
            while_token,
            cond: Box::new(cond),
            body,
        })
    }
}

impl Tokenize for CtrlExprWhile {
    fn tokenize(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        let CtrlExprWhile {
            label,
            while_token,
            cond,
            body,
        } = self;
        let child_tokens = body
            .iter()
            .map(|child| child.tokenize(mode, ident_gen, parent_token));

        quote! {
            #label #while_token #cond {
                #(#child_tokens)*
            }
        }
    }
}
