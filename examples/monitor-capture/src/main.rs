use std::thread;
use std::time::Duration;

use libobs_sources::windows::{MonitorCaptureSourceBuilder, MonitorCaptureSourceUpdater};
use libobs_wrapper::bootstrap::ObsBootstrap;
use libobs_wrapper::context::{ObsContext, ObsContextReturn};
use libobs_wrapper::data::ObsObjectUpdater;
use libobs_wrapper::encoders::ObsContextEncoders;
use libobs_wrapper::sources::ObsSourceBuilder;
use libobs_wrapper::utils::traits::ObsUpdatable;
use libobs_wrapper::utils::{AudioEncoderInfo, ObsPath, OutputInfo, StartupInfo, VideoEncoderInfo};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Start the OBS context
    let startup_info = StartupInfo::default();
    let context = ObsContext::new(startup_info).await?;
    let context = match context {
        ObsContextReturn::Done(c) => Some(c),
        ObsContextReturn::Restart => {
            ObsContext::spawn_updater().await?;
            None
        }
    };

    if context.is_none() {
        println!("OBS has been updated, restarting...");
        return Ok(());
    }

    let mut context = context.unwrap();
    let mut scene = context.scene("main").await?;
    let monitors = MonitorCaptureSourceBuilder::get_monitors()?;

    let mut monitor_capture = context
        .source_builder::<MonitorCaptureSourceBuilder, _>("Monitor Capture")
        .await?
        .set_monitor(&monitors[0])
        .add_to_scene(&mut scene)
        .await?;

    // Register the source
    scene.add_and_set(0).await?;

    // Set up output to ./recording.mp4
    let mut output_settings = context.data().await?;
    output_settings
        .set_string("path", ObsPath::from_relative("recording.mp4").build())
        .await?;

    let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);
    let mut output = context.output(output_info).await?;

    // Register the video encoder
    let mut video_settings = context.data().await?;
    video_settings
        .bulk_update()
        .set_int("bf", 2)
        .set_bool("psycho_aq", true)
        .set_bool("lookahead", true)
        .set_string("profile", "high")
        .set_string("preset", "hq")
        .set_string("rate_control", "cbr")
        .set_int("bitrate", 10000)
        .update()
        .await?;

    let video_info = VideoEncoderInfo::new(
        context.get_best_video_encoder().await?,
        "video_encoder",
        Some(video_settings),
        None,
    );

    let video_handler = context.get_video_ptr().await?;
    output.video_encoder(video_info, video_handler).await?;

    // Register the audio encoder
    let mut audio_settings = context.data().await?;
    audio_settings.set_int("bitrate", 160).await?;

    let audio_info =
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

    let audio_handler = context.get_audio_ptr().await?;
    output.audio_encoder(audio_info, 0, audio_handler).await?;

    output.start().await?;

    println!("recording for 5 seconds and switching monitor...");
    thread::sleep(Duration::from_secs(5));

    // Switching monitor
    monitor_capture
        .create_updater::<MonitorCaptureSourceUpdater>()
        .await?
        .set_monitor(&monitors[1])
        .update()
        .await?;

    println!("recording for another 5 seconds...");
    thread::sleep(Duration::from_secs(5));

    // Success!
    output.stop().await?;

    Ok(())
}
