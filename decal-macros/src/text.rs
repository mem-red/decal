use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr,
    Token,
    braced,
    parenthesized,
    parse::{
        Parse,
        ParseStream,
    },
    parse2,
    punctuated::Punctuated,
    token,
};

struct Style {
    attr: syn::Ident,
    #[allow(dead_code)]
    colon: Token![:],
    value: Expr,
}

impl Parse for Style {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attr: input.parse()?,
            colon: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct Item {
    expr: Expr,
    style: Option<Vec<Style>>,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // item with style
        if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);

            let expr: Expr = content.parse()?;
            content.parse::<Token![,]>()?;

            let style_expr;
            braced!(style_expr in content);

            let styles = Punctuated::<Style, Token![,]>::parse_terminated(&style_expr)?
                .into_iter()
                .collect();

            return Ok(Self {
                expr,
                style: Some(styles),
            });
        }

        // normal item
        Ok(Self {
            expr: input.parse()?,
            style: None,
        })
    }
}

struct TextSpans(Punctuated<Item, Token![,]>);

impl Parse for TextSpans {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated(input)?))
    }
}

fn expand(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let TextSpans(items) = match parse2(tokens.clone()) {
        Ok(parsed) => parsed,
        Err(err) => return err.to_compile_error(),
    };

    let num_items = items.len();
    let span_expr = items.into_iter().map(|item| {
        let expr = item.expr;

        if let Some(styles) = item.style {
            let mut expr = quote! {
                decal::prelude::TextSpan::from(#expr.into_text_span())
            };

            for Style { attr, value, .. } in styles {
                expr = quote! { #expr.#attr(#value) };
            }

            return quote! { text_spans.push(#expr); };
        }

        quote! { text_spans.extend(#expr.into_text_spans()); }
    });

    quote! {{
        let mut text_spans = Vec::with_capacity(#num_items);
        #(#span_expr)*
        text_spans
    }}
}

pub(crate) fn text_impl(input: TokenStream) -> TokenStream {
    let stream = proc_macro2::TokenStream::from(input);
    expand(stream).into()
}
