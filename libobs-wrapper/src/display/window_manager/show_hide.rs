use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOWNA};

use crate::display::ObsDisplayRef;

pub trait ShowHideTrait {
    fn show(&mut self);
    fn hide(&mut self);
    fn is_visible(&self) -> bool;
}

impl ShowHideTrait for ObsDisplayRef {
    fn show(&mut self) {
        unsafe {
            let _ = ShowWindow(self.manager.borrow().hwnd.0, SW_SHOWNA);
        }
        self.manager.borrow_mut().is_hidden = false;
    }

    fn hide(&mut self) {
        unsafe {
            let _ = ShowWindow(self.manager.borrow().hwnd.0, SW_HIDE);
        }

        self.manager.borrow_mut().is_hidden = true;
    }

    fn is_visible(&self) -> bool {
        !self.manager.borrow().is_hidden
    }
}
