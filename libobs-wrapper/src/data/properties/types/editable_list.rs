use getters0::Getters;

use crate::data::properties::{assert_type, get_enum, get_opt_str, ObsEditableListType};

use super::PropertyCreationInfo;

#[derive(Debug, Getters, Clone)]
#[skip_new]
pub struct ObsEditableListProperty {
    name: String,
    description: Option<String>,
    list_type: ObsEditableListType,
    filter: String,
    default_path: String,
}



impl From<PropertyCreationInfo> for ObsEditableListProperty {
    fn from(PropertyCreationInfo { name, description, pointer }: PropertyCreationInfo) -> Self {
        assert_type!(EditableList, pointer);

        let list_type = get_enum!(pointer, list_type, ObsEditableListType);
        let filter = get_opt_str!(pointer, path_filter).unwrap_or_default();
        let default_path = get_opt_str!(pointer, path_default_path).unwrap_or_default();

        Self {
            name,
            description,
            list_type,
            filter,
            default_path,
        }
    }
}