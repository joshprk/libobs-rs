use core::fmt;
use std::fmt::Display;

use num_derive::{FromPrimitive, ToPrimitive};

#[cfg(target_family = "windows")]
pub(crate) type OsEnumType = i32;

#[cfg(not(target_family = "windows"))]
pub(crate) type OsEnumType = u32;

#[cfg_attr(target_family = "windows", repr(i32))]
#[cfg_attr(not(target_family = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the video output format used by the
/// OBS video context. Used in `ObsVideoInfo`.
pub enum ObsVideoFormat {
    AYUV = libobs::video_format_VIDEO_FORMAT_AYUV as OsEnumType,
    BGR3 = libobs::video_format_VIDEO_FORMAT_BGR3 as OsEnumType,
    BGRA = libobs::video_format_VIDEO_FORMAT_BGRA as OsEnumType,
    BGRX = libobs::video_format_VIDEO_FORMAT_BGRX as OsEnumType,
    I010 = libobs::video_format_VIDEO_FORMAT_I010 as OsEnumType,
    I210 = libobs::video_format_VIDEO_FORMAT_I210 as OsEnumType,
    I40A = libobs::video_format_VIDEO_FORMAT_I40A as OsEnumType,
    I412 = libobs::video_format_VIDEO_FORMAT_I412 as OsEnumType,
    I420 = libobs::video_format_VIDEO_FORMAT_I420 as OsEnumType,
    I422 = libobs::video_format_VIDEO_FORMAT_I422 as OsEnumType,
    I42A = libobs::video_format_VIDEO_FORMAT_I42A as OsEnumType,
    I444 = libobs::video_format_VIDEO_FORMAT_I444 as OsEnumType,
    NONE = libobs::video_format_VIDEO_FORMAT_NONE as OsEnumType,
    NV12 = libobs::video_format_VIDEO_FORMAT_NV12 as OsEnumType,
    P010 = libobs::video_format_VIDEO_FORMAT_P010 as OsEnumType,
    P216 = libobs::video_format_VIDEO_FORMAT_P216 as OsEnumType,
    P416 = libobs::video_format_VIDEO_FORMAT_P416 as OsEnumType,
    R10L = libobs::video_format_VIDEO_FORMAT_R10L as OsEnumType,
    RGBA = libobs::video_format_VIDEO_FORMAT_RGBA as OsEnumType,
    UYVY = libobs::video_format_VIDEO_FORMAT_UYVY as OsEnumType,
    V210 = libobs::video_format_VIDEO_FORMAT_V210 as OsEnumType,
    Y800 = libobs::video_format_VIDEO_FORMAT_Y800 as OsEnumType,
    YA2L = libobs::video_format_VIDEO_FORMAT_YA2L as OsEnumType,
    YUVA = libobs::video_format_VIDEO_FORMAT_YUVA as OsEnumType,
    YUY2 = libobs::video_format_VIDEO_FORMAT_YUY2 as OsEnumType,
    YVYU = libobs::video_format_VIDEO_FORMAT_YVYU as OsEnumType,
}

#[cfg_attr(target_family = "windows", repr(i32))]
#[cfg_attr(not(target_family = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the colorspace that an OBS video context
/// uses. Used in `ObsVideoInfo`.
pub enum ObsColorspace {
    CS2100HLG = libobs::video_colorspace_VIDEO_CS_2100_HLG as OsEnumType,
    CS2100PQ = libobs::video_colorspace_VIDEO_CS_2100_PQ as OsEnumType,
    CS601 = libobs::video_colorspace_VIDEO_CS_601 as OsEnumType,
    CS709 = libobs::video_colorspace_VIDEO_CS_709 as OsEnumType,
    Default = libobs::video_colorspace_VIDEO_CS_DEFAULT as OsEnumType,
    CSRGB = libobs::video_colorspace_VIDEO_CS_SRGB as OsEnumType,
}

