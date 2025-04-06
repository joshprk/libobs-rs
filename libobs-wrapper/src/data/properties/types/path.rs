use getters0::Getters;

use crate::data::properties::{get_enum, get_opt_str, macros::assert_type, ObsPathType};

use super::PropertyCreationInfo;

#[derive(Debug, Getters, Clone)]
#[skip_new]
pub struct ObsPathProperty {
    name: String,
    description: String,
    path_type: ObsPathType,
    filter: String,
    default_path: String,
}

impl From<PropertyCreationInfo> for ObsPathProperty {
    fn from(PropertyCreationInfo { name, description, pointer }: PropertyCreationInfo) -> Self {
        assert_type!(Path, pointer);

        let path_type = get_enum!(pointer, path_type, ObsPathType);
        let filter = get_opt_str!(pointer, path_filter).unwrap_or_default();
        let default_path = get_opt_str!(pointer, path_default_path).unwrap_or_default();
        Self {
            name,
            description,
            path_type,
            filter,
            default_path,
        }
    }
}