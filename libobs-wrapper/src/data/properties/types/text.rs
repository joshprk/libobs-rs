use getters0::Getters;

use crate::data::properties::{get_enum, macros::assert_type, ObsTextInfoType, ObsTextType};

use super::PropertyCreationInfo;

#[derive(Debug, Getters, Clone)]
#[skip_new]
pub struct ObsTextProperty {
    name: String,
    description: String,
    monospace: bool,
    text_type: ObsTextType,
    info_type: ObsTextInfoType,
    word_wrap: bool,
}

impl From<PropertyCreationInfo> for ObsTextProperty {
    fn from(
        PropertyCreationInfo {
            name,
            description,
            pointer,
        }: PropertyCreationInfo,
    ) -> Self {
        assert_type!(Text, pointer);

        let info_type = get_enum!(pointer, text_info_type, ObsTextInfoType);
        let text_type = get_enum!(pointer, text_type, ObsTextType);

        let monospace = unsafe { libobs::obs_property_text_monospace(pointer) };
        let word_wrap = unsafe { libobs::obs_property_text_info_word_wrap(pointer) };

        ObsTextProperty {
            name,
            description,
            monospace,
            text_type,
            info_type,
            word_wrap,
        }
    }
}
