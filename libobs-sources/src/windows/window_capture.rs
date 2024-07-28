use libobs_source_macro::obs_source_builder;

#[cfg(feature="window-list")]
use libobs_window_helper::{get_all_windows, WindowInfo, WindowSearchMode};
use num_derive::{FromPrimitive, ToPrimitive};

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the priority of the window capture source.
/// Used in `WindowCaptureSourceBuilder`
pub enum ObsWindowPriority {
    /// The window class names must be the same. This means that windows are of the same type.
    Class = libobs::window_priority_WINDOW_PRIORITY_CLASS,
    /// Window titles must match otherwise, find window with the same class
    Title = libobs::window_priority_WINDOW_PRIORITY_TITLE,
    /// Match title, otherwise find window with the same executable
    Executable = libobs::window_priority_WINDOW_PRIORITY_EXE,
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the priority of the window capture source.
/// Used in `WindowCaptureSourceBuilder`
pub enum ObsWindowCaptureMethod {
	/// Automatically selects the best method based on the window.
    MethodAuto,
    /// Uses BitBlt to capture the window. BitBlt (Windows 7 and up)
	MethodBitBlt,
    /// Uses Windows Graphics Capture to capture the window. Windows 10 (1903 and up)
	MethodWgc,
}

/// Provides a easy to use builder for the window capture source.
#[derive(Debug)]
#[obs_source_builder("window_capture")]
pub struct WindowCaptureSourceBuilder {
    #[obs_property(type_t="enum")]
    /// Sets the capture method for the window capture
    /// - `BitBlt` - Uses BitBlt to capture the window. This is the fastest method, but may not work with all windows.
    /// - `GDI` - Uses GDI to capture the window. This is slower than BitBlt, but works with more windows.
    /// - `DWM` - Uses the Desktop Window Manager to capture the window. This is the slowest method, but works with all windows.
    /// - `Auto` - Automatically selects the best method based on the window.
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
    #[obs_property(type_t = "string")]
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
    compatibility: bool
}

#[cfg(feature="window-list")]
impl WindowCaptureSourceBuilder {
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
