// Test actual recording with bootstrapper-downloaded binaries
use libobs_bootstrapper::{ObsBootstrapper, ObsBootstrapperOptions};
use libobs_wrapper::{
    context::ObsContext,
    encoders::{ObsAudioEncoderType, ObsVideoEncoderBuilder},
    utils::{AudioEncoderInfo, OutputInfo, StartupInfo},
};

#[cfg(target_os = "macos")]
use libobs_wrapper::sources::ObsSourceBuilder;

#[cfg(target_os = "macos")]
use libobs_sources::macos::ScreenCaptureSourceBuilder;

#[cfg(target_os = "windows")]
use libobs_sources::windows::MonitorCaptureSourceBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== Bootstrapper + Recording Test ===\n");

    // Step 1: Bootstrap OBS binaries
    println!("Bootstrapping OBS...");
    match ObsBootstrapper::bootstrap(&ObsBootstrapperOptions::default()).await? {
        libobs_bootstrapper::ObsBootstrapperResult::None => {
            println!("✓ OBS ready\n");
        }
        libobs_bootstrapper::ObsBootstrapperResult::Restart => {
            println!("✓ Downloaded - restart required");
            return Ok(());
        }
    }

    // Step 2: Initialize context
    let mut context = ObsContext::new(StartupInfo::default())?;
    println!("✓ Context initialized");

    // Step 3: Create scene and source (platform-specific)
    let mut scene = context.scene("Recording Scene")?;
    
    #[cfg(target_os = "macos")]
    {
        use libobs_wrapper::sources::ObsSourceBuilder;
        let _source = context
            .source_builder::<ScreenCaptureSourceBuilder, _>("Screen Capture")?
            .set_display(0)
            .set_show_cursor(true)
            .add_to_scene(&mut scene)?;
        println!("✓ macOS screen capture ready");
    }
    
    #[cfg(target_os = "windows")]
    {
        use libobs_wrapper::sources::ObsSourceBuilder;
        let monitors = MonitorCaptureSourceBuilder::get_monitors()?;
        let _source = context
            .source_builder::<MonitorCaptureSourceBuilder, _>("Monitor Capture")?
            .set_monitor(&monitors[0])
            .add_to_scene(&mut scene)?;
        println!("✓ Windows monitor capture ready");
    }
    
    scene.set_to_channel(0)?;

    // Step 4: Setup output - MP4 with ffmpeg_muxer
    let mut output_settings = context.data()?;
    
    #[cfg(target_os = "macos")]
    let desktop = std::env::var("HOME")? + "/Desktop/bootstrapper_recording.mp4";
    
    #[cfg(target_os = "windows")]
    let desktop = std::env::var("USERPROFILE")? + "\\Desktop\\bootstrapper_recording.mp4";
    
    output_settings.set_string("path", desktop.as_str())?;

    let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);
    let mut output = context.output(output_info)?;
    println!("✓ MP4 output created: {}", desktop);

    // Step 5: Setup video encoder (use obs_x264 - works with FLV)
    let mut video_settings = context.data()?;
    video_settings
        .bulk_update()
        .set_string("rate_control", "CBR")
        .set_int("bitrate", 2500)
        .set_string("preset", "veryfast")
        .update()?;

    let mut video_encoder = ObsVideoEncoderBuilder::new(context.clone(), "obs_x264");
    video_encoder.set_settings(video_settings);
    video_encoder.set_to_output(&mut output, "video_encoder")?;

    let mut audio_settings = context.data()?;
    audio_settings.set_int("bitrate", 128)?;

    let audio_info = AudioEncoderInfo::new(
        ObsAudioEncoderType::FFMPEG_AAC,
        "audio_encoder",
        Some(audio_settings),
        None,
    );
    output.create_and_set_audio_encoder(audio_info, 0)?;
    println!("✓ Encoders configured");

    // Step 6: Record!
    println!("\n🔴 Starting recording...");
    output.start()?;
    println!("✓ Recording started!");

    std::thread::sleep(std::time::Duration::from_secs(5));

    let _ = output.stop();
    std::thread::sleep(std::time::Duration::from_millis(500));

    println!("⏹️  Recording stopped\n");
    println!("✅ Saved to: {}", desktop);
    println!("📁 File size: ");

    // Check file was created
    if std::path::Path::new(&desktop).exists() {
        let metadata = std::fs::metadata(&desktop)?;
        println!("   {} bytes", metadata.len());
        println!("\n🎬 Open with: open {}", desktop);
    } else {
        println!("   ⚠️ File not found - check errors above");
    }

    Ok(())
}
