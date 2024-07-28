use libobs::wrapper::sources::ObsSourceBuilder;
use libobs_source_macro::obs_source_builder;
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

/// Provides a easy to use builder for the window capture source.
#[derive(Debug)]
#[obs_source_builder("window_capture")]
pub struct WindowCaptureSourceBuilder {
    #[obs_property(type_t="enum")]
    priority: ObsWindowPriority,
    #[obs_property(type_t="string")]
    window: String
}



impl WindowCaptureSourceBuilder {
    /// Gets a list of windows that can be captured by this source.
    pub fn get_windows(mode: WindowSearchMode) -> anyhow::Result<Vec<WindowInfo>> {
        get_all_windows(mode, false)
    }
}
