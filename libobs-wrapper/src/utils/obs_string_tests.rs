#[cfg(test)]
mod tests {
    use super::super::ObsString;

    #[test]
    fn test_obs_string_new() {
        let obs_string = ObsString::new("test_string");
        assert_eq!(format!("{}", obs_string), "test_string");
    }

    #[test]
    fn test_obs_string_new_with_nul_bytes() {
        // NUL bytes should be stripped
        let obs_string = ObsString::new("test\0string");
        assert_eq!(format!("{}", obs_string), "teststring");
    }

    #[test]
    fn test_obs_string_from_str() {
        let obs_string: ObsString = "hello".into();
        assert_eq!(format!("{}", obs_string), "hello");
    }

    #[test]
    fn test_obs_string_from_str_with_nul() {
        let obs_string: ObsString = "hel\0lo".into();
        assert_eq!(format!("{}", obs_string), "hello");
    }

    #[test]
    fn test_obs_string_from_string() {
        let s = String::from("test_value");
        let obs_string: ObsString = s.into();
        assert_eq!(format!("{}", obs_string), "test_value");
    }

    #[test]
    fn test_obs_string_from_string_with_nul() {
        let s = String::from("test\0value");
        let obs_string: ObsString = s.into();
        assert_eq!(format!("{}", obs_string), "testvalue");
    }

    #[test]
    fn test_obs_string_from_vec_u8() {
        let bytes = b"hello".to_vec();
        let obs_string: ObsString = bytes.into();
        assert_eq!(format!("{}", obs_string), "hello");
    }

    #[test]
    fn test_obs_string_from_vec_u8_with_nul() {
        let bytes = vec![104, 101, 0, 108, 108, 111]; // "he\0llo"
        let obs_string: ObsString = bytes.into();
        assert_eq!(format!("{}", obs_string), "hello");
    }

    #[test]
    fn test_obs_string_empty() {
        let obs_string = ObsString::new("");
        assert_eq!(format!("{}", obs_string), "");
    }

    #[test]
    fn test_obs_string_clone() {
        let obs_string1 = ObsString::new("clone_test");
        let obs_string2 = obs_string1.clone();
        assert_eq!(format!("{}", obs_string1), format!("{}", obs_string2));
    }

    #[test]
    fn test_obs_string_eq() {
        let obs_string1 = ObsString::new("equal");
        let obs_string2 = ObsString::new("equal");
        assert_eq!(obs_string1, obs_string2);
    }

    #[test]
    fn test_obs_string_ne() {
        let obs_string1 = ObsString::new("value1");
        let obs_string2 = ObsString::new("value2");
        assert_ne!(obs_string1, obs_string2);
    }

    #[test]
    fn test_obs_string_as_ptr_not_null() {
        let obs_string = ObsString::new("pointer_test");
        let ptr = obs_string.as_ptr();
        assert!(!ptr.0.is_null());
    }

    #[test]
    fn test_obs_string_default() {
        let obs_string = ObsString::default();
        assert_eq!(format!("{}", obs_string), "");
    }

    #[test]
    fn test_obs_string_ord() {
        let obs_string1 = ObsString::new("aaa");
        let obs_string2 = ObsString::new("bbb");
        assert!(obs_string1 < obs_string2);
    }

    #[test]
    fn test_obs_string_unicode() {
        let obs_string = ObsString::new("こんにちは");
        assert_eq!(format!("{}", obs_string), "こんにちは");
    }

    #[test]
    fn test_obs_string_special_chars() {
        let obs_string = ObsString::new("!@#$%^&*()");
        assert_eq!(format!("{}", obs_string), "!@#$%^&*()");
    }
}
