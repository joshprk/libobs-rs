use std::{convert::Infallible, str::FromStr};

use crate::utils::ObsString;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum ObsVideoEncoderType {
    OBS_QSV11,
    OBS_QSV11_AV1,
    FFMPEG_NVENC,
    JIM_AV1_NVENC,
    H265_TEXTURE_AMF,
    FFMPEG_HEVC_NVENC,
    H264_TEXTURE_AMF,
    AV1_TEXTURE_AMF,
    OBS_X264,
    Other(String),
}

impl FromStr for ObsVideoEncoderType {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        return Ok(match value {
            "obs_qsv11" => ObsVideoEncoderType::OBS_QSV11,
            "obs_qsv11_av1" => ObsVideoEncoderType::OBS_QSV11_AV1,
            "ffmpeg_nvenc" => ObsVideoEncoderType::FFMPEG_NVENC,
            "jim_av1_nvenc" => ObsVideoEncoderType::JIM_AV1_NVENC,
            "h265_texture_amf" => ObsVideoEncoderType::H265_TEXTURE_AMF,
            "ffmpeg_hevc_nvenc" => ObsVideoEncoderType::FFMPEG_HEVC_NVENC,
            "h264_texture_amf" => ObsVideoEncoderType::H264_TEXTURE_AMF,
            "av1_texture_amf" => ObsVideoEncoderType::AV1_TEXTURE_AMF,
            "obs_x264" => ObsVideoEncoderType::OBS_X264,
            e => ObsVideoEncoderType::Other(e.to_string()),
        });
    }
}

impl Into<ObsString> for ObsVideoEncoderType {
    fn into(self) -> ObsString {
        return match self {
            ObsVideoEncoderType::OBS_QSV11 => ObsString::new("obs_qsv11"),
            ObsVideoEncoderType::OBS_QSV11_AV1 => ObsString::new("obs_qsv11_av1"),
            ObsVideoEncoderType::FFMPEG_NVENC => ObsString::new("ffmpeg_nvenc"),
            ObsVideoEncoderType::JIM_AV1_NVENC => ObsString::new("jim_av1_nvenc"),
            ObsVideoEncoderType::H265_TEXTURE_AMF => ObsString::new("h265_texture_amf"),
            ObsVideoEncoderType::FFMPEG_HEVC_NVENC => ObsString::new("ffmpeg_hevc_nvenc"),
            ObsVideoEncoderType::H264_TEXTURE_AMF => ObsString::new("h264_texture_amf"),
            ObsVideoEncoderType::AV1_TEXTURE_AMF => ObsString::new("av1_texture_amf"),
            ObsVideoEncoderType::OBS_X264 => ObsString::new("obs_x264"),
            ObsVideoEncoderType::Other(e) => ObsString::new(&e),
        };
    }
}

// from https://github.com/FFFFFFFXXXXXXX/libobs-recorder
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum ObsAudioEncoderType {
    JIM_AV1,
    JIM_NVENC,
    FFMPEG_NVENC,
    AMD_AMF_AV1,
    AMD_AMF_H264,
    OBS_QSV11_AV1,
    OBS_QSV11_H264,
    OBS_X264,
    Other(String),
}

impl FromStr for ObsAudioEncoderType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(match s {
            "jim_av1" => ObsAudioEncoderType::JIM_AV1,
            "jim_nvenc" => ObsAudioEncoderType::JIM_NVENC,
            "ffmpeg_nvenc" => ObsAudioEncoderType::FFMPEG_NVENC,
            "amd_amf_av1" => ObsAudioEncoderType::AMD_AMF_AV1,
            "amd_amf_h264" => ObsAudioEncoderType::AMD_AMF_H264,
            "obs_qsv11_av1" => ObsAudioEncoderType::OBS_QSV11_AV1,
            "obs_qsv11_h264" => ObsAudioEncoderType::OBS_QSV11_H264,
            "obs_x264" => ObsAudioEncoderType::OBS_X264,
            e => ObsAudioEncoderType::Other(e.to_string()),
        });
    }
}

impl Into<ObsString> for ObsAudioEncoderType {
    fn into(self) -> ObsString {
        return match self {
            ObsAudioEncoderType::JIM_AV1 => ObsString::new("jim_av1"),
            ObsAudioEncoderType::JIM_NVENC => ObsString::new("jim_nvenc"),
            ObsAudioEncoderType::FFMPEG_NVENC => ObsString::new("ffmpeg_nvenc"),
            ObsAudioEncoderType::AMD_AMF_AV1 => ObsString::new("amd_amf_av1"),
            ObsAudioEncoderType::AMD_AMF_H264 => ObsString::new("amd_amf_h264"),
            ObsAudioEncoderType::OBS_QSV11_AV1 => ObsString::new("obs_qsv11_av1"),
            ObsAudioEncoderType::OBS_QSV11_H264 => ObsString::new("obs_qsv11_h264"),
            ObsAudioEncoderType::OBS_X264 => ObsString::new("obs_x264"),
            ObsAudioEncoderType::Other(e) => ObsString::new(&e),
        };
    }
}
