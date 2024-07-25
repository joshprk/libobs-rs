use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

use crate::wrapper::{ObsData, ObsString};

use super::{ObsSourceBuilder, ObsSourceBuilderPrivate};

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the priority of the window capture source.
/// Used in `WindowCaptureSourceBuilder`
pub enum ObsWindowPriority {
    /// The window class names must be the same. This means that windows are of the same type.
    Class = crate::window_priority_WINDOW_PRIORITY_CLASS,
    /// Window titles must match otherwise, find window with the same class
    Title = crate::window_priority_WINDOW_PRIORITY_TITLE,
    /// Match title, otherwise find window with the same executable
    Executable = crate::window_priority_WINDOW_PRIORITY_EXE,
}

/// Provides a easy to use builder for the window capture source.
#[derive(Debug)]
pub struct WindowCaptureSourceBuilder {
    settings: Option<ObsData>,
    hotkeys: Option<ObsData>,
    name: ObsString,
}

//TODO: Add other information, like obs id and stuff
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub title: ObsString,
    pub class: ObsString,
    pub executable: ObsString,
}

impl ObsSourceBuilderPrivate for WindowCaptureSourceBuilder {
    fn take_settings(&mut self) -> Option<ObsData> {
        self.settings.take()
    }

    fn take_hotkeys(&mut self) -> Option<ObsData> {
        self.hotkeys.take()
    }
}

impl ObsSourceBuilder for WindowCaptureSourceBuilder {
    fn new(name: impl Into<ObsString>) -> Self {
        Self {
            settings: None,
            hotkeys: None,
            name: name.into(),
        }
    }

    fn get_id() -> ObsString {
        "window_capture".into()
    }

    fn get_settings(&self) -> &Option<ObsData> {
        &self.settings
    }

    fn get_settings_mut(&mut self) -> &mut Option<ObsData> {
        &mut self.settings
    }

    fn get_hotkeys(&self) -> &Option<ObsData> {
        &self.hotkeys
    }

    fn get_hotkeys_mut(&mut self) -> &mut Option<ObsData> {
        &mut self.hotkeys
    }

    fn get_name(&self) -> ObsString {
        self.name.clone()
    }
}

impl WindowCaptureSourceBuilder {
    pub fn set_window(mut self, window: impl Into<ObsString>) -> Self {
        self.get_or_create_settings() //
            .set_string("window", window);
        self
    }

    /// Gets a list of windows that can be captured by this source.
    pub fn get_windows() -> Vec<WindowInfo> {
        todo!()
    }

    pub fn set_priority(mut self, priority: ObsWindowPriority) -> Self {
        let priority = priority.to_i32().unwrap();

        self.get_or_create_settings()
            .set_int("priority", priority as i64);

        self
    }
}
