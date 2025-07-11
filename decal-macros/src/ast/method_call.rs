use syn::{
    Expr, Ident, Result as SynResult, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

pub struct NodeMethodCall {
    pub name: Ident,
    pub args: Punctuated<Expr, Comma>,
}

impl Parse for NodeMethodCall {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);
        let args = content.parse_terminated(Expr::parse, Token![,])?;
        Ok(Self { name, args })
    }
}