#[cfg_attr(target_family = "windows", repr(i32))]
#[cfg_attr(not(target_family = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the minimum and maximum color levels that
/// an OBS video context is allowed to encode. Used in
/// `ObsVideoInfo.`
pub enum ObsVideoRange {
    Default = libobs::video_range_type_VIDEO_RANGE_DEFAULT as OsEnumType,
    Partial = libobs::video_range_type_VIDEO_RANGE_PARTIAL as OsEnumType,
    Full = libobs::video_range_type_VIDEO_RANGE_FULL as OsEnumType,
}

#[cfg_attr(target_family = "windows", repr(i32))]
#[cfg_attr(not(target_family = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes how libobs should reconcile non-matching
/// base and output resolutions when creating a video
/// context.
pub enum ObsScaleType {
    Area = libobs::obs_scale_type_OBS_SCALE_AREA as OsEnumType,
    Bicubic = libobs::obs_scale_type_OBS_SCALE_BICUBIC as OsEnumType,
    Bilinear = libobs::obs_scale_type_OBS_SCALE_BILINEAR as OsEnumType,
    Disable = libobs::obs_scale_type_OBS_SCALE_DISABLE as OsEnumType,
    Lanczos = libobs::obs_scale_type_OBS_SCALE_LANCZOS as OsEnumType,
    Point = libobs::obs_scale_type_OBS_SCALE_POINT as OsEnumType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Describes which graphics backend should be used
/// in the OBS video context. Used in `ObsVideoInfo`.
pub enum ObsGraphicsModule {
    OpenGL,
    DirectX11,
    Metal,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Status types returned after attempting to
/// reset the OBS video context using the
/// function `obs_reset_video`.
pub enum ObsResetVideoStatus {
    /// `obs_reset_video` was successful.
    Success = libobs::OBS_VIDEO_SUCCESS,
    /// The adapter is not supported as it
    /// lacks capabilities.
    NotSupported = libobs::OBS_VIDEO_NOT_SUPPORTED as u32,
    /// A parameter is invalid.
    InvalidParameter = libobs::OBS_VIDEO_INVALID_PARAM as u32,
    /// An output is currently running, preventing
    /// resetting the video context.
    CurrentlyActive = libobs::OBS_VIDEO_CURRENTLY_ACTIVE as u32,
    /// Generic error occured when attempting to
    /// reset the OBS video context.
    Failure = libobs::OBS_VIDEO_FAIL as u32,
}

/// Audio samples per second options that are
/// supported by libobs.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObsSamplesPerSecond {
    /// 44.1 kHz
    F44100 = 44100,
    /// 48.0 kHz
    F48000 = 48000,
}

#[cfg_attr(target_family = "windows", repr(i32))]
#[cfg_attr(not(target_family = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsSpeakerLayout {
    S2Point1 = libobs::speaker_layout_SPEAKERS_2POINT1 as OsEnumType,
    S4Point0 = libobs::speaker_layout_SPEAKERS_4POINT0 as OsEnumType,
    S4Point1 = libobs::speaker_layout_SPEAKERS_4POINT1 as OsEnumType,
    S5Point1 = libobs::speaker_layout_SPEAKERS_5POINT1 as OsEnumType,
    S7Point1 = libobs::speaker_layout_SPEAKERS_7POINT1 as OsEnumType,
    Mono = libobs::speaker_layout_SPEAKERS_MONO as OsEnumType,
    Stereo = libobs::speaker_layout_SPEAKERS_STEREO as OsEnumType,
    Unknown = libobs::speaker_layout_SPEAKERS_UNKNOWN as OsEnumType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObsOutputStopSignal {
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

impl fmt::Display for ObsOutputStopSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ObsOutputStopSignal::Success => "Success",
            ObsOutputStopSignal::BadPath => "Bad Path",
            ObsOutputStopSignal::ConnectFailed => "Connect Failed",
            ObsOutputStopSignal::InvalidStream => "Invalid Stream",
            ObsOutputStopSignal::Error => "Error",
            ObsOutputStopSignal::Disconnected => "Disconnected",
            ObsOutputStopSignal::Unsupported => "Unsupported",
            ObsOutputStopSignal::NoSpace => "No Space",
            ObsOutputStopSignal::EncodeError => "Encode Error",
        };
        write!(f, "{}", s)
    }
}

