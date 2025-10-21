use crate::{display::ObsDisplayRef, run_with_obs, utils::ObsError};

pub trait MiscDisplayTrait {
    fn update_color_space(&self) -> Result<(), ObsError>;

    fn is_enabled(&self) -> Result<bool, ObsError>;

    fn set_enabled(&self, enabled: bool) -> Result<(), ObsError>;

    fn set_background_color(&self, r: u8, g: u8, b: u8) -> Result<(), ObsError>;
}

impl MiscDisplayTrait for ObsDisplayRef {
    fn update_color_space(&self) -> Result<(), ObsError> {
        let display_ptr = self.display.clone();
        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_update_color_space(display_ptr)
        })
    }


    fn is_enabled(&self) -> Result<bool, ObsError> {
        let display_ptr = self.display.clone();
        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_enabled(display_ptr)
        })
    }


    fn set_enabled(&self, enabled: bool) -> Result<(), ObsError> {
        let display_ptr = self.display.clone();

        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_set_enabled(display_ptr, enabled)
        })
    }

    fn set_background_color(&self, r: u8, g: u8, b: u8) -> Result<(), ObsError> {
        let color: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        let display_ptr = self.display.clone();

        run_with_obs!(self.runtime, (display_ptr), move || unsafe {
            libobs::obs_display_set_background_color(display_ptr, color)
        })
    }
}
