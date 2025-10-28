use libobs::audio_output;
use std::{borrow::Borrow, ptr, sync::Arc};

use crate::{
    data::ObsData,
    impl_obs_drop, run_with_obs,
    runtime::ObsRuntime,
    unsafe_send::Sendable,
    utils::{AudioEncoderInfo, ObsError, ObsString},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ObsAudioEncoder {
    pub(crate) encoder: Sendable<*mut libobs::obs_encoder_t>,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,
    pub(crate) settings: Option<ObsData>,
    pub(crate) hotkey_data: Option<ObsData>,
    pub(crate) runtime: ObsRuntime,
}

impl ObsAudioEncoder {
    pub fn new_from_info(
        info: AudioEncoderInfo,
        mixer_idx: usize,
        handler: Sendable<*mut audio_output>,
        runtime: ObsRuntime,
    ) -> Result<Arc<Self>, ObsError> {
        #[allow(deprecated)]
        let encoder = Self::new(
            info.id,
            info.name,
            info.settings,
            mixer_idx,
            info.hotkey_data,
            runtime.clone(),
        )?;

        let encoder_ptr = encoder.encoder.clone();
        run_with_obs!(encoder.runtime, (encoder_ptr, handler), move || unsafe {
            libobs::obs_encoder_set_audio(encoder_ptr, handler);
        })?;

        Ok(encoder)
    }

    #[deprecated = "Use `ObsAudioEncoder::new_from_info` instead, this will be removed in a future release."]
    pub fn new<T: Into<ObsString> + Sync + Send, K: Into<ObsString> + Sync + Send>(
        id: T,
        name: K,
        settings: Option<ObsData>,
        mixer_idx: usize,
        hotkey_data: Option<ObsData>,
        runtime: ObsRuntime,
    ) -> Result<Arc<Self>, ObsError> {
        let id = id.into();
        let name = name.into();

        let settings_ptr = match settings.borrow() {
            Some(x) => x.as_ptr(),
            None => Sendable(ptr::null_mut()),
        };

        let hotkey_data_ptr = match hotkey_data.borrow() {
            Some(x) => x.as_ptr(),
            None => Sendable(ptr::null_mut()),
        };

        let id_ptr = id.as_ptr();
        let name_ptr = name.as_ptr();

        let encoder = run_with_obs!(
            runtime,
            (hotkey_data_ptr, settings_ptr, id_ptr, name_ptr),
            move || unsafe {
                let ptr = libobs::obs_audio_encoder_create(
                    id_ptr,
                    name_ptr,
                    settings_ptr,
                    mixer_idx,
                    hotkey_data_ptr,
                );
                Sendable(ptr)
            }
        )?;

        if encoder.0.is_null() {
            return Err(ObsError::NullPointer);
        }

        Ok(Arc::new(Self {
            encoder,
            id,
            name,
            settings,
            hotkey_data,
            runtime,
        }))
    }

    /// This is only needed once for global audio context
    pub fn set_audio_context(
        &mut self,
        handler: Sendable<*mut audio_output>,
    ) -> Result<(), ObsError> {
        let encoder_ptr = self.encoder.clone();

        run_with_obs!(self.runtime, (handler, encoder_ptr), move || unsafe {
            libobs::obs_encoder_set_audio(encoder_ptr, handler)
        })
    }
}

impl_obs_drop!(ObsAudioEncoder, (encoder), move || unsafe {
    libobs::obs_encoder_release(encoder)
});
