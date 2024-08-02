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

#[cfg(target_os = "windows")]
type OsEnumType = i32;
#[cfg(not(target_os = "windows"))]
type OsEnumType = u32;

/// Error type for OBS function calls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObsError {
    /// The `obs_startup` function failed on libobs.
    Failure,
    /// Failed to lock mutex describing whether there is a
    /// thread using libobs or not. Report to crate maintainer.
    MutexFailure,
    /// Some or no thread is already using libobs. This is a bug!
    ThreadFailure,
    /// Unable to reset video.
    ResetVideoFailure(ObsResetVideoStatus),
    /// Unable to reset video because the program attempted to
    /// change the graphics module. This is a bug!
    ResetVideoFailureGraphicsModule,
    /// The function returned a null pointer, often indicating
    /// an error with creating the object of the requested
    /// pointer.
    NullPointer,
    OutputAlreadyActive,
    OutputStartFailure(Option<String>),
    OutputStopFailure(Option<String>),
}

impl Display for ObsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OBS Error: ")?;

        match self {
            ObsError::Failure => write!(f, "`obs-startup` function failed on libobs"),
            ObsError::MutexFailure => write!(f, "Failed to lock mutex describing whether there is a thread using libobs or not. Report to crate maintainer."),
            ObsError::ThreadFailure => write!(f, "Some or no thread is already using libobs. This is a bug!"),
            ObsError::ResetVideoFailure(status) => write!(f, "Could not reset obs video. Status: {:?}", status),
            ObsError::ResetVideoFailureGraphicsModule => write!(f, "Unable to reset video because the program attempted to change the graphics module. This is a bug!"),
            ObsError::NullPointer => write!(f, "The function returned a null pointer, often indicating an error with creating the object of the requested pointer."),
            ObsError::OutputAlreadyActive => write!(f, "Output is already active."),
            ObsError::OutputStartFailure(s) => write!(f, "Output failed to start. Error is {:?}", s),
            ObsError::OutputStopFailure(s) => write!(f, "Output failed to stop. Error is {:?}", s),
        }
    }
}

impl std::error::Error for ObsError {}

/// String wrapper for OBS function calls.
///
/// This struct wraps `CString` internally with included helper
/// functions. Note that any NUL byte is stripped before
/// conversion to a `CString` to prevent panicking.
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObsString {
    c_string: CString,
}

impl ObsString {
    /// Creates a new `ObsString` wrapper for C-type
    /// strings used by libobs. Note that all NUL
    /// bytes are removed before conversion to a
    /// `ObsString` as C-type strings do not allow
    /// premature NUL bytes.
    ///
    /// These are CString wrappers internally, with
    /// included helper functions to reduce repetitive
    /// code and ensure safety.
    pub fn new(value: &str) -> Self {
        Self::from(value)
    }

    /// Returns a safe pointer to a C-type string
    /// used by libobs. This pointer will be valid
    /// for as long as this ObsString exists.
    ///
    /// Note that this pointer is read-only--writing
    /// to it is undefined behavior.
    pub fn as_ptr(&self) -> *const c_char {
        self.c_string.as_ptr()
    }
}

impl ToString for ObsString {
    fn to_string(&self) -> String {
        // We can use the lossy method here since the c_string is guaranteed to be UTF-8.
        self.c_string.to_string_lossy().to_string()
    }
}

impl From<&str> for ObsString {
    fn from(value: &str) -> Self {
        let value = value.replace("\0", "");
        Self {
            c_string: CString::new(value).unwrap(),
        }
    }
}

impl From<Vec<u8>> for ObsString {
    fn from(value: Vec<u8>) -> Self {
        let mut value = value
            .into_iter()
            .filter(|x| *x != b'\0')
            .collect::<Vec<u8>>();

        value.push(b'\0');

        Self {
            c_string: CString::from_vec_with_nul(value).unwrap(),
        }
    }
}

/// Contains `obs_data` and its related strings. Note that
/// this struct prevents string pointers from being freed
/// by keeping them owned.
#[derive(Debug)]
pub struct ObsData {
    obs_data: *mut obs_data,
    strings: Vec<ObsString>,
}

