use obs_properties::obs_properties_to_functions;
use parse::UpdaterInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr};

mod docs;
mod parse;
mod fields;
mod obs_properties;

#[proc_macro_attribute]
//TODO more documents here
/// This macro is used to generate an updater pattern for an obs object (for example a source).
/// For more examples look at libobs-sources
pub fn obs_object_updater(attr: TokenStream, item: TokenStream) -> TokenStream {
    let u_input = parse_macro_input!(attr as UpdaterInput);
    let id_value = u_input.name.value();
    let updatable_type = u_input.updatable_type;

    let input = parse_macro_input!(item as DeriveInput);

    let i_ident = input.ident;
    let updater_name = format_ident!("{}", i_ident);

    let visibility = input.vis;
    let attributes = input.attrs;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let (fields_tokens, field_initializers) = fields::generate_fields(&fields);
    let functions = obs_properties_to_functions(
        &fields,
        quote! {
            use libobs_wrapper::data::ObsObjectUpdater;
            self.get_settings_mut()
        },
    );

    let updatable_type2 = updatable_type.clone();
    let expanded = quote! {
        #(#attributes)*
        #[allow(dead_code)]
        #visibility struct #updater_name<'a> {
            #(#fields_tokens,)*
            settings: libobs_wrapper::data::ObsData,
            updatable: &'a mut #updatable_type2
        }

        impl <'a> libobs_wrapper::data::ObsObjectUpdater<'a> for #updater_name<'a> {
            type ToUpdate = #updatable_type;

            fn create_update(updatable: &'a mut Self::ToUpdate) -> Self {
                Self {
                    settings: libobs_wrapper::data::ObsData::new(),
                    updatable,
                    #(#field_initializers,)*
                }
            }

            fn get_settings(&self) -> &libobs_wrapper::data::ObsData {
                &self.settings
            }

            fn get_settings_mut(&mut self) -> &mut libobs_wrapper::data::ObsData {
                &mut self.settings
            }

            fn get_id() -> libobs_wrapper::utils::ObsString {
                #id_value.into()
            }

            fn update(self) {
                use libobs_wrapper::utils::traits::ObsUpdatable;
                let settings = self.settings;
                self.updatable.update_raw(settings);
            }
        }

        impl <'a> #updater_name <'a> {
            #(#functions)*
        }
    };

    TokenStream::from(expanded)
}

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
/// use libobs_wrapper::data::StringEnum;
/// use libobs_source_macro::obs_object_builder;
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
/// #[obs_object_builder("window_capture")]
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
pub fn obs_object_builder(attr: TokenStream, item: TokenStream) -> TokenStream {
    let id = parse_macro_input!(attr as LitStr);

    let input = parse_macro_input!(item as DeriveInput);

    let i_ident = input.ident;
    let builder_name = format_ident!("{}", i_ident);

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
    let (fields_tokens, field_initializers) = fields::generate_fields(&fields);

    let functions = obs_properties_to_functions(
        &fields,
        quote! {
            use libobs_wrapper::data::ObsObjectBuilder;
            self.get_or_create_settings()
        },
    );

    let expanded = quote! {
        #(#attributes)*
        #[allow(dead_code)]
        #visibility struct #builder_name #generics {
            #(#fields_tokens,)*
            settings: Option<libobs_wrapper::data::ObsData>,
            hotkeys: Option<libobs_wrapper::data::ObsData>,
            name: libobs_wrapper::utils::ObsString
        }

        impl libobs_wrapper::data::ObsObjectBuilder for #builder_name {
            fn new(name: impl Into<libobs_wrapper::utils::ObsString>) -> Self {
                Self {
                    name: name.into(),
                    hotkeys: None,
                    settings: None,
                    #(#field_initializers,)*
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

        impl #builder_name {
            #(#functions)*
        }
    };

    TokenStream::from(expanded)
}
