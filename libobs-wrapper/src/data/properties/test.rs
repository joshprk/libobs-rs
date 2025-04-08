#[cfg(test)]
mod tests {
    use crate::{data::properties::ObsPropertyObject, sources::ObsSourceRef, utils::ObsString};

    #[test]
    fn test_get_properties() {
        let source = ObsSourceRef::new("monitor_capture", "Test source", None, None).unwrap();

        let result = source.get_properties();
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_get_properties_by_id() {
        let id = ObsString::new("monitor_capture");
        let result = ObsSourceRef::get_properties_by_id(id);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}