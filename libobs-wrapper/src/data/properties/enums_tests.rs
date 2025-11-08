#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::{FromPrimitive, ToPrimitive};

    #[test]
    fn test_obs_property_type_values() {
        assert_eq!(ObsPropertyType::Bool, ObsPropertyType::Bool);
        assert_ne!(ObsPropertyType::Bool, ObsPropertyType::Int);
    }

    #[test]
    fn test_obs_property_type_clone() {
        let prop = ObsPropertyType::Text;
        let cloned = prop.clone();
        assert_eq!(prop, cloned);
    }

    #[test]
    fn test_obs_property_type_debug() {
        let prop = ObsPropertyType::Button;
        let debug_str = format!("{:?}", prop);
        assert!(debug_str.contains("Button"));
    }

    #[test]
    fn test_obs_combo_format_values() {
        assert_eq!(ObsComboFormat::String, ObsComboFormat::String);
        assert_ne!(ObsComboFormat::String, ObsComboFormat::Int);
    }

    #[test]
    fn test_obs_combo_format_clone() {
        let format = ObsComboFormat::Float;
        let cloned = format.clone();
        assert_eq!(format, cloned);
    }

    #[test]
    fn test_obs_combo_type_values() {
        assert_eq!(ObsComboType::List, ObsComboType::List);
        assert_ne!(ObsComboType::List, ObsComboType::Radio);
    }

    #[test]
    fn test_obs_text_type_values() {
        assert_eq!(ObsTextType::Password, ObsTextType::Password);
        assert_ne!(ObsTextType::Password, ObsTextType::Multiline);
    }

    #[test]
    fn test_obs_text_info_type_values() {
        assert_eq!(ObsTextInfoType::Warning, ObsTextInfoType::Warning);
        assert_ne!(ObsTextInfoType::Warning, ObsTextInfoType::Error);
    }

    #[test]
    fn test_obs_number_type_values() {
        assert_eq!(ObsNumberType::Slider, ObsNumberType::Slider);
        assert_ne!(ObsNumberType::Slider, ObsNumberType::Scroller);
    }

    #[test]
    fn test_obs_path_type_values() {
        assert_eq!(ObsPathType::Directory, ObsPathType::Directory);
        assert_ne!(ObsPathType::Directory, ObsPathType::File);
    }

    #[test]
    fn test_obs_editable_list_type_values() {
        assert_eq!(ObsEditableListType::Files, ObsEditableListType::Files);
        assert_ne!(ObsEditableListType::Files, ObsEditableListType::Strings);
    }

    #[test]
    fn test_obs_group_type_values() {
        assert_eq!(ObsGroupType::Normal, ObsGroupType::Normal);
        assert_ne!(ObsGroupType::Normal, ObsGroupType::Checkable);
    }

    #[test]
    fn test_obs_button_type_values() {
        assert_eq!(ObsButtonType::Default, ObsButtonType::Default);
        assert_ne!(ObsButtonType::Default, ObsButtonType::Url);
    }

    #[test]
    fn test_all_enums_are_copy() {
        let prop = ObsPropertyType::Bool;
        let _copy1 = prop;
        let _copy2 = prop; // Should work because it's Copy
    }
}
