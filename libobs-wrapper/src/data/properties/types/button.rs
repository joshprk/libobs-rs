use getters0::Getters;

use crate::data::properties::{get_enum, get_opt_str, macros::assert_type, ObsButtonType};

use super::PropertyCreationInfo;

#[derive(Debug, Getters, Clone)]
#[skip_new]
pub struct ObsButtonProperty {
    name: String,
    description: String,
    button_type: ObsButtonType,
    url: Option<String>,
}

impl From<PropertyCreationInfo> for ObsButtonProperty {
    fn from(
        PropertyCreationInfo {
            name,
            description,
            pointer,
        }: PropertyCreationInfo,
    ) -> Self {
        assert_type!(Button, pointer);

        let url = get_opt_str!(pointer, button_url);
        let button_type = get_enum!(pointer, button_type, ObsButtonType);

        Self {
            name,
            description,
            button_type,
            url,
        }
    }
}
