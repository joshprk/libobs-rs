use std::sync::atomic::Ordering;

use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOWNA};

use crate::display::ObsDisplayRef;

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
pub trait ShowHideTrait {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn show(&mut self);
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn hide(&mut self);
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn is_visible(&self) -> bool;
}

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl ShowHideTrait for ObsDisplayRef {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn show(&mut self) {
        log::trace!("show");
        let m = self.manager.read().await;
        unsafe {
            let _ = ShowWindow(m.hwnd.0, SW_SHOWNA);
        }

        m.is_hidden.store(false, Ordering::Relaxed);
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn hide(&mut self) {
        log::trace!("hide");
        let m = self.manager.read().await;
        unsafe {
            let _ = ShowWindow(m.hwnd.0, SW_HIDE);
        }

        m.is_hidden.store(true, Ordering::Relaxed);
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn is_visible(&self) -> bool {
        self.manager.read().await.is_hidden.load(Ordering::Relaxed)
    }
}
