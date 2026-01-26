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
    Expr,
    Label,
    Pat,
    Result as SynResult,
    Token,
    braced,
    parse::{
        Parse,
        ParseStream,
    },
};

pub(crate) struct CtrlExprForLoop {
    label: Option<Label>,
    for_token: Token![for],
    pat: Box<Pat>,
    in_token: Token![in],
    expr: Box<Expr>,
    body: Vec<NodeChild>,
}

impl Parse for CtrlExprForLoop {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let label: Option<Label> = input.parse()?;
        let for_token: Token![for] = input.parse()?;
        let pat = Pat::parse_multi_with_leading_vert(input)?;
        let in_token: Token![in] = input.parse()?;
        let expr = input.call(Expr::parse_without_eager_brace)?;

        let content;
        braced!(content in input);
        let body = parse_children(&content)?;

        Ok(Self {
            label,
            for_token,
            pat: Box::new(pat),
            in_token,
            expr: Box::new(expr),
            body,
        })
    }
}

impl Tokenize for CtrlExprForLoop {
    fn tokenize(
        &self,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        let CtrlExprForLoop {
            label,
            for_token,
            pat,
            in_token,
            expr,
            body,
        } = self;
        let child_tokens = body
            .iter()
            .map(|child| child.tokenize(ident_gen, parent_token));

        quote! {
            #label #for_token #pat #in_token #expr {
                #(#child_tokens)*
            }
        }
    }
}
