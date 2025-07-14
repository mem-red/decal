use super::{CtrlExpr, TokenGenMode};
use crate::{
    IdentGen,
    ast::child::{NodeChild, Tokenize, parse_children},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Token, braced, parse::Parse, token};

pub struct CtrlExprIf {
    if_token: Token![if],
    cond: Box<Expr>,
    then_branch: Vec<NodeChild>,
    else_branch: Option<(Token![else], Vec<NodeChild>)>,
}

impl Parse for CtrlExprIf {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let if_token: Token![if] = input.parse()?;
        let cond = input.call(Expr::parse_without_eager_brace)?;

        let content;
        braced!(content in input);
        let then_branch = parse_children(&content)?;

        let mut expr = CtrlExprIf {
            if_token,
            cond: Box::new(cond),
            then_branch,
            else_branch: None,
        };

        if input.peek(Token![else]) {
            let else_token: Token![else] = input.parse()?;
            let lookahead = input.lookahead1();

            if lookahead.peek(Token![if]) {
                let nested_if = CtrlExpr::If(Self::parse(input)?);
                expr.else_branch = Some((else_token, vec![NodeChild::CtrlExpr(nested_if)]));
            } else if lookahead.peek(token::Brace) {
                let content;
                braced!(content in input);
                let else_branch = parse_children(&content)?;
                expr.else_branch = Some((else_token, else_branch));
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(expr)
    }
}

impl Tokenize for CtrlExprIf {
    fn tokenize(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        let CtrlExprIf {
            if_token,
            cond,
            then_branch,
            else_branch,
        } = self;
        let then_branch_tokens = then_branch
            .iter()
            .map(|child| child.tokenize(mode, ident_gen, parent_token))
            .collect::<Vec<_>>();

        let else_tokens = match else_branch {
            Some((else_token, else_children)) => {
                if else_children.len() == 1
                    && matches!(
                        else_children.first(),
                        Some(NodeChild::CtrlExpr(CtrlExpr::If(_)))
                    )
                {
                    // Simple else-if statement
                    let nested_else_if_tokens =
                        else_children[0].tokenize(mode, ident_gen, parent_token);
                    quote! {
                        #else_token #nested_else_if_tokens
                    }
                } else {
                    // Multiple children in else branch, wrap in braces
                    let else_branch_tokens = else_children
                        .iter()
                        .map(|child| child.tokenize(mode, ident_gen, parent_token));
                    quote! {
                        #else_token {
                            #(#else_branch_tokens)*
                        }
                    }
                }
            }
            _ => quote! {},
        };

        quote! {
            #if_token #cond {
                #(#then_branch_tokens)*
            }
            #else_tokens
        }
    }
}
