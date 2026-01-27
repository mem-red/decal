use crate::{
    IdentGen,
    ast::{
        child::{
            NodeChild,
            Tokenize,
            parse_children,
        },
        constants::ATOMIC_NODES,
        method_call::MethodCall,
    },
};
use proc_macro2::{
    Span,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::{
    Block,
    Error as SynError,
    Expr,
    Ident,
    Result as SynResult,
    Token,
    braced,
    parenthesized,
    parse::{
        Parse,
        ParseStream,
    },
    punctuated::Punctuated,
    token::{
        self,
        Comma,
    },
};

pub(crate) struct Node {
    pub(crate) name: Ident,
    pub(crate) args: Punctuated<Expr, Comma>,
    pub(crate) children: Vec<NodeChild>,
    pub(crate) methods: Vec<MethodCall>,
}

impl Parse for Node {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name: Ident = input.parse()?;
        let is_scene = name == "Scene";
        let is_snippet = name == "Snippet";

        let args = if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            content.parse_terminated(Expr::parse, Comma)?
        } else {
            Punctuated::new()
        };

        let children = if input.peek(token::Brace) {
            if ATOMIC_NODES.contains(&name.to_string().as_str()) {
                return Err(input.error("this node cannot contain children"));
            }

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
            return Err(input.error("expected `{` after the `Snippet` node"));
        } else {
            Vec::new()
        };

        // Validate scene node
        if is_scene {
            let err_msg = if !children.is_empty() {
                Some("`Scene` node cannot contain children")
            } else if args.len() != 1 {
                Some("`Scene` node expects a single argument")
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
        ident_gen: &mut IdentGen,
        parent_token: Option<&proc_macro2::Ident>,
    ) -> TokenStream {
        // this is a root node
        if parent_token.is_none() {
            if self.name == "Scene" {
                return SynError::new_spanned(&self.name, "top-level node most not be a scene")
                    .to_compile_error();
            }

            if self.name == "Snippet" {
                return SynError::new_spanned(&self.name, "top-level node most not be a snippet")
                    .to_compile_error();
            }
        }

        let scene_ident = Ident::new("scene", Span::mixed_site());
        let node_kind_ident = format_ident!("{}", self.name);
        let node_token = ident_gen.uniq(&format!("{}_node", self.name.to_string().to_lowercase()));
        let ctor_args = &self.args;

        // Chain method calls for the node
        let method_call_tokens = self
            .methods
            .iter()
            .map(|MethodCall { name, args }| quote! { .#name(#args) });

        let children_tokens = self
            .children
            .iter()
            .map(|child| child.to_tokens(ident_gen, Some(&node_token)))
            .collect::<Vec<_>>();

        if self.name == "Snippet" {
            return quote! { #(#children_tokens)* };
        }

        let node_expr = quote! {
            #node_kind_ident::new(#ctor_args)
                #(#method_call_tokens)*
            .finish()
        };

        match parent_token {
            // Root node
            None => quote! {
                {
                    use decal::prelude::*;
                    let mut #scene_ident = Scene::new(#node_expr);
                    let mut #node_token = #scene_ident.root_id();
                    #(#children_tokens)*
                    #scene_ident
                }
            },
            // Child node
            Some(parent_id) => {
                if self.name == "Scene" {
                    let args = &self.args;
                    quote! { #scene_ident.append_scene(#parent_id, #args); }
                } else {
                    quote! {
                        let #node_token = #scene_ident.append_child(
                            #parent_id,
                            #node_expr
                        );
                        #(#children_tokens)*
                    }
                }
            }
        }
    }
}
