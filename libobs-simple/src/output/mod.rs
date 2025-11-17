use libobs_wrapper::data::output::ObsOutputRef;
use std::path::PathBuf;

pub enum StreamingEncoderPresetSoftware {
    UltraFast,
    SuperFast,
    VeryFast,
    Faster,
    Fast,
}

pub enum StreamingEncoderPresetHardware {
    Speed,
    Balanced,
    Quality,
}

pub enum StreamingVideoEncoder {
    /// x264
    Software(StreamingEncoderPresetSoftware),
    Hardware(StreamingEncoderPresetHardware),
}

pub enum StreamingAudioEncoder {
    AAC,
}

pub struct StreamingOutputSettings {
    video_bitrate: u32,
    audio_bitrate: u32,
    video_encoder: StreamingVideoEncoder,
    custom_encoder_settings: Option<String>,
    audio_encoder: StreamingAudioEncoder,
}

impl Default for StreamingOutputSettings {
    fn default() -> Self {
        StreamingOutputSettings {
            video_bitrate: 6000,
            audio_bitrate: 160,
            video_encoder: StreamingVideoEncoder::Software(
                StreamingEncoderPresetSoftware::VeryFast,
            ),
            audio_encoder: StreamingAudioEncoder::AAC,
            custom_encoder_settings: None,
        }
    }
}

pub enum RecordingVideoEncoder {
    /// x264
    Software,
    /// x264, lower CPU usage, increases file size
    SoftwareLowerCPU,
    HardwareHEVC,
    HardwareH264,
}

pub enum RecordingAudioEncoderType {
    AAC,
    Opus,
}

pub struct RecordingAudioEncoder {
    encoder_type: RecordingAudioEncoderType,
    /// Set true for enabled audio track
    tracks: &'static [bool; 6],
}

impl RecordingAudioEncoder {
    pub fn new(encoder_type: RecordingAudioEncoderType, tracks: &'static [bool; 6]) -> Self {
        Self {
            encoder_type,
            tracks,
        }
    }
}

pub enum RecordingFormats {
    /// .flv
    FlashVideo,
    /// .mkv
    MatroskaVideo,
    /// .mp4
    Mpeg4,
    /// .mov
    QuickTime,
    /// .mp4
    HybridMP4,
    /// .mov
    HybridMov,
    /// .mp4
    FragmentedMP4,
    /// .mov
    FragmentedMOV,
    /// MPEG-TS .ts
    MpegTs,
}

pub enum RecordingQuality {
    /// High Quality, Medium File Size
    High(RecordingVideoEncoder, RecordingAudioEncoder),
    /// Indistinguishable Quality, Large File Size
    Indistinguishable(RecordingVideoEncoder, RecordingAudioEncoder),
    /// Lossless Quality, **TREMENDOUSLY** Large File Size
    Lossless(RecordingVideoEncoder, RecordingAudioEncoder),

    SameAsStream,
}

pub struct RecordingOutputSettings {
    path: Option<PathBuf>,
    generate_file_name_without_spaces: bool,
    /// If None, uses same quality as set for streaming
    recording_quality: RecordingQuality,
    recording_format: RecordingFormats,
    custom_muxer_settings: Option<String>,
}

impl Default for RecordingOutputSettings {
    fn default() -> Self {
        Self {
            path: None,
            generate_file_name_without_spaces: false,
            recording_quality: RecordingQuality::SameAsStream,
            recording_format: RecordingFormats::HybridMP4,
            custom_muxer_settings: None,
        }
    }
}

pub struct SimpleOutputBuilder {
    recording: RecordingOutputSettings,
    streaming: StreamingOutputSettings,
}

impl Default for SimpleOutputBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleOutputBuilder {
    pub fn new() -> Self {
        SimpleOutputBuilder {
            streaming: StreamingOutputSettings::default(),
            recording: RecordingOutputSettings::default(),
        }
    }

    pub fn build() -> ObsOutputRef {
        todo!()
    }
}
