use libobs_source_macro::obs_object_impl;
#[cfg(feature = "window-list")]
use libobs_window_helper::{get_all_windows, WindowInfo, WindowSearchMode};
use libobs_wrapper::sources::{ObsSource, ObsSourceBuilder};

use crate::macro_helper::define_object_manager;

use super::{ObsWindowCaptureMethod, ObsWindowPriority};

define_object_manager!(
    /// Provides a easy to use builder for the window capture source.
    #[derive(Debug)]
    struct WindowCaptureSource("window_capture") for ObsSource {
    #[obs_property(type_t = "enum")]
    /// Sets the capture method for the window capture
    capture_method: ObsWindowCaptureMethod,

    /// Sets the priority of the window capture source.
    /// Used to determine in which order windows are searched for.
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority of the window capture source.
    ///
    /// # Returns
    ///
    /// The updated `WindowCaptureSourceBuilder` instance.
    #[obs_property(type_t = "enum")]
    priority: ObsWindowPriority,

    /// Sets the window to capture.
    ///
    /// # Arguments
    ///
    /// * `window` - The window to capture, represented as `ObsString`. Must be in the format of an obs window id
    ///
    /// # Returns
    ///
    /// The updated `WindowCaptureSourceBuilder` instance.
    #[obs_property(type_t = "string", settings_key = "window")]
    window_raw: String,

    #[obs_property(type_t = "bool")]
    /// Sets whether the cursor should be captured
    cursor: bool,

    #[obs_property(type_t = "bool")]
    /// Whether to capture audio from window source (BETA) <br>
    /// When enabled, creates an "Application Audio Capture" source that automatically updates to the currently captured window/application. <br>
    /// Note that if Desktop Audio is configured, this could result in doubled audio.
    capture_audio: bool,

    #[obs_property(type_t = "bool")]
    /// Whether to force SDR color space for the window capture source.
    force_sdr: bool,

    #[obs_property(type_t = "bool")]
    /// Whether to capture the window's client area only (without borders, title bar and the main menu bar).
    client_area: bool,

    #[obs_property(type_t = "bool")]
    compatibility: bool,
});

#[obs_object_impl]
#[cfg(feature = "window-list")]
impl WindowCaptureSource {
    /// Gets a list of windows that can be captured by this source.
    pub fn get_windows(mode: WindowSearchMode) -> anyhow::Result<Vec<WindowInfo>> {
        get_all_windows(mode, false)
    }

    /// Sets the window to capture.
    ///
    /// # Arguments
    ///
    /// * `window` - The window to capture. A list of available windows can be retrieved using `WindowCaptureSourceBuilder::get_windows`
    ///
    /// # Returns
    ///
    /// The updated `WindowCaptureSourceBuilder` instance.
    pub fn set_window(self, window: &WindowInfo) -> Self {
        self.set_window_raw(window.obs_id.as_str())
    }
}

impl ObsSourceBuilder for WindowCaptureSourceBuilder {}