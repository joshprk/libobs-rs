#[cfg(test)]
mod tests {
    use super::ObsBootstrapError;

    #[test]
    fn test_general_error_display() {
        let error = ObsBootstrapError::GeneralError("test error".to_string());
        let display_str = format!("{}", error);
        assert!(display_str.contains("Bootstrapper error"));
        assert!(display_str.contains("test error"));
    }

    #[test]
    fn test_download_error_display() {
        let error = ObsBootstrapError::DownloadError("download failed".to_string());
        let display_str = format!("{}", error);
        assert!(display_str.contains("download error"));
        assert!(display_str.contains("download failed"));
    }

    #[test]
    fn test_extract_error_display() {
        let error = ObsBootstrapError::ExtractError("extraction failed".to_string());
        let display_str = format!("{}", error);
        assert!(display_str.contains("extract error"));
        assert!(display_str.contains("extraction failed"));
    }

    #[test]
    fn test_error_clone() {
        let error1 = ObsBootstrapError::GeneralError("test".to_string());
        let error2 = error1.clone();
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_eq() {
        let error1 = ObsBootstrapError::GeneralError("test".to_string());
        let error2 = ObsBootstrapError::GeneralError("test".to_string());
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_ne() {
        let error1 = ObsBootstrapError::GeneralError("test1".to_string());
        let error2 = ObsBootstrapError::GeneralError("test2".to_string());
        assert_ne!(error1, error2);
    }

    #[test]
    fn test_error_debug() {
        let error = ObsBootstrapError::GeneralError("test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("GeneralError"));
    }

    #[test]
    fn test_error_is_std_error() {
        let error = ObsBootstrapError::GeneralError("test".to_string());
        let _: &dyn std::error::Error = &error;
    }

    #[test]
    fn test_different_error_types_ne() {
        let error1 = ObsBootstrapError::GeneralError("test".to_string());
        let error2 = ObsBootstrapError::DownloadError("test".to_string());
        assert_ne!(error1, error2);
    }
}
