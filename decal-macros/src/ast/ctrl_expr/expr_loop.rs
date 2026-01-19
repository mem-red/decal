use super::TokenGenMode;
use crate::{
    IdentGen,
    ast::child::{
        NodeChild,
        Tokenize,
        parse_children,
    },
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Label,
    Result as SynResult,
    Token,
    braced,
    parse::{
        Parse,
        ParseStream,
    },
};

pub(crate) struct CtrlExprLoop {
    label: Option<Label>,
    loop_token: Token![loop],
    body: Vec<NodeChild>,
}

impl Parse for CtrlExprLoop {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let label: Option<Label> = input.parse()?;
        let loop_token: Token![loop] = input.parse()?;

        let content;
        braced!(content in input);
        let body = parse_children(&content)?;

        Ok(Self {
            label,
            loop_token,
            body,
        })
    }
}

impl Tokenize for CtrlExprLoop {
    fn tokenize(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        let CtrlExprLoop {
            label,
            loop_token,
            body,
        } = self;
        let child_tokens = body
            .iter()
            .map(|child| child.tokenize(mode, ident_gen, parent_token));

        quote! {
            #label #loop_token {
                #(#child_tokens)*
            }
        }
    }
}
