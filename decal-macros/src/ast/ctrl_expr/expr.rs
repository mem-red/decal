use super::{CtrlExprForLoop, CtrlExprIf, CtrlExprMatch};
use crate::IdentGen;
use proc_macro2::{Ident as PM2Ident, TokenStream};
use syn::{
    Lifetime, Result as SynResult, Token,
    parse::{Parse, ParseStream},
    token,
};

pub trait Tokenize {
    fn tokenize(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream;
}

#[derive(Debug)]
pub enum TokenGenMode<'a> {
    Full { root_found: &'a mut bool },
    Partial,
}

pub enum CtrlExpr {
    If(CtrlExprIf),
    Match(CtrlExprMatch),
    ForLoop(CtrlExprForLoop),
    NotAnExpr, // Pass-through for rendering DSL nodes
}

impl Parse for CtrlExpr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![if]) {
            Ok(Self::If(input.parse()?))
        } else if input.peek(Token![while]) {
            todo!()
        } else if input.peek(Token![loop]) {
            todo!()
        } else if input.peek(Token![match]) {
            Ok(Self::Match(input.parse()?))
        } else if input.peek(Token![for])
            || (input.peek(Lifetime) && input.peek2(Token![:]) && input.peek3(Token![for]))
        {
            Ok(Self::ForLoop(input.parse()?))
        } else if input.peek(Token![try]) && input.peek2(token::Brace) {
            todo!()
        } else {
            Ok(Self::NotAnExpr)
        }
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
            CtrlExpr::If(expr) => expr.tokenize(mode, ident_gen, parent_token),
            CtrlExpr::Match(expr) => expr.tokenize(mode, ident_gen, parent_token),
            CtrlExpr::ForLoop(expr) => expr.tokenize(mode, ident_gen, parent_token),
            CtrlExpr::NotAnExpr => unreachable!(),
        }
    }
}
