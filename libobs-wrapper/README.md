# libobs-wrapper

[![Crates.io](https://img.shields.io/crates/v/libobs-wrapper.svg)](https://crates.io/crates/libobs-wrapper)
[![Documentation](https://docs.rs/libobs-wrapper/badge.svg)](https://docs.rs/libobs-wrapper)

A safe, ergonomic Rust wrapper around the OBS (Open Broadcaster Software) Studio library. This crate provides a high-level interface for recording and streaming functionality using OBS's powerful capabilities, without having to deal with unsafe C/C++ code directly.

## Features

- **Thread Safety**: Uses a dedicated thread to communicate with OBS, allowing safe cross-thread usage
- **Async API**: Full async support with optional blocking API (via `blocking` feature)
- **Resource Safety**: RAII-based resource management for OBS objects
- **Runtime Bootstrapping**: Optional automatic download and setup of OBS binaries at runtime
- **Scene Management**: Create and manipulate scenes, sources, and outputs
- **Video Recording**: Configure and record video with various encoders
- **Audio Support**: Configure audio sources and encoders
- **Display Management**: Create and control OBS displays

## Prerequisites

The library needs OBS binaries in your target directory. There are multiple ways to set this up:

### Option 1: Using cargo-obs-build (Recommended for development)

Install the `cargo-obs-build` tool:

```bash
cargo install cargo-obs-build
```

Add the following to your `Cargo.toml`:

```toml
[package.metadata]
# The libobs version to use (can either be a specific version or "latest")
# If not specified, the version will be selected based on the libobs crate version.
# libobs-version = "31.0.3"
# Optional: The directory to store the OBS build
# libobs-cache-dir = "../obs-build"
```

Install OBS in your target directory:

```bash
# For debug builds
cargo obs-build --out-dir target/debug

# For release builds
cargo obs-build --out-dir target/release

# For testing
cargo obs-build --out-dir target/(debug|release)/deps
```

> [!NOTE]
> You can specify a `GITHUB_TOKEN` environment variable to increase the rate limit when downloading releases from GitHub. This is especially useful for CI environments.

### Option 2: Using the OBS Bootstrapper (Recommended for distribution)

For applications that need to bundle OBS binaries or handle runtime installation, we recommend using the [`libobs-bootstrapper`](https://crates.io/crates/libobs-bootstrapper) crate. This separate crate provides functionality to download and install OBS binaries at runtime, which is particularly useful for distributing applications without requiring users to install OBS separately.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
libobs-wrapper = "4.0.1"
libobs-bootstrapper = "0.1.0"
async-trait = "0.1"  # For implementing the bootstrap status handler
```

See the [libobs-bootstrapper documentation](https://docs.rs/libobs-bootstrapper) for detailed setup instructions and examples of implementing custom progress handlers.
```

With this approach, the bootstrapper will:
- Download OBS binaries if needed, showing progress in a nice progress bar
- Extract the files, continuing to show progress
- Return a restart signal if necessary
- Otherwise provide an initialized OBS context ready to use

If the bootstrapper returns `ObsContextReturn::Restart`, your application should exit and will be automatically restarted with the updated binaries.

## Basic Usage

Here's a simple example of recording a monitor screen (assuming the `bootstrapper` feature is enabled):

```rust
use libobs_wrapper::{
    context::ObsContext,
    data::ObsData,
    utils::{StartupInfo, OutputInfo},
    sources::MonitorCaptureSourceBuilder,
    data::video::ObsVideoInfoBuilder,
    encoders::video::ObsVideoEncoder,
    encoders::audio::ObsAudioEncoder,
};
use std::time::Duration;

fn record_screen() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OBS with default settings
    use libobs_bootstrapper::{ObsBootstrapper, ObsBootstrapperOptions, ObsBootstrapConsoleHandler};
    
    match ObsBootstrapper::bootstrap(&ObsBootstrapperOptions::default()).await? {
        ObsBootstrapperResult::None => (),
        ObsBootstrapperResult::Restart => {
            println!("OBS has been downloaded and extracted. The application will now restart.");
            ObsBootstrapper::spawn_updater(options).await?;
            std::process::exit(0);
        }
    }
    
    let context = ObsContext::new()?;
    
    // Configure output (recording to file)
    let mut output_settings = context.data()?;
    output_settings.set_string("path", "recording.mp4");
    
    let output_info = OutputInfo::new("ffmpeg_muxer", "recording_output", 
                                      Some(output_settings), None);
    let mut output = context.output(output_info)?;
    
    // Configure video encoder
    let mut video_settings = context.data()?;
    video_settings.set_int("bitrate", 6000);
    video_settings.set_string("rate_control", "CBR");
    video_settings.set_string("preset", "medium");
    
    // Get video handler and attach encoder to output
    let video_handler = context.get_video_ptr()?;
    output.video_encoder(
        VideoEncoderInfo::new(ObsVideoEncoderType::OBS_X264, "video_encoder", 
                              Some(video_settings), None),
        video_handler
    )?;
    
    // Configure audio encoder
    let mut audio_settings = context.data()?;
    audio_settings.set_int("bitrate", 160);
    
    // Get audio handler and attach encoder to output
    let audio_handler = context.get_audio_ptr()?;
    output.audio_encoder(
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", 
                              Some(audio_settings), None),
        0, audio_handler
    )?;
    
    // Create a scene and add a monitor capture source
    let mut scene = context.scene("recording_scene")?;
    
    // Get available monitors and set up capture for the first one
    let monitors = MonitorCaptureSourceBuilder::get_monitors()?;
    if !monitors.is_empty() {
        let monitor = &monitors[0];
        let capture_source = MonitorCaptureSourceBuilder::new("screen_capture")
            .set_monitor(monitor)
            .add_to_scene(&mut scene)?;
            
        // Set the scene as active and start recording
        scene.add_and_set(0)?;
        output.start()?;
        
        println!("Recording started");
        std::thread::sleep(Duration::from_secs(10));
        println!("Recording stopped");
        
        output.stop()?;
    } else {
        println!("No monitors found");
    }
    
    Ok(())
}
```

## Advanced Usage

For more advanced usage examples, check out:
- Monitor capture example with full configuration: [examples/monitor_capture.rs](../examples/monitor_capture.rs)
- Tauri integration example: [examples/tauri-app](../examples/tauri-app)
- Runtime bootstrapping example: [examples/download-at-runtime](../examples/download-at-runtime)

For even easier source creation and management, consider using the [`libobs-sources`](https://crates.io/crates/libobs-sources) crate which builds on top of this wrapper.

## Features
- `no_blocking_drops` - Spawns a tokio thread using `tokio::task::spawn_blocking`, so drops don't block your Application (experimental, make sure you have a tokio runtime running)
- `generate_bindings` - When enabled, forces the underlying bindings from `libobs` to generate instead of using the cached ones.
- `color-logger` - Enables coloring for the console
- `dialog_crash_handler` - Adds a default crash handler, which shows the error and an option to copy the stacktrace to the clipboard

## Common Issues

### Missing DLLs or Crashes on Startup

If you're experiencing crashes or missing DLL errors:
1. Make sure OBS binaries are correctly installed using either cargo-obs-build or the bootstrapper
2. Check that you're using the correct OBS version compatible with this wrapper
3. Verify that all required DLLs are in your executable directory

### Memory Leaks

The library handles most memory management automatically, but you should avoid resetting the OBS context repeatedly as this can cause small memory leaks (due to an OBS limitation). There is `1` memory leak caused by `obs_add_data_path` (which is called internally from this lib). Unfortunately, this memory leak can not be fixed because of how OBS internally works.

## License

This project is licensed under the GPL-3.0 License - see the LICENSE file for details.

## Acknowledgments

- The OBS Project for the amazing OBS Studio software
- Contributors to the libobs-rs ecosystem