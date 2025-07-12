use crate::{
    IdentGen,
    ast::child::{NodeChild, parse_children},
};
use proc_macro2::{Ident as PM2Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Block, Expr, Label, Lifetime, Pat, Result as SynResult, Token, braced,
    parse::{Parse, ParseStream},
    token,
};

#[derive(Debug)]
pub enum TokenGenMode<'a> {
    Full { root_found: &'a mut bool },
    Partial,
}

pub struct CtrlExprIf {
    pub if_token: Token![if],
    pub cond: Box<Expr>,
    pub then_branch: Vec<NodeChild>,
    pub else_branch: Option<(Token![else], Vec<NodeChild>)>,
}

pub struct CtrlExprForLoop {
    pub label: Option<Label>,
    pub for_token: Token![for],
    pub pat: Box<Pat>,
    pub in_token: Token![in],
    pub expr: Box<Expr>,
    pub body: Vec<NodeChild>,
}

pub enum CtrlExpr {
    If(CtrlExprIf),
    ForLoop(CtrlExprForLoop),
    Snipped(Block),
    NotAnExpr, // Pass-through for rendering custom nodes.
}

// Parsing

impl Parse for CtrlExpr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![for])
            || (input.peek(Lifetime) && input.peek2(Token![:]) && input.peek3(Token![for]))
        {
            Self::parse_for_loop_expr(input)
            // TODO: Improve below condition
        } else if input.peek(Token![if]) {
            Self::parse_if_expr(input)
        } else {
            Ok(Self::NotAnExpr)
        }
    }
}

impl CtrlExpr {
    fn parse_if_expr(input: ParseStream) -> SynResult<Self> {
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
                let nested_if = Self::parse_if_expr(input)?;
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

        Ok(Self::If(expr))
    }

    fn parse_for_loop_expr(input: ParseStream) -> SynResult<Self> {
        let label: Option<Label> = input.parse()?;
        let for_token: Token![for] = input.parse()?;
        let pat = Pat::parse_multi_with_leading_vert(input)?;
        let in_token: Token![in] = input.parse()?;
        let expr = input.call(Expr::parse_without_eager_brace)?;

        let content;
        braced!(content in input);
        let body = parse_children(&content)?;

        Ok(Self::ForLoop(CtrlExprForLoop {
            label,
            for_token,
            pat: Box::new(pat),
            in_token,
            expr: Box::new(expr),
            body,
        }))
    }
}

// Token stream conversion

impl CtrlExpr {
    pub fn to_tokens_with_mode(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_tkn: Option<&PM2Ident>,
    ) -> TokenStream {
        match self {
            CtrlExpr::ForLoop(for_loop_expr) => {
                Self::tokenize_for_loop_expr(for_loop_expr, mode, ident_gen, parent_tkn)
            }
            CtrlExpr::If(if_expr) => Self::tokenize_if_expr(if_expr, mode, ident_gen, parent_tkn),
            CtrlExpr::Snipped(block) => {
                let stmts_tokens = block.stmts.iter().map(|stmt| stmt.to_token_stream());
                quote! { #(#stmts_tokens)* }
            }
            CtrlExpr::NotAnExpr => unreachable!(),
        }
    }

    // Private

    fn tokenize_if_expr(
        if_expr: &CtrlExprIf,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_tkn: Option<&PM2Ident>,
    ) -> TokenStream {
        let CtrlExprIf {
            if_token,
            cond,
            then_branch,
            else_branch,
        } = if_expr;
        let then_branch_tokens = then_branch
            .iter()
            .map(|child| child.to_tokens_with_mode(mode, ident_gen, parent_tkn))
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
                        else_children[0].to_tokens_with_mode(mode, ident_gen, parent_tkn);

                    quote! {
                        #else_token #nested_else_if_tokens
                    }
                } else {
                    // Multiple children in else branch, wrap in braces
                    let else_branch_tokens = else_children
                        .iter()
                        .map(|child| child.to_tokens_with_mode(mode, ident_gen, parent_tkn));

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

    fn tokenize_for_loop_expr(
        for_loop_expr: &CtrlExprForLoop,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_tkn: Option<&PM2Ident>,
    ) -> TokenStream {
        let CtrlExprForLoop {
            label,
            for_token,
            pat,
            in_token,
            expr,
            body,
        } = for_loop_expr;
        let child_tokens = body
            .iter()
            .map(|child| child.to_tokens_with_mode(mode, ident_gen, parent_tkn));

        quote! {
            #label #for_token #pat #in_token #expr {
                #(#child_tokens)*
            }
        }
    }
}
