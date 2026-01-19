use super::TokenGenMode;
use crate::{
    IdentGen,
    ast::child::{
        NodeChild,
        Tokenize,
    },
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Expr,
    Pat,
    Result as SynResult,
    Token,
    braced,
    parse::{
        Parse,
        ParseStream,
    },
    token,
};

pub(crate) struct CtrlExprMatch {
    match_token: Token![match],
    expr: Box<Expr>,
    brace_token: token::Brace,
    arms: Vec<MatchArm>,
}

impl Parse for CtrlExprMatch {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let match_token: Token![match] = input.parse()?;
        let expr = Expr::parse_without_eager_brace(input)?;

        let content;
        let brace_token = braced!(content in input);
        let arms = MatchArm::parse_multiple(&content)?;

        Ok(Self {
            match_token,
            expr: Box::new(expr),
            brace_token,
            arms,
        })
    }
}

impl Tokenize for CtrlExprMatch {
    fn tokenize(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        let CtrlExprMatch {
            match_token,
            expr,
            brace_token,
            arms,
        } = self;
        let mut tokens = TokenStream::new();
        match_token.to_tokens(&mut tokens);
        expr.to_tokens(&mut tokens);
        brace_token.surround(&mut tokens, |tokens| {
            for (i, arm) in arms.iter().enumerate() {
                arm.tokenize(mode, ident_gen, parent_token)
                    .to_tokens(tokens);
                let is_last = i == arms.len() - 1;
                if !is_last && arm.requires_comma() && arm.comma.is_none() {
                    <Token![,]>::default().to_tokens(tokens);
                }
            }
        });
        tokens
    }
}

// Arm

struct MatchArm {
    pat: Pat,
    guard: Option<(Token![if], Box<Expr>)>,
    fat_arrow_token: Token![=>],
    body: NodeChild,
    comma: Option<Token![,]>,
}

impl MatchArm {
    fn parse_multiple(input: ParseStream) -> SynResult<Vec<Self>> {
        let mut arms = Vec::new();
        while !input.is_empty() {
            arms.push(input.call(MatchArm::parse)?);
        }
        Ok(arms)
    }

    fn child_requires_comma(child: &NodeChild) -> bool {
        matches!(child, NodeChild::Node(_))
    }

    fn requires_comma(&self) -> bool {
        matches!(self.body, NodeChild::Node(_))
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
                requires_comma = Self::child_requires_comma(&body);
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

impl Tokenize for MatchArm {
    fn tokenize(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.pat.to_tokens(&mut tokens);
        if let Some((if_token, guard)) = &self.guard {
            if_token.to_tokens(&mut tokens);
            guard.to_tokens(&mut tokens);
        }
        self.fat_arrow_token.to_tokens(&mut tokens);
        token::Brace::default().surround(&mut tokens, |tokens| {
            self.body
                .tokenize(mode, ident_gen, parent_token)
                .to_tokens(tokens)
        });
        self.comma.to_tokens(&mut tokens);
        tokens
    }
}
