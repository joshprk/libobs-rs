# libobs-rs

Simple and safe video recording through libobs.

Currently only tested on Windows. Linux and MacOS likely work, but they are untested. These will receive support later down the road.

The API is currently unstable and will definitely have breaking revisions in the future.

To build on Linux, you must install the libobs-dev package, as well as the bindgen dependencies.
```
sudo apt-get libobs-dev llvm-dev libclang-dev clang
```

Compiled Windows DLLs for libobs can be found at https://github.com/joshprk/libobs-rs/releases/tag/deps

## Quick Start

```rs
use std::time::Duration;
use std::thread;

use libobs::wrapper::{
    StartupInfo, ObsContext, OutputInfo, ObsData, VideoEncoderInfo, 
    AudioEncoderInfo, SourceInfo, ObsPath
};

pub fn main() {
    // Start the OBS context
    let startup_info = StartupInfo::default();
    let context = ObsContext::new(startup_info).unwrap();

    // Set up output to ./recording.mp4
    let output_settings = ObsData::new();
    output_settings
        .set_string("path", ObsPath::from_relative("recording.mp4").build());

    let output_info = OutputInfo::new(
        "ffmpeg_muxer", "output", Some(output_settings), None
    );

    let Ok(output) = context.output(output_info).unwrap();

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
        Some(video_enc_settings),
        None,
    );

    let video_handler = ObsContext::get_video_ptr().unwrap();
    output.video_encoder(video_info, video_handler);
    
    // Register the audio encoder
    let mut audio_settings = ObsData::new();
    audio_settings.set_int("bitrate", 160);

    let audio_info = AudioEncoderInfo::new(
        "ffmpeg_aac", 
        "audio_encoder", 
        Some(audio_enc_settings), 
        None
    );

    let audio_handler = ObsContext::get_audio_ptr().unwrap()
    output.audio_encoder(audio_info, 0, audio_handler);

    // Create the video source using game capture
    let video_source_data = ObsData::new();
    video_source_data
        .set_string("capture_mode", "window")
        .set_string("window", window_name)
        .set_bool("capture_cursor", true);
        
    let video_source_info = SourceInfo::new(
        "game_capture", 
        "video_source", 
        Some(video_source_data), 
        None
    );

    // Register the source and record
    output.source(video_source_info, 0);
    output.start();

    println!("recording for 10 seconds...");
    thread::sleep(Duration::new(10, 0));

    // Success!
    output.stop();
}
```