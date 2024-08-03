use std::borrow::Borrow;
use std::ffi::{c_char, CStr, CString};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{env, ptr};

use display_info::DisplayInfo;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::{
    audio_output, obs_audio_info, obs_audio_info2, obs_data, obs_encoder, obs_output, obs_source,
    obs_video_info, video_output,
};

use super::{AudioEncoderInfo, SourceInfo, VideoEncoderInfo};



extern "C" fn signal_handler(data: crate::calldata_t) {

}