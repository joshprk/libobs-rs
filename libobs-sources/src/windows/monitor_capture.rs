use display_info::DisplayInfo;
use libobs_source_macro::obs_source_builder;

/// Provides a easy to use builder for the monitor capture source.
#[derive(Debug)]
#[obs_source_builder("monitor_capture")]
pub struct MonitorCaptureSourceBuilder {
    #[obs_property(type_t = "int")]
    /// Sets the monitor to capture.
    monitor: i32,

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
}

#[cfg(test)]
mod tests {
    use super::MonitorCaptureSourceBuilder;

    #[test]
    pub fn test_hi() {
        MonitorCaptureSourceBuilder::get_monitors().unwrap();
    }
}