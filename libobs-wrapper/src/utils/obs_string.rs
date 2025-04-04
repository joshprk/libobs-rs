use std::ffi::CString;
use std::os::raw::c_char;

/// String wrapper for OBS function calls.
///
/// This struct wraps `CString` internally with included helper
/// functions. Note that any NUL byte is stripped before
/// conversion to a `CString` to prevent panicking.
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObsString {
    c_string: CString,
}

impl ObsString {
    /// Creates a new `ObsString` wrapper for C-type
    /// strings used by libobs. Note that all NUL
    /// bytes are removed before conversion to a
    /// `ObsString` as C-type strings do not allow
    /// premature NUL bytes.
    ///
    /// These are CString wrappers internally, with
    /// included helper functions to reduce repetitive
    /// code and ensure safety.
    pub fn new(value: &str) -> Self {
        Self::from(value)
    }

    /// Returns a safe pointer to a C-type string
    /// used by libobs. This pointer will be valid
    /// for as long as this ObsString exists.
    ///
    /// Note that this pointer is read-only--writing
    /// to it is undefined behavior.
    pub fn as_ptr(&self) -> *const c_char {
        self.c_string.as_ptr()
    }
}

impl ToString for ObsString {
    fn to_string(&self) -> String {
        // We can use the lossy method here since the c_string is guaranteed to be UTF-8.
        self.c_string.to_string_lossy().to_string()
    }
}

impl From<&str> for ObsString {
    fn from(value: &str) -> Self {
        let value = value.replace("\0", "");
        Self {
            c_string: CString::new(value).unwrap(),
        }
    }
}

impl From<Vec<u8>> for ObsString {
    fn from(value: Vec<u8>) -> Self {
        let mut value = value
            .into_iter()
            .filter(|x| *x != b'\0')
            .collect::<Vec<u8>>();

        value.push(b'\0');

        Self {
            c_string: CString::from_vec_with_nul(value).unwrap(),
        }
    }
}

impl From<String> for ObsString {
    fn from(value: String) -> Self {
        let value = value.replace("\0", "");
        Self {
            c_string: CString::new(value).unwrap(),
        }
    }
}