use crate::{
    IdentGen,
    ast::{
        child::{NodeChild, Tokenize, parse_children},
        ctrl_expr::TokenGenMode,
        method_call::NodeMethodCall,
    },
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    Block, Error as SynError, Expr, Ident, Result as SynResult, Token, braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{self, Comma},
};

pub(crate) struct Node {
    pub(crate) name: Ident,
    pub(crate) args: Punctuated<Expr, Comma>,
    pub(crate) children: Vec<NodeChild>,
    pub(crate) methods: Vec<NodeMethodCall>,
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
                vec![NodeChild::Snippet(Block {
                    brace_token,
                    stmts: content.call(Block::parse_within)?,
                })]
            } else {
                parse_children(&content)?
            }
        } else if is_snippet {
            return Err(input.error("expected `{` after the [`decal::prelude::Snippet`] node"));
        } else {
            Vec::new()
        };

        // Validate fragment node
        if is_fragment {
            let err_msg = if !children.is_empty() {
                Some("[`decal::prelude::Fragment`] node cannot contain children")
            } else if args.len() != 1 {
                Some("[`decal::prelude::Fragment`] node expects a single argument")
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

impl Tokenize for Node {
    fn tokenize(
        &self,
        mode: &mut TokenGenMode,
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        // Validation
        if let TokenGenMode::Full { root_found } = mode {
            match parent_token {
                None => {
                    if self.name != "Root" {
                        return SynError::new_spanned(
                            &self.name,
                            "expected the top-level node to be a [`decal::prelude::Root`] node",
                        )
                        .to_compile_error();
                    }
                    if **root_found {
                        return SynError::new_spanned(
                            &self.name,
                            "only one [`decal::prelude::Root`] node is allowed",
                        )
                        .to_compile_error();
                    }
                    **root_found = true;
                }
                Some(_) if self.name == "Root" => {
                    return SynError::new_spanned(
                        &self.name,
                        "nested [`decal::prelude::Root`] nodes are not allowed",
                    )
                    .to_compile_error();
                }
                _ => {}
            }
        } else {
            if self.name == "Root" {
                return SynError::new_spanned(
                    &self.name,
                    "[`decal::prelude::Root`] node is not allowed inside fragments",
                )
                .to_compile_error();
            }
        }

        let is_root = self.name == "Root";
        let name_lowercased = self.name.to_string().to_lowercase();
        let node_kind_ident = format_ident!("{}", self.name);
        let node_token = if is_root {
            format_ident!("{}", name_lowercased, span = Span::call_site())
        } else {
            ident_gen.uniq(&format!("{}_node", name_lowercased))
        };
        let ctor_args = &self.args;

        // Chain method calls for the node
        let method_call_tokens = self.methods.iter().map(|method| {
            let method_name = &method.name;
            let method_args = &method.args;
            quote! { .#method_name(#method_args) }
        });

        let node_expr = quote! {
            #node_kind_ident::new(#ctor_args)
                #(#method_call_tokens)*
            .build()
        };

        let children_tokens = if let TokenGenMode::Full { root_found } = mode {
            self.children
                .iter()
                .map(|child| child.to_tokens(ident_gen, Some(&node_token), root_found))
                .collect::<Vec<_>>()
        } else {
            self.children
                .iter()
                .map(|child| child.to_tokens_partial(ident_gen, Some(&node_token)))
                .collect::<Vec<_>>()
        };

        match parent_token {
            // Root node
            None => quote! {
                {
                    use decal::prelude::*;
                    let mut decal = Decal::new(#node_expr);
                    let mut #node_token = decal.root_id();
                    #(#children_tokens)*
                    decal
                }
            },
            // Child node
            Some(parent_id) => self.generate_non_root_node_tokens(
                parent_id,
                &node_token,
                &node_expr,
                quote! { #(#children_tokens)* },
            ),
        }
    }
}

impl Node {
    /// Generates token stream for non-root nodes.
    ///
    /// # Parameters
    /// - `parent_id`: The parent node's identifier.
    /// - `node_token`: The unique identifier variable for this node's token.
    /// - `node_expr`: The expression that constructs this node.
    /// - `children_tokens`: The generated token stream for this node's children.
    ///
    /// # Returns
    /// A [`TokenStream`] for the non-root node.
    fn generate_non_root_node_tokens(
        &self,
        parent_id: &proc_macro2::Ident,
        node_token: &Ident,
        node_expr: &TokenStream,
        children_tokens: TokenStream,
    ) -> TokenStream {
        if self.name == "Fragment" {
            let args = &self.args;
            quote! { decal.append_fragment(#parent_id, #args); }
        } else if self.name == "Snippet" {
            quote! { #children_tokens }
        } else {
            quote! {
                let #node_token = decal.append_child(
                    #parent_id,
                    #node_expr
                );
                #children_tokens
            }
        }
    }
}
