use display_info::DisplayInfo;
use libobs_wrapper::sources::{ObsSource, ObsSourceBuilder};

use crate::macro_helper::define_object_builder;

// Usage example
define_object_builder!(
    MonitorCaptureSource("monitor_capture") for ObsSource,
    monitor_raw: i64, obs_property(type_t = "int", settings_key = "monitor"),
    monitor_id_raw: String, obs_property(type_t = "string", settings_key = "monitor_id"),
    capture_cursor: bool, obs_property(type_t = "bool"),
    compatibility: bool, obs_property(type_t = "bool")
);

impl MonitorCaptureSourceBuilder {
    /// Gets all available monitors
    pub fn get_monitors() -> anyhow::Result<Vec<DisplayInfo>> {
        Ok(DisplayInfo::all()?)
    }

    pub fn set_monitor(self, monitor: &DisplayInfo) -> Self {
        let s = self.set_monitor_raw(monitor.id as i64);
        s.set_monitor_id_raw(monitor.name.as_str())
    }
}

impl ObsSourceBuilder for MonitorCaptureSourceBuilder {}
