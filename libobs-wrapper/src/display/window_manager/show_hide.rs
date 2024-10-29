use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOWNA};

use crate::display::ObsDisplay;

pub trait ShowHideTrait {
    fn show(&mut self);
    fn hide(&mut self);
    fn is_visible(&self) -> bool;
}

impl ShowHideTrait for ObsDisplay {
    fn show(&mut self) {
        unsafe {
            ShowWindow(self.manager.hwnd.0, SW_SHOWNA);
        }
        self.manager.is_hidden = false;
    }

    fn hide(&mut self) {
        unsafe {
            ShowWindow(self.manager.hwnd.0, SW_HIDE);
        }

        self.manager.is_hidden = true;
    }

    fn is_visible(&self) -> bool {
        !self.manager.is_hidden
    }
}
