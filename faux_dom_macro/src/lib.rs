extern crate proc_macro;
use faux_dom_shared::{FauxNode, Properties};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Token;
use syn::{parse_macro_input, token, ExprBlock, LitStr, Token};

struct RsxInput {
    nodes: Vec<FauxNode>,
}

impl Parse for RsxInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            if input.peek(Token![<]) && input.peek2(Token![>]) {
                input.parse::<Token![<]>()?;
                input.parse::<Token![>]>()?;
                let content = input.parse::<RsxInput>()?;
                input.parse::<Token![<]>()?;
                input.parse::<Token![/]>()?;
                input.parse::<Token![>]>()?;
                nodes.push(FauxNode::Fragment(content.nodes));
            } else if input.peek(Token![<]) && input.peek2(syn::Ident) {
                nodes.push(parse_tag(&input)?);
            } else if input.peek(LitStr) {
                let text = input.parse::<LitStr>()?.value();
                nodes.push(FauxNode::Text(text));
            } else if input.peek(token::Brace) {
                let expr = input.parse::<ExprBlock>()?;
                nodes.push(FauxNode::Expr(expr));
            } else if input.peek(Token![<]) {
                return Ok(RsxInput { nodes });
            } else {
                return Err(syn::Error::new(Span::call_site(), "Unexpected token"));
            }
        }
        Ok(RsxInput { nodes })
    }
}

fn parse_tag(input: &ParseStream) -> Result<FauxNode, syn::Error> {
    input.parse::<Token![<]>()?;
    let tag = input.parse::<Ident>()?;

    match tag.to_string().as_str() {
        "div" => parse_div_tag(input, tag),
        _ => Err(syn::Error::new(tag.span(), "Unsupported tag")),
    }
}

fn parse_properties(input: &ParseStream) -> Result<Properties, syn::Error> {
    let mut properties = Properties { class: None };
    while !input.peek(Token![>]) {
        let property = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<LitStr>()?;
        if property.to_string() == "class" {
            properties.class = Some(value.value());
        } else {
            return Err(syn::Error::new(property.span(), "Unsupported property"));
        }
    }

    Ok(properties)
}

fn parse_div_tag(input: &ParseStream, opening_tag: Ident) -> Result<FauxNode, syn::Error> {
    let properties = parse_properties(input)?;

    input.parse::<Token![>]>()?;
    let content = input.parse::<RsxInput>()?;
    input.parse::<Token![<]>()?;
    input.parse::<Token![/]>()?;
    let closing_tag = input.parse::<Ident>()?;

    if closing_tag.to_string() != opening_tag.to_string() {
        return Err(syn::Error::new(
            closing_tag.span(),
            "Mismatched closing tag",
        ));
    }

    input.parse::<Token![>]>()?;
    Ok(FauxNode::Div(content.nodes, properties))
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    // Parse the input tokens into our RsxInput struct
    let parsed_input = parse_macro_input!(input as RsxInput);

    // Generate the Rust code from the parsed input
    let expanded = generate_code(parsed_input);

    // Convert the generated code back into a TokenStream
    TokenStream::from(expanded)
}

// This function takes the parsed input and generates the Rust code
fn generate_code(input: RsxInput) -> proc_macro2::TokenStream {
    let nodes = input.nodes;

    let generated = quote! {
        {
            let children = vec![#(#nodes)*];
            FauxNode::Fragment(children)
        }
    };

    generated
}
