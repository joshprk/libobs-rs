use super::PropertyCreationInfo;
use crate::data::properties::{assert_type, get_enum, ObsComboFormat, ObsComboType};
use getters0::Getters;
use std::ffi::CStr;

#[derive(Debug, Getters, Clone)]
#[skip_new]
pub struct ObsListProperty {
    name: String,
    description: String,
    list_type: ObsComboType,
    format: ObsComboFormat,
    items: Vec<ObsListItem>,
}

#[derive(Debug, Getters, Clone)]
#[skip_new]
pub struct ObsListItem {
    name: String,
    value: ObsListItemValue,
    disabled: bool,
}

#[derive(Debug, Clone)]
pub enum ObsListItemValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Invalid
}

impl ObsListItem {
    fn new(name: String, value: ObsListItemValue, disabled: bool) -> Self {
        Self {
            name,
            value,
            disabled,
        }
    }
}

impl From<PropertyCreationInfo> for ObsListProperty {
    fn from(
        PropertyCreationInfo {
            name,
            description,
            pointer,
        }: PropertyCreationInfo,
    ) -> Self {
        assert_type!(List, pointer);

        let list_type = get_enum!(pointer, list_type, ObsComboType);
        let format = get_enum!(pointer, list_format, ObsComboFormat);

        let count = unsafe { libobs::obs_property_list_item_count(pointer) };
        let mut items = Vec::with_capacity(count);

        for i in 0..count {
            let list_name = unsafe {
                CStr::from_ptr(libobs::obs_property_list_item_name(pointer, i))
                    .to_str()
                    .unwrap_or_default()
                    .to_string()
            };
            let is_disabled = unsafe { libobs::obs_property_list_item_disabled(pointer, i) };
            let value = match format {
                ObsComboFormat::Invalid => ObsListItemValue::Invalid,
                ObsComboFormat::Int => {
                    let int_val = unsafe { libobs::obs_property_list_item_int(pointer, i) };
                    ObsListItemValue::Int(int_val)
                }
                ObsComboFormat::Float => {
                    let float_val = unsafe { libobs::obs_property_list_item_float(pointer, i) };
                    ObsListItemValue::Float(float_val)
                }
                ObsComboFormat::String => {
                    let string_val = unsafe {
                        CStr::from_ptr(libobs::obs_property_list_item_string(pointer, i))
                            .to_str()
                            .unwrap_or_default()
                            .to_string()
                    };
                    ObsListItemValue::String(string_val)
                }
                ObsComboFormat::Bool => {
                    let bool_val = unsafe { libobs::obs_property_list_item_bool(pointer, i) };
                    ObsListItemValue::Bool(bool_val)
                }
            };
            items.push(ObsListItem::new(list_name, value, is_disabled));
        }

        Self {
            name,
            description,
            list_type,
            format,
            items,
        }
    }
}
