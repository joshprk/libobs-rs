use std::fmt::Debug;

#[cfg(windows)]
use super::windows::DisplayWindowManager;
use crate::{display::ObsWindowHandle, unsafe_send::Sendable, utils::ObsError};

pub trait MiscDisplayTrait {
    fn update_color_space(&self) -> Result<(), ObsError>;

    fn is_enabled(&self) -> Result<bool, ObsError>;

    fn set_enabled(&self, enabled: bool) -> Result<(), ObsError>;

    fn set_background_color(&self, r: u8, g: u8, b: u8) -> Result<(), ObsError>;
}

pub trait WindowPositionTrait {
    fn set_render_at_bottom(&self, render_at_bottom: bool) -> Result<(), ObsError>;
    fn get_render_at_bottom(&self) -> Result<bool, ObsError>;
    fn set_pos(&self, x: i32, y: i32) -> Result<(), ObsError>;
    fn set_size(&self, width: u32, height: u32) -> Result<(), ObsError>;
    fn set_scale(&self, scale: f32) -> Result<(), ObsError>;

    fn get_pos(&self) -> Result<(i32, i32), ObsError>;

    fn get_size(&self) -> Result<(u32, u32), ObsError>;

    fn get_scale(&self) -> Result<f32, ObsError>;
}

pub trait ShowHideTrait {
    /// Shows the window.
    fn show(&mut self) -> Result<(), ObsError>;

    /// Hides the window.
    fn hide(&mut self) -> Result<(), ObsError>;

    /// Returns true if the window is visible.
    fn is_visible(&self) -> Result<bool, ObsError>;
}


pub(crate) trait PrivateSetDisplayHandle {
    fn set_display_handle(&mut self, handle: Sendable<*mut libobs::obs_display>);
}

pub trait GeneralDisplayWindowManager:
    PrivateSetDisplayHandle + MiscDisplayTrait + WindowPositionTrait + ShowHideTrait + Debug + Send + Sync
{
}


pub(crate) fn new_general_window_manager_from_child(
    parent: ObsWindowHandle,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Result<(Box<dyn GeneralDisplayWindowManager>, Option<ObsWindowHandle>), ObsError>
{
    #[cfg(windows)]
    let mgr = super::windows::DisplayWindowManager::new_child(parent, x, y, width, height)?;

    #[cfg(target_os = "linux")]
    let mgr = super::linux::DisplayWindowManager::new(parent, x, y, width, height)?;

    #[cfg(target_os="linux")]
    return Ok((Box::new(mgr), None));

    #[cfg(windows)]
    return Ok((Box::new(mgr), Some(ObsWindowHandle(mgr.get_window_handle()))));
}

pub(crate) fn new_general_window_manager(
    window_handle: ObsWindowHandle,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Result<Box<dyn GeneralDisplayWindowManager>, ObsError>
{
    #[cfg(windows)]
    let mgr = super::windows::DisplayWindowManager::new(window_handle, x, y, width, height)?;

    #[cfg(target_os = "linux")]
    let mgr = super::linux::DisplayWindowManager::new(window_handle, x, y, width, height)?;

    return Ok(Box::new(mgr));
}
