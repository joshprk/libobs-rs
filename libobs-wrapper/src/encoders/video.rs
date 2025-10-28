use libobs::{obs_encoder, video_output};
use std::{ptr, sync::Arc};

use crate::{
    data::ObsData,
    impl_obs_drop, run_with_obs,
    runtime::ObsRuntime,
    unsafe_send::Sendable,
    utils::{ObsError, ObsString, VideoEncoderInfo},
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
    pub fn new_from_info(
        info: VideoEncoderInfo,
        handler: Sendable<*mut video_output>,
        runtime: ObsRuntime,
    ) -> Result<Arc<Self>, ObsError> {
        #[allow(deprecated)]
        let encoder = Self::new(info.id, info.name, info.settings, info.hotkey_data, runtime)?;

        let encoder_ptr = encoder.encoder.clone();
        run_with_obs!(encoder.runtime, (encoder_ptr, handler), move || unsafe {
            libobs::obs_encoder_set_video(encoder_ptr, handler);
        })?;

        Ok(encoder)
    }

    #[deprecated = "Use `ObsVideoEncoder::new_from_info` instead, this will be removed in a future release."]
    pub fn new<T: Into<ObsString> + Sync + Send, K: Into<ObsString> + Sync + Send>(
        id: T,
        name: K,
        settings: Option<ObsData>,
        hotkey_data: Option<ObsData>,
        runtime: ObsRuntime,
    ) -> Result<Arc<Self>, ObsError> {
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
                let ptr = libobs::obs_video_encoder_create(
                    id_ptr,
                    name_ptr,
                    settings_ptr,
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

    pub fn as_ptr(&self) -> Sendable<*mut obs_encoder> {
        self.encoder.clone()
    }

    /// This is only needed once for global video context
    pub fn set_video_context(
        &mut self,
        handler: Sendable<*mut video_output>,
    ) -> Result<(), ObsError> {
        let self_ptr = self.as_ptr();
        run_with_obs!(self.runtime, (handler, self_ptr), move || unsafe {
            libobs::obs_encoder_set_video(self_ptr, handler);
        })
    }
}

impl_obs_drop!(ObsVideoEncoder, (encoder), move || unsafe {
    libobs::obs_encoder_release(encoder);
});