impl ObsData {
    /// Creates a new empty `ObsData` wrapper for the
    /// libobs `obs_data` data structure.
    ///
    /// `ObsData` can then be populated using the set
    /// functions, which take ownership of the
    /// `ObsString` types to prevent them from being
    /// dropped prematurely. This makes it safer than
    /// using `obs_data` directly from libobs.
    pub fn new() -> Self {
        let obs_data = unsafe { crate::obs_data_create() };
        let strings = Vec::new();
        ObsData { obs_data, strings }
    }

    /// Returns a pointer to the raw `obs_data`
    /// represented by `ObsData`.
    pub fn as_ptr(&self) -> *mut obs_data {
        self.obs_data
    }

    /// Sets a string in `obs_data` and stores it so
    /// it in `ObsData` does not get freed.
    pub fn set_string(
        &mut self,
        key: impl Into<ObsString>,
        value: impl Into<ObsString>,
    ) -> &mut Self {
        let key = key.into();
        let value = value.into();

        unsafe { crate::obs_data_set_string(self.obs_data, key.as_ptr(), value.as_ptr()) }

        self.strings.push(key);
        self.strings.push(value);

        self
    }

    /// Sets an int in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub fn set_int(&mut self, key: impl Into<ObsString>, value: i64) -> &mut Self {
        let key = key.into();

        unsafe { crate::obs_data_set_int(self.obs_data, key.as_ptr(), value.into()) }

        self.strings.push(key);

        self
    }

    /// Sets a bool in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub fn set_bool(&mut self, key: impl Into<ObsString>, value: bool) -> &mut Self {
        let key = key.into();

        unsafe { crate::obs_data_set_bool(self.obs_data, key.as_ptr(), value) }

        self.strings.push(key);

        self
    }

    /// Sets a double in `obs_data` and stores the key
    /// in `ObsData` so it does not get freed.
    pub fn set_double(&mut self, key: impl Into<ObsString>, value: f64) -> &mut Self {
        let key = key.into();

        unsafe { crate::obs_data_set_double(self.obs_data, key.as_ptr(), value) }

        self.strings.push(key);

        self
    }
}

impl Drop for ObsData {
    fn drop(&mut self) {
        unsafe { crate::obs_data_release(self.obs_data) }
    }
}

/// Builds into an `ObsString` that represents a path used
/// by libobs.
///
/// Note that only this path only supports UTF-8 for the
/// entire absolute path because libobs only supports
/// UTF-8.
#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObsPath {
    path: PathBuf,
}

impl ObsPath {
    /// Creates a new `ObsPath` strictly using the path
    /// `path_str` without any modifications.
    ///
    /// If you want to create a relative path, use
    /// `ObsPath::from_relative`.
    pub fn new(path_str: &str) -> Self {
        Self {
            path: Path::new(path_str).into(),
        }
    }

    /// Creates a new `ObsPath` with `path_str`
    /// appended to the path of the directory which the
    /// executable file is in.
    ///
    /// If you want to create an absolute path, use
    /// `ObsPath::new`.
    pub fn from_relative(path_str: &str) -> Self {
        let mut relative_path = env::current_exe().unwrap();

        relative_path.pop();

        let obs_path = Self {
            path: relative_path,
        };

        let path_str = path_str.trim_matches('/');

        obs_path.push(path_str)
    }

    /// Modifies the path to point to the path
    /// `path_str` appended to the current path which
    /// `ObsPath` is pointing to.
    pub fn push(mut self, value: &str) -> Self {
        let split = value.split(['/', '\\'].as_ref());

        for item in split {
            if item.len() > 0 {
                self.path.push(item);
            }
        }

        self
    }

    /// Modifies the path to point to its current
    /// parent. This is analogous to `Obs::push(".")`.
    pub fn pop(mut self) -> Self {
        self.path.pop();
        self
    }

    /// Consumes the `ObsPath` to create a new
    /// immutable ObsString that encodes a UTF-8
    /// C-type string which describes the path that
    /// the `ObsPath` is pointing to.
    ///
    /// Note that this function is lossy in that
    /// any non-Unicode data is completely removed
    /// from the string. This is because libobs
    /// does not support non-Unicode characters in
    /// its path.
    pub fn build(self) -> ObsString {
        let mut bytes = self.path.display().to_string().replace("\\", "/");

        if self.path.is_dir() {
            bytes = bytes + "/";
        }
        let obs_string = ObsString::from(bytes.as_str());

        drop(self);
        obs_string
    }
}

