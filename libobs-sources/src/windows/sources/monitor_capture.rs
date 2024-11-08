use display_info::DisplayInfo;
use libobs_source_macro::{obs_object_builder, obs_object_updater};
use libobs_wrapper::sources::{ObsSource, ObsSourceBuilder};
use paste::paste;

macro_rules! define_object_builder {
    ($obs_id:literal, $struct_name:ident, $updatable_name: ident, $($field_name:ident: $field_type:ty, $obs_property:meta),*) => {
        #[allow(dead_code)]
        /// This struct is just so the compiler isn't confused
        struct $struct_name {}

        paste! {
            #[derive(Debug)]
            #[obs_object_builder($obs_id)]
            pub struct [<$struct_name Builder>] {
                $(
                    #[$obs_property]
                    $field_name: $field_type,
                )*
            }

            #[obs_object_updater($obs_id, $updatable_name)]
            pub struct [<$struct_name Updater>] {
                $(
                    #[$obs_property]
                    $field_name: $field_type,
                )*
            }
        }
    };
}

// Usage example
define_object_builder!(
    "monitor_capture", MonitorCaptureSource, ObsSource,
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
