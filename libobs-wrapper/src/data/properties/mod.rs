mod enums;
mod macros;
pub mod prop_impl;
pub mod types;

use std::ffi::CStr;

use libobs::obs_properties;
use macros::*;

pub use enums::*;
use num_traits::FromPrimitive;
use types::*;

use crate::{run_with_obs, runtime::ObsRuntime, unsafe_send::Sendable, utils::{ObsError, ObsString}};

#[derive(Debug, Clone)]
pub enum ObsProperty {
    /// A property that is not valid
    Invalid(String),
    /// A boolean property
    Bool,
    /// An integer property
    Int(ObsNumberProperty<i32>),
    /// A float property
    Float(ObsNumberProperty<f64>),
    /// A text property
    Text(ObsTextProperty),
    /// A path property
    Path(ObsPathProperty),
    /// A list property
    List(ObsListProperty),
    /// A color property
    Color(ObsColorProperty),
    /// A button property
    Button(ObsButtonProperty),
    /// A font property
    Font(ObsFontProperty),
    /// An editable list property
    EditableList(ObsEditableListProperty),
    /// A frame rate property
    FrameRate(ObsFrameRateProperty),
    /// A group property
    Group(ObsGroupProperty),
    /// A color alpha property
    ColorAlpha(ObsColorAlphaProperty),
}

#[async_trait::async_trait]
pub trait ObsPropertyObjectPrivate: ObsPropertyObject {
    async fn get_properties_raw(&self) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError>;
    async fn get_properties_by_id_raw(id: ObsString, runtime: ObsRuntime) -> Result<Sendable<*mut libobs::obs_properties_t>, ObsError>;
}

async fn get_properties_inner(
    properties_raw: Sendable<*mut obs_properties>,
    runtime: ObsRuntime,
) -> Result<Vec<ObsProperty>, ObsError> {
    let properties_raw = properties_raw.0;
    if properties_raw.is_null() {
        return Ok(vec![]);
    }

    run_with_obs!(runtime, (properties_raw), move || {
        let mut result = Vec::new();
        let mut property = unsafe { libobs::obs_properties_first(properties_raw) };
        while !property.is_null() {
            let name = unsafe { libobs::obs_property_name(property) };
            let name = unsafe { CStr::from_ptr(name as _) };
            let name = name.to_string_lossy().to_string();

            let p_type = unsafe { libobs::obs_property_get_type(property) };
            let p_type = ObsPropertyType::from_i32(p_type);

            match p_type {
                Some(p_type) => {
                    result.push(p_type.to_property_struct(property));
                }
                None => result.push(ObsProperty::Invalid(name)),
            }

            // Move to the next property
            unsafe { libobs::obs_property_next(&mut property) };
        }

        unsafe { libobs::obs_properties_destroy(properties_raw) };
        result
    })
}

/// This trait is implemented for all obs objects that can have properties
#[async_trait::async_trait]
pub trait ObsPropertyObject {
    /// Returns the properties of the object
    async fn get_properties(&self) -> Result<Vec<ObsProperty>, ObsError>;
    async fn get_properties_by_id(id: ObsString, runtime: ObsRuntime) -> Result<Vec<ObsProperty>, ObsError> {
        let properties_raw = Self::get_properties_by_id_raw(id, runtime.clone()).await?;
        get_properties_inner(properties_raw, runtime).await
    }
}
