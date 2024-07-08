use std::env;
use std::ffi::{c_char, CStr, CString};
use std::num::{NonZero, NonZeroU8};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObsString {
    inner: CString,
}

impl ObsString {
    pub fn new<T: Into<Vec<u8>>>(t: T) -> Self {
        let v = t.into()
            .into_iter()
            .filter(|c| *c != b'\0')
            .map(|x| NonZeroU8::new(x).unwrap())
            .collect::<Vec<NonZero<u8>>>();

        Self { inner: CString::from(v) }
    }

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

#[derive(Clone, Debug, Default, Hash, PartialEq, PartialOrd)]
pub struct ObsPath {
    inner: PathBuf,
}

impl ObsPath {
    pub fn new<P: AsRef<Path>>(p: P) -> Self {
        let mut r = env::current_exe()
            .unwrap()
            .to_path_buf();

        r.pop();
        r.push(p);

        Self { inner: r }
    }

    pub fn push<P: AsRef<Path>>(&mut self, p: P) {
        self.inner.push(p)
    }

    pub fn pop(&mut self) -> bool {
        self.inner.pop()
    }

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

        if is_dir {
            bytes.push(b'/');
        }

        ObsString::new(bytes)
    }
}

impl Into<ObsString> for ObsPath {
    fn into(self) -> ObsString {
        self.into_obs_string()
    }
}