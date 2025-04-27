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
libobs-version = "31.0.3"
# Optional: The directory to store the OBS build
# libobs-cache-dir = "../obs-build"
```

Install OBS in your target directory:

```bash
# For debug builds
cargo obs-build target/debug

# For release builds
cargo obs-build target/release

# For testing
cargo obs-build target/(debug|release)/deps
```

### Option 2: Using the OBS Bootstrapper (Recommended for distribution)

The library includes a bootstrapper that can download and install OBS binaries at runtime, which is useful for distributing applications without requiring users to install OBS separately.

1. Add a placeholder `obs.dll` file to your executable directory that will be replaced by the bootstrapper:
   - Download a dummy DLL from [libobs-builds releases](https://github.com/sshcrack/libobs-builds/releases)
   - Use the version that matches your target OBS version
   - Rename the downloaded file to `obs.dll`

2. Enable the bootstrapper feature in your `Cargo.toml`:

```toml
[dependencies]
libobs-wrapper = { version = "0.1", features = ["bootstrapper"] }
async-trait = "0.1"  # For implementing the bootstrap status handler
indicatif = "0.17"   # Optional: For progress bars
```

3. Create a bootstrap handler to track progress and initialize OBS:

```rust
use indicatif::{ProgressBar, ProgressStyle};
use libobs_wrapper::{
    bootstrap::{
        ObsBootstrapperOptions,
        status_handler::ObsBootstrapStatusHandler,
    },
    context::ObsContext,
};
use std::{sync::Arc, time::Duration};

#[derive(Debug, Clone)]
struct ObsBootstrapProgress(Arc<ProgressBar>);

impl ObsBootstrapProgress {
    pub fn new() -> Self {
        let bar = ProgressBar::new(200).with_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap(),
        );

        bar.set_message("Initializing bootstrapper...");
        bar.enable_steady_tick(Duration::from_millis(50));

        Self(Arc::new(bar))
    }

    pub fn done(&self) {
        self.0.finish();
    }
}

#[async_trait::async_trait]
impl ObsBootstrapStatusHandler for ObsBootstrapProgress {
    async fn handle_downloading(&mut self, prog: f32, msg: String) -> anyhow::Result<()> {
        self.0.set_message(msg);
        self.0.set_position((prog * 100.0) as u64);
        Ok(())
    }
    async fn handle_extraction(&mut self, prog: f32, msg: String) -> anyhow::Result<()> {
        self.0.set_message(msg);
        self.0.set_position(100 + (prog * 100.0) as u64);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a progress handler
    let handler = ObsBootstrapProgress::new();

    // Initialize OBS with bootstrapper
    let context = ObsContext::builder()
        .enable_bootstrapper(handler.clone(), ObsBootstrapperOptions::default())
        .start()
        .await?;

    // Handle potential restart
    let context = match context {
        libobs_wrapper::context::ObsContextReturn::Done(c) => c,
        libobs_wrapper::context::ObsContextReturn::Restart => {
            println!("OBS has been downloaded and extracted. The application will now restart.");
            return Ok(());
        }
    };

    handler.done();
    println!("OBS initialized successfully!");

    // Now you can use the context for recording, etc.
    
    Ok(())
}
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

async fn record_screen() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OBS with default settings
    let handler = ObsBootstrapConsoleHandler::default();
    let context = ObsContext::builder()
        .enable_bootstrapper(handler.clone(), ObsBootstrapperOptions::default())
        .start()
        .await
        .unwrap();

    let context = match context {
        libobs_wrapper::context::ObsContextReturn::Done(c) => c,
        libobs_wrapper::context::ObsContextReturn::Restart => {
            println!("OBS has been downloaded and extracted. The application will now restart.");
            return;
        }
    };
    
    // Configure output (recording to file)
    let mut output_settings = context.data().await?;
    output_settings.set_string("path", "recording.mp4");
    
    let output_info = OutputInfo::new("ffmpeg_muxer", "recording_output", 
                                      Some(output_settings), None);
    let mut output = context.output(output_info).await?;
    
    // Configure video encoder
    let mut video_settings = context.data().await?;
    video_settings.set_int("bitrate", 6000);
    video_settings.set_string("rate_control", "CBR");
    video_settings.set_string("preset", "medium");
    
    // Get video handler and attach encoder to output
    let video_handler = context.get_video_ptr().await?;
    output.video_encoder(
        VideoEncoderInfo::new(ObsVideoEncoderType::OBS_X264, "video_encoder", 
                              Some(video_settings), None),
        video_handler
    ).await?;
    
    // Configure audio encoder
    let mut audio_settings = context.data().await?;
    audio_settings.set_int("bitrate", 160);
    
    // Get audio handler and attach encoder to output
    let audio_handler = context.get_audio_ptr().await?;
    output.audio_encoder(
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", 
                              Some(audio_settings), None),
        0, audio_handler
    ).await?;
    
    // Create a scene and add a monitor capture source
    let mut scene = context.scene("recording_scene").await?;
    
    // Get available monitors and set up capture for the first one
    let monitors = MonitorCaptureSourceBuilder::get_monitors()?;
    if !monitors.is_empty() {
        let monitor = &monitors[0];
        let capture_source = MonitorCaptureSourceBuilder::new("screen_capture")
            .set_monitor(monitor)
            .add_to_scene(&mut scene)
            .await?;
            
        // Set the scene as active and start recording
        scene.add_and_set(0).await?;
        output.start().await?;
        
        println!("Recording started");
        std::thread::sleep(Duration::from_secs(10));
        println!("Recording stopped");
        
        output.stop().await?;
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

- `blocking` - Provides a blocking API instead of async (useful for applications that don't need async)
- `bootstrapper` - Enables the OBS bootstrapper for runtime download and installation
- `debug-tracing` - Enables additional debug tracing for libobs calls

## Common Issues

### Missing DLLs or Crashes on Startup

If you're experiencing crashes or missing DLL errors:
1. Make sure OBS binaries are correctly installed using either cargo-obs-build or the bootstrapper
2. Check that you're using the correct OBS version compatible with this wrapper
3. Verify that all required DLLs are in your executable directory

### Memory Leaks

The library handles most memory management automatically, but you should:
1. Properly drop or release OBS resources when done with them
2. Avoid resetting the OBS context repeatedly as this can cause small memory leaks (due to an OBS limitation)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The OBS Project for the amazing OBS Studio software
- Contributors to the libobs-rs ecosystem