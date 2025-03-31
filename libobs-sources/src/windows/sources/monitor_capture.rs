use display_info::DisplayInfo;
use libobs_source_macro::obs_object_impl;
use libobs_wrapper::sources::{ObsSourceRef, ObsSourceBuilder};

use crate::macro_helper::define_object_manager;

use super::ObsDisplayCaptureMethod;

// Usage example
define_object_manager!(
    /// Provides a easy to use builder for the monitor capture source.
    #[derive(Debug)]
    struct MonitorCaptureSource("monitor_capture") for ObsSourceRef {
        #[obs_property(type_t = "string", settings_key = "monitor_id")]
        monitor_id_raw: String,

        #[obs_property(type_t = "bool")]
        /// Sets whether the cursor should be captured.
        capture_cursor: bool,

        #[obs_property(type_t = "bool")]
        /// Compatibility mode for the monitor capture source.
        compatibility: bool,

        #[obs_property(type_t="enum", settings_key = "method")]
        /// Sets the capture method for the monitor capture source.
        capture_method: ObsDisplayCaptureMethod,
    }
);

#[obs_object_impl]
impl MonitorCaptureSource {
    /// Gets all available monitors
    pub fn get_monitors() -> anyhow::Result<Vec<DisplayInfo>> {
        Ok(DisplayInfo::all()?)
    }

    pub fn set_monitor(self, monitor: &DisplayInfo) -> Self {
        self.set_monitor_id_raw(monitor.name.as_str())
    }
}

impl ObsSourceBuilder for MonitorCaptureSourceBuilder {}
