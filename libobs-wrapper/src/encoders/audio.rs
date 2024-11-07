use libobs::{obs_audio_encoder_create, obs_encoder_release};
use std::{borrow::Borrow, ptr};

use crate::{data::ObsData, unsafe_send::WrappedObsEncoder, utils::{ObsError, ObsString}};


#[derive(Debug)]
#[allow(dead_code)]
pub struct ObsAudioEncoder {
    pub(crate) encoder: WrappedObsEncoder,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,
    pub(crate) settings: Option<ObsData>,
    pub(crate) hotkey_data: Option<ObsData>,
}

impl ObsAudioEncoder {
    pub fn new(
        id: impl Into<ObsString>,
        name: impl Into<ObsString>,
        settings: Option<ObsData>,
        mixer_idx: usize,
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

        let encoder = unsafe {
            obs_audio_encoder_create(
                id.as_ptr(),
                name.as_ptr(),
                settings_ptr,
                mixer_idx,
                hotkey_data_ptr,
            )
        };

        if encoder == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        Ok(Self {
            encoder: WrappedObsEncoder(encoder),
            id,
            name,
            settings,
            hotkey_data,
        })
    }
}

impl Drop for ObsAudioEncoder {
    fn drop(&mut self) {
        unsafe { obs_encoder_release(self.encoder.0) }
    }
}