impl Into<ObsString> for ObsPath {
    fn into(self) -> ObsString {
        self.build()
    }
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the video output format used by the
/// OBS video context. Used in `ObsVideoInfo`.
pub enum ObsVideoFormat {
    AYUV = crate::video_format_VIDEO_FORMAT_AYUV,
    BGR3 = crate::video_format_VIDEO_FORMAT_BGR3,
    BGRA = crate::video_format_VIDEO_FORMAT_BGRA,
    BGRX = crate::video_format_VIDEO_FORMAT_BGRX,
    I010 = crate::video_format_VIDEO_FORMAT_I010,
    I210 = crate::video_format_VIDEO_FORMAT_I210,
    I40A = crate::video_format_VIDEO_FORMAT_I40A,
    I412 = crate::video_format_VIDEO_FORMAT_I412,
    I420 = crate::video_format_VIDEO_FORMAT_I420,
    I422 = crate::video_format_VIDEO_FORMAT_I422,
    I42A = crate::video_format_VIDEO_FORMAT_I42A,
    I444 = crate::video_format_VIDEO_FORMAT_I444,
    NONE = crate::video_format_VIDEO_FORMAT_NONE,
    NV12 = crate::video_format_VIDEO_FORMAT_NV12,
    P010 = crate::video_format_VIDEO_FORMAT_P010,
    P216 = crate::video_format_VIDEO_FORMAT_P216,
    P416 = crate::video_format_VIDEO_FORMAT_P416,
    R10L = crate::video_format_VIDEO_FORMAT_R10L,
    RGBA = crate::video_format_VIDEO_FORMAT_RGBA,
    UYVY = crate::video_format_VIDEO_FORMAT_UYVY,
    V210 = crate::video_format_VIDEO_FORMAT_V210,
    Y800 = crate::video_format_VIDEO_FORMAT_Y800,
    YA2L = crate::video_format_VIDEO_FORMAT_YA2L,
    YUVA = crate::video_format_VIDEO_FORMAT_YUVA,
    YUY2 = crate::video_format_VIDEO_FORMAT_YUY2,
    YVYU = crate::video_format_VIDEO_FORMAT_YVYU,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the colorspace that an OBS video context
/// uses. Used in `ObsVideoInfo`.
pub enum ObsColorspace {
    CS2100HLG = crate::video_colorspace_VIDEO_CS_2100_HLG,
    CS2100PQ = crate::video_colorspace_VIDEO_CS_2100_PQ,
    CS601 = crate::video_colorspace_VIDEO_CS_601,
    CS709 = crate::video_colorspace_VIDEO_CS_709,
    Default = crate::video_colorspace_VIDEO_CS_DEFAULT,
    CSRGB = crate::video_colorspace_VIDEO_CS_SRGB,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the minimum and maximum color levels that
/// an OBS video context is allowed to encode. Used in
/// `ObsVideoInfo.`
pub enum ObsVideoRange {
    Default = crate::video_range_type_VIDEO_RANGE_DEFAULT,
    Partial = crate::video_range_type_VIDEO_RANGE_PARTIAL,
    Full = crate::video_range_type_VIDEO_RANGE_FULL,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes how libobs should reconcile non-matching
/// base and output resolutions when creating a video
/// context.
pub enum ObsScaleType {
    Area = crate::obs_scale_type_OBS_SCALE_AREA,
    Bicubic = crate::obs_scale_type_OBS_SCALE_BICUBIC,
    Bilinear = crate::obs_scale_type_OBS_SCALE_BILINEAR,
    Disable = crate::obs_scale_type_OBS_SCALE_DISABLE,
    Lanczos = crate::obs_scale_type_OBS_SCALE_LANCZOS,
    Point = crate::obs_scale_type_OBS_SCALE_POINT,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Describes which graphics backend should be used
/// in the OBS video context. Used in `ObsVideoInfo`.
pub enum ObsGraphicsModule {
    OpenGL,
    DirectX11,
}

/// A wrapper for `obs_video_info`, which is used
/// to pass information to libobs for the new OBS
/// video context after resetting the old OBS
/// video context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObsVideoInfo {
    ovi: obs_video_info,
    // False positive. This is necessary to ensure
    // that the graphics module string in the
    // `obs_video_info` struct does not free.
    #[allow(dead_code)]
    graphics_module: ObsString,
}

impl ObsVideoInfo {
    /// Creates a new `ObsVideoInfo`.
    ///
    /// Note that this function is not meant to
    /// be used externally. The recommended,
    /// supported way to build new `ObsVideoInfo`
    /// structs is through `ObsVideoInfoBuilder`.
    pub fn new(ovi: obs_video_info, graphics_module: ObsString) -> Self {
        Self {
            ovi,
            graphics_module,
        }
    }

