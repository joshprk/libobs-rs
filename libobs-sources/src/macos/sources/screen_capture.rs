//! Screen capture source for macOS
//! 
//! Bindings to OBS's `screen_capture` source which internally uses ScreenCaptureKit.
//! This source captures the entire screen or a specific display.

use libobs_source_macro::obs_object_impl;
use libobs_wrapper::sources::ObsSourceRef;

use crate::macro_helper::define_object_manager;

define_object_manager!(
    /// Builder for the screen capture source on macOS.
    /// Captures the entire screen or a specific display.
    #[derive(Debug)]
    struct ScreenCaptureSource("screen_capture") for ObsSourceRef {
        #[obs_property(type_t = "int", settings_key = "display")]
        /// The display ID to capture (0 for main display)
        display: i64,

        #[obs_property(type_t = "bool")]
        /// Whether to show the cursor in the capture
        show_cursor: bool,

        #[obs_property(type_t = "bool")]
        /// Whether to capture audio (macOS 13+)
        audio_capture: bool,
    }
);

#[obs_object_impl]
impl ScreenCaptureSource {
    // Helper methods can be added here
}

