#[cfg(test)]
mod tests {
    use super::{StrLen, ToUtf8String};

    #[test]
    fn test_strlen_empty_slice() {
        let data: [u16; 0] = [];
        assert_eq!(data.strlen(), 0);
    }

    #[test]
    fn test_strlen_no_null_terminator() {
        let data: [u16; 5] = [1, 2, 3, 4, 5];
        assert_eq!(data.strlen(), 5);
    }

    #[test]
    fn test_strlen_with_null_terminator() {
        let data: [u16; 5] = [1, 2, 3, 0, 5];
        assert_eq!(data.strlen(), 3);
    }

    #[test]
    fn test_strlen_only_null() {
        let data: [u16; 1] = [0];
        assert_eq!(data.strlen(), 0);
    }

    #[test]
    fn test_strslice_empty() {
        let data: [u16; 0] = [];
        assert_eq!(data.strslice().len(), 0);
    }

    #[test]
    fn test_strslice_with_null() {
        let data: [u16; 5] = [1, 2, 3, 0, 5];
        let slice = data.strslice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice, &[1, 2, 3]);
    }

    #[test]
    fn test_strslice_no_null() {
        let data: [u16; 3] = [1, 2, 3];
        let slice = data.strslice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice, &[1, 2, 3]);
    }

    #[test]
    fn test_to_utf8_hello() {
        // "Hello" in UTF-16
        let data: Vec<u16> = vec![72, 101, 108, 108, 111];
        assert_eq!(data.to_utf8(), "Hello");
    }

    #[test]
    fn test_to_utf8_with_null_terminator() {
        // "Hi" with null terminator in UTF-16
        let data: Vec<u16> = vec![72, 105, 0];
        assert_eq!(data.to_utf8(), "Hi");
    }

    #[test]
    fn test_to_utf8_empty() {
        let data: Vec<u16> = vec![];
        assert_eq!(data.to_utf8(), "");
    }

    #[test]
    fn test_to_utf8_only_null() {
        let data: Vec<u16> = vec![0];
        assert_eq!(data.to_utf8(), "");
    }

    #[test]
    fn test_to_utf8_with_multiple_nulls() {
        // "AB" with multiple null terminators
        let data: Vec<u16> = vec![65, 66, 0, 0, 0];
        assert_eq!(data.to_utf8(), "AB");
    }

    #[test]
    fn test_strlen_i32_type() {
        let data: [i32; 5] = [1, 2, 3, 0, 5];
        assert_eq!(data.strlen(), 3);
    }

    #[test]
    fn test_strlen_u8_type() {
        let data: [u8; 5] = [1, 2, 3, 0, 5];
        assert_eq!(data.strlen(), 3);
    }
}
