mod builder;
pub use builder::*;

use libobs::{
    obs_scene_item, obs_source_create, obs_source_release, obs_source_reset_settings, obs_source_t, obs_source_update
};

use crate::{
    data::{immutable::ImmutableObsData, ObsData},
    impl_obs_drop, run_with_obs,
    runtime::ObsRuntime,
    unsafe_send::Sendable,
    utils::{traits::ObsUpdatable, ObsError, ObsString},
};
use std::{ptr, sync::Arc};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ObsSourceRef {
    pub(crate) source: Sendable<*mut obs_source_t>,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,
    pub(crate) settings: Arc<ImmutableObsData>,
    pub(crate) hotkey_data: Arc<ImmutableObsData>,
    pub(crate) scene_item: Option<Sendable<*mut obs_scene_item>>,

    _guard: Arc<_ObsSourceGuard>,
    pub(crate) runtime: ObsRuntime,
}

impl ObsSourceRef {
    pub async fn new<T: Into<ObsString> + Sync + Send, K: Into<ObsString> + Sync + Send>(
        id: T,
        name: K,
        mut settings: Option<ObsData>,
        mut hotkey_data: Option<ObsData>,
        runtime: ObsRuntime,
    ) -> Result<Self, ObsError> {
        let id = id.into();
        let name = name.into();

        let settings = match settings.take() {
            Some(x) => ImmutableObsData::from(x),
            None => ImmutableObsData::new(&runtime).await?,
        };

        let hotkey_data = match hotkey_data.take() {
            Some(x) => ImmutableObsData::from(x),
            None => ImmutableObsData::new(&runtime).await?,
        };

        let hotkey_data_ptr = hotkey_data.as_ptr();
        let settings_ptr = settings.as_ptr();
        let id_ptr = id.as_ptr();
        let name_ptr = name.as_ptr();

        let source = run_with_obs!(
            runtime,
            (hotkey_data_ptr, settings_ptr, id_ptr, name_ptr),
            move || unsafe {
                Sendable(obs_source_create(
                    id_ptr,
                    name_ptr,
                    settings_ptr,
                    hotkey_data_ptr,
                ))
            }
        )?;

        if source.0 == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        Ok(Self {
            source: source.clone(),
            id,
            name,
            settings: Arc::new(settings),
            hotkey_data: Arc::new(hotkey_data),
            _guard: Arc::new(_ObsSourceGuard {
                source,
                runtime: runtime.clone(),
            }),
            scene_item: None,
            runtime,
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

#[async_trait::async_trait]
impl ObsUpdatable for ObsSourceRef {
    async fn update_raw(&mut self, data: ObsData) -> Result<(), ObsError> {
        let source_ptr = self.source.clone();
        run_with_obs!(self.runtime, (source_ptr), move || unsafe {
            obs_source_update(source_ptr, data.as_ptr().0);
        })
    }

    async fn reset_and_update_raw(&mut self, data: ObsData) -> Result<(), ObsError> {
        let source_ptr = self.source.clone();
        run_with_obs!(self.runtime, (source_ptr), move || unsafe {
            obs_source_reset_settings(source_ptr, data.as_ptr().0);
        })
    }

    fn runtime(&self) -> ObsRuntime {
        self.runtime.clone()
    }
}

#[derive(Debug)]
struct _ObsSourceGuard {
    source: Sendable<*mut obs_source_t>,
    runtime: ObsRuntime,
}

impl_obs_drop!(_ObsSourceGuard, (source), move || unsafe {
    obs_source_release(source);
});
