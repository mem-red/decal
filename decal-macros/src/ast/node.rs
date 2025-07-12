use crate::{
    IdentGen,
    ast::{
        child::{NodeChild, parse_children},
        ctrl_expr::CtrlExpr,
        method_call::NodeMethodCall,
    },
};
use proc_macro2::{Ident as PM2Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    Block, Error as SynError, Expr, Ident, Result as SynResult, Token, braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{self, Comma},
};

pub struct Node {
    pub name: Ident,
    pub args: Punctuated<Expr, Comma>,
    pub children: Vec<NodeChild>,
    pub methods: Vec<NodeMethodCall>,
}

impl Parse for Node {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name: Ident = input.parse()?;
        let is_fragment = name == "Fragment";
        let is_snippet = name == "Snippet";

        let args = if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            content.parse_terminated(Expr::parse, Comma)?
        } else {
            Punctuated::new()
        };

        let children = if input.peek(token::Brace) {
            let content;
            let brace_token = braced!(content in input);

            if is_snippet {
                let block = Block {
                    brace_token,
                    stmts: content.call(Block::parse_within)?,
                };

                vec![NodeChild::CtrlExpr(CtrlExpr::Snipped(block))]
            } else {
                parse_children(&content)?
            }
        } else if is_snippet {
            return Err(input.error("expected `{` after the [decal::prelude::Snippet] node"));
        } else {
            Vec::new()
        };

        // Validate fragment node
        if is_fragment {
            let err_msg = if !children.is_empty() {
                Some("[decal::prelude::Fragment] node cannot contain children")
            } else if args.len() != 1 {
                Some("[decal::prelude::Fragment] node expects a single argument")
            } else {
                None
            };

            if let Some(err_msg) = err_msg {
                return Err(SynError::new_spanned(name, err_msg));
            }
        }

        let mut methods = Vec::new();
        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            methods.push(input.parse()?);
        }

        Ok(Node {
            name,
            args,
            children,
            methods,
        })
    }
}

struct ToTokensReturnValue {
    pub node_kind_ident: Ident,
    pub node_var: Ident,
    pub node_tkn: Ident,
    pub node_assignment: TokenStream,
    pub method_call_tokens: TokenStream,
    pub children_tokens: TokenStream,
}

impl Node {
    pub fn to_tokens(
        &self,
        ident_gen: &mut IdentGen,
        parent_tkn: Option<&PM2Ident>,
        root_found: &mut bool,
    ) -> TokenStream {
        // Root node validation
        if parent_tkn.is_none() {
            if self.name != "Root" {
                return SynError::new_spanned(
                    &self.name,
                    "expected the top-level node to be a [decal::prelude::Root] node",
                )
                .to_compile_error();
            }

            if *root_found {
                return SynError::new_spanned(
                    &self.name,
                    "only one [decal::prelude::Root] node is allowed",
                )
                .to_compile_error();
            }

            *root_found = true;
        } else if self.name == "Root" {
            return SynError::new_spanned(
                &self.name,
                "nested [decal::prelude::Root] nodes are not allowed",
            )
            .to_compile_error();
        }

        
        
        let mut root_found = Some(root_found);
        let ToTokensReturnValue {
            node_kind_ident,
            node_var,
            node_tkn,
            node_assignment,
            method_call_tokens,
            children_tokens,
        } = self.to_tokens_impl(ident_gen, &mut root_found);

        // Root node
        if parent_tkn.is_none() {
            quote! {
                use decal::prelude::*;
                #node_assignment
                #method_call_tokens
                let (mut decal, #node_tkn) = Decal::new(#node_var);
                #children_tokens
                decal
            }
        } else {
            // Non-root node
            let parent_tkn = match parent_tkn {
                Some(token) => token,
                None => {
                    return SynError::new_spanned(
                        &self.name,
                        "expected a parent for this non-root node",
                    )
                    .to_compile_error();
                }
            };

            quote! {
                #node_assignment
                #method_call_tokens
                let #node_tkn = #parent_tkn.append(
                    decal.arena_mut(),
                    Node::new(NodeKind::#node_kind_ident(#node_var))
                );
                #children_tokens
            }
        }
    }

    pub fn to_tokens_partial(
        &self,
        ident_gen: &mut IdentGen,
        parent_tkn: Option<&PM2Ident>,
    ) -> TokenStream {
        if self.name == "Root" {
            return SynError::new_spanned(
                &self.name,
                "[decal::prelude::Root] node is not allowed inside fragments",
            )
            .to_compile_error();
        }

        let mut root_found = None;
        let ToTokensReturnValue {
            node_kind_ident,
            node_var,
            node_tkn,
            node_assignment,
            method_call_tokens,
            children_tokens,
        } = self.to_tokens_impl(ident_gen, &mut root_found);

        if let Some(parent_tkn) = parent_tkn {
            quote! {
                #node_assignment
                #method_call_tokens
                let #node_tkn = #parent_tkn.append(
                    fragment.arena_mut(),
                    Node::new(NodeKind::#node_kind_ident(#node_var))
                );
                #children_tokens
            }
        } else {
            quote! {
                {
                    use decal::prelude::*;
                    #node_assignment
                    #method_call_tokens
                    let (mut fragment, #node_tkn) = DecalFragment::new(NodeKind::#node_kind_ident(#node_var));
                    #children_tokens
                    fragment
                }
            }
        }
    }

    fn to_tokens_impl(
        &self,
        ident_gen: &mut IdentGen,
        root_found: &mut Option<&mut bool>,
    ) -> ToTokensReturnValue {
        let name_lowercased = self.name.to_string().to_lowercase();
        let node_kind_ident = format_ident!("{}", self.name);
        let node_var = ident_gen.uniq(&format!("{}_node", name_lowercased));
        let node_tkn = ident_gen.uniq(&format!("{}_tkn", name_lowercased));

        let ctor_args = &self.args;
        let node_assignment = quote! { let mut #node_var = #node_kind_ident::new(#ctor_args); };

        let method_call_tokens = self.methods.iter().map(|method| {
            let method_name = &method.name;
            let method_args = &method.args;
            quote! { #node_var.#method_name(#method_args); }
        });

        let children_tokens = self.children.iter().map(|child| {
            if let Some(root_found) = root_found {
                child.to_tokens(ident_gen, Some(&node_tkn), *root_found)
            } else {
                child.to_tokens_partial(ident_gen, Some(&node_tkn))
            }
        });

        ToTokensReturnValue {
            node_kind_ident,
            node_var: node_var.clone(),
            node_tkn: node_tkn.clone(),
            node_assignment,
            method_call_tokens: quote! { #(#method_call_tokens)* },
            children_tokens: quote! { #(#children_tokens)* },
        }
    }
}
