//! A library for creating OBS sources without having to figure out what properties are used by sources.
//! Example usage (for window capture only on windows):
//! ```
//! use libobs::wrapper::sources::ObsSourceBuilder;
//! use libobs::wrapper::{StartupInfo, ObsContext, OutputInfo};
//! use libobs_window_helper::WindowSearchMode;
//! use libobs_sources::windows::WindowCaptureSourceBuilder;
//!
//! # // Create an obs context first
//! # // Start the OBS context
//! # let startup_info = StartupInfo::default();
//! # let mut context = ObsContext::new(startup_info).unwrap();

//! # let output_info = OutputInfo::new(
//! #     "ffmpeg_muxer", "output", None, None
//! # );
//!
//! let output = context.output(output_info).unwrap();
//!
//! // Do other initialization for video encoders, audio encoders, etc.
//!
//! let windows = WindowCaptureSourceBuilder::get_windows(WindowSearchMode::IncludeMinimized).unwrap();
//! let example_window = windows.get(0).unwrap();
//!
//! WindowCaptureSourceBuilder::new("Test Window Capture")
//! .set_window(example_window)
//! // Obs Output is created from `ObsContext`
//! .add_to_output(output, 0);
//! `````

#[cfg(target_family = "windows")]
/// Windows-specific functionality. Contains a builder for e.g. `window-capture` sources.
pub mod windows;