    /// Returns an `ObsVideoInfo` pointer.
    pub fn as_ptr(&mut self) -> *mut obs_video_info {
        ptr::addr_of_mut!(self.ovi)
    }

    pub fn graphics_module(&self) -> &ObsString {
        &self.graphics_module
    }
}

impl Default for ObsVideoInfo {
    fn default() -> Self {
        ObsVideoInfoBuilder::new().build()
    }
}

/// A structure intended to help make
/// creating new `ObsVideoInfo` structs
/// easier for resetting the OBS video
/// context.
#[derive(Clone, Debug)]
pub struct ObsVideoInfoBuilder {
    adapter: u32,
    graphics_module: ObsGraphicsModule,
    fps_num: u32,
    fps_den: u32,
    base_width: u32,
    base_height: u32,
    output_width: u32,
    output_height: u32,
    output_format: ObsVideoFormat,
    gpu_conversion: bool,
    colorspace: ObsColorspace,
    range: ObsVideoRange,
    scale_type: ObsScaleType,
}

impl ObsVideoInfoBuilder {
    /// Creates a new `ObsVideoInfoBuilder`
    /// for creating new `ObsVideoInfo` to
    /// pass to the video context reset
    /// function.
    ///
    /// This function comes with
    /// sensible default values and chooses
    /// the backend depending on which
    /// if the OS supports DX11 (Windows)
    /// or not (OpenGL on MacOS and Unix).
    pub fn new() -> Self {
        let display_infos = DisplayInfo::all().unwrap();
        let (mut width, mut height) = (1920, 1080);
        for display_info in display_infos {
            if display_info.is_primary {
                width = display_info.width;
                height = display_info.height;
                break;
            }
        }

        Self {
            adapter: 0,
            #[cfg(target_family = "unix")]
            graphics_module: ObsGraphicsModule::OpenGL,
            #[cfg(target_family = "windows")]
            graphics_module: ObsGraphicsModule::DirectX11,
            fps_num: 30,
            fps_den: 1,
            base_width: width,
            base_height: height,
            output_width: width,
            output_height: height,
            output_format: ObsVideoFormat::NV12,
            gpu_conversion: true,
            colorspace: ObsColorspace::CS709,
            range: ObsVideoRange::Default,
            scale_type: ObsScaleType::Lanczos,
        }
    }

    /// Consumes the `ObsVideoInfoBuilder`
    /// to create an `ObsVideoInfo`.
    pub fn build(self) -> ObsVideoInfo {
        let graphics_mod_str = match self.graphics_module {
            ObsGraphicsModule::OpenGL => ObsString::new("libobs-opengl"),
            ObsGraphicsModule::DirectX11 => ObsString::new("libobs-d3d11"),
        };

        let ovi = obs_video_info {
            adapter: self.adapter,
            graphics_module: graphics_mod_str.as_ptr(),
            fps_num: self.fps_num,
            fps_den: self.fps_den,
            base_width: self.base_width,
            base_height: self.base_height,
            output_width: self.output_width,
            output_height: self.output_height,
            output_format: self.output_format as OsEnumType,
            gpu_conversion: self.gpu_conversion,
            colorspace: self.colorspace as OsEnumType,
            range: self.range as OsEnumType,
            scale_type: self.scale_type as OsEnumType,
        };

        drop(self);

        ObsVideoInfo {
            ovi: ovi,
            graphics_module: graphics_mod_str,
        }
    }

