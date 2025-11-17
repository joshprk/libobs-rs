use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field, LitStr, MetaNameValue, Token};

use crate::docs::collect_doc;

pub fn obs_properties_to_functions(
    fields: &Punctuated<Field, Comma>,
    settings_getter: TokenStream,
) -> Vec<TokenStream> {
    let obs_properties = fields
        .iter()
        .filter_map(|f| {
            let attr = f.attrs.iter().find(|e| e.path().is_ident("obs_property"));

            attr.map(|a| (f, a))
        })
        .collect::<Vec<_>>();

    let mut functions = Vec::new();
    for (field, attr) in obs_properties {
        let field_type = &field.ty;
        let field_name = field.ident.as_ref().unwrap();

        let name_values: Punctuated<MetaNameValue, Token![,]> = attr
            .parse_args_with(Punctuated::parse_terminated)
            .unwrap_or_else(|_| {
                panic!(
                    "Field {} has invalid obs_property, should be name value",
                    field_name
                )
            });

        let type_t = &name_values
            .iter()
            .find(|e| *e.path.get_ident().unwrap() == "type_t")
            .expect("type_t is required for obs_property")
            .value;

        let type_t = match type_t {
            syn::Expr::Lit(e) => match &e.lit {
                syn::Lit::Str(s) => s.value(),
                _ => panic!("type_t must be a string"),
            },
            _ => panic!("type_t must be a string"),
        };

        #[allow(unused_variables)]
        let mut obs_settings_name = field_name.to_string();
        let pot_name = &name_values
            .iter()
            .find(|e| *e.path.get_ident().unwrap() == "settings_key");

        if let Some(n) = pot_name {
            obs_settings_name = match &n.value {
                syn::Expr::Lit(e) => match &e.lit {
                    syn::Lit::Str(s) => s.value(),
                    _ => panic!("setings_key must be a string"),
                },
                _ => panic!("settings_key must be a string"),
            };
        }

        let (_docs_str, docs_attr) = collect_doc(&field.attrs);

        let obs_settings_key = LitStr::new(&obs_settings_name, Span::call_site());
        let set_field = quote::format_ident!("set_{}", field_name);
        let type_t_str = type_t.as_str();
        let to_add = match type_t_str {
            "enum" => {
                quote! {
                    #(#docs_attr)*
                    pub fn #set_field(mut self, #field_name: #field_type) -> Self {
                        use num_traits::ToPrimitive;
                        let val = #field_name.to_i32().unwrap();

                        #settings_getter
                            .set_int_ref(#obs_settings_key, val as i64);

                        self
                    }
                }
            }
            "enum_string" => {
                quote! {
                    #(#docs_attr)*
                    pub fn #set_field(mut self, #field_name: #field_type) -> Self {
                        use libobs_wrapper::data::StringEnum;

                        #settings_getter
                            .set_string_ref(#obs_settings_key, #field_name.to_str());

                        self
                    }
                }
            }
            "string" => {
                quote! {
                    #(#docs_attr)*
                    pub fn #set_field<T: Into<libobs_wrapper::utils::ObsString> + Sync + Send>(mut self, #field_name: T) -> Self {
                        #settings_getter
                            .set_string_ref(#obs_settings_key, #field_name);
                        self
                    }
                }
            }
            "bool" => {
                quote! {
                    #(#docs_attr)*
                    pub fn #set_field(mut self, #field_name: bool) -> Self {
                        #settings_getter
                            .set_bool_ref(#obs_settings_key, #field_name);
                        self
                    }
                }
            }
            "int" => {
                quote! {
                    #(#docs_attr)*
                    pub fn #set_field(mut self, #field_name: i64) -> Self {
                        #settings_getter
                            .set_int_ref(#obs_settings_key, #field_name);
                        self
                    }
                }
            }
            _ => panic!(
                "Unsupported type_t {}. Should either be `enum`, `string`, `bool` or `int`",
                type_t
            ),
        };

        functions.push(to_add);
    }

    functions
}
