use core::fmt;

use num_derive::{FromPrimitive, ToPrimitive};

use super::utils::ObsString;

#[cfg(target_os = "windows")]
pub(crate) type OsEnumType = i32;
#[cfg(not(target_os = "windows"))]
pub(crate) type OsEnumType = u32;

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the video output format used by the
/// OBS video context. Used in `ObsVideoInfo`.
pub enum ObsVideoFormat {
    AYUV = libobs::video_format_VIDEO_FORMAT_AYUV,
    BGR3 = libobs::video_format_VIDEO_FORMAT_BGR3,
    BGRA = libobs::video_format_VIDEO_FORMAT_BGRA,
    BGRX = libobs::video_format_VIDEO_FORMAT_BGRX,
    I010 = libobs::video_format_VIDEO_FORMAT_I010,
    I210 = libobs::video_format_VIDEO_FORMAT_I210,
    I40A = libobs::video_format_VIDEO_FORMAT_I40A,
    I412 = libobs::video_format_VIDEO_FORMAT_I412,
    I420 = libobs::video_format_VIDEO_FORMAT_I420,
    I422 = libobs::video_format_VIDEO_FORMAT_I422,
    I42A = libobs::video_format_VIDEO_FORMAT_I42A,
    I444 = libobs::video_format_VIDEO_FORMAT_I444,
    NONE = libobs::video_format_VIDEO_FORMAT_NONE,
    NV12 = libobs::video_format_VIDEO_FORMAT_NV12,
    P010 = libobs::video_format_VIDEO_FORMAT_P010,
    P216 = libobs::video_format_VIDEO_FORMAT_P216,
    P416 = libobs::video_format_VIDEO_FORMAT_P416,
    R10L = libobs::video_format_VIDEO_FORMAT_R10L,
    RGBA = libobs::video_format_VIDEO_FORMAT_RGBA,
    UYVY = libobs::video_format_VIDEO_FORMAT_UYVY,
    V210 = libobs::video_format_VIDEO_FORMAT_V210,
    Y800 = libobs::video_format_VIDEO_FORMAT_Y800,
    YA2L = libobs::video_format_VIDEO_FORMAT_YA2L,
    YUVA = libobs::video_format_VIDEO_FORMAT_YUVA,
    YUY2 = libobs::video_format_VIDEO_FORMAT_YUY2,
    YVYU = libobs::video_format_VIDEO_FORMAT_YVYU,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the colorspace that an OBS video context
/// uses. Used in `ObsVideoInfo`.
pub enum ObsColorspace {
    CS2100HLG = libobs::video_colorspace_VIDEO_CS_2100_HLG,
    CS2100PQ = libobs::video_colorspace_VIDEO_CS_2100_PQ,
    CS601 = libobs::video_colorspace_VIDEO_CS_601,
    CS709 = libobs::video_colorspace_VIDEO_CS_709,
    Default = libobs::video_colorspace_VIDEO_CS_DEFAULT,
    CSRGB = libobs::video_colorspace_VIDEO_CS_SRGB,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the minimum and maximum color levels that
/// an OBS video context is allowed to encode. Used in
/// `ObsVideoInfo.`
pub enum ObsVideoRange {
    Default = libobs::video_range_type_VIDEO_RANGE_DEFAULT,
    Partial = libobs::video_range_type_VIDEO_RANGE_PARTIAL,
    Full = libobs::video_range_type_VIDEO_RANGE_FULL,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes how libobs should reconcile non-matching
/// base and output resolutions when creating a video
/// context.
pub enum ObsScaleType {
    Area = libobs::obs_scale_type_OBS_SCALE_AREA,
    Bicubic = libobs::obs_scale_type_OBS_SCALE_BICUBIC,
    Bilinear = libobs::obs_scale_type_OBS_SCALE_BILINEAR,
    Disable = libobs::obs_scale_type_OBS_SCALE_DISABLE,
    Lanczos = libobs::obs_scale_type_OBS_SCALE_LANCZOS,
    Point = libobs::obs_scale_type_OBS_SCALE_POINT,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Describes which graphics backend should be used
/// in the OBS video context. Used in `ObsVideoInfo`.
pub enum ObsGraphicsModule {
    OpenGL,
    DirectX11,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Status types returned after attempting to
/// reset the OBS video context using the
/// function `obs_reset_video`.
pub enum ObsResetVideoStatus {
    /// `obs_reset_video` was successful.
    Success = libobs::OBS_VIDEO_SUCCESS as i32,
    /// The adapter is not supported as it
    /// lacks capabilities.
    NotSupported = libobs::OBS_VIDEO_NOT_SUPPORTED,
    /// A parameter is invalid.
    InvalidParameter = libobs::OBS_VIDEO_INVALID_PARAM,
    /// An output is currently running, preventing
    /// resetting the video context.
    CurrentlyActive = libobs::OBS_VIDEO_CURRENTLY_ACTIVE,
    /// Generic error occured when attempting to
    /// reset the OBS video context.
    Failure = libobs::OBS_VIDEO_FAIL,
}

/// Audio samples per second options that are
/// supported by libobs.
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObsSamplesPerSecond {
    /// 44.1 kHz
    F44100 = 44100,
    /// 48.0 kHz
    F48000 = 48000,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsSpeakerLayout {
    S2Point1 = libobs::speaker_layout_SPEAKERS_2POINT1,
    S4Point0 = libobs::speaker_layout_SPEAKERS_4POINT0,
    S4Point1 = libobs::speaker_layout_SPEAKERS_4POINT1,
    S5Point1 = libobs::speaker_layout_SPEAKERS_5POINT1,
    S7Point1 = libobs::speaker_layout_SPEAKERS_7POINT1,
    Mono = libobs::speaker_layout_SPEAKERS_MONO,
    Stereo = libobs::speaker_layout_SPEAKERS_STEREO,
    Unknown = libobs::speaker_layout_SPEAKERS_UNKNOWN,
}

// from https://github.com/FFFFFFFXXXXXXX/libobs-recorder
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum ObsVideoEncoderType {
    JIM_AV1,
    JIM_NVENC,
    FFMPEG_NVENC,
    AMD_AMF_AV1,
    AMD_AMF_H264,
    OBS_QSV11_AV1,
    OBS_QSV11_H264,
    OBS_X264,
}

impl From<&str> for ObsVideoEncoderType {
    fn from(value: &str) -> ObsVideoEncoderType {
        return match value {
            "jim_av1" => ObsVideoEncoderType::JIM_AV1,
            "jim_nvenc" => ObsVideoEncoderType::JIM_NVENC,
            "ffmpeg_nvenc" => ObsVideoEncoderType::FFMPEG_NVENC,
            "amd_amf_av1" => ObsVideoEncoderType::AMD_AMF_AV1,
            "amd_amf_h264" => ObsVideoEncoderType::AMD_AMF_H264,
            "obs_qsv11_av1" => ObsVideoEncoderType::OBS_QSV11_AV1,
            "obs_qsv11_h264" => ObsVideoEncoderType::OBS_QSV11_H264,
            "obs_x264" => ObsVideoEncoderType::OBS_X264,
            _ => ObsVideoEncoderType::OBS_X264,
        };
    }
}

impl Into<ObsString> for ObsVideoEncoderType {
    fn into(self) -> ObsString {
        return match self {
            ObsVideoEncoderType::JIM_AV1 => ObsString::new("jim_av1"),
            ObsVideoEncoderType::JIM_NVENC => ObsString::new("jim_nvenc"),
            ObsVideoEncoderType::FFMPEG_NVENC => ObsString::new("ffmpeg_nvenc"),
            ObsVideoEncoderType::AMD_AMF_AV1 => ObsString::new("amd_amf_av1"),
            ObsVideoEncoderType::AMD_AMF_H264 => ObsString::new("amd_amf_h264"),
            ObsVideoEncoderType::OBS_QSV11_AV1 => ObsString::new("obs_qsv11_av1"),
            ObsVideoEncoderType::OBS_QSV11_H264 => ObsString::new("obs_qsv11_h264"),
            ObsVideoEncoderType::OBS_X264 => ObsString::new("obs_x264")
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObsOutputSignal {
    /// Successfully stopped
    Success,
    /// The specified path was invalid
    BadPath,
    /// Failed to connect to a server
    ConnectFailed,
    /// Invalid stream path
    InvalidStream,
    /// Generic error
    Error,
    /// Unexpectedly disconnected
    Disconnected,
    /// The settings, video/audio format, or codecs are unsupported by this output
    Unsupported,
    /// Ran out of disk space
    NoSpace,
    /// Encoder error
    EncodeError,
}

impl fmt::Display for ObsOutputSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ObsOutputSignal::Success => "Success",
            ObsOutputSignal::BadPath => "Bad Path",
            ObsOutputSignal::ConnectFailed => "Connect Failed",
            ObsOutputSignal::InvalidStream => "Invalid Stream",
            ObsOutputSignal::Error => "Error",
            ObsOutputSignal::Disconnected => "Disconnected",
            ObsOutputSignal::Unsupported => "Unsupported",
            ObsOutputSignal::NoSpace => "No Space",
            ObsOutputSignal::EncodeError => "Encode Error",
        };
        write!(f, "{}", s)
    }
}

impl Into<i32> for ObsOutputSignal {
    fn into(self) -> i32 {
        match self {
            ObsOutputSignal::Success => libobs::OBS_OUTPUT_SUCCESS as i32,
            ObsOutputSignal::BadPath => libobs::OBS_OUTPUT_BAD_PATH,
            ObsOutputSignal::ConnectFailed => libobs::OBS_OUTPUT_CONNECT_FAILED,
            ObsOutputSignal::InvalidStream => libobs::OBS_OUTPUT_INVALID_STREAM,
            ObsOutputSignal::Error => libobs::OBS_OUTPUT_ERROR,
            ObsOutputSignal::Disconnected => libobs::OBS_OUTPUT_DISCONNECTED,
            ObsOutputSignal::Unsupported => libobs::OBS_OUTPUT_UNSUPPORTED,
            ObsOutputSignal::NoSpace => libobs::OBS_OUTPUT_NO_SPACE,
            ObsOutputSignal::EncodeError => libobs::OBS_OUTPUT_ENCODE_ERROR,
        }
    }
}

impl TryFrom<i32> for ObsOutputSignal {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, <ObsOutputSignal as TryFrom<i32>>::Error> {
        match value {
            x if x == libobs::OBS_OUTPUT_SUCCESS as i32 => Ok(ObsOutputSignal::Success),
            x if x == libobs::OBS_OUTPUT_BAD_PATH => Ok(ObsOutputSignal::BadPath),
            x if x == libobs::OBS_OUTPUT_CONNECT_FAILED => Ok(ObsOutputSignal::ConnectFailed),
            x if x == libobs::OBS_OUTPUT_INVALID_STREAM => Ok(ObsOutputSignal::InvalidStream),
            x if x == libobs::OBS_OUTPUT_ERROR => Ok(ObsOutputSignal::Error),
            x if x == libobs::OBS_OUTPUT_DISCONNECTED => Ok(ObsOutputSignal::Disconnected),
            x if x == libobs::OBS_OUTPUT_UNSUPPORTED => Ok(ObsOutputSignal::Unsupported),
            x if x == libobs::OBS_OUTPUT_NO_SPACE => Ok(ObsOutputSignal::NoSpace),
            x if x == libobs::OBS_OUTPUT_ENCODE_ERROR => Ok(ObsOutputSignal::EncodeError),
            _ => Err("Invalid value"),
        }
    }
}

/*


#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsOutputSignals {
    /// Successfully stopped
    Success = libobs::OBS_OUTPUT_SUCCESS,
    /// The specified path was invalid
    BadPath = libobs::OBS_OUTPUT_BAD_PATH,
    /// Failed to connect to a server
    ConnectFailed = libobs::OBS_OUTPUT_CONNECT_FAILED,
    /// Invalid stream path
    InvalidStream = libobs::OBS_OUTPUT_INVALID_STREAM,
    /// Generic error
    Error = libobs::OBS_OUTPUT_ERROR,
    /// Unexpectedly disconnected
    Disconnected = libobs::OBS_OUTPUT_DISCONNECTED,
    /// The settings, video/audio format, or codecs are unsupported by this output
    Unsupported = libobs::OBS_OUTPUT_UNSUPPORTED,
    /// Ran out of disk space
    NoSpace = libobs::OBS_OUTPUT_NO_SPACE,
    /// Encoder error
    EncodeError = libobs::OBS_OUTPUT_ENCODE_ERROR,
}
*/