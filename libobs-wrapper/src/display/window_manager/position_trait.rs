use async_trait::async_trait;
use windows::Win32::{
    Foundation::HWND,
    Graphics::Gdi::{RedrawWindow, RDW_ERASE, RDW_INVALIDATE},
    UI::WindowsAndMessaging::{
        SetWindowPos, HWND_BOTTOM, SWP_NOACTIVATE, SWP_NOCOPYBITS, SWP_NOSIZE, SWP_NOZORDER,
        SWP_SHOWWINDOW,
    },
};

use crate::{display::ObsDisplayRef, run_with_obs};

#[async_trait]
pub trait WindowPositionTrait {
    async fn set_render_at_bottom(&self, render_at_bottom: bool);
    async fn get_render_at_bottom(&self) -> bool;
    async fn set_pos(&self, x: i32, y: i32) -> windows::core::Result<()>;
    async fn set_size(&self, width: u32, height: u32) -> anyhow::Result<()>;
    async fn set_scale(&self, scale: f32);

    async fn get_pos(&self) -> (i32, i32);
    fn get_pos_blocking(&self) -> (i32, i32);

    async fn get_size(&self) -> (u32, u32);
    fn get_size_blocking(&self) -> (u32, u32);

    async fn get_scale(&self) -> f32;
}

#[async_trait]
impl WindowPositionTrait for ObsDisplayRef {
    async fn set_render_at_bottom(&self, render_at_bottom: bool) {
        log::trace!("Set render bottom");
        self.manager.write().await.render_at_bottom = render_at_bottom;
    }

    async fn get_render_at_bottom(&self) -> bool {
        self.manager.read().await.render_at_bottom
    }

    async fn set_pos(&self, x: i32, y: i32) -> windows::core::Result<()> {
        log::trace!("Set pos {x} {y}");
        let mut m = self.manager.write().await;

        assert!(
            m.obs_display.is_some(),
            "Invalid state. The display should have been created and set, but it wasn't."
        );

        let insert_after = if m.render_at_bottom {
            HWND_BOTTOM
        } else {
            HWND::default()
        };

        m.x = x;
        m.y = y;

        unsafe {
            let flags = SWP_NOCOPYBITS | SWP_NOSIZE | SWP_NOACTIVATE;
            // Just use dummy values as size is not changed
            SetWindowPos(
                m.hwnd.0,
                Some(insert_after),
                x,
                y,
                1 as i32,
                1 as i32,
                flags,
            )?;
        }

        Ok(())
    }

    async fn get_pos(&self) -> (i32, i32) {
        let m = self.manager.read().await;
        (m.x, m.y)
    }
    fn get_pos_blocking(&self) -> (i32, i32) {
        let m = self.manager.blocking_read();
        (m.x, m.y)
    }

    async fn get_size(&self) -> (u32, u32) {
        let m = self.manager.read().await;
        (m.width, m.height)
    }

    fn get_size_blocking(&self) -> (u32, u32) {
        let m = self.manager.blocking_read();
        (m.width, m.height)
    }

    async fn set_size(&self, width: u32, height: u32) -> anyhow::Result<()> {
        log::trace!("Set size {width} {height}");
        let mut m = self.manager.write().await;
        assert!(
            m.obs_display.is_some(),
            "Invalid state. The display should have been created and set, but it wasn't."
        );

        m.width = width;
        m.height = height;

        let pointer = m.obs_display.as_ref().unwrap().clone();
        unsafe {
            SetWindowPos(
                m.hwnd.0,
                None,
                m.x,
                m.y,
                width as i32,
                height as i32,
                SWP_NOCOPYBITS | SWP_NOACTIVATE | SWP_NOZORDER | SWP_SHOWWINDOW,
            )?;

            let _ = RedrawWindow(Some(m.hwnd.0), None, None, RDW_ERASE | RDW_INVALIDATE);
        }

        run_with_obs!(self.runtime, (pointer), move || unsafe {
            libobs::obs_display_resize(pointer, width, height);
        })?;
        Ok(())
    }

    async fn set_scale(&self, scale: f32) {
        log::trace!("Set scale {scale}");
        self.manager.write().await.scale = scale;
    }

    async fn get_scale(&self) -> f32 {
        self.manager.read().await.scale
    }
}
