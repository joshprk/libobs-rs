mod lib_support;
pub use lib_support::*;
use libobs::{obs_source, obs_source_create, obs_source_release};

use std::{borrow::Borrow, ptr};
use crate::{data::ObsData, utils::{ObsError, ObsString}};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ObsSource {
    pub(crate) source: *mut obs_source,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,
    pub(crate) settings: Option<ObsData>,
    pub(crate) hotkey_data: Option<ObsData>,
}

impl ObsSource {
    pub fn new(
        id: impl Into<ObsString>,
        name: impl Into<ObsString>,
        settings: Option<ObsData>,
        hotkey_data: Option<ObsData>,
    ) -> Result<Self, ObsError> {
        let id = id.into();
        let name = name.into();

        let settings_ptr = match settings.borrow() {
            Some(x) => x.as_ptr(),
            None => ptr::null_mut(),
        };

        let hotkey_data_ptr = match hotkey_data.borrow() {
            Some(x) => x.as_ptr(),
            None => ptr::null_mut(),
        };

        let source = unsafe {
            obs_source_create(id.as_ptr(), name.as_ptr(), settings_ptr, hotkey_data_ptr)
        };

        if source == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        Ok(Self {
            source,
            id,
            name,
            settings,
            hotkey_data,
        })
    }
}

impl Drop for ObsSource {
    fn drop(&mut self) {
        unsafe { obs_source_release(self.source) }
    }
}