    /// Sets the GPU adapter device
    /// that the video output is coming
    /// from.
    pub fn adapter(mut self, value: u32) -> Self {
        self.adapter = value;
        self
    }

    /// Sets the graphics backend
    /// that libobs uses to record.
    pub fn graphics_module(mut self, value: ObsGraphicsModule) -> Self {
        self.graphics_module = value;
        self
    }

    /// Sets the framerate of the
    /// output video. Note that this
    /// value may not reflect the
    /// final framerate if `fps_den`
    /// is not equal to 1.
    pub fn fps_num(mut self, value: u32) -> Self {
        self.fps_num = value;
        self
    }

    /// Divides the FPS numerator to
    /// allow for fractional FPS
    /// counts on output.
    pub fn fps_den(mut self, value: u32) -> Self {
        self.fps_den = value;
        self
    }

    /// Sets the width of the screen
    /// being recorded.
    pub fn base_width(mut self, value: u32) -> Self {
        self.base_width = value;
        self
    }

    /// Sets the height of the screen
    /// being recorded.
    pub fn base_height(mut self, value: u32) -> Self {
        self.base_height = value;
        self
    }

    /// Sets the width of the video
    /// output.
    pub fn output_width(mut self, value: u32) -> Self {
        self.output_width = value;
        self
    }

    /// Sets the height of the video
    /// output.
    pub fn output_height(mut self, value: u32) -> Self {
        self.output_height = value;
        self
    }

    /// Sets the format in which the
    /// video will be output.
    pub fn output_format(mut self, value: ObsVideoFormat) -> Self {
        self.output_format = value;
        self
    }

    /// Sets whether the GPU will handle
    /// conversion in the video.
    pub fn gpu_conversion(mut self, value: bool) -> Self {
        self.gpu_conversion = value;
        self
    }

    /// Sets the video colorspace.
    pub fn colorspace(mut self, value: ObsColorspace) -> Self {
        self.colorspace = value;
        self
    }

    /// Sets the video range.
    pub fn range(mut self, value: ObsVideoRange) -> Self {
        self.range = value;
        self
    }

    /// Sets the video scaling type.
    pub fn scale_type(mut self, value: ObsScaleType) -> Self {
        self.scale_type = value;
        self
    }
}

impl Default for ObsVideoInfoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Status types returned after attempting to
/// reset the OBS video context using the
/// function `obs_reset_video`.
pub enum ObsResetVideoStatus {
    /// `obs_reset_video` was successful.
    Success = crate::OBS_VIDEO_SUCCESS as i32,
    /// The adapter is not supported as it
    /// lacks capabilities.
    NotSupported = crate::OBS_VIDEO_NOT_SUPPORTED,
    /// A parameter is invalid.
    InvalidParameter = crate::OBS_VIDEO_INVALID_PARAM,
    /// An output is currently running, preventing
    /// resetting the video context.
    CurrentlyActive = crate::OBS_VIDEO_CURRENTLY_ACTIVE,
    /// Generic error occured when attempting to
    /// reset the OBS video context.
    Failure = crate::OBS_VIDEO_FAIL,
}

/// Information passed to libobs when attempting to
/// reset the audio context using `obs_reset_audio`.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObsAudioInfo {
    samples_per_sec: ObsSamplesPerSecond,
    speakers: ObsSpeakerLayout,
}

impl ObsAudioInfo {
    pub fn new(samples_per_second: ObsSamplesPerSecond, speakers: ObsSpeakerLayout) -> Self {
        Self {
            samples_per_sec: samples_per_second,
            speakers: speakers,
        }
    }

