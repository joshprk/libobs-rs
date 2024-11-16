use std::sync::atomic::Ordering;

use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOWNA};

use crate::display::ObsDisplayRef;

pub trait ShowHideTrait {
    fn show(&mut self);
    fn hide(&mut self);
    fn is_visible(&self) -> bool;
}

impl ShowHideTrait for ObsDisplayRef {
    fn show(&mut self) {
        log::trace!("show");
        let m = self.manager.read();
        unsafe {
            let _ = ShowWindow(m.hwnd.0, SW_SHOWNA);
        }

        m.is_hidden.store(false, Ordering::Relaxed);
    }

    fn hide(&mut self) {
        log::trace!("hide");
        let m  = self.manager.read();
        unsafe {
            let _ = ShowWindow(m.hwnd.0, SW_HIDE);
        }

        m.is_hidden.store(true, Ordering::Relaxed);
    }

    fn is_visible(&self) -> bool {
        self.manager.read().is_hidden.load(Ordering::Relaxed)
    }
}
