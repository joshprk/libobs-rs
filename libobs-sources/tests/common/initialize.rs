use env_logger::Env;
use libobs_wrapper::{context::{ObsContext, ObsContextReturn}, data::output::ObsOutputRef, encoders::{ObsContextEncoders, ObsVideoEncoderType}, enums::ObsLogLevel, logger::ObsLogger, utils::{AudioEncoderInfo, ObsString, OutputInfo, StartupInfo}};
use std::{env::current_dir, fs::File, io::Write};

pub async fn initialize_obs<'a, T: Into<ObsString> + Send + Sync>(rec_file: T) -> (ObsContext, ObsOutputRef) {
    initialize_obs_with_log(rec_file, false).await
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
pub async fn initialize_obs_with_log<'a, T: Into<ObsString> + Send + Sync>(rec_file: T, file_logger: bool) -> (ObsContext, ObsOutputRef) {
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug")).is_test(true).try_init();

    // Start the OBS context
    #[allow(unused_mut)]
    let mut startup_info = StartupInfo::default();
    if file_logger {
        let _l = DebugLogger { f: File::create(current_dir().unwrap().join("obs.log")).unwrap() };
        //startup_info = startup_info.set_logger(Box::new(_l));
    }

    let context = ObsContext::new(startup_info).await.unwrap();
    let mut context = match context {
        ObsContextReturn::Done(c) => c,
        ObsContextReturn::Restart => panic!("OBS context restarted"),
    };

    // Set up output to ./recording.mp4
    let mut output_settings = context.data().await.unwrap();
    output_settings.set_string("path", rec_file).await.unwrap();

    let output_name = "output";
    let output_info = OutputInfo::new("ffmpeg_muxer", output_name, Some(output_settings), None);

    let mut output = context.output(output_info).await.unwrap();

    // Register the video encoder
    let mut video_settings = context.data().await.unwrap();
    video_settings
        .bulk_update()
        .set_int("bf", 0)
        .set_bool("psycho_aq", true)
        .set_bool("lookahead", true)
        .set_string("profile", "high")
        .set_string("preset", "fast")
        .set_string("rate_control", "cbr")
        .set_int("bitrate", 10000)
        .update().await.unwrap();

    let encoders = context.available_video_encoders().await.unwrap();

    println!("Available encoders: {:?}", encoders);
    let encoder =  encoders.into_iter().find(|e| {
        let  t = e.get_encoder_id();
        t == &ObsVideoEncoderType::H264_TEXTURE_AMF || t == &ObsVideoEncoderType::AV1_TEXTURE_AMF
    }).unwrap();

    println!("Using encoder {:?}", encoder);
    encoder.set_to_output(&mut output, "video_encoder", Some(video_settings), None).await.unwrap();

    // Register the audio encoder
    let mut audio_settings = context.data().await.unwrap();
    audio_settings.set_int("bitrate", 160).await.unwrap();

    let audio_info =
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

    let audio_handler = context.get_audio_ptr().await.unwrap();
    output.audio_encoder(audio_info, 0, audio_handler).await.unwrap();

    (context, output)
}