impl From<ObsOutputStopSignal> for i32 {
    fn from(val: ObsOutputStopSignal) -> Self {
        match val {
            ObsOutputStopSignal::Success => libobs::OBS_OUTPUT_SUCCESS as i32,
            ObsOutputStopSignal::BadPath => libobs::OBS_OUTPUT_BAD_PATH,
            ObsOutputStopSignal::ConnectFailed => libobs::OBS_OUTPUT_CONNECT_FAILED,
            ObsOutputStopSignal::InvalidStream => libobs::OBS_OUTPUT_INVALID_STREAM,
            ObsOutputStopSignal::Error => libobs::OBS_OUTPUT_ERROR,
            ObsOutputStopSignal::Disconnected => libobs::OBS_OUTPUT_DISCONNECTED,
            ObsOutputStopSignal::Unsupported => libobs::OBS_OUTPUT_UNSUPPORTED,
            ObsOutputStopSignal::NoSpace => libobs::OBS_OUTPUT_NO_SPACE,
            ObsOutputStopSignal::EncodeError => libobs::OBS_OUTPUT_ENCODE_ERROR,
        }
    }
}

impl TryFrom<i32> for ObsOutputStopSignal {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, <ObsOutputStopSignal as TryFrom<i32>>::Error> {
        match value {
            x if x == libobs::OBS_OUTPUT_SUCCESS as i32 => Ok(ObsOutputStopSignal::Success),
            x if x == libobs::OBS_OUTPUT_BAD_PATH => Ok(ObsOutputStopSignal::BadPath),
            x if x == libobs::OBS_OUTPUT_CONNECT_FAILED => Ok(ObsOutputStopSignal::ConnectFailed),
            x if x == libobs::OBS_OUTPUT_INVALID_STREAM => Ok(ObsOutputStopSignal::InvalidStream),
            x if x == libobs::OBS_OUTPUT_ERROR => Ok(ObsOutputStopSignal::Error),
            x if x == libobs::OBS_OUTPUT_DISCONNECTED => Ok(ObsOutputStopSignal::Disconnected),
            x if x == libobs::OBS_OUTPUT_UNSUPPORTED => Ok(ObsOutputStopSignal::Unsupported),
            x if x == libobs::OBS_OUTPUT_NO_SPACE => Ok(ObsOutputStopSignal::NoSpace),
            x if x == libobs::OBS_OUTPUT_ENCODE_ERROR => Ok(ObsOutputStopSignal::EncodeError),
            _ => Err("Invalid value"),
        }
    }
}

#[cfg_attr(target_family = "windows", repr(i32))]
#[cfg_attr(not(target_family = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsEncoderType {
    Video = libobs::obs_encoder_type_OBS_ENCODER_VIDEO as OsEnumType,
    Audio = libobs::obs_encoder_type_OBS_ENCODER_AUDIO as OsEnumType,
}

#[cfg_attr(target_family = "windows", repr(i32))]
#[cfg_attr(not(target_family = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsLogLevel {
    Error = libobs::LOG_ERROR as OsEnumType,
    Warning = libobs::LOG_WARNING as OsEnumType,
    Info = libobs::LOG_INFO as OsEnumType,
    Debug = libobs::LOG_DEBUG as OsEnumType,
}

impl Display for ObsLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "color-logger")]
impl ObsLogLevel {
    pub fn colorize(&self, s: &str) -> String {
        use colored::Colorize;

        match self {
            ObsLogLevel::Error => s.on_red().to_string(),
            ObsLogLevel::Warning => s.yellow().to_string(),
            ObsLogLevel::Info => s.green().bold().to_string(),
            ObsLogLevel::Debug => s.blue().to_string(),
        }
    }
}
