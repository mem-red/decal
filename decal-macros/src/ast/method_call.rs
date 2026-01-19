use syn::{
    Expr,
    Ident,
    Result as SynResult,
    Token,
    parenthesized,
    parse::{
        Parse,
        ParseStream,
    },
    punctuated::Punctuated,
    token::Comma,
};

pub(crate) struct MethodCall {
    pub(crate) name: Ident,
    pub(crate) args: Punctuated<Expr, Comma>,
}

impl Parse for MethodCall {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);
        let args = content.parse_terminated(Expr::parse, Token![,])?;
        Ok(Self { name, args })
    }
}
