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
pub mod video;
pub use enums::*;

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
pub trait ObsContextEncoders {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_best_video_encoder(&self) -> Result<ObsVideoEncoderType, ObsError>;

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_best_audio_encoder(&self) -> Result<ObsAudioEncoderType, ObsError>;

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_available_audio_encoders(&self) -> Result<Vec<ObsAudioEncoderType>, ObsError>;

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_available_video_encoders(&self) -> Result<Vec<ObsVideoEncoderType>, ObsError>;
}

#[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
async fn get_encoders_raw(
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
    .await
}

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl ObsContextEncoders for ObsContext {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_best_video_encoder(&self) -> Result<ObsVideoEncoderType, ObsError> {
        Ok(self
            .get_available_video_encoders()
            .await?
            .first()
            .unwrap()
            .clone())
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_best_audio_encoder(&self) -> Result<ObsAudioEncoderType, ObsError> {
        Ok(self
            .get_available_audio_encoders()
            .await?
            .first()
            .unwrap()
            .clone())
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_available_audio_encoders(&self) -> Result<Vec<ObsAudioEncoderType>, ObsError> {
        Ok(get_encoders_raw(ObsEncoderType::Audio, &self.runtime)
            .await?
            .into_iter()
            .map(|x| ObsAudioEncoderType::from_str(&x).unwrap())
            .collect::<Vec<_>>())
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn get_available_video_encoders(&self) -> Result<Vec<ObsVideoEncoderType>, ObsError> {
        Ok(get_encoders_raw(ObsEncoderType::Video, &self.runtime)
            .await?
            .into_iter()
            .map(|x| ObsVideoEncoderType::from_str(&x).unwrap())
            .collect::<Vec<_>>())
    }
}
