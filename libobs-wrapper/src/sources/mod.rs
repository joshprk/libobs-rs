mod builder;
pub use builder::*;

use libobs::{obs_source_create, obs_source_release, obs_source_update};

use crate::{
    data::ObsData,
    unsafe_send::WrappedObsSource,
    utils::{traits::ObsUpdatable, ObsError, ObsString},
};
use std::{borrow::Borrow, ptr};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ObsSource {
    pub(crate) source: WrappedObsSource,
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

        let source =
            unsafe { obs_source_create(id.as_ptr(), name.as_ptr(), settings_ptr, hotkey_data_ptr) };

        if source == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        Ok(Self {
            source: WrappedObsSource(source),
            id,
            name,
            settings,
            hotkey_data,
        })
    }

    pub fn settings(&self) -> Option<&ObsData> {
        self.settings.as_ref()
    }

    pub fn hotkey_data(&self) -> Option<&ObsData> {
        self.hotkey_data.as_ref()
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl Drop for ObsSource {
    fn drop(&mut self) {
        unsafe { obs_source_release(self.source.0) }
    }
}

impl ObsUpdatable for ObsSource {
    fn update_raw(&mut self, data: ObsData) {
        unsafe { obs_source_update(self.source.0, data.as_ptr()) }
        self.settings = Some(data);
    }

    fn reset_and_update(&mut self, data: ObsData) {
        unsafe { obs_source_}
    }
}
