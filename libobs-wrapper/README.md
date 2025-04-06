# LibOBS Wrapper

This is a simple wrapper for the `libobs-new` crate.

## Prerequisites
Make sure that the OBS binaries are in your target directory. There's even a tool to help you build OBS from source! <br>
Install the tool
```bash
cargo install cargo-obs-build
```

Add the following to your `Cargo.toml`
```toml
[package.metadata]
# The libobs version to use (can either be a specific version or "latest")
libobs-version="31.0.3"
# The directory in which to store the OBS build (optional)
# libobs-cache-dir="../obs-build"

```

Install OBS in your target directory
```bash
# for debugging
cargo obs-build target/debug
# for release
cargo obs-build target/release
# for testing
cargo obs-build target/(debug|release)/deps
```

## OBS Bootstrapper

The library includes a bootstrapper that can download and install OBS binaries at runtime, which is useful for distributing applications without requiring users to install OBS separately.

### Usage

1. Add a dummy `obs.dll` file to your executable directory. This file will be replaced by the bootstrapper.
   - You can download a dummy DLL from [https://github.com/sshcrack/libobs-builds/releases](https://github.com/sshcrack/libobs-builds/releases)
   - Make sure to use the version that matches your target OBS version
   - Rename the downloaded file to `obs.dll`

2. Initialize the bootstrapper at application startup:

```rust
use libobs_wrapper::context::ObsContext;
use libobs_wrapper::bootstrap::{ObsBootstrap, ObsDownloaderOptions, BootstrapStatus};
use futures_util::StreamExt;

async fn bootstrap() -> anyhow::Result<()> {
    // Configure the download options
    let options = ObsDownloaderOptions::default();
    
    // Start the bootstrap process
    let bootstrap_stream = ObsContext::bootstrap(options).await?;
    pin_mut!(bootstrap_stream);
    
    // Process bootstrap status updates
    while let Some(status) = bootstrap_stream.next().await {
        match status {
            BootstrapStatus::Downloading(progress, message) => {
                println!("Downloading: {}% - {}", progress * 100.0, message);
            },
            BootstrapStatus::Extracting(progress, message) => {
                println!("Extracting: {}% - {}", progress * 100.0, message);
            },
            BootstrapStatus::Error(err) => {
                return Err(err);
            },
            BootstrapStatus::RestartRequired => {
                println!("OBS has been updated. Application needs to restart.");
                // Spawn the updater and exit
                ObsContext::spawn_updater().await?;
                std::process::exit(0);
            }
        }
    }

    // You can use ObsContext now

    Ok(())
}
```

3. When the bootstrapper returns `BootstrapStatus::RestartRequired`, call `ObsContext::spawn_updater()` and exit your application. The updater will:
   - Wait for your application to exit
   - Replace `obs.dll` with the newly downloaded version
   - Restart your application with the same command-line arguments

## Usage

Note: This is the usage without using the [libobs-sources](https://crates.io/crates/libobs-sources) crate (which makes it significantly easier to create sources)
```rust
use std::thread;
use std::time::Duration;

use libobs_wrapper::context::ObsContext;
use libobs_wrapper::data::ObsData;
use libobs_wrapper::utils::{
    AudioEncoderInfo, ObsPath, OutputInfo, SourceInfo, StartupInfo, VideoEncoderInfo,
};

pub fn main() {
    // Start the OBS context
    let startup_info = StartupInfo::default();
    let mut context = ObsContext::new(startup_info).unwrap();

    // Set up output to ./recording.mp4
    let mut output_settings = ObsData::new();
    output_settings.set_string("path", ObsPath::from_relative("recording.mp4").build());

    let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);

    let output = context.output(output_info).unwrap();

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

    // Create the video source using game capture
    let mut video_source_data = ObsData::new();
    video_source_data
        .set_string("capture_mode", "window")
        .set_string("window", "")
        .set_bool("capture_cursor", true);

    let video_source_info = SourceInfo::new(
        "game_capture",
        "video_source",
        Some(video_source_data),
        None,
    );

    // Register the source and record
    output.source(video_source_info, 0).unwrap();
    output.start().unwrap();

    println!("recording for 10 seconds...");
    thread::sleep(Duration::new(10, 0));

    // Open any fullscreen application and
    // Success!
    output.stop().unwrap();
}
```
