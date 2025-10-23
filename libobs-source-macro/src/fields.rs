use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field};

pub fn generate_struct_fields(
    fields: &Punctuated<Field, Comma>,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let maybe_obs_properties = fields
        .iter()
        .map(|f| {
            let attr = f.attrs.iter().find(|e| e.path().is_ident("obs_property"));

            (f, attr)
        })
        .collect::<Vec<_>>();

    let obs_properties = maybe_obs_properties
        .iter()
        .filter_map(|(f, attr)| if attr.is_some() { Some(f) } else { None })
        .collect::<Vec<_>>();

    let non_obs_properties = maybe_obs_properties
        .iter()
        .filter_map(|(f, attr)| if let Some(_a) = attr { None } else { Some(f) })
        .collect::<Vec<_>>();

    let mut struct_fields = Vec::<TokenStream>::new();
    let mut struct_initializers = Vec::<TokenStream>::new();

    for field in obs_properties {
        let name = &field.ident;
        struct_fields.push(quote! {
            /// IGNORE THIS FIELD. This is just so intellisense doesn't get confused and isn't complaining
            #name: u8
        });

        struct_initializers.push(quote! {
            #name: 0
        });
    }

    for ele in non_obs_properties {
        let name = &ele.ident;
        #[allow(dead_code)]
        let field_type = &ele.ty;
        struct_fields.push(quote! {
            #name: #field_type
        });

        struct_initializers.push(quote! {
            #name: Default::default()
        });
    }

    (struct_fields, struct_initializers)
}
