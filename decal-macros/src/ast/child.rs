use super::constants::VALID_NODES;
use super::ctrl_expr::{CtrlExpr, TokenGenMode};
use super::node::Node;
use crate::IdentGen;
use proc_macro2::{Ident as PM2Ident, TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::{Block, Ident, Result as SynResult, parse::Parse, parse::ParseStream, token};

pub(crate) trait Tokenize {
    fn tokenize(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream;
}

pub(crate) enum NodeChild {
    Node(Node),
    Snippet(Block),
    CtrlExpr(CtrlExpr),
}

fn is_valid_node(node_name: &Ident) -> bool {
    VALID_NODES.contains(&node_name.to_string().as_str())
}

impl Parse for NodeChild {
    fn parse(input: ParseStream) -> SynResult<Self> {
        match input.parse::<CtrlExpr>()? {
            CtrlExpr::NotAnExpr => {
                // parse DSL nodes
                if input.peek(Ident) && (input.peek2(token::Paren) || input.peek2(token::Brace)) {
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
            // parse control expressions
            expr => Ok(NodeChild::CtrlExpr(expr)),
        }
    }
}

impl Tokenize for NodeChild {
    fn tokenize(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        match self {
            NodeChild::Node(node) => node.tokenize(mode, ident_gen, parent_token),
            NodeChild::Snippet(block) => {
                let block_tokens = block.stmts.iter().map(|stmt| stmt.to_token_stream());
                quote! { #(#block_tokens)* }
            }
            NodeChild::CtrlExpr(expr) => expr.tokenize(mode, ident_gen, parent_token),
        }
    }
}

impl NodeChild {
    pub(crate) fn to_tokens(
        &self,
        ident_gen: &mut IdentGen,
        parent_token: Option<&PM2Ident>,
        root_found: &mut bool,
    ) -> TokenStream {
        let mut mode = TokenGenMode::Full { root_found };
        self.tokenize(&mut mode, ident_gen, parent_token)
    }

    pub(crate) fn to_tokens_partial(
        &self,
        ident_gen: &mut IdentGen,
        parent_token: Option<&PM2Ident>,
    ) -> TokenStream {
        let mut mode = TokenGenMode::Partial;
        self.tokenize(&mut mode, ident_gen, parent_token)
    }
}

pub(crate) fn parse_children(input: ParseStream) -> SynResult<Vec<NodeChild>> {
    let mut children = Vec::new();
    while !input.is_empty() {
        children.push(input.parse()?);
    }
    Ok(children)
}
