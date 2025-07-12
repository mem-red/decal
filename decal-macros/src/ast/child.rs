use super::ctrl_expr::{CtrlExpr, TokenGenMode};
use super::node::Node;
use crate::IdentGen;
use proc_macro2::{Ident as PM2Ident, TokenStream, TokenTree};
use syn::{Ident, Result as SynResult, parse::Parse, parse::ParseStream, token};

pub enum NodeChild {
    Node(Node),
    CtrlExpr(CtrlExpr),
}

static VALID_NODES: &[&str] = &["Root", "Fragment", "Snippet", "Column", "Row", "Text"];

fn is_valid_node(node_name: &Ident) -> bool {
    VALID_NODES.contains(&node_name.to_string().as_str())
}

impl Parse for NodeChild {
    fn parse(input: ParseStream) -> SynResult<Self> {
        match input.parse::<CtrlExpr>()? {
            CtrlExpr::NotAnExpr => {
                // Parse node instead
                if input.peek(Ident) && (input.peek2(token::Paren) || input.peek2(token::Brace)) {
                    // Peek the ident without consuming input to validate it first
                    let ident: Ident = input.fork().parse()?;

                    if is_valid_node(&ident) {
                          Ok(NodeChild::Node(input.parse()?))
                    } else {
                        Err(syn::Error::new_spanned(
                            ident.clone(),
                            format!(
                                "`{}` is not a valid node. Expected one of: {}",
                                ident,
                                VALID_NODES.join(", ")
                            ),
                        ))
                    }
                } else {
                    let token: TokenTree = input.parse()?;
                    Err(syn::Error::new_spanned(
                        token,
                        "unexpected token: not a control expression or a valid node",
                    ))
                }
            }
            // Parse normal expressions
            expr => Ok(NodeChild::CtrlExpr(expr)),
        }
    }
}

impl NodeChild {
    pub fn to_tokens(
        &self,
        ident_gen: &mut IdentGen,
        parent_tkn: Option<&PM2Ident>,
        root_found: &mut bool,
    ) -> TokenStream {
        let mut mode = TokenGenMode::Full { root_found };
        self.to_tokens_with_mode(&mut mode, ident_gen, parent_tkn)
    }

    pub fn to_tokens_partial(
        &self,
        ident_gen: &mut IdentGen,
        parent_tkn: Option<&PM2Ident>,
    ) -> TokenStream {
        let mut mode = TokenGenMode::Partial;
        self.to_tokens_with_mode(&mut mode, ident_gen, parent_tkn)
    }

    pub fn to_tokens_with_mode(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_tkn: Option<&PM2Ident>,
    ) -> TokenStream {
        match self {
            NodeChild::Node(node) => match mode {
                TokenGenMode::Full { root_found: rf } => node.to_tokens(ident_gen, parent_tkn, *rf),
                TokenGenMode::Partial => node.to_tokens_partial(ident_gen, parent_tkn),
            },
            NodeChild::CtrlExpr(expr) => expr.to_tokens_with_mode(mode, ident_gen, parent_tkn),
        }
    }
}

pub fn parse_children(input: ParseStream) -> SynResult<Vec<NodeChild>> {
    let mut children = Vec::new();
    while !input.is_empty() {
        children.push(input.parse()?);
    }
    Ok(children)
}
