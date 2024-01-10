use quote::{quote, ToTokens, TokenStreamExt};
use syn::ExprBlock;

pub struct Properties {
    pub class: Option<String>,
}

pub enum FauxNode {
    Text(String),
    Div(Vec<FauxNode>, Properties),
    Expr(ExprBlock),
    Fragment(Vec<FauxNode>),
}

impl ToTokens for Properties {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let class_tokens = match &self.class {
            Some(class) => quote! { Some(#class.to_string()) },
            None => quote! { None },
        };

        tokens.extend(quote! {
            Properties {
                class: #class_tokens,
            }
        });
    }
}

impl ToTokens for FauxNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FauxNode::Text(text) => {
                tokens.append_all(quote! {
                    FauxNode::Text(#text.to_string())
                });
            }
            FauxNode::Div(children, properties) => {
                let children_tokens: Vec<_> =
                    children.iter().map(|child| quote! { #child }).collect();
                tokens.append_all(quote! {
                    FauxNode::Div(vec![#(#children_tokens),*], #properties)
                });
            }
            FauxNode::Expr(expr) => {
                tokens.append_all(quote! {
                    #expr
                });
            }
            FauxNode::Fragment(children) => {
                let children_tokens: Vec<_> =
                    children.iter().map(|child| quote! { #child }).collect();
                tokens.append_all(quote! {
                    FauxNode::Fragment(vec![#(#children_tokens),*])
                });
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TryFrom failed: {0}")]
    TryFrom(String),
}
