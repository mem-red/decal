use super::control_expr::ControlExpr;
use super::node::Node;
use crate::IdentGen;
use proc_macro2::{Ident as PM2Ident, TokenStream, TokenTree};
use quote::quote;
use syn::{Ident, Result as SynResult, parse::Parse, parse::ParseStream, token};

pub enum NodeChild {
    Node(Node),
    ControlExpr(ControlExpr),
}

static VALID_NODES: &[&str] = &["Root", "Column", "Row"];

fn is_valid_node(node_name: &Ident) -> bool {
    VALID_NODES.contains(&node_name.to_string().as_str())
}

impl Parse for NodeChild {
    fn parse(input: ParseStream) -> SynResult<Self> {
        match input.parse::<ControlExpr>()? {
            ControlExpr::NotAnExpr => {
                // Parse node instead
                if input.peek(Ident) && (input.peek2(token::Paren) || input.peek2(token::Brace)) {
                    // Peek the ident without consuming input to validate it first
                    let ident: Ident = input.fork().parse()?;

                    if is_valid_node(&ident) {
                        Ok(NodeChild::Node(input.parse()?))
                    } else {
                        Err(syn::Error::new_spanned(
                            ident.clone(),
                            format!("`{}` is not a valid node", ident),
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
            // Parse normal experssions
            expr => Ok(NodeChild::ControlExpr(expr)),
        }
    }
}

#[derive(Debug)]
enum TokenGenMode<'a> {
    Full { root_found: &'a mut bool },
    Partial,
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

    fn to_tokens_with_mode(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_tkn: Option<&PM2Ident>,
    ) -> TokenStream {
        match self {
            NodeChild::Node(node) => match mode {
                TokenGenMode::Full { root_found } => {
                    node.to_tokens(ident_gen, parent_tkn, *root_found)
                }
                TokenGenMode::Partial => node.to_tokens_partial(ident_gen, parent_tkn),
            },
            NodeChild::ControlExpr(expr) => match expr {
                ControlExpr::ForLoop {
                    label,
                    for_token,
                    pat,
                    in_token,
                    expr,
                    body,
                } => {
                    let child_tokens = body
                        .iter()
                        .map(|child| child.to_tokens_with_mode(mode, ident_gen, parent_tkn));

                    quote! {
                        #label #for_token #pat #in_token #expr {
                            #(#child_tokens)*
                        }
                    }
                }
                ControlExpr::NotAnExpr => unreachable!(),
            },
        }
    }
}

pub fn parse_children(input: ParseStream) -> SynResult<Vec<NodeChild>> {
    let mut children = vec![];
    while !input.is_empty() {
        children.push(input.parse()?);
    }
    Ok(children)
}
