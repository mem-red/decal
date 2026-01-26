use super::{
    CtrlExprBreak,
    CtrlExprForLoop,
    CtrlExprIf,
    CtrlExprLoop,
    CtrlExprMatch,
    CtrlExprWhile,
};
use crate::{
    IdentGen,
    ast::child::Tokenize,
};
use proc_macro2::TokenStream;
use syn::{
    Lifetime,
    Result as SynResult,
    Token,
    parse::{
        Parse,
        ParseStream,
    },
};

pub(crate) enum CtrlExpr {
    If(CtrlExprIf),
    Match(CtrlExprMatch),
    Loop(CtrlExprLoop),
    ForLoop(CtrlExprForLoop),
    While(CtrlExprWhile),
    Break(CtrlExprBreak),
    NotAnExpr, // Pass-through for rendering DSL nodes
}

impl Parse for CtrlExpr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![if]) {
            Ok(Self::If(input.parse()?))
        } else if input.peek(Token![match]) {
            Ok(Self::Match(input.parse()?))
        } else if input.peek(Token![break]) {
            Ok(Self::Break(input.parse()?))
        } else if input.peek(Token![while])
            || (input.peek(Lifetime) && input.peek2(Token![:]) && input.peek3(Token![while]))
        {
            Ok(Self::While(input.parse()?))
        } else if input.peek(Token![loop])
            || (input.peek(Lifetime) && input.peek2(Token![:]) && input.peek3(Token![loop]))
        {
            Ok(Self::Loop(input.parse()?))
        } else if input.peek(Token![for])
            || (input.peek(Lifetime) && input.peek2(Token![:]) && input.peek3(Token![for]))
        {
            Ok(Self::ForLoop(input.parse()?))
        } else {
            Ok(Self::NotAnExpr)
        }
    }
}

impl Tokenize for CtrlExpr {
    fn tokenize(
        &self,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        match self {
            CtrlExpr::If(expr) => expr.tokenize(ident_gen, parent_token),
            CtrlExpr::Match(expr) => expr.tokenize(ident_gen, parent_token),
            CtrlExpr::Loop(expr) => expr.tokenize(ident_gen, parent_token),
            CtrlExpr::ForLoop(expr) => expr.tokenize(ident_gen, parent_token),
            CtrlExpr::While(expr) => expr.tokenize(ident_gen, parent_token),
            CtrlExpr::Break(expr) => expr.tokenize(ident_gen, parent_token),
            CtrlExpr::NotAnExpr => unreachable!(),
        }
    }
}
