/// Standalone reproduction for OBS crash when switching encoders
///
/// Expected result: Crash eventually occurs on NVENC
/// It's not very consistent, if it doesn't crash, just run it again until it does :)
use std::{collections::HashMap, sync::Arc, thread, time::Duration};

use color_eyre::Result;
use libobs_sources::{ObsSourceBuilder, windows::MonitorCaptureSourceBuilder};
use libobs_wrapper::{
    context::ObsContext,
    data::video::ObsVideoInfoBuilder,
    encoders::{ObsVideoEncoderType, audio::ObsAudioEncoder, video::ObsVideoEncoder},
    enums::ObsScaleType,
    logger::ObsLogger,
    utils::{AudioEncoderInfo, ObsPath, OutputInfo, VideoEncoderInfo},
};

const ROUNDS: usize = 6;
const RECORDING_DURATION: u64 = 2;

// Simplified encoder type enum for this repro
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EncoderType {
    X264,
    NvEnc,
}

impl EncoderType {
    fn to_obs_encoder_type(&self) -> ObsVideoEncoderType {
        match self {
            EncoderType::X264 => ObsVideoEncoderType::OBS_X264,
            EncoderType::NvEnc => ObsVideoEncoderType::H264_TEXTURE_AMF,
        }
    }

    fn id(&self) -> &str {
        match self {
            EncoderType::X264 => "x264",
            EncoderType::NvEnc => "texture_amf",
        }
    }
}

#[derive(Debug)]
struct SimpleLogger;
impl ObsLogger for SimpleLogger {
    fn log(&mut self, level: libobs_wrapper::enums::ObsLogLevel, msg: String) {
        use libobs_wrapper::enums::ObsLogLevel;
        match level {
            ObsLogLevel::Error => eprintln!("[OBS ERROR] {msg}"),
            ObsLogLevel::Warning => eprintln!("[OBS WARN] {msg}"),
            ObsLogLevel::Info => println!("[OBS INFO] {msg}"),
            ObsLogLevel::Debug => println!("[OBS DEBUG] {msg}"),
        }
    }
}

struct ReproState {
    obs_context: ObsContext,
    output: libobs_wrapper::data::output::ObsOutputRef,
    audio_encoder: Arc<ObsAudioEncoder>,

    // Key point: storing encoders by type to reuse them (like production code)
    video_encoders: HashMap<EncoderType, Arc<ObsVideoEncoder>>,
    scene: libobs_wrapper::scenes::ObsSceneRef,
    monitor_capture: libobs_wrapper::sources::ObsSourceRef,
}

impl ReproState {
    fn new() -> Result<Self> {
        println!("\n=== Creating OBS Context ===");

        let video_info = ObsVideoInfoBuilder::new()
            .adapter(0)
            .fps_num(60)
            .fps_den(1)
            .base_width(1920)
            .base_height(1080)
            .output_width(1920)
            .output_height(1080)
            .scale_type(ObsScaleType::Bicubic)
            .build();

        let mut obs_context = ObsContext::new(
            ObsContext::builder()
                .set_logger(Box::new(SimpleLogger))
                .set_video_info(video_info),
        )?;

        println!("=== Creating Output (reused for all recordings) ===");
        let output_settings = obs_context.data()?;
        let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);
        let output = obs_context.output(output_info)?;

