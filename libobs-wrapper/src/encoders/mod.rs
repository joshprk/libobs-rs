use std::{ffi::CStr, os::raw::c_char, str::FromStr};

use num_traits::ToPrimitive;

use crate::{
    context::ObsContext,
    enums::ObsEncoderType,
    run_with_obs,
    runtime::ObsRuntime,
    utils::{ObsError, ENCODER_HIDE_FLAGS},
};

pub mod audio;
mod enums;
mod property_helper;
pub use property_helper::*;
pub mod video;
pub use enums::*;

pub trait ObsContextEncoders {
    #[deprecated(note = "Use `best_video_encoder` instead.")]
    fn get_best_video_encoder(&self) -> Result<ObsVideoEncoderType, ObsError>;

    fn best_video_encoder(&self) -> Result<ObsVideoEncoderBuilder, ObsError>;

    #[deprecated(note = "Use `best_audio_encoder` instead.")]
    fn get_best_audio_encoder(&self) -> Result<ObsAudioEncoderType, ObsError>;

    fn best_audio_encoder(&self) -> Result<ObsAudioEncoderBuilder, ObsError>;

    #[deprecated(note = "Use `available_audio_encoders` instead.")]
    fn get_available_audio_encoders(&self) -> Result<Vec<ObsAudioEncoderType>, ObsError>;

    fn available_audio_encoders(&self) -> Result<Vec<ObsAudioEncoderBuilder>, ObsError>;

    #[deprecated(note = "Use `available_video_encoders` instead.")]
    fn get_available_video_encoders(&self) -> Result<Vec<ObsVideoEncoderType>, ObsError>;

    fn available_video_encoders(&self) -> Result<Vec<ObsVideoEncoderBuilder>, ObsError>;
}

fn get_encoders_raw(
    encoder_type: ObsEncoderType,
    runtime: &ObsRuntime,
) -> Result<Vec<String>, ObsError> {
    let type_primitive = encoder_type.to_i32().unwrap();

    run_with_obs!(runtime, move || {
        let mut n = 0;
        let mut encoders = Vec::new();

        let mut ptr: *const c_char = unsafe { std::mem::zeroed() };
        while unsafe { libobs::obs_enum_encoder_types(n, &mut ptr) } {
            n += 1;
            let cstring = unsafe { CStr::from_ptr(ptr) };
            if let Ok(enc) = cstring.to_str() {
                unsafe {
                    let is_hidden = libobs::obs_get_encoder_caps(ptr) & ENCODER_HIDE_FLAGS != 0;
                    if is_hidden || libobs::obs_get_encoder_type(ptr) != type_primitive {
                        continue;
                    }
                }

                log::debug!("Found encoder: {}", enc);
                encoders.push(enc.into());
            }
        }

        encoders.sort_unstable();
        encoders
    })
}

impl ObsContextEncoders for ObsContext {
    fn best_video_encoder(&self) -> Result<ObsVideoEncoderBuilder, ObsError> {
        let encoders = self.available_video_encoders()?;
        encoders
            .into_iter()
            .next()
            .ok_or(ObsError::NoAvailableEncoders)
    }

    fn best_audio_encoder(&self) -> Result<ObsAudioEncoderBuilder, ObsError> {
        let encoders = self.available_audio_encoders()?;
        encoders
            .into_iter()
            .next()
            .ok_or(ObsError::NoAvailableEncoders)
    }

    fn available_audio_encoders(&self) -> Result<Vec<ObsAudioEncoderBuilder>, ObsError> {
        Ok(get_encoders_raw(ObsEncoderType::Audio, &self.runtime)?
            .into_iter()
            .map(|x| ObsAudioEncoderBuilder::new(self.clone(), &x))
            .collect::<Vec<_>>())
    }

    fn available_video_encoders(&self) -> Result<Vec<ObsVideoEncoderBuilder>, ObsError> {
        Ok(get_encoders_raw(ObsEncoderType::Video, &self.runtime)?
            .into_iter()
            .map(|x| ObsVideoEncoderBuilder::new(self.clone(), &x))
            .collect::<Vec<_>>())
    }

    fn get_best_video_encoder(&self) -> Result<ObsVideoEncoderType, ObsError> {
        #[allow(deprecated)]
        Ok(self
            .get_available_video_encoders()?
            .first()
            .unwrap()
            .clone())
    }

    fn get_best_audio_encoder(&self) -> Result<ObsAudioEncoderType, ObsError> {
        #[allow(deprecated)]
        Ok(self
            .get_available_audio_encoders()?
            .first()
            .unwrap()
            .clone())
    }

    fn get_available_audio_encoders(&self) -> Result<Vec<ObsAudioEncoderType>, ObsError> {
        Ok(get_encoders_raw(ObsEncoderType::Audio, &self.runtime)?
            .into_iter()
            .map(|x| ObsAudioEncoderType::from_str(&x).unwrap())
            .collect::<Vec<_>>())
    }

    fn get_available_video_encoders(&self) -> Result<Vec<ObsVideoEncoderType>, ObsError> {
        Ok(get_encoders_raw(ObsEncoderType::Video, &self.runtime)?
            .into_iter()
            .map(|x| ObsVideoEncoderType::from_str(&x).unwrap())
            .collect::<Vec<_>>())
    }
}
