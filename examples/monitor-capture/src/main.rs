use std::thread;
use std::time::Duration;

use libobs_simple::sources::windows::{MonitorCaptureSourceBuilder, MonitorCaptureSourceUpdater};
use libobs_wrapper::context::ObsContext;
use libobs_wrapper::data::ObsObjectUpdater;
use libobs_wrapper::encoders::ObsContextEncoders;
use libobs_wrapper::sources::ObsSourceBuilder;
use libobs_wrapper::utils::traits::ObsUpdatable;
use libobs_wrapper::utils::{AudioEncoderInfo, ObsPath, OutputInfo, StartupInfo};

pub fn main() -> anyhow::Result<()> {
    // Start the OBS context
    let startup_info = StartupInfo::default();
    let mut context = ObsContext::new(startup_info)?;

    let mut scene = context.scene("main")?;
    let monitors = MonitorCaptureSourceBuilder::get_monitors()?;

    let mut monitor_capture = context
        .source_builder::<MonitorCaptureSourceBuilder, _>("Monitor Capture")?
        .set_monitor(&monitors[0])
        .add_to_scene(&mut scene)?;

    // Register the source
    scene.set_to_channel(0)?;

    // Set up output to ./recording.mp4
    let mut output_settings = context.data()?;
    output_settings.set_string("path", ObsPath::from_relative("recording.mp4").build())?;

    let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);
    let mut output = context.output(output_info)?;

    // Register the video encoder
    let mut video_settings = context.data()?;
    video_settings
        .bulk_update()
        .set_int("bf", 2)
        .set_bool("psycho_aq", true)
        .set_bool("lookahead", true)
        .set_string("profile", "high")
        .set_string("preset", "hq")
        .set_string("rate_control", "cbr")
        .set_int("bitrate", 10000)
        .update()?;

    let mut video_encoder = context.best_video_encoder()?;
    video_encoder.set_settings(video_settings);
    video_encoder.set_to_output(&mut output, "video_encoder")?;

    // Register the audio encoder
    let mut audio_settings = context.data()?;
    audio_settings.set_int("bitrate", 160)?;

    let audio_info =
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

    output.create_and_set_audio_encoder(audio_info, 0)?;

    output.start()?;

    println!("recording for 5 seconds and switching monitor...");
    thread::sleep(Duration::from_secs(5));

    // Switching monitor
    monitor_capture
        .create_updater::<MonitorCaptureSourceUpdater>()?
        .set_monitor(&monitors[1])
        .update()?;

    println!("recording for another 5 seconds...");
    thread::sleep(Duration::from_secs(5));

    // Success!
    output.stop()?;

    Ok(())
}