        println!("=== Creating Audio Encoder (reused for all recordings) ===");
        let mut audio_settings = obs_context.data()?;
        audio_settings.set_int("bitrate", 160)?;
        let audio_info =
            AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);
        let audio_encoder =
            ObsAudioEncoder::new_from_info(audio_info, 0, obs_context.runtime().clone())?;

        let mut scene = obs_context.scene("main")?;
        let monitors = MonitorCaptureSourceBuilder::get_monitors().unwrap();

        let monitor_capture = obs_context
            .source_builder::<MonitorCaptureSourceBuilder, _>("Monitor Capture")?
            .set_monitor(&monitors[0])
            .add_to_scene(&mut scene)?;

        // Register the source
        scene.set_to_channel(0)?;

        Ok(Self {
            obs_context,
            output,
            audio_encoder,
            video_encoders: HashMap::new(),
            scene,
            monitor_capture,
        })
    }

    fn simulate_recording(
        &mut self,
        encoder_type: EncoderType,
        recording_num: usize,
    ) -> Result<()> {
        println!(
            "\n=== Recording {} with {} ===",
            recording_num,
            encoder_type.id()
        );

        // Create a dummy output path (replace with your own)
        let output_path = format!("test_recording_{}.mp4", encoder_type.id());

        // KEY FIX: Reset video settings between recordings (like production code does)
        // This simulates changing game resolution and may trigger the bug
        let video_info = ObsVideoInfoBuilder::new()
            .adapter(0)
            .fps_num(60)
            .fps_den(1)
            .base_width(1920)
            .base_height(1080)
            .output_width(1920)
            .output_height(1080)
            .scale_type(ObsScaleType::Bicubic)
            .build();
        println!("  → Resetting video settings (simulating resolution change)");
        self.obs_context.reset_video(video_info)?;

        // Update output path
        let mut output_settings = self.obs_context.data()?;
        output_settings.set_string("path", ObsPath::new(&output_path).build())?;
        self.output.update_settings(output_settings)?;

        // Create encoder settings
        let mut encoder_settings = self.obs_context.data()?;
        encoder_settings.set_int("bitrate", 2500)?;
        encoder_settings.set_string("rate_control", "CBR")?;
        encoder_settings.set_string("profile", "high")?;
        encoder_settings.set_int("bf", 2)?;
        encoder_settings.set_bool("psycho_aq", true)?;
        encoder_settings.set_bool("lookahead", true)?;

        // Add encoder-specific settings
        match encoder_type {
            EncoderType::X264 => {
                encoder_settings.set_string("preset", "veryfast")?;
            }
            EncoderType::NvEnc => {
                encoder_settings.set_string("preset2", "p4")?;
                encoder_settings.set_string("tune", "hq")?;
            }
        }

        // Use unique encoder names per type (more realistic)
        let encoder_name = format!("video_encoder_{}", encoder_type.id());

        let video_encoder = if let Some(existing) = self.video_encoders.get(&encoder_type) {
            println!("  → Reusing existing {} encoder", encoder_type.id());
            existing.clone()
        } else {
            println!("  → Creating new {} encoder", encoder_type.id());
            let encoder = ObsVideoEncoder::new_from_info(
                VideoEncoderInfo::new(
                    encoder_type.to_obs_encoder_type(),
                    encoder_name.as_str(),
                    Some(encoder_settings.clone()),
                    None,
                ),
                self.obs_context.runtime().clone(),
            )?;
            self.video_encoders.insert(encoder_type, encoder.clone());
            encoder
        };

        // Set encoders on output
        println!("  → Setting video encoder on output");
        self.output.set_video_encoder(video_encoder)?;

        println!("  → Setting audio encoder on output");
        self.output
            .set_audio_encoder(self.audio_encoder.clone(), 0)?;

        // Start output
        println!("  → Starting output...");
        self.output.start()?;

        // Simulate recording
        println!("  → Recording for {RECORDING_DURATION} seconds...");
        thread::sleep(Duration::from_secs(RECORDING_DURATION));

        // Stop output
        println!("  → Stopping output...");
        self.output.stop()?;

        println!("  → Recording {} completed successfully", recording_num);

        // Small delay between recordings
        thread::sleep(Duration::from_millis(500));

        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  OBS Encoder Switch Crash Reproduction                       ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("This test attempts to reproduce a crash when switching between encoders.");
    println!();

    let mut state = ReproState::new()?;

    for i in 0..ROUNDS {
        state.simulate_recording(
            if i % 2 == 0 {
                EncoderType::NvEnc
            } else {
                EncoderType::X264
            },
            i,
        )?;
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║  ✓ Test completed without crash!                             ║");
    println!("║  If you see this, the bug may have been fixed.               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    Ok(())
}
