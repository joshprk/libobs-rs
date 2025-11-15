//! macOS-specific OBS sources
//!
//! Provides safe Rust bindings for macOS capture sources.
//! These wrap OBS's existing mac-capture plugin which handles
//! the actual capture implementation using ScreenCaptureKit.
//!
//! Available sources:
//! - screen_capture - Entire screen/monitor capture
//! - display_capture - Display capture  
//! - window_capture - Individual window capture
//! - Audio capture devices

pub mod sources;

pub use sources::*;
