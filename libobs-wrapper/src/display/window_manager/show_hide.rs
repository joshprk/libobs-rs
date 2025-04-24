use std::sync::atomic::Ordering;

use async_trait::async_trait;
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOWNA};

use crate::display::ObsDisplayRef;

#[async_trait]
pub trait ShowHideTrait {
    async fn show(&mut self);
    async fn hide(&mut self);
    async fn is_visible(&self) -> bool;
}

#[async_trait]
impl ShowHideTrait for ObsDisplayRef {
    async fn show(&mut self) {
        log::trace!("show");
        let m = self.manager.read().await;
        unsafe {
            let _ = ShowWindow(m.hwnd.0, SW_SHOWNA);
        }

        m.is_hidden.store(false, Ordering::Relaxed);
    }

    async fn hide(&mut self) {
        log::trace!("hide");
        let m  = self.manager.read().await;
        unsafe {
            let _ = ShowWindow(m.hwnd.0, SW_HIDE);
        }

        m.is_hidden.store(true, Ordering::Relaxed);
    }

    async fn is_visible(&self) -> bool {
        self.manager.read().await.is_hidden.load(Ordering::Relaxed)
    }
}
