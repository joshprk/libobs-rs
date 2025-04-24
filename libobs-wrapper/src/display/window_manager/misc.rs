use async_trait::async_trait;

use crate::{display::ObsDisplayRef, run_with_obs, utils::ObsError};

#[async_trait]
pub trait MiscDisplayTrait {
    async fn update_color_space(&self) -> Result<(), ObsError>;
    async fn is_enabled(&self) -> Result<bool, ObsError>;
    async fn set_enabled(&self, enabled: bool) -> Result<(), ObsError>;
    async fn set_background_color(&self, r: u8, g: u8, b: u8) -> Result<(), ObsError>;
}

#[async_trait]
impl MiscDisplayTrait for ObsDisplayRef {
    async fn update_color_space(&self) -> Result<(), ObsError> {
        let display_ptr = self.display.clone();
        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_update_color_space(display_ptr)
        })
    }

    async fn is_enabled(&self) -> Result<bool, ObsError> {
        let display_ptr = self.display.clone();
        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_enabled(display_ptr)
        })
    }

    async fn set_enabled(&self, enabled: bool) -> Result<(), ObsError> {
        let display_ptr = self.display.clone();

        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_set_enabled(display_ptr, enabled)
        })
    }

    async fn set_background_color(&self, r: u8, g: u8, b: u8) -> Result<(), ObsError> {
        let color: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        let display_ptr = self.display.clone();

        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_set_background_color(display_ptr, color)
        })
    }
}
