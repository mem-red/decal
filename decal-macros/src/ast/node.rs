use crate::{
    IdentGen,
    ast::{
        child::{NodeChild, parse_children},
        method_call::NodeMethodCall,
    },
};
use proc_macro2::{Ident as PM2Ident, TokenStream};
use quote::{format_ident, quote};
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
                vec![NodeChild::Snippet(Block {
                    brace_token,
                    stmts: content.call(Block::parse_within)?,
                })]
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

/// Holds the return values for the [`Node::to_tokens_impl`] method.
struct ToTokensReturnValue {
    /// The [`NodeKind`](decal::prelude::NodeKind) variant.
    node_kind_ident: Ident,
    /// The unique identifier variable for this node's token.
    node_token: Ident,
    /// The expression that constructs this node.
    node_expr: TokenStream,
    /// The generated token stream for this node's children.
    children_tokens: TokenStream,
}

impl Node {
    /// Generates a [`TokenStream`] representing this node and its children.
    ///
    /// # Parameters
    /// - `ident_gen`: A mutable reference to an [identifier generator](IdentGen).
    /// - `parent_token`: The parent node's identifier, or `None` if this is the root.
    /// - `root_found`: A mutable boolean flag indicating if the root node has already been found.
    ///
    /// # Returns
    /// A [`TokenStream`] for macro expansion.
    pub fn to_tokens(
        &self,
        ident_gen: &mut IdentGen,
        parent_token: Option<&PM2Ident>,
        root_found: &mut bool,
    ) -> TokenStream {
        // Root node validation
        match parent_token {
            None => {
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
            }
            Some(_) if self.name == "Root" => {
                return SynError::new_spanned(
                    &self.name,
                    "nested [decal::prelude::Root] nodes are not allowed",
                )
                .to_compile_error();
            }
            _ => {}
        }

        let mut root_found = Some(root_found);
        let ToTokensReturnValue {
            node_kind_ident,
            node_token,
            node_expr,
            children_tokens,
        } = self.to_tokens_impl(ident_gen, &mut root_found);

        match parent_token {
            // Root node
            None => quote! {
                {
                    use decal::prelude::*;
                    let mut decal = Decal::new(#node_expr);
                    let #node_token = decal.root();
                    #children_tokens
                    decal
                }
            },
            // Child node
            Some(parent_token) => self.generate_non_root_node_tokens(
                parent_token,
                &node_kind_ident,
                &node_token,
                &node_expr,
                &children_tokens,
            ),
        }
    }

    /// Generates a [`TokenStream`] for this node and its children as part of a fragment.
    ///
    /// # Parameters
    /// - `ident_gen`: A mutable reference to an [identifier generator](IdentGen).
    /// - `parent_token`: The parent node's identifier, or `None` if this is the fragment root.
    ///
    /// # Returns
    /// A [`TokenStream`] for macro expansion.
    pub fn to_tokens_partial(
        &self,
        ident_gen: &mut IdentGen,
        parent_token: Option<&PM2Ident>,
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
            node_token,
            node_expr,
            children_tokens,
        } = self.to_tokens_impl(ident_gen, &mut root_found);

        match parent_token {
            // Fragment root node
            None => quote! {
                {
                    use decal::prelude::*;
                    let mut decal = DecalFragment::new(NodeKind::#node_kind_ident(#node_expr));
                    let #node_token = decal.root();
                    #children_tokens
                    decal
                }
            },
            // Child node
            Some(parent_token) => self.generate_non_root_node_tokens(
                parent_token,
                &node_kind_ident,
                &node_token,
                &node_expr,
                &children_tokens,
            ),
        }
    }

    /// Generates token stream for non-root nodes.
    ///
    /// # Parameters
    /// - `parent_token`: The parent node's identifier.
    /// - `node_kind_ident`: The [`NodeKind`](decal::prelude::NodeKind) variant.
    /// - `node_token`: The unique identifier variable for this node's token.
    /// - `node_expr`: The expression that constructs this node.
    /// - `children_tokens`: The generated token stream for this node's children.
    ///
    /// # Returns
    /// A [`TokenStream`] for the non-root node.
    fn generate_non_root_node_tokens(
        &self,
        parent_token: &PM2Ident,
        node_kind_ident: &Ident,
        node_token: &Ident,
        node_expr: &TokenStream,
        children_tokens: &TokenStream,
    ) -> TokenStream {
        if self.name == "Fragment" {
            let args = &self.args;
            quote! { decal.append_fragment(#parent_token, #args.clone()); }
        } else if self.name == "Snippet" {
            quote! { #children_tokens }
        } else {
            quote! {
                let #node_token = decal.append_child(
                    #parent_token,
                    Node::new(NodeKind::#node_kind_ident(#node_expr))
                );
                #children_tokens
            }
        }
    }

    /// Generates the node's construction and its children.
    ///
    /// # Parameters
    /// - `ident_gen`: A mutable reference to an [identifier generator](IdentGen).
    /// - `root_found`: An optional mutable reference to the root-found flag.
    ///
    /// # Returns
    /// A [`ToTokensReturnValue`] for code generation.
    fn to_tokens_impl(
        &self,
        ident_gen: &mut IdentGen,
        root_found: &mut Option<&mut bool>,
    ) -> ToTokensReturnValue {
        let name_lowercased = self.name.to_string().to_lowercase();
        let node_kind_ident = format_ident!("{}", self.name);
        let node_token = ident_gen.uniq(&format!("{}_token", name_lowercased));
        let ctor_args = &self.args;

        // Chain method calls for the node
        let method_call_tokens = self.methods.iter().map(|method| {
            let method_name = &method.name;
            let method_args = &method.args;
            quote! { .#method_name(#method_args) }
        });

        let node_expr = if self.methods.is_empty() {
            quote! { #node_kind_ident::new(#ctor_args) }
        } else {
            // Deref the "&mut Node" returned from the last method call
            quote! {
                *#node_kind_ident::new(#ctor_args)
                    #(#method_call_tokens)*
            }
        };

        let children_tokens = self.children.iter().map(|child| {
            if let Some(root_found) = root_found {
                child.to_tokens(ident_gen, Some(&node_token), *root_found)
            } else {
                child.to_tokens_partial(ident_gen, Some(&node_token))
            }
        });

        ToTokensReturnValue {
            node_kind_ident,
            node_token: node_token.clone(),
            node_expr,
            children_tokens: quote! { #(#children_tokens)* },
        }
    }
}
