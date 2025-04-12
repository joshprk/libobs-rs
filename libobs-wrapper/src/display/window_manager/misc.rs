use crate::display::ObsDisplayRef;

pub trait MiscDisplayTrait {
    fn update_color_space(&self);
    fn is_enabled(&self) -> bool;
    fn set_enabled(&self, enabled: bool);
    fn set_background_color(&self, r: u8, g: u8, b: u8);
}

impl MiscDisplayTrait for ObsDisplayRef {
    fn update_color_space(&self) {
        unsafe {
            libobs::obs_display_update_color_space(self.display.0);
        }
    }

    fn is_enabled(&self) -> bool {
        unsafe { libobs::obs_display_enabled(self.display.0) }
    }

    fn set_enabled(&self, enabled: bool) {
        unsafe { libobs::obs_display_set_enabled(self.display.0, enabled) };
    }

    fn set_background_color(&self, r: u8, g: u8, b: u8) {
        let color: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

        unsafe { libobs::obs_display_set_background_color(self.display.0, color) };
    }
}