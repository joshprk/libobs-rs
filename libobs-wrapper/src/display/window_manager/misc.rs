use crate::{display::ObsDisplayRef, run_with_obs, utils::ObsError};

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
pub trait MiscDisplayTrait {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn update_color_space(&self) -> Result<(), ObsError>;

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn is_enabled(&self) -> Result<bool, ObsError>;

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn set_enabled(&self, enabled: bool) -> Result<(), ObsError>;

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn set_background_color(&self, r: u8, g: u8, b: u8) -> Result<(), ObsError>;
}

#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl MiscDisplayTrait for ObsDisplayRef {
    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn update_color_space(&self) -> Result<(), ObsError> {
        let display_ptr = self.display.clone();
        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_update_color_space(display_ptr)
        }).await
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn is_enabled(&self) -> Result<bool, ObsError> {
        let display_ptr = self.display.clone();
        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_enabled(display_ptr)
        }).await
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn set_enabled(&self, enabled: bool) -> Result<(), ObsError> {
        let display_ptr = self.display.clone();

        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_set_enabled(display_ptr, enabled)
        }).await
    }

    #[cfg_attr(feature = "blocking", remove_async_await::remove_async_await)]
    async fn set_background_color(&self, r: u8, g: u8, b: u8) -> Result<(), ObsError> {
        let color: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        let display_ptr = self.display.clone();

        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_set_background_color(display_ptr, color)
        }).await
    }
}
