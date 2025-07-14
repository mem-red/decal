use crate::{
    IdentGen,
    ast::child::{NodeChild, parse_children},
};
use proc_macro2::{Ident as PM2Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Expr, Label, Lifetime, Pat, Result as SynResult, Stmt, Token, braced,
    parse::{Parse, ParseStream},
    token,
};

#[derive(Debug)]
pub enum TokenGenMode<'a> {
    Full { root_found: &'a mut bool },
    Partial,
}

// If

pub struct CtrlExprIf {
    pub if_token: Token![if],
    pub cond: Box<Expr>,
    pub then_branch: Vec<NodeChild>,
    pub else_branch: Option<(Token![else], Vec<NodeChild>)>,
}

// Match

pub struct MatchArm {
    pub pat: Pat,
    pub guard: Option<(Token![if], Box<Expr>)>,
    pub fat_arrow_token: Token![=>],
    pub body: NodeChild,
    pub comma: Option<Token![,]>,
}

pub struct CtrlExprMatch {
    pub match_token: Token![match],
    pub expr: Box<Expr>,
    pub brace_token: token::Brace,
    pub arms: Vec<MatchArm>,
}

// For loop

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
    Match(CtrlExprMatch),
    ForLoop(CtrlExprForLoop),
    NotAnExpr, // Pass-through for rendering custom nodes.
}

// Parsing

impl Parse for CtrlExpr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // if input.peek(Token![while]) {
        //     Expr::While(input.parse()?)
        // } else if input.peek(Token![for])
        //     && !(input.peek2(Token![<]) && (input.peek3(Lifetime) || input.peek3(Token![>])))
        // {
        //     Expr::ForLoop(input.parse()?)
        // } else if input.peek(Token![loop]) {
        //     Expr::Loop(input.parse()?)
        // } else if input.peek(Token![match]) {
        //     Expr::Match(input.parse()?)
        // } else if input.peek(Token![try]) && input.peek2(token::Brace) {
        //     Expr::TryBlock(input.parse()?)
        // }

        if input.peek(Token![if]) {
            Self::parse_if_expr(input)
        } else if input.peek(Token![match]) {
            Self::parse_match_expr(input)
        } else if input.peek(Token![for])
            || (input.peek(Lifetime) && input.peek2(Token![:]) && input.peek3(Token![for]))
        {
            Self::parse_for_loop_expr(input)
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

    fn parse_match_expr(input: ParseStream) -> SynResult<Self> {
        let match_token: Token![match] = input.parse()?;
        let expr = Expr::parse_without_eager_brace(input)?;

        let content;
        let brace_token = braced!(content in input);
        let arms = MatchArm::parse_multiple(&content)?;

        Ok(Self::Match(CtrlExprMatch {
            match_token,
            expr: Box::new(expr),
            brace_token,
            arms,
        }))
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

impl Parse for MatchArm {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let requires_comma;
        Ok(Self {
            pat: Pat::parse_multi_with_leading_vert(input)?,
            guard: {
                if input.peek(Token![if]) {
                    let if_token: Token![if] = input.parse()?;
                    let guard: Expr = input.parse()?;
                    Some((if_token, Box::new(guard)))
                } else {
                    None
                }
            },
            fat_arrow_token: input.parse()?,
            body: {
                let body = input.parse()?;
                requires_comma = Self::body_requires_comma(&body);
                body
            },
            comma: {
                if requires_comma && !input.is_empty() {
                    Some(input.parse()?)
                } else {
                    input.parse()?
                }
            },
        })
    }
}

impl MatchArm {
    fn parse_multiple(input: ParseStream) -> SynResult<Vec<Self>> {
        let mut arms = Vec::new();
        while !input.is_empty() {
            arms.push(input.call(MatchArm::parse)?);
        }
        Ok(arms)
    }

    fn body_requires_comma(body: &NodeChild) -> bool {
        matches!(body, NodeChild::Node(_))
    }

    fn requires_comma(&self) -> bool {
        matches!(self.body, NodeChild::Node(_))
    }

    pub fn to_tokens_with_mode(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&PM2Ident>,
        tokens: &mut TokenStream,
    ) {
        self.pat.to_tokens(tokens);
        if let Some((if_token, guard)) = &self.guard {
            if_token.to_tokens(tokens);
            guard.to_tokens(tokens);
        }
        self.fat_arrow_token.to_tokens(tokens);
        token::Brace::default().surround(tokens, |tokens| {
            self.body
                .to_tokens_with_mode(mode, ident_gen, parent_token)
                .to_tokens(tokens)
        });
        self.comma.to_tokens(tokens);
    }
}

// Token stream conversion

impl CtrlExpr {
    pub fn to_tokens_with_mode(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&PM2Ident>,
    ) -> TokenStream {
        match self {
            CtrlExpr::If(if_expr) => Self::tokenize_if_expr(if_expr, mode, ident_gen, parent_token),
            CtrlExpr::Match(match_expr) => {
                Self::tokenize_match_expr(match_expr, mode, ident_gen, parent_token)
            }
            CtrlExpr::ForLoop(for_loop_expr) => {
                Self::tokenize_for_loop_expr(for_loop_expr, mode, ident_gen, parent_token)
            }
            CtrlExpr::NotAnExpr => unreachable!(),
        }
    }

    // Private

    fn tokenize_if_expr(
        if_expr: &CtrlExprIf,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&PM2Ident>,
    ) -> TokenStream {
        let CtrlExprIf {
            if_token,
            cond,
            then_branch,
            else_branch,
        } = if_expr;
        let then_branch_tokens = then_branch
            .iter()
            .map(|child| child.to_tokens_with_mode(mode, ident_gen, parent_token))
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
                        else_children[0].to_tokens_with_mode(mode, ident_gen, parent_token);

                    quote! {
                        #else_token #nested_else_if_tokens
                    }
                } else {
                    // Multiple children in else branch, wrap in braces
                    let else_branch_tokens = else_children
                        .iter()
                        .map(|child| child.to_tokens_with_mode(mode, ident_gen, parent_token));

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

    fn tokenize_match_expr(
        match_expr: &CtrlExprMatch,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&PM2Ident>,
    ) -> TokenStream {
        let CtrlExprMatch {
            match_token,
            expr,
            brace_token,
            arms,
        } = match_expr;

        let mut tokens = TokenStream::new();
        match_token.to_tokens(&mut tokens);
        expr.to_tokens(&mut tokens);

        brace_token.surround(&mut tokens, |tokens| {
            for (i, arm) in arms.iter().enumerate() {
                arm.to_tokens_with_mode(mode, ident_gen, parent_token, tokens);
                let is_last = i == arms.len() - 1;
                if !is_last && arm.requires_comma() && arm.comma.is_none() {
                    <Token![,]>::default().to_tokens(tokens);
                }
            }
        });

        tokens
    }

    fn tokenize_for_loop_expr(
        for_loop_expr: &CtrlExprForLoop,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&PM2Ident>,
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
            .map(|child| child.to_tokens_with_mode(mode, ident_gen, parent_token));

        quote! {
            #label #for_token #pat #in_token #expr {
                #(#child_tokens)*
            }
        }
    }
}
