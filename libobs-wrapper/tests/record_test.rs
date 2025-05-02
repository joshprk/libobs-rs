#[cfg(not(feature = "blocking"))]
mod require_non_blocking {
    use std::thread;
    use std::time::Duration;

    use libobs_wrapper::context::ObsContext;
    use libobs_wrapper::encoders::ObsContextEncoders;
    use libobs_wrapper::enums::ObsLogLevel;
    use libobs_wrapper::logger::ObsLogger;
    use libobs_wrapper::utils::{
        AudioEncoderInfo, ObsPath, OutputInfo, SourceInfo, StartupInfo,
    };

    #[derive(Debug)]
    struct TestLogger;
    impl ObsLogger for TestLogger {
        fn log(&mut self, level: ObsLogLevel, msg: String) {
            println!("[{:?}] {}", level, msg);
        }
    }

    #[tokio::test]
    pub async fn record_test() {
        // Start the OBS context
        let startup_info = StartupInfo::default().set_logger(Box::new(TestLogger {}));
        let context = ObsContext::new(startup_info).await.unwrap();
        let mut context = match context {
            libobs_wrapper::context::ObsContextReturn::Done(c) => c,
            libobs_wrapper::context::ObsContextReturn::Restart => panic!("Restart not supported"),
        };

        let mut scene = context.scene("main").await.unwrap();

        // Create the video source using game capture
        let mut video_source_data = context.data().await.unwrap();
        video_source_data
            .bulk_update()
            .set_string("capture_mode", "window")
            .set_string("window", "")
            .set_bool("capture_cursor", true)
            .update()
            .await
            .unwrap();

        let video_source_info = SourceInfo::new(
            "game_capture",
            "video_source",
            Some(video_source_data),
            None,
        );

        scene.add_source(video_source_info).await.unwrap();

        // Register the source and record

        scene.set_to_channel(0).await.unwrap();

        // Set up output to ./recording.mp4
        let mut output_settings = context.data().await.unwrap();
        output_settings
            .set_string("path", ObsPath::from_relative("recording.mp4").build())
            .await
            .unwrap();

        let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);

        let mut output = context.output(output_info).await.unwrap();

        let mut video_encoder = context.best_video_encoder().await.unwrap();

        // Register the video encoder
        let mut video_settings = context.data().await.unwrap();
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
            .await
            .unwrap();

        video_encoder.set_settings(video_settings);
        video_encoder.set_to_output(&mut output, "video_encoder").await.unwrap();

        // Register the audio encoder
        let mut audio_settings = context.data().await.unwrap();
        audio_settings.set_int("bitrate", 160).await.unwrap();

        let audio_info =
            AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

        let audio_handler = context.get_audio_ptr().await.unwrap();
        output
            .audio_encoder(audio_info, 0, audio_handler)
            .await
            .unwrap();

        output.start().await.unwrap();

        println!("recording for 10 seconds...");
        thread::sleep(Duration::new(10, 0));

        // Open any fullscreen application and
        // Success!
        output.stop().await.unwrap();
    }
}
