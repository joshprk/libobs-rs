use std::{
    env,
    path::{Path, PathBuf},
};

use super::ObsString;

/// Builds into an `ObsString` that represents a path used
/// by libobs.
///
/// Note that only this path only supports UTF-8 for the
/// entire absolute path because libobs only supports
/// UTF-8.
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObsPath {
    path: PathBuf,
}

impl ObsPath {
    /// Creates a new `ObsPath` strictly using the path
    /// `path_str` without any modifications.
    ///
    /// If you want to create a relative path, use
    /// `ObsPath::from_relative`.
    pub fn new(path_str: &str) -> Self {
        Self {
            path: Path::new(path_str).into(),
        }
    }

    /// Creates a new `ObsPath` with `path_str`
    /// appended to the path of the directory which the
    /// executable file is in.
    ///
    /// If you want to create an absolute path, use
    /// `ObsPath::new`.
    pub fn from_relative(path_str: &str) -> Self {
        let mut relative_path = env::current_exe().unwrap();

        relative_path.pop();

        let obs_path = Self {
            path: relative_path,
        };

        let path_str = path_str.trim_matches('/');

        obs_path.push(path_str)
    }

    /// Modifies the path to point to the path
    /// `path_str` appended to the current path which
    /// `ObsPath` is pointing to.
    pub fn push(mut self, value: &str) -> Self {
        let split = value.split(['/', '\\'].as_ref());

        for item in split {
            if item.len() > 0 {
                self.path.push(item);
            }
        }

        self
    }

    /// Modifies the path to point to its current
    /// parent. This is analogous to `Obs::push(".")`.
    pub fn pop(mut self) -> Self {
        self.path.pop();
        self
    }

    /// Consumes the `ObsPath` to create a new
    /// immutable ObsString that encodes a UTF-8
    /// C-type string which describes the path that
    /// the `ObsPath` is pointing to.
    ///
    /// Note that this function is lossy in that
    /// any non-Unicode data is completely removed
    /// from the string. This is because libobs
    /// does not support non-Unicode characters in
    /// its path.
    pub fn build(self) -> ObsString {
        let mut bytes = self.path.display().to_string().replace("\\", "/");

        if self.path.is_dir() {
            bytes = bytes + "/";
        }
        let obs_string = ObsString::from(bytes.as_str());

        drop(self);
        obs_string
    }
}

impl Into<ObsString> for ObsPath {
    fn into(self) -> ObsString {
        self.build()
    }
}