    pub fn as_ptr(&self) -> *const obs_audio_info {
        self as *const Self as *const obs_audio_info
    }
}

impl Default for ObsAudioInfo {
    fn default() -> Self {
        Self {
            samples_per_sec: ObsSamplesPerSecond::F44100,
            speakers: ObsSpeakerLayout::Stereo,
        }
    }
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

/// Information passed to libobs when attempting to
/// reset the audio context using the newer, more
/// detailed function `obs_reset_audio2`.
pub type ObsAudioInfo2 = obs_audio_info2;

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ObsSpeakerLayout {
    S2Point1 = crate::speaker_layout_SPEAKERS_2POINT1,
    S4Point0 = crate::speaker_layout_SPEAKERS_4POINT0,
    S4Point1 = crate::speaker_layout_SPEAKERS_4POINT1,
    S5Point1 = crate::speaker_layout_SPEAKERS_5POINT1,
    S7Point1 = crate::speaker_layout_SPEAKERS_7POINT1,
    Mono = crate::speaker_layout_SPEAKERS_MONO,
    Stereo = crate::speaker_layout_SPEAKERS_STEREO,
    Unknown = crate::speaker_layout_SPEAKERS_UNKNOWN,
}

#[derive(Debug)]
pub struct ObsOutput {
    output: *mut obs_output,
    id: ObsString,
    name: ObsString,
    settings: Option<ObsData>,
    hotkey_data: Option<ObsData>,
    video_encoders: Vec<ObsVideoEncoder>,
    audio_encoders: Vec<ObsAudioEncoder>,
    sources: Vec<ObsSource>,
}

extern "C" {

}

impl ObsOutput {
    pub fn new(
        id: impl Into<ObsString>,
        name: impl Into<ObsString>,
        settings: Option<ObsData>,
        hotkey_data: Option<ObsData>,
    ) -> Result<Self, ObsError> {
        // Likely unnecessary as this is private and only
        // constructible with ObsContext member functions.
        /*if let Ok(thread_id) = crate::wrapper::OBS_THREAD_ID.lock() {
            if *thread_id != Some(thread::current().id()) {
                return Err(ObsError::CreateThreadError)
            }
        } else {
            panic!();
        }*/

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

        let output = unsafe {
            crate::obs_output_create(id.as_ptr(), name.as_ptr(), settings_ptr, hotkey_data_ptr)
        };

        if output == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        //TODO connect signal handler
        let handler = unsafe { crate::obs_output_get_signal_handler(output) };
        let handler = unsafe { crate::obs_signal_handler_connect(handler, Some(signal_handler), ptr::null_mut()) };

        Ok(Self {
            output,
            id,
            name,
            settings,
            hotkey_data,
            video_encoders: vec![],
            audio_encoders: vec![],
            sources: vec![],
        })
    }

    pub fn get_video_encoders(&mut self) -> &mut Vec<ObsVideoEncoder> {
        &mut self.video_encoders
    }

    pub fn video_encoder(
        &mut self,
        info: VideoEncoderInfo,
        handler: *mut video_output,
    ) -> Result<&mut ObsVideoEncoder, ObsError> {
        let video_enc = ObsVideoEncoder::new(info.id, info.name, info.settings, info.hotkey_data);

        return match video_enc {
            Ok(x) => {
                unsafe { crate::obs_encoder_set_video(x.encoder, handler) }
                unsafe { crate::obs_output_set_video_encoder(self.output, x.encoder) }
                self.video_encoders.push(x);

                Ok(self.video_encoders.last_mut().unwrap())
            }
            Err(x) => Err(x),
        };
    }

    pub fn audio_encoder(
        &mut self,
        info: AudioEncoderInfo,
        mixer_idx: usize,
        handler: *mut audio_output,
    ) -> Result<&mut ObsAudioEncoder, ObsError> {
        let audio_enc = ObsAudioEncoder::new(
            info.id,
            info.name,
            info.settings,
            mixer_idx,
            info.hotkey_data,
        );

        return match audio_enc {
            Ok(x) => {
                unsafe { crate::obs_encoder_set_audio(x.encoder, handler) }
                unsafe { crate::obs_output_set_audio_encoder(self.output, x.encoder, mixer_idx) }
                self.audio_encoders.push(x);

                Ok(self.audio_encoders.last_mut().unwrap())
            }
            Err(x) => Err(x),
        };
    }

    pub fn source(&mut self, info: SourceInfo, channel: u32) -> Result<&mut ObsSource, ObsError> {
        let source = ObsSource::new(info.id, info.name, info.settings, info.hotkey_data);

        return match source {
            Ok(x) => {
                unsafe { crate::obs_set_output_source(channel, x.source) }
                self.sources.push(x);
                Ok(self.sources.last_mut().unwrap())
            }
            Err(x) => Err(x),
        };
    }

