use libobs::{obs_encoder, obs_encoder_release, obs_video_encoder_create};
use std::{borrow::Borrow, ptr};

use crate::{data::ObsData, utils::{ObsError, ObsString}};


#[derive(Debug)]
pub struct ObsVideoEncoder {
    pub(crate) encoder: *mut obs_encoder,
    pub(crate) id: ObsString,
    pub(crate) name: ObsString,
    pub(crate) settings: Option<ObsData>,
    pub(crate) hotkey_data: Option<ObsData>,
}

impl ObsVideoEncoder {
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

        let encoder = unsafe {
            obs_video_encoder_create(
                id.as_ptr(),
                name.as_ptr(),
                settings_ptr,
                hotkey_data_ptr,
            )
        };

        if encoder == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        Ok(Self {
            encoder,
            id,
            name,
            settings,
            hotkey_data,
        })
    }

    pub fn as_ptr(&mut self) -> *mut obs_encoder {
        self.encoder
    }
}

impl Drop for ObsVideoEncoder {
    fn drop(&mut self) {
        unsafe { obs_encoder_release(self.encoder) }
    }
}
