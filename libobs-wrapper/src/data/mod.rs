use std::{ffi::{CStr, CString}, sync::Arc};

use anyhow::bail;
use libobs::{
    obs_data, obs_data_create, obs_data_release, obs_data_set_bool, obs_data_set_double, obs_data_set_int, obs_data_set_string
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
mod updater;
pub use updater::*;

#[derive(Debug)]
pub(crate) struct _ObsDataDropGuard {
    obs_data: Sendable<*mut obs_data>,
    pub(crate) runtime: ObsRuntime,
}

/// Contains `obs_data` and its related strings. Note that
/// this struct prevents string pointers from being freed
/// by keeping them owned.
/// Update: The strings are actually copied by obs itself, we don't need to store them
#[derive(Debug)]
pub struct ObsData {
    obs_data: Sendable<*mut obs_data>,
    pub(crate) runtime: ObsRuntime,
    pub(crate) _drop_guard: Arc<_ObsDataDropGuard>
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
        let obs_data = run_with_obs!(runtime, move || unsafe { Sendable(obs_data_create()) })?;

        Ok(ObsData {
            obs_data: obs_data.clone(),
            runtime: runtime.clone(),
            _drop_guard: Arc::new(_ObsDataDropGuard {
                obs_data,
                runtime,
            }),
        })
    }

    pub fn bulk_update(& mut self) -> ObsDataUpdater {
        ObsDataUpdater {
            changes: Vec::new(),
            obs_data: self.obs_data.clone(),
            _drop_guard: self._drop_guard.clone(),
        }
    }

    /// Returns a pointer to the raw `obs_data`
    /// represented by `ObsData`.
    pub fn as_ptr(&self) -> Sendable<*mut obs_data> {
        self.obs_data.clone()
    }

    /// Sets a string in `obs_data` and stores it so
    /// it in `ObsData` does not get freed.
    pub async fn set_string<T: Into<ObsString> + Send + Sync, K: Into<ObsString> + Send + Sync>(
        &mut self,
        key: T,
        value: K,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();
        let value = value.into();

        let key_ptr = key.as_ptr();
        let value_ptr = value.as_ptr();
        let data_ptr = self.obs_data.clone();

        run_with_obs!(self.runtime, (data_ptr, key_ptr, value_ptr), move || unsafe {
            obs_data_set_string(data_ptr, key_ptr, value_ptr)
        })?;

        Ok(self)
    }

    /// Sets an int in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub async fn set_int<T: Into<ObsString> + Sync + Send>(
        &mut self,
        key: T,
        value: i64,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();

        run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            obs_data_set_int(data_ptr, key_ptr, value.into());
        })?;

        Ok(self)
    }

    /// Sets a bool in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub async fn set_bool<T: Into<ObsString> + Sync + Send>(
        &mut self,
        key: T,
        value: bool,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();
        run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            obs_data_set_bool(data_ptr, key_ptr, value.into());
        })?;

        Ok(self)
    }

    /// Sets a double in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub async fn set_double<T: Into<ObsString> + Sync + Send>(
        &mut self,
        key: T,
        value: f64,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();

        run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            obs_data_set_double(data_ptr, key_ptr, value.into());
        })?;

        Ok(self)
    }

    pub async fn from_json(json: &str, runtime: ObsRuntime) -> anyhow::Result<Self> {
        let cstr = CString::new(json)?;

        let cstr_ptr = Sendable(cstr.as_ptr());
        let result = run_with_obs!(runtime, (cstr_ptr), move || unsafe {
            Sendable(libobs::obs_data_create_from_json(cstr_ptr))
        })?;

        if result.0.is_null() {
            bail!("Failed to set JSON in obs_data");
        }

        Ok(ObsData {
            obs_data: result.clone(),
            runtime: runtime.clone(),
            _drop_guard: Arc::new(_ObsDataDropGuard {
                obs_data: result,
                runtime,
            }),
        })
    }

    pub async fn get_json(&self) -> anyhow::Result<String> {
        let data_ptr = self.obs_data.clone();
        let ptr = run_with_obs!(self.runtime, (data_ptr), move || unsafe {
            Sendable(libobs::obs_data_get_json(data_ptr))
        })?;

        if ptr.0.is_null() {
            bail!("Failed to get JSON from obs_data");
        }

        let ptr = unsafe { CStr::from_ptr(ptr.0) };
        Ok(ptr.to_str()?.to_string())
    }
}

impl_obs_drop!(_ObsDataDropGuard, (obs_data), move || unsafe {
    obs_data_release(obs_data)
});

impl ObsData {
    pub async fn clone(&self) -> anyhow::Result<Self> {
        let json = self.get_json().await?;

        Self::from_json(json.as_str(), self.runtime.clone()).await
    }
}
