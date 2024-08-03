use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, Data, DeriveInput, Expr, Fields, LitStr,
    MetaNameValue, Token,
};

#[allow(unused_assignments)]
#[proc_macro_attribute]
/// This macro is used to generate a builder pattern for an obs source. <br>
/// The attribute should be the id of the source.<br>
/// The struct should have named fields, each field should have an attribute `#[obs_property(type_t="your_type")]`. <br>
/// `type_t` can be `enum`, `enum_string`, `string`, `bool` or `int`. <br>
/// - `enum`: the field should be an enum with `num_derive::{FromPrimitive, ToPrimitive}`.
/// - `enum_string`: the field should be an enum which implements `StringEnum`.
/// - `string`: the field should be a string.
/// - `bool`: the field should be a bool.
/// - `type_t`: `int`, the field should be an i64.
/// The attribute can also have a `settings_key` which is the key used in the settings, if this attribute is not given, the macro defaults to the field name. <br>
/// Documentation is inherited from the field to the setter function.<br>
/// Example: <br>
/// ```
/// use libobs_wrapper::sources::StringEnum;
/// use libobs_source_macro::obs_source_builder;
/// use num_derive::{FromPrimitive, ToPrimitive};
///
/// #[repr(i32)]
/// #[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// pub enum ObsWindowCaptureMethod {
///     MethodAuto = libobs::window_capture_method_METHOD_AUTO,
/// 	MethodBitBlt = libobs::window_capture_method_METHOD_BITBLT,
/// 	MethodWgc = libobs::window_capture_method_METHOD_WGC,
/// }
///
/// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// pub enum ObsGameCaptureRgbaSpace {
///     SRgb,
///     RGBA2100pq
/// }
///
/// impl StringEnum for ObsGameCaptureRgbaSpace {
///     fn to_str(&self) -> &str {
///         match self {
///             ObsGameCaptureRgbaSpace::SRgb => "sRGB",
///             ObsGameCaptureRgbaSpace::RGBA2100pq => "Rec. 2100 (PQ)"
///         }
///     }
/// }
///
/// /// Provides a easy to use builder for the window capture source.
/// #[derive(Debug)]
/// #[obs_source_builder("window_capture")]
/// pub struct WindowCaptureSourceBuilder {
///     #[obs_property(type_t="enum")]
///     /// Sets the capture method for the window capture
///     capture_method: ObsWindowCaptureMethod,
///
///     /// Sets the window to capture.
///     #[obs_property(type_t = "string", settings_key = "window")]
///     window_raw: String,
///
///     #[obs_property(type_t = "bool")]
///     /// Sets whether the cursor should be captured
///     cursor: bool,
///
///     /// Sets the capture mode for the game capture source. Look at doc for `ObsGameCaptureMode`
///     #[obs_property(type_t = "enum_string")]
///     capture_mode: ObsGameCaptureMode,
/// }
/// ```
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
                        use libobs_wrapper::sources::ObsSourceBuilder;
                        let val = #field_name.to_i32().unwrap();

                        self.get_or_create_settings()
                            .set_int(#obs_settings_key, val as i64);

                        self
                    }
                }
            },
            "enum_string" => {
                quote! {
                    #(#docs_attr)*
                    pub fn #set_field(mut self, #field_name: #field_type) -> Self {
                        use libobs_wrapper::sources::StringEnum;
                        use libobs_wrapper::sources::ObsSourceBuilder;

                        self.get_or_create_settings()
                            .set_string(#obs_settings_key, #field_name.to_str());

                        self
                    }
                }
            },
            "string" => {
                quote! {
                    #(#docs_attr)*
                    pub fn #set_field(mut self, #field_name: impl Into<libobs_wrapper::utils::ObsString>) -> Self {
                        use libobs_wrapper::sources::ObsSourceBuilder;
                        self.get_or_create_settings()
                            .set_string(#obs_settings_key, #field_name);
                        self
                    }
                }
            }
            "bool" => {
                quote! {
                    #(#docs_attr)*
                    pub fn #set_field(mut self, #field_name: bool) -> Self {
                        use libobs_wrapper::sources::ObsSourceBuilder;
                        self.get_or_create_settings()
                            .set_bool(#obs_settings_key, #field_name);
                        self
                    }
                }
            },
            "int" => {
                quote! {
                    #(#docs_attr)*
                    pub fn #set_field(mut self, #field_name: i64) -> Self {
                        use libobs_wrapper::sources::ObsSourceBuilder;
                        self.get_or_create_settings()
                            .set_int(#obs_settings_key, #field_name);
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

    let expanded = quote! {
        #(#attributes)*
        #[allow(dead_code)]
        #visibility struct #name #generics {
            #(#fields_tokens,)*
            settings: Option<libobs_wrapper::data::ObsData>,
            hotkeys: Option<libobs_wrapper::data::ObsData>,
            name: libobs_wrapper::utils::ObsString
        }

        impl libobs_wrapper::sources::ObsSourceBuilder for #name {
            fn new(name: impl Into<libobs_wrapper::utils::ObsString>) -> Self {
                Self {
                    #(#field_initializers,)*
                    settings: None,
                    hotkeys: None,
                    name: name.into(),
                }
            }

            fn get_settings(&self) -> &Option<libobs_wrapper::data::ObsData> {
                &self.settings
            }

            fn get_settings_mut(&mut self) -> &mut Option<libobs_wrapper::data::ObsData> {
                &mut self.settings
            }

            fn get_hotkeys(&self) -> &Option<libobs_wrapper::data::ObsData> {
                &self.hotkeys
            }

            fn get_hotkeys_mut(&mut self) -> &mut Option<libobs_wrapper::data::ObsData> {
                &mut self.hotkeys
            }

            fn get_name(&self) -> libobs_wrapper::utils::ObsString {
                self.name.clone()
            }

            fn get_id() -> libobs_wrapper::utils::ObsString {
                #id_value.into()
            }
        }

        impl #name {
            #(#functions)*
        }
    };

    TokenStream::from(expanded)
}

fn collect_doc(attrs: &Vec<Attribute>) -> (Vec<String>, Vec<&Attribute>) {
    let mut docs_str = Vec::new();
    let mut docs_attr = Vec::new();
    for attr in attrs {
        let name_val = match &attr.meta {
            syn::Meta::NameValue(n) => n,
            _ => continue,
        };

        let is_doc = name_val.path.is_ident("doc");
        if !is_doc {
            continue;
        }

        let lit = match &name_val.value {
            Expr::Lit(l) => match &l.lit {
                syn::Lit::Str(s) => s.value(),
                _ => continue,
            },
            _ => continue,
        };

        docs_str.push(lit);
        docs_attr.push(attr);
    }

    (docs_str, docs_attr)
}
