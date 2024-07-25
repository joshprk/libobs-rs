use crate::wrapper::ObsData;

pub struct WindowCaptureSourceBuilder {
    settings: Option<ObsData>,
    hotkeys: Option<ObsData>
}

pub enum WindowPriority {
    
}

impl WindowCaptureSourceBuilder {
    pub fn new() -> Self {
        Self {
            settings: None,
            hotkeys: None
        }
    }

    pub fn get_id() -> String {
        "window_capture".to_string()
    }

    pub fn get_settings(&mut self) -> &mut ObsData {
        self.settings.get_or_insert_with(ObsData::new)
    }

    pub fn set_window(mut self, window: &str) -> Self {
        self.get_settings().set_string("window", window);
        self
    }

    pub fn set_priority(mut self, priority: WindowPriority) {

    }

    pub fn build(self) -> WindowCaptureSource {
        WindowCaptureSource {
            settings: self.settings,
            hotkeys: self.hotkeys
        }
    }
}