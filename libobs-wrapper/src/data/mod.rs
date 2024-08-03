use libobs::{obs_data, obs_data_create, obs_data_release, obs_data_set_bool, obs_data_set_double, obs_data_set_int, obs_data_set_string};

use crate::utils::ObsString;

pub mod video;
pub mod audio;
pub mod output;

/// Contains `obs_data` and its related strings. Note that
/// this struct prevents string pointers from being freed
/// by keeping them owned.
#[derive(Debug)]
pub struct ObsData {
    obs_data: *mut obs_data,
    strings: Vec<ObsString>,
}

impl ObsData {
    /// Creates a new empty `ObsData` wrapper for the
    /// libobs `obs_data` data structure.
    ///
    /// `ObsData` can then be populated using the set
    /// functions, which take ownership of the
    /// `ObsString` types to prevent them from being
    /// dropped prematurely. This makes it safer than
    /// using `obs_data` directly from libobs.
    pub fn new() -> Self {
        let obs_data = unsafe { obs_data_create() };
        let strings = Vec::new();
        ObsData { obs_data, strings }
    }

    /// Returns a pointer to the raw `obs_data`
    /// represented by `ObsData`.
    pub fn as_ptr(&self) -> *mut obs_data {
        self.obs_data
    }

    /// Sets a string in `obs_data` and stores it so
    /// it in `ObsData` does not get freed.
    pub fn set_string(
        &mut self,
        key: impl Into<ObsString>,
        value: impl Into<ObsString>,
    ) -> &mut Self {
        let key = key.into();
        let value = value.into();

        unsafe { obs_data_set_string(self.obs_data, key.as_ptr(), value.as_ptr()) }

        self.strings.push(key);
        self.strings.push(value);

        self
    }

    /// Sets an int in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub fn set_int(&mut self, key: impl Into<ObsString>, value: i64) -> &mut Self {
        let key = key.into();

        unsafe { obs_data_set_int(self.obs_data, key.as_ptr(), value.into()) }

        self.strings.push(key);

        self
    }

    /// Sets a bool in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub fn set_bool(&mut self, key: impl Into<ObsString>, value: bool) -> &mut Self {
        let key = key.into();

        unsafe { obs_data_set_bool(self.obs_data, key.as_ptr(), value) }

        self.strings.push(key);

        self
    }

    /// Sets a double in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub fn set_double(&mut self, key: impl Into<ObsString>, value: f64) -> &mut Self {
        let key = key.into();

        unsafe { obs_data_set_double(self.obs_data, key.as_ptr(), value) }

        self.strings.push(key);

        self
    }
}

impl Drop for ObsData {
    fn drop(&mut self) {
        unsafe { obs_data_release(self.obs_data) }
    }
}