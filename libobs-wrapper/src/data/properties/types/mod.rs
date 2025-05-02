//! # Important Notice
//! All structs in this module use direct obs calls to get the data from the obs_property_t struct. **ALWAYS MAKE SURE THIS IS RUNNING ON THE OBS THREAD**

mod button;
impl_general_property!(Color);
mod editable_list;
impl_general_property!(Font);
impl_general_property!(FrameRate);
impl_general_property!(Group);
impl_general_property!(ColorAlpha);
mod list;
mod number;
mod path;
mod text;

pub(crate) struct PropertyCreationInfo {
    pub name: String,
    pub description: Option<String>,
    pub pointer: *mut libobs::obs_property,
}

use std::ffi::CStr;

pub use button::*;
pub use editable_list::*;
use libobs::obs_property;
pub use list::*;
pub use number::*;
pub use path::*;
pub use text::*;

use super::{macros::impl_general_property, ObsProperty, ObsPropertyType};

impl ObsPropertyType {
    pub fn to_property_struct(&self, pointer: *mut obs_property) -> ObsProperty {
        let name = unsafe { libobs::obs_property_name(pointer) };
        let name = unsafe { CStr::from_ptr(name) };
        let name = name.to_string_lossy().to_string();

        let description = unsafe { libobs::obs_property_description(pointer) };
        let description = if description.is_null() {
            None
        } else {
            let description = unsafe { CStr::from_ptr(description) };
            Some(description.to_string_lossy().to_string())
        };

        let info = PropertyCreationInfo {
            name,
            description,
            pointer,
        };

        match self {
            ObsPropertyType::Invalid => ObsProperty::Invalid,
            ObsPropertyType::Bool => ObsProperty::Bool,
            ObsPropertyType::Int => ObsProperty::Int(ObsNumberProperty::<i32>::from(info)),
            ObsPropertyType::Float => ObsProperty::Float(ObsNumberProperty::<f64>::from(info)),
            ObsPropertyType::Text => ObsProperty::Text(ObsTextProperty::from(info)),
            ObsPropertyType::Path => ObsProperty::Path(ObsPathProperty::from(info)),
            ObsPropertyType::List => ObsProperty::List(ObsListProperty::from(info)),
            ObsPropertyType::Color => ObsProperty::Color(ObsColorProperty::from(info)),
            ObsPropertyType::Button => ObsProperty::Button(ObsButtonProperty::from(info)),
            ObsPropertyType::Font => ObsProperty::Font(ObsFontProperty::from(info)),
            ObsPropertyType::EditableList => ObsProperty::EditableList(ObsEditableListProperty::from(info)),
            ObsPropertyType::FrameRate => ObsProperty::FrameRate(ObsFrameRateProperty::from(info)),
            ObsPropertyType::Group => ObsProperty::Group(ObsGroupProperty::from(info)),
            ObsPropertyType::ColorAlpha => ObsProperty::ColorAlpha(ObsColorAlphaProperty::from(info))
        }
    }
}
