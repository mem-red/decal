use crate::ast::child::{NodeChild, parse_children};
use syn::{
    Expr, Label, Lifetime, Pat, Result as SynResult, Token, braced,
    parse::{Parse, ParseStream},
};

pub enum ControlExpr {
    ForLoop {
        label: Option<Label>,
        for_token: Token![for],
        pat: Box<Pat>,
        in_token: Token![in],
        expr: Box<Expr>,
        body: Vec<NodeChild>,
    },
    NotAnExpr, // Pass-through for rendering custom nodes.
}

impl Parse for ControlExpr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // TODO: peek2 or peek3 for loop with lifetime
        if input.peek(Token![for]) || (input.peek(Lifetime) && input.peek3(Token![for])) {
            Self::parse_for_loop_expr(input)
        } else {
            Ok(Self::NotAnExpr)
        }
    }
}

impl ControlExpr {
    fn parse_for_loop_expr(input: ParseStream) -> SynResult<Self> {
        let label: Option<Label> = input.parse()?;
        let for_token: Token![for] = input.parse()?;
        let pat = Pat::parse_multi_with_leading_vert(input)?;
        let in_token: Token![in] = input.parse()?;
        let expr = input.call(Expr::parse_without_eager_brace)?;

        let content;
        braced!(content in input);
        let body = parse_children(&content)?;

        Ok(Self::ForLoop {
            label,
            for_token,
            pat: Box::new(pat),
            in_token,
            expr: Box::new(expr),
            body,
        })
    }
}