    pub fn start(&mut self) -> Result<(), ObsError> {
        if unsafe { !crate::obs_output_active(self.output) } {
            let res = unsafe { crate::obs_output_start(self.output) };
            if res {
                return Ok(());
            }

            let err = unsafe { crate::obs_output_get_last_error(self.output) };
            let c_str = unsafe { CStr::from_ptr(err) };
            let err_str = c_str.to_str().ok().map(|x| x.to_string());

            return Err(ObsError::OutputStartFailure(err_str));
        }

        Err(ObsError::OutputAlreadyActive)
    }

    pub fn stop(&mut self) -> Result<(), ObsError> {
        if unsafe { crate::obs_output_active(self.output) } {
            unsafe { crate::obs_output_stop(self.output) }

            let still_active = unsafe { crate::obs_output_active(self.output) };
            if !still_active {
                return Ok(());
            }

            let err = unsafe { crate::obs_output_get_last_error(self.output) };
            let err_str = if err != ptr::null_mut() {
            let c_str = unsafe { CStr::from_ptr(err) };
            c_str.to_str().ok().map(|x| x.to_string())
            } else {
                Some("Unknown error.".to_string())
            };

            return Err(ObsError::OutputStopFailure(err_str));
        }

        return Err(ObsError::OutputStopFailure(Some("Output is not active.".to_string())));
    }

    // Getters
    pub fn name(&self) -> &ObsString {
        &self.name
    }

    pub fn id(&self) -> &ObsString {
        &self.id
    }

    pub fn settings(&self) -> &Option<ObsData> {
        &self.settings
    }

    pub fn hotkey_data(&self) -> &Option<ObsData> {
        &self.hotkey_data
    }

    pub fn video_encoders(&self) -> &Vec<ObsVideoEncoder> {
        &self.video_encoders
    }

    pub fn audio_encoders(&self) -> &Vec<ObsAudioEncoder> {
        &self.audio_encoders
    }

    pub fn sources(&self) -> &Vec<ObsSource> {
        &self.sources
    }
}

impl Drop for ObsOutput {
    fn drop(&mut self) {
        unsafe { crate::obs_output_release(self.output) }
    }
}

#[derive(Debug)]
pub struct ObsVideoEncoder {
    encoder: *mut obs_encoder,
    id: ObsString,
    name: ObsString,
    settings: Option<ObsData>,
    hotkey_data: Option<ObsData>,
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
            crate::obs_video_encoder_create(
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
        unsafe { crate::obs_encoder_release(self.encoder) }
    }
}

#[derive(Debug)]
pub struct ObsAudioEncoder {
    encoder: *mut obs_encoder,
    id: ObsString,
    name: ObsString,
    settings: Option<ObsData>,
    hotkey_data: Option<ObsData>,
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
            crate::obs_audio_encoder_create(
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
            encoder,
            id,
            name,
            settings,
            hotkey_data,
        })
    }
}

impl Drop for ObsAudioEncoder {
    fn drop(&mut self) {
        unsafe { crate::obs_encoder_release(self.encoder) }
    }
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
            ObsVideoEncoderType::OBS_X264 => ObsString::new("obs_x264"),
            _ => ObsString::new("obs_x264"),
        };
    }
}

#[derive(Debug)]
pub struct ObsSource {
    source: *mut obs_source,
    id: ObsString,
    name: ObsString,
    settings: Option<ObsData>,
    hotkey_data: Option<ObsData>,
}

impl ObsSource {
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

        let source = unsafe {
            crate::obs_source_create(id.as_ptr(), name.as_ptr(), settings_ptr, hotkey_data_ptr)
        };

        if source == ptr::null_mut() {
            return Err(ObsError::NullPointer);
        }

        Ok(Self {
            source,
            id,
            name,
            settings,
            hotkey_data,
        })
    }
}

impl Drop for ObsSource {
    fn drop(&mut self) {
        unsafe { crate::obs_source_release(self.source) }
    }
}
