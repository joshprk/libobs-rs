use env_logger::Env;
use libobs_wrapper::{context::ObsContext, data::ObsData, encoders::{ObsContextEncoders, ObsVideoEncoderType}, enums::ObsLogLevel, logger::ObsLogger, utils::{AudioEncoderInfo, ObsString, OutputInfo, StartupInfo, VideoEncoderInfo}};
use std::{env::current_dir, fs::File, io::Write};

pub fn initialize_obs<'a>(rec_file: ObsString) -> (ObsContext, String) {
    initialize_obs_with_log(rec_file, false)
}

#[derive(Debug)]
struct DebugLogger {
    f: File
}
impl ObsLogger for DebugLogger {
    fn log(&mut self, level: libobs_wrapper::enums::ObsLogLevel, msg: String) {
        if level == ObsLogLevel::Debug {
            return;
        }

        self.f.write_all(format!("{}\n", msg).as_bytes()).unwrap();
    }
}

/// The string returned is the name of the obs output
pub fn initialize_obs_with_log<'a>(rec_file: ObsString, file_logger: bool) -> (ObsContext, String) {
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug")).is_test(true).try_init();

    // Start the OBS context
    #[allow(unused_mut)]
    let mut startup_info = StartupInfo::default();
    if file_logger {
        let _l = DebugLogger { f: File::create(current_dir().unwrap().join("obs.log")).unwrap() };
        //startup_info = startup_info.set_logger(Box::new(_l));
    }

    let mut context = ObsContext::new(startup_info).unwrap();

    // Set up output to ./recording.mp4
    let mut output_settings = ObsData::new();
    output_settings.set_string("path", rec_file);

    let output_name = "output";
    let output_info = OutputInfo::new("ffmpeg_muxer", output_name, Some(output_settings), None);

    let mut output = context.output(output_info).unwrap();

    // Register the video encoder
    let mut video_settings = ObsData::new();
    video_settings
        .set_int("bf", 0)
        .set_bool("psycho_aq", true)
        .set_bool("lookahead", true)
        .set_string("profile", "high")
        .set_string("preset", "fast")
        .set_string("rate_control", "cbr")
        .set_int("bitrate", 10000);

    let encoders = ObsContext::get_available_video_encoders();

    println!("Available encoders: {:?}", encoders);
    let encoder =  encoders.iter().find(|e| **e == ObsVideoEncoderType::H264_TEXTURE_AMF || **e == ObsVideoEncoderType::AV1_TEXTURE_AMF).unwrap();

    println!("Using encoder {:?}", encoder);
    let video_info = VideoEncoderInfo::new(
        encoder.clone(),
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

    (context, output_name.to_string())
}
