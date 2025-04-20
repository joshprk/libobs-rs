mod builder;
pub use builder::*;

use libobs::{obs_source_create, obs_source_release, obs_source_reset_settings, obs_source_update};

use crate::{
    data::{immutable::ImmutableObsData, ObsData},
    unsafe_send::WrappedObsSource,
    utils::{traits::ObsUpdatable, ObsError, ObsString},
};
use std::{ptr, rc::Rc};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ObsSourceRef {
    pub(crate) source: Arc<WrappedObsSource>,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,
    pub(crate) settings: Arc<ImmutableObsData>,
    pub(crate) hotkey_data: Arc<ImmutableObsData>,

    _guard: Arc<_ObsSourceGuard>,
}

impl ObsSourceRef {
    pub fn new(
        id: impl Into<ObsString>,
        name: impl Into<ObsString>,
        mut settings: Option<ObsData>,
        mut hotkey_data: Option<ObsData>,
    ) -> Result<Self, ObsError> {
        let id = id.into();
        let name = name.into();

        let settings = match settings.take() {
            Some(x) => ImmutableObsData::from(x),
            None => ImmutableObsData::new(),
        };

        let hotkey_data = match hotkey_data.take() {
            Some(x) => ImmutableObsData::from(x),
            None => ImmutableObsData::new(),
        };

        let source = unsafe {
            obs_source_create(
                id.as_ptr(),
                name.as_ptr(),
                settings.as_ptr(),
                hotkey_data.as_ptr(),
            )
        };

        if source == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        Ok(Self {
            source: Arc::new(WrappedObsSource(source)),
            id,
            name,
            settings: Arc::new(settings),
            hotkey_data: Arc::new(hotkey_data),
            _guard: Arc::new(_ObsSourceGuard {
                source: WrappedObsSource(source),
            }),
        })
    }

    pub fn settings(&self) -> &ImmutableObsData {
        &self.settings
    }

    pub fn hotkey_data(&self) -> &ImmutableObsData {
        &self.hotkey_data
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl ObsUpdatable for ObsSourceRef {
    fn update_raw(&mut self, data: ObsData) {
        unsafe { obs_source_update(self.source.0, data.as_ptr()) }
    }

    fn reset_and_update_raw(&mut self, data: ObsData) {
        unsafe {
            obs_source_reset_settings(self.source.0, data.as_ptr());
        }
    }
}

#[derive(Debug)]
struct _ObsSourceGuard {
    source: WrappedObsSource,
}

impl Drop for _ObsSourceGuard {
    fn drop(&mut self) {
        unsafe { obs_source_release(self.source.0) }
    }
}
