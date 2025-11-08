#[cfg(test)]
mod tests {
    use super::super::{ObsPath, ObsString};

    #[test]
    fn test_obs_path_new() {
        let path = ObsPath::new("C:/test/path");
        let obs_string = path.build();
        assert_eq!(format!("{}", obs_string), "C:/test/path");
    }

    #[test]
    fn test_obs_path_new_with_backslash() {
        let path = ObsPath::new("C:\\test\\path");
        let obs_string = path.build();
        // Backslashes should be converted to forward slashes
        assert_eq!(format!("{}", obs_string), "C:/test/path");
    }

    #[test]
    fn test_obs_path_push_single() {
        let path = ObsPath::new("C:/base");
        let path = path.push("subfolder");
        let obs_string = path.build();
        assert!(format!("{}", obs_string).contains("subfolder"));
    }

    #[test]
    fn test_obs_path_push_multiple() {
        let path = ObsPath::new("C:/base");
        let path = path.push("sub1/sub2/sub3");
        let obs_string = path.build();
        let path_str = format!("{}", obs_string);
        assert!(path_str.contains("sub1"));
        assert!(path_str.contains("sub2"));
        assert!(path_str.contains("sub3"));
    }

    #[test]
    fn test_obs_path_push_with_backslashes() {
        let path = ObsPath::new("C:/base");
        let path = path.push("sub1\\sub2");
        let obs_string = path.build();
        let path_str = format!("{}", obs_string);
        assert!(path_str.contains("sub1"));
        assert!(path_str.contains("sub2"));
    }

    #[test]
    fn test_obs_path_push_with_leading_slash() {
        let path = ObsPath::new("C:/base");
        let path = path.push("/subfolder");
        let obs_string = path.build();
        assert!(format!("{}", obs_string).contains("subfolder"));
    }

    #[test]
    fn test_obs_path_push_with_trailing_slash() {
        let path = ObsPath::new("C:/base");
        let path = path.push("subfolder/");
        let obs_string = path.build();
        assert!(format!("{}", obs_string).contains("subfolder"));
    }

    #[test]
    fn test_obs_path_push_empty_string() {
        let path = ObsPath::new("C:/base");
        let path_before = path.clone();
        let path = path.push("");
        assert_eq!(path_before, path);
    }

    #[test]
    fn test_obs_path_pop() {
        let path = ObsPath::new("C:/test/path");
        let path = path.pop();
        let obs_string = path.build();
        let path_str = format!("{}", obs_string);
        assert!(path_str.contains("test"));
        assert!(!path_str.contains("path"));
    }

    #[test]
    fn test_obs_path_push_then_pop() {
        let path = ObsPath::new("C:/base");
        let path = path.push("temp");
        let path = path.pop();
        let obs_string = path.build();
        assert_eq!(format!("{}", obs_string), "C:/base");
    }

    #[test]
    fn test_obs_path_default() {
        let path = ObsPath::default();
        let obs_string = path.build();
        assert_eq!(format!("{}", obs_string), "");
    }

    #[test]
    fn test_obs_path_clone() {
        let path1 = ObsPath::new("C:/test");
        let path2 = path1.clone();
        assert_eq!(path1, path2);
    }

    #[test]
    fn test_obs_path_eq() {
        let path1 = ObsPath::new("C:/test");
        let path2 = ObsPath::new("C:/test");
        assert_eq!(path1, path2);
    }

    #[test]
    fn test_obs_path_ne() {
        let path1 = ObsPath::new("C:/test1");
        let path2 = ObsPath::new("C:/test2");
        assert_ne!(path1, path2);
    }

    #[test]
    fn test_obs_path_into_obs_string() {
        let path = ObsPath::new("C:/test");
        let obs_string: ObsString = path.into();
        assert_eq!(format!("{}", obs_string), "C:/test");
    }

    #[test]
    fn test_obs_path_chaining() {
        let path = ObsPath::new("C:/base")
            .push("folder1")
            .push("folder2")
            .push("file.txt");
        let obs_string = path.build();
        let path_str = format!("{}", obs_string);
        assert!(path_str.contains("base"));
        assert!(path_str.contains("folder1"));
        assert!(path_str.contains("folder2"));
        assert!(path_str.contains("file.txt"));
    }

    #[test]
    fn test_obs_path_ord() {
        let path1 = ObsPath::new("C:/aaa");
        let path2 = ObsPath::new("C:/bbb");
        assert!(path1 < path2);
    }
}
