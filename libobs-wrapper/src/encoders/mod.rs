use std::{ffi::CStr, os::raw::c_char};

use num_traits::ToPrimitive;

use crate::{
    context::ObsContext,
    enums::ObsEncoderType,
    utils::ENCODER_HIDE_FLAGS,
};

pub mod audio;
pub mod video;
mod enums;
pub use enums::*;




pub trait ObsContextEncoders {
    fn get_best_video_encoder() -> ObsVideoEncoderType;
    fn get_best_audio_encoder() -> ObsAudioEncoderType;
    fn get_available_audio_encoders() -> Vec<ObsAudioEncoderType>;
    fn get_available_video_encoders() -> Vec<ObsVideoEncoderType>;
}

fn get_encoders_raw(encoder_type: ObsEncoderType) -> Vec<String> {
    let type_primitive = encoder_type.to_i32().unwrap();

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

            println!("Found encoder: {}", enc);
            encoders.push(enc.into());
        }
    }
    encoders.sort_unstable();
    encoders
}

impl ObsContextEncoders for ObsContext {
    fn get_best_video_encoder() -> ObsVideoEncoderType {
        Self::get_available_video_encoders()
            .first()
            .unwrap()
            .clone()
    }

    fn get_best_audio_encoder() -> ObsAudioEncoderType {
        Self::get_available_audio_encoders()
            .first()
            .unwrap()
            .clone()
    }

    fn get_available_audio_encoders() -> Vec<ObsAudioEncoderType> {
        get_encoders_raw(ObsEncoderType::Audio)
            .into_iter()
            .map(|x| x.as_str().into())
            .collect()
    }

    fn get_available_video_encoders() -> Vec<ObsVideoEncoderType> {
        get_encoders_raw(ObsEncoderType::Video)
            .into_iter()
            .map(|x| x.as_str().into())
            .collect()
    }
}