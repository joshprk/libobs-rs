//! A library for creating OBS sources without having to figure out what properties are used by sources.
//! Example usage:
//! ```
//! use libobs::wrapper::sources::ObsSourceBuilder;
//! use libobs_window_helper::WindowSearchMode;
//! use windows::WindowCaptureSourceBuilder;
//!
//! let windows = WindowCaptureSourceBuilder::get_windows(WindowSearchMode::IncludeMinimized).unwrap();
//! let example_window = windows.get(0).unwrap();
//!
//! WindowCaptureSourceBuilder::new("Test Window Capture")
//! .set_window(example_window)
//! // Obs Output is created from `ObsContext`
//! .add_to_output(obs_output, 0);
//! `````



#[cfg(target_family="windows")]
/// Windows-specific functionality. Contains a builder for e.g. `window-capture` sources.
pub mod windows;
