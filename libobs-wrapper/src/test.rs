use std::thread;
use std::time::Duration;

use crate::context::ObsContext;
use crate::data::ObsData;
use crate::utils::{
    AudioEncoderInfo, ObsPath, OutputInfo, SourceInfo, StartupInfo, VideoEncoderInfo,
};

#[test]
pub fn main_test() {
    // Start the OBS context
    let startup_info = StartupInfo::default();
    let mut context = ObsContext::new(startup_info).unwrap();

    // Set up output to ./recording.mp4
    let mut output_settings = ObsData::new();
    output_settings.set_string("path", ObsPath::from_relative("recording.mp4").build());

    let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);

    let output = context.output(output_info).unwrap();

    // Register the video encoder
    let mut video_settings = ObsData::new();
    video_settings
        .set_int("bf", 2)
        .set_bool("psycho_aq", true)
        .set_bool("lookahead", true)
        .set_string("profile", "high")
        .set_string("preset", "hq")
        .set_string("rate_control", "cbr")
        .set_int("bitrate", 10000);

    let video_info = VideoEncoderInfo::new(
        ObsContext::get_best_encoder(),
        "video_encoder",
        Some(video_settings),
        None,
    );

    let video_handler = ObsContext::get_video_ptr().unwrap();
    output.video_encoder(video_info, video_handler).unwrap();

    // Register the audio encoder
    let mut audio_settings = ObsData::new();
    audio_settings.set_int("bitrate", 160);

    let audio_info =
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

    let audio_handler = ObsContext::get_audio_ptr().unwrap();
    output.audio_encoder(audio_info, 0, audio_handler).unwrap();

    // Create the video source using game capture
    let mut video_source_data = ObsData::new();
    video_source_data
        .set_string("capture_mode", "window")
        .set_string("window", "")
        .set_bool("capture_cursor", true);

    let video_source_info = SourceInfo::new(
        "game_capture",
        "video_source",
        Some(video_source_data),
        None,
    );

    // Register the source and record
    output.source(video_source_info, 0).unwrap();
    output.start().unwrap();

    println!("recording for 10 seconds...");
    thread::sleep(Duration::new(10, 0));

    // Open any fullscreen application and
    // Success!
    output.stop().unwrap();
}
