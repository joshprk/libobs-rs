use std::env;
use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::num::NonZero;
use std::num::NonZeroU8;
use std::path::Path;
use std::path::PathBuf;

use crate::ffi::obs_audio_info;
use crate::ffi::obs_video_info;

/// String type specifically designed to interface with libobs.
///
/// It internally wraps a CString and is completely immutable during
/// its lifetime. Furthermore, all ObsStrings are valid C-style strings
/// as all null bytes are removed when constructing the internal CString.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObsString {
    inner: CString,
}

impl ObsString {
    /// Creates a new ObsString where the given input can be loselessly
    /// translated into an UTF-8 byte array and have all its null bytes 
    /// stripped before an internal CString is made.
    pub fn new<T: Into<Vec<u8>>>(t: T) -> Self {
        let v = t.into()
            .into_iter()
            .filter(|c| *c != b'\0')
            .map(|x| NonZeroU8::new(x).unwrap())
            .collect::<Vec<NonZero<u8>>>();

        Self { inner: CString::from(v) }
    }
    
    /// Returns the FFI pointer of the internal CString.
    pub fn as_ptr(&self) -> *const c_char {
        self.inner.as_ptr()
    }
}

impl From<String> for ObsString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for ObsString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<CString> for ObsString {
    fn from(s: CString) -> Self {
        Self::new(s)
    }
}

impl From<&CStr> for ObsString {
    fn from(s: &CStr) -> Self {
        Self::new(s.to_owned())
    }
}

impl From<Vec<u8>> for ObsString {
    fn from(v: Vec<u8>) -> Self {
        Self::new(v)
    }
}

/// Mutable path type specifically for inputting path arguments
/// to this library.
///
/// This type internally wraps a PathBuf and can easily be converted
/// into an ``ObsString``. This type can be considered a "builder"
/// for ``ObsString``, but for paths.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd)]
pub struct ObsPath {
    inner: PathBuf,
}

impl ObsPath {
    /// Creates a new ObsPath from the relative path.
    ///
    /// This function takes an argument which is appended to the 
    /// directory of the executable that this library is called from.
    /// If an absolute path is necessary, simply input an absolute path.
    pub fn new<P: AsRef<Path>>(p: P) -> Self {
        let mut r = env::current_exe()
            .unwrap()
            .to_path_buf();

        r.pop();
        r.push(p);

        Self { inner: r }
    }
    
    /// Appends an extra level to the path.
    pub fn push<P: AsRef<Path>>(&mut self, p: P) {
        self.inner.push(p)
    }

    /// Mutates the path to its parent.
    pub fn pop(&mut self) -> bool {
        self.inner.pop()
    }

    /// Consumes the ObsPath to create an ObsString.
    pub fn into_obs_string(self) -> ObsString {
        let is_dir = self.inner.is_dir();
        let mut bytes = self.inner
            .into_os_string()
            .into_encoded_bytes()
            .into_iter()
            .filter_map(|c| {
                if c == b'\\' {
                    return Some(b'/')
                }
                Some(c)
            })
            .collect::<Vec<u8>>();

        if let Some(last) = bytes.last() {
            if is_dir && *last != b'/' {
                bytes.push(b'/');
            }
        }

        ObsString::new(bytes)
    }
}

impl Into<ObsString> for ObsPath {
    fn into(self) -> ObsString {
        self.into_obs_string()
    }
}

impl<P> From<P> for ObsPath where P: AsRef<Path> {
    fn from(value: P) -> Self {
        ObsPath::new(value)
    }
}

pub struct AudioInfo {
    inner: obs_audio_info
}

impl AudioInfo {
    pub fn new() -> Self {
        todo!()
    }
}

pub struct VideoInfo {
    inner: obs_video_info
}
