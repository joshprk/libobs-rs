use libobs::{
    obs_encoder, obs_encoder_release, obs_encoder_set_video, obs_video_encoder_create, video_output,
};
use std::ptr;

use crate::{
    data::ObsData,
    impl_obs_drop, run_with_obs,
    runtime::ObsRuntime,
    unsafe_send::Sendable,
    utils::{ObsError, ObsString},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ObsVideoEncoder {
    pub(crate) encoder: Sendable<*mut obs_encoder>,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,
    pub(crate) settings: Option<ObsData>,
    pub(crate) hotkey_data: Option<ObsData>,
    pub(crate) runtime: ObsRuntime,
}

impl ObsVideoEncoder {
    pub fn new<T: Into<ObsString> + Sync + Send, K: Into<ObsString> + Sync + Send>(
        id: T,
        name: K,
        settings: Option<ObsData>,
        hotkey_data: Option<ObsData>,
        runtime: ObsRuntime,
    ) -> Result<Self, ObsError> {
        let id = id.into();
        let name = name.into();

        let settings_ptr = match &settings {
            Some(x) => x.as_ptr(),
            None => Sendable(ptr::null_mut()),
        };

        let hotkey_data_ptr = match &hotkey_data {
            Some(x) => x.as_ptr(),
            None => Sendable(ptr::null_mut()),
        };

        let id_ptr = id.as_ptr();
        let name_ptr = name.as_ptr();
        let encoder = run_with_obs!(
            runtime,
            (id_ptr, name_ptr, hotkey_data_ptr, settings_ptr),
            move || unsafe {
                let ptr = obs_video_encoder_create(id_ptr, name_ptr, settings_ptr, hotkey_data_ptr);
                Sendable(ptr)
            }
        )?;

        if encoder.0 == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        Ok(Self {
            encoder,
            id,
            name,
            settings,
            hotkey_data,
            runtime,
        })
    }

    pub fn as_ptr(&self) -> Sendable<*mut obs_encoder> {
        self.encoder.clone()
    }

    /// This is only needed once for global video context
    pub fn set_video_context(&mut self, handler: Sendable<*mut video_output>) -> Result<(), ObsError> {
        let self_ptr = self.as_ptr();
        run_with_obs!(self.runtime, (handler, self_ptr), move || unsafe {
            Sendable(obs_encoder_set_video(self_ptr, handler));
        })
    }
}

impl_obs_drop!(ObsVideoEncoder, (encoder), move || unsafe {
    obs_encoder_release(encoder);
});
