# libOBS Sources
This crate makes it really easy to create new sources for OBS Studio using the [libobs-wrapper](https://crates.io/crates/libobs-wrapper) crate.

(This example is outdated, look at tests and examples for more up-to-date code)
## Example
```rust
fn main() {
    let rec_file = ObsPath::from_relative("monitor_capture.mp4").build();
    let path_out = PathBuf::from(rec_file.to_string());

    // Start the OBS context
    let startup_info = StartupInfo::default();

    // You can also create use a logger to log to a file
    // let _l = DebugLogger { f: File::create(current_dir().unwrap().join("obs.log")).unwrap() };
    // startup_info = startup_info.set_logger(Box::new(_l));

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
    let mut scene = context.scene("main");

    let monitor = MonitorCaptureSourceBuilder::get_monitors().unwrap()[1].clone();
    println!("Using monitor {:?}", monitor);
    let mut capture_source = MonitorCaptureSourceBuilder::new("monitor_test")
        .set_monitor(&monitor)
        .add_to_scene(&mut scene)
        .unwrap();

    scene.add_and_set(0);
    output.start().unwrap();

    println!("Recording started");
    std::thread::sleep(Duration::from_secs(5));
    println!("Recording stop");

    output.stop().unwrap();
    // And now your monitor is recorded to ./recording.mp4 !
}
```