use proc_macro2::TokenStream;
use syn::{Field, punctuated::Punctuated, token::Comma};
use quote::quote;

pub fn generate_fields(fields: &Punctuated<Field, Comma>) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let fields_tokens = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            /// IGNORE THIS FIELD. This is just so intellisense doesn't get confused and isn't complaining
            #name: u8
        }
    }).collect::<Vec<_>>();

    let field_initializers = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: 0
        }
    }).collect::<Vec<_>>();

    (fields_tokens, field_initializers)
}