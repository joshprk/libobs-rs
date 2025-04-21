use std::ffi::{CStr, CString};

use anyhow::bail;
use libobs::{
    obs_data, obs_data_create, obs_data_release, obs_data_set_bool, obs_data_set_double,
    obs_data_set_int, obs_data_set_string,
};

use crate::{
    impl_obs_drop, run_with_obs, runtime::ObsRuntime, unsafe_send::Sendable, utils::{ObsError, ObsString}
};

pub mod audio;
pub mod immutable;
mod lib_support;
pub mod output;
pub mod properties;
pub mod video;
pub use lib_support::*;

/// Contains `obs_data` and its related strings. Note that
/// this struct prevents string pointers from being freed
/// by keeping them owned.
#[derive(Debug)]
pub struct ObsData {
    obs_data: Sendable<*mut obs_data>,
    strings: Vec<ObsString>,
    pub(crate) runtime: ObsRuntime,
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
    pub async fn new(runtime: ObsRuntime) -> Result<Self, ObsError> {
        let obs_data = run_with_obs!(runtime, move || unsafe { obs_data_create() })?;
        let strings = Vec::new();

        Ok(ObsData {
            obs_data: Sendable(obs_data),
            strings,
            runtime,
        })
    }

    /// Returns a pointer to the raw `obs_data`
    /// represented by `ObsData`.
    pub fn as_ptr(&self) -> *mut obs_data {
        self.obs_data.0
    }

    /// Sets a string in `obs_data` and stores it so
    /// it in `ObsData` does not get freed.
    pub async fn set_string(
        &mut self,
        key: impl Into<ObsString>,
        value: impl Into<ObsString>,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();
        let value = value.into();

        let key_ptr = key.as_ptr();
        let value_ptr = value.as_ptr();
        let data_ptr = self.obs_data.0;

        run_with_obs!(self.runtime, (data_ptr, key_ptr, value_ptr), move || unsafe {
            obs_data_set_string(data_ptr, key_ptr, value_ptr)
        })?;

        self.strings.push(key);
        self.strings.push(value);

        Ok(self)
    }

    /// Sets an int in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub async fn set_int(
        &mut self,
        key: impl Into<ObsString>,
        value: i64,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.0;

        run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            obs_data_set_int(data_ptr, key_ptr, value.into())
        })?;

        self.strings.push(key);

        Ok(self)
    }

    /// Sets a bool in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub async fn set_bool(
        &mut self,
        key: impl Into<ObsString>,
        value: bool,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.0;
        run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            obs_data_set_bool(data_ptr, key_ptr, value.into())
        })?;

        self.strings.push(key);

        Ok(self)
    }

    /// Sets a double in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub async fn set_double(
        &mut self,
        key: impl Into<ObsString>,
        value: f64,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.0;

        run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            obs_data_set_double(data_ptr, key_ptr, value.into())
        })?;

        self.strings.push(key);

        Ok(self)
    }

    pub async fn from_json(json: &str, runtime: ObsRuntime) -> anyhow::Result<Self> {
        let cstr = CString::new(json)?;
        let strings = Vec::new();

        let cstr_ptr = cstr.as_ptr();

        let result = run_with_obs!(runtime, (cstr_ptr), move || unsafe {
            libobs::obs_data_create_from_json(cstr_ptr)
        })?;

        if result.is_null() {
            bail!("Failed to set JSON in obs_data");
        }

        Ok(ObsData {
            obs_data: Sendable(result),
            strings,
            runtime,
        })
    }

    pub async fn get_json(&self) -> anyhow::Result<String> {
        let data_ptr = self.obs_data.0;
        let ptr = run_with_obs!(self.runtime, (data_ptr), move || unsafe {
            libobs::obs_data_get_json(data_ptr)
        })?;

        if ptr.is_null() {
            bail!("Failed to get JSON from obs_data");
        }

        let ptr = unsafe { CStr::from_ptr(ptr) };
        Ok(ptr.to_str()?.to_string())
    }
}

impl_obs_drop!(ObsData, (obs_data), move || unsafe {
    obs_data_release(obs_data.0)
});

impl ObsData {
    pub async fn clone(&self) -> anyhow::Result<Self> {
        let json = self.get_json().await?;

        Self::from_json(json.as_str(), self.runtime.clone()).await
    }
}
