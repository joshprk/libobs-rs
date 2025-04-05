use getters0::Getters;

use crate::data::properties::{ObsComboFormat, ObsComboType};

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

#[derive(Debug, Getters, Clone)]
#[skip_new]
pub enum ObsListItemValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}