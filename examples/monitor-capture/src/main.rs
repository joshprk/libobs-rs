use std::thread;
use std::time::Duration;

use libobs_sources::windows::{MonitorCaptureSourceBuilder, MonitorCaptureSourceUpdater};
use libobs_wrapper::context::ObsContext;
use libobs_wrapper::data::{ObsData, ObsObjectBuilder, ObsObjectUpdater};
use libobs_wrapper::encoders::ObsContextEncoders;
use libobs_wrapper::sources::ObsSourceBuilder;
use libobs_wrapper::utils::traits::ObsUpdatable;
use libobs_wrapper::utils::{
    AudioEncoderInfo, ObsPath, OutputInfo, StartupInfo, VideoEncoderInfo,
};

pub fn main() {
    // Start the OBS context
    let startup_info = StartupInfo::default();
    let mut context = ObsContext::new(startup_info).unwrap();

    let mut scene = context.scene("main");
    let monitors = MonitorCaptureSourceBuilder::get_monitors().unwrap();
    let mut monitor_capture = MonitorCaptureSourceBuilder::new("Monitor Capture")
        .set_monitor(&monitors[0])
        .add_to_scene(&mut scene)
        .unwrap();

    // Register the source
    scene.add_and_set(0);

    // Set up output to ./recording.mp4
    let mut output_settings = ObsData::new();
    output_settings.set_string("path", ObsPath::from_relative("recording.mp4").build());

    let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);

    let mut output = context.output(output_info).unwrap();

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
        ObsContext::get_best_video_encoder(),
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

    output.start().unwrap();

    println!("recording for 5 seconds and switching monitor...");
    thread::sleep(Duration::from_secs(5));

    // Switching monitor
    monitor_capture
        .create_updater::<MonitorCaptureSourceUpdater>()
        .set_monitor(&monitors[1])
        .update();

    println!("recording for another 5 seconds...");
    thread::sleep(Duration::from_secs(5));

    // Success!
    output.stop().unwrap();
}
