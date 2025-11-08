#[cfg(test)]
mod tests {
    use super::should_update;

    #[test]
    fn test_should_update_same_version() {
        // Assuming current version constants, we need a test that uses them
        // This test verifies valid version string parsing
        let result = should_update("30.2.2");
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_update_invalid_format_two_parts() {
        let result = should_update("30.2");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid version string"));
    }

    #[test]
    fn test_should_update_invalid_format_four_parts() {
        let result = should_update("30.2.2.1");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid version string"));
    }

    #[test]
    fn test_should_update_invalid_format_empty() {
        let result = should_update("");
        assert!(result.is_err());
    }

    #[test]
    fn test_should_update_invalid_major_non_numeric() {
        let result = should_update("abc.2.2");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid version string"));
    }

    #[test]
    fn test_should_update_invalid_minor_non_numeric() {
        let result = should_update("30.abc.2");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid version string"));
    }

    #[test]
    fn test_should_update_invalid_patch_non_numeric() {
        let result = should_update("30.2.abc");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid version string"));
    }

    #[test]
    fn test_should_update_with_spaces() {
        let result = should_update("30 .2.2");
        assert!(result.is_err());
    }

    #[test]
    fn test_should_update_with_leading_zeros() {
        // Leading zeros should still parse correctly
        let result = should_update("030.002.002");
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_update_zero_version() {
        let result = should_update("0.0.0");
        assert!(result.is_ok());
        // This should indicate an update is needed
        assert!(result.unwrap());
    }

    #[test]
    fn test_should_update_very_high_version() {
        let result = should_update("999.999.999");
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_update_negative_numbers() {
        let result = should_update("-1.2.3");
        assert!(result.is_err());
    }

    #[test]
    fn test_should_update_special_chars() {
        let result = should_update("30!2@2");
        assert!(result.is_err());
    }
}
