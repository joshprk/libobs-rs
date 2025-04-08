mod enums;
mod macros;
pub mod types;
pub mod prop_impl;

use std::ffi::CStr;

use libobs::obs_properties;
use macros::*;

pub use enums::*;
use num_traits::FromPrimitive;
use types::*;

use crate::utils::ObsString;

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

pub trait ObsPropertyObjectPrivate {
    fn get_properties_raw(&self) -> *mut libobs::obs_properties_t;
    fn get_properties_by_id_raw(id: ObsString) -> *mut libobs::obs_properties_t;
}

fn get_properties_inner(properties_raw: *mut obs_properties) -> anyhow::Result<Vec<ObsProperty>> {
    let mut result = Vec::new();

    if properties_raw.is_null() {
        return Ok(result);
    }

    let mut property = unsafe { libobs::obs_properties_first(properties_raw) };
    while !property.is_null() {
        let name = unsafe { libobs::obs_property_name(property) };
        let name = unsafe { CStr::from_ptr(name as _) };
        let name = name.to_str()?.to_string();

        let p_type = unsafe { libobs::obs_property_get_type(property) };
        let p_type = ObsPropertyType::from_i32(p_type);

        match p_type {
            Some(p_type) => {
                result.push(p_type.to_property_struct(property)?);
            }
            None => result.push(ObsProperty::Invalid(name)),
        }

        // Move to the next property
        unsafe { libobs::obs_property_next(&mut property) };
    }

    unsafe { libobs::obs_properties_destroy(properties_raw) };

    Ok(result)
}

/// This trait is implemented for all obs objects that can have properties
pub trait ObsPropertyObject: ObsPropertyObjectPrivate {
    /// Returns the properties of the object
    fn get_properties(&self) -> anyhow::Result<Vec<ObsProperty>> {
        let properties_raw = self.get_properties_raw();
        get_properties_inner(properties_raw)
    }

    fn get_properties_by_id(id: ObsString) -> anyhow::Result<Vec<ObsProperty>> {
        let properties_raw = Self::get_properties_by_id_raw(id);
        get_properties_inner(properties_raw)
    }
}
