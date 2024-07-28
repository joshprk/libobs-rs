use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Fields, LitStr,
    MetaNameValue, Token,
};

#[allow(unused_assignments)]
#[proc_macro_attribute]
/// Note that fields are just for the attributes, they are not actually implemented for the struct
pub fn obs_source_builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    let id = parse_macro_input!(attr as LitStr);

    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;
    let generics = input.generics;
    let visibility = input.vis;
    let attributes = input.attrs;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };


    let id_value = id.value();
    let fields_tokens = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            /// IGNORE THIS FIELD. This is just so intellisense doesn't get confused and isn't complaining
            #name: u8
        }
    });

    let field_initializers = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: 0
        }
    });

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
            .expect(&format!(
                "Field {} has invalid obs_property, should be name value",
                field_name
            ));

        let type_t = &name_values
            .iter()
            .find(|e| e.path.get_ident().unwrap().to_string() == "type_t")
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
            .find(|e| e.path.get_ident().unwrap().to_string() == "settings_key");

        if let Some(n) = pot_name {
            obs_settings_name = match &n.value {
                syn::Expr::Lit(e) => match &e.lit {
                    syn::Lit::Str(s) => s.value(),
                    _ => panic!("setings_key must be a string"),
                },
                _ => panic!("settings_key must be a string"),
            };
        }

        let set_field = quote::format_ident!("set_{}", field_name);
        let type_t_str = type_t.as_str();
        let to_add = match type_t_str {
            "enum" => {
                quote! {
                    pub fn #set_field(mut self, #field_name: #field_type) -> Self {
                        use num_traits::ToPrimitive;
                        use libobs::wrapper::sources::ObsSourceBuilder;
                        let val = #field_name.to_i32().unwrap();

                        println!("Hi set {}", "#obs_settings_name");
                        //self.get_or_create_settings()
                        //    .set_int("#obs_settings_name", val as i64);

                        self
                    }
                }
            }
            "string" => {
                quote! {
                    pub fn #set_field(mut self, #field_name: impl Into<libobs::wrapper::ObsString>) -> Self {
                        use libobs::wrapper::sources::ObsSourceBuilder;
                        println!("Hi set {}", "#obs_settings_name");
                        //self.get_or_create_settings()
//                            .set_string("#field_name", #field_name);
                        self
                    }
                }
            }
            _ => panic!("Unsupported type_t {}", type_t),
        };

        functions.push(to_add);
    }

    let expanded = quote! {
        #(#attributes)*
        #visibility struct #name #generics {
            #(#fields_tokens,)*
            settings: Option<libobs::wrapper::ObsData>,
            hotkeys: Option<libobs::wrapper::ObsData>,
            name: libobs::wrapper::ObsString
        }

        impl libobs::wrapper::sources::ObsSourceBuilder for #name {
            fn new(name: impl Into<libobs::wrapper::ObsString>) -> Self {
                Self {
                    #(#field_initializers,)*
                    settings: None,
                    hotkeys: None,
                    name: name.into(),
                }
            }

            fn get_settings(&self) -> &Option<libobs::wrapper::ObsData> {
                &self.settings
            }

            fn get_settings_mut(&mut self) -> &mut Option<libobs::wrapper::ObsData> {
                &mut self.settings
            }

            fn get_hotkeys(&self) -> &Option<libobs::wrapper::ObsData> {
                &self.hotkeys
            }

            fn get_hotkeys_mut(&mut self) -> &mut Option<libobs::wrapper::ObsData> {
                &mut self.hotkeys
            }

            fn get_name(&self) -> libobs::wrapper::ObsString {
                self.name.clone()
            }

            fn get_id() -> libobs::wrapper::ObsString {
                #id_value.into()
            }
        }

        impl #name {
            #(#functions)*
        }
    };

    TokenStream::from(expanded)
}
