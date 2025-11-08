use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

use crate::{
    impl_obs_drop, run_with_obs,
    runtime::ObsRuntime,
    unsafe_send::Sendable,
    utils::{ObsError, ObsString},
};
use libobs::obs_data;

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
/// Cloning `ObsData` is blocking and will create a new `ObsData` instance. Recommended is to use `ObsData::full_clone()` instead.
/// ## Panics
/// If the underlying JSON representation can not be parsed.
//NOTE: Update: The strings are actually copied by obs itself, we don't need to store them
#[derive(Debug)]
pub struct ObsData {
    obs_data: Sendable<*mut obs_data>,
    pub(crate) runtime: ObsRuntime,
    pub(crate) _drop_guard: Arc<_ObsDataDropGuard>,
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
    pub fn new(runtime: ObsRuntime) -> Result<Self, ObsError> {
        let obs_data = run_with_obs!(runtime, move || unsafe {
            Sendable(libobs::obs_data_create())
        })?;

        Ok(ObsData {
            obs_data: obs_data.clone(),
            runtime: runtime.clone(),
            _drop_guard: Arc::new(_ObsDataDropGuard { obs_data, runtime }),
        })
    }

    pub fn bulk_update(&mut self) -> ObsDataUpdater {
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
    pub fn set_string<T: Into<ObsString> + Send + Sync, K: Into<ObsString> + Send + Sync>(
        &mut self,
        key: T,
        value: K,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();
        let value = value.into();

        let key_ptr = key.as_ptr();
        let value_ptr = value.as_ptr();
        let data_ptr = self.obs_data.clone();

        run_with_obs!(
            self.runtime,
            (data_ptr, key_ptr, value_ptr),
            move || unsafe { libobs::obs_data_set_string(data_ptr, key_ptr, value_ptr) }
        )?;

        Ok(self)
    }

    pub fn get_string<T: Into<ObsString> + Send + Sync>(
        &self,
        key: T,
    ) -> Result<Option<String>, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();

        let result = run_with_obs!(self.runtime, (data_ptr, key_ptr), move || unsafe {
            if libobs::obs_data_has_user_value(data_ptr, key_ptr)
                || libobs::obs_data_has_default_value(data_ptr, key_ptr)
            {
                Some(Sendable(libobs::obs_data_get_string(data_ptr, key_ptr)))
            } else {
                None
            }
        })?;

        if result.is_none() {
            return Ok(None);
        }

        let result = result.unwrap();
        if result.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        let result = unsafe { CStr::from_ptr(result.0) };
        let result = result
            .to_str()
            .map_err(|_| ObsError::StringConversionError)?;

        Ok(Some(result.to_string()))
    }

    /// Sets an int in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub fn set_int<T: Into<ObsString> + Sync + Send>(
        &mut self,
        key: T,
        value: i64,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();

        run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            libobs::obs_data_set_int(data_ptr, key_ptr, value);
        })?;

        Ok(self)
    }

    pub fn get_int<T: Into<ObsString> + Sync + Send>(
        &self,
        key: T,
    ) -> Result<Option<i64>, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();

        let result = run_with_obs!(self.runtime, (data_ptr, key_ptr), move || unsafe {
            if libobs::obs_data_has_user_value(data_ptr, key_ptr)
                || libobs::obs_data_has_default_value(data_ptr, key_ptr)
            {
                Some(libobs::obs_data_get_int(data_ptr, key_ptr))
            } else {
                None
            }
        })?;

        Ok(result)
    }

    /// Sets a bool in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub fn set_bool<T: Into<ObsString> + Sync + Send>(
        &mut self,
        key: T,
        value: bool,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();
        run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            libobs::obs_data_set_bool(data_ptr, key_ptr, value);
        })?;

        Ok(self)
    }

    pub fn get_bool<T: Into<ObsString> + Sync + Send>(
        &self,
        key: T,
    ) -> Result<Option<bool>, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();

        let result = run_with_obs!(self.runtime, (data_ptr, key_ptr), move || unsafe {
            if libobs::obs_data_has_user_value(data_ptr, key_ptr)
                || libobs::obs_data_has_default_value(data_ptr, key_ptr)
            {
                Some(libobs::obs_data_get_bool(data_ptr, key_ptr))
            } else {
                None
            }
        })?;

        Ok(result)
    }

    /// Sets a double in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub fn set_double<T: Into<ObsString> + Sync + Send>(
        &mut self,
        key: T,
        value: f64,
    ) -> Result<&mut Self, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();

        run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            libobs::obs_data_set_double(data_ptr, key_ptr, value);
        })?;

        Ok(self)
    }

    pub fn get_double<T: Into<ObsString> + Sync + Send>(
        &self,
        key: T,
    ) -> Result<Option<f64>, ObsError> {
        let key = key.into();

        let key_ptr = key.as_ptr();
        let data_ptr = self.obs_data.clone();

        let result = run_with_obs!(self.runtime, (key_ptr, data_ptr), move || unsafe {
            if libobs::obs_data_has_user_value(data_ptr, key_ptr)
                || libobs::obs_data_has_default_value(data_ptr, key_ptr)
            {
                Some(libobs::obs_data_get_double(data_ptr, key_ptr))
            } else {
                None
            }
        })?;

        Ok(result)
    }

    pub fn from_json(json: &str, runtime: ObsRuntime) -> Result<Self, ObsError> {
        let cstr = CString::new(json).map_err(|_| ObsError::JsonParseError)?;

        let cstr_ptr = Sendable(cstr.as_ptr());
        let result = run_with_obs!(runtime, (cstr_ptr), move || unsafe {
            Sendable(libobs::obs_data_create_from_json(cstr_ptr))
        })?;

        if result.0.is_null() {
            return Err(ObsError::JsonParseError);
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

    pub fn get_json(&self) -> Result<String, ObsError> {
        let data_ptr = self.obs_data.clone();
        let ptr = run_with_obs!(self.runtime, (data_ptr), move || unsafe {
            Sendable(libobs::obs_data_get_json(data_ptr))
        })?;

        if ptr.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        let ptr = unsafe { CStr::from_ptr(ptr.0) };
        let ptr = ptr.to_str().map_err(|_| ObsError::JsonParseError)?;

        Ok(ptr.to_string())
    }
}

impl_obs_drop!(_ObsDataDropGuard, (obs_data), move || unsafe {
    libobs::obs_data_release(obs_data)
});

impl Clone for ObsData {
    fn clone(&self) -> Self {
        let json = self.get_json().unwrap();
        Self::from_json(json.as_str(), self.runtime.clone()).unwrap()
    }
}
