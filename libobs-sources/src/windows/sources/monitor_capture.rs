use display_info::DisplayInfo;
use libobs_source_macro::obs_object_builder;
use libobs_wrapper::sources::ObsSourceBuilder;

/// Provides a easy to use builder for the monitor capture source.
#[derive(Debug)]
#[obs_object_builder("monitor_capture")]
pub struct MonitorCaptureSourceBuilder {
    #[obs_property(type_t = "int", settings_key = "monitor")]
    /// Sets the monitor to capture.
    monitor_raw: i64,

    #[obs_property(type_t = "string", settings_key = "monitor_id")]
    monitor_id_raw: String,

    #[obs_property(type_t = "bool")]
    /// Sets whether the cursor should be captured.
    capture_cursor: bool,

    #[obs_property(type_t = "bool")]
    /// Compatibility mode for the monitor capture source.
    compatibility: bool,
}

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