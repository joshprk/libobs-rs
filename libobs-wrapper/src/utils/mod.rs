mod obs_string;
mod error;
mod info;
mod path;
pub(crate) mod initialization;
pub mod traits;

use std::ffi::CStr;

use libobs::obs_module_failure_info;
pub use obs_string::*;
pub use error::*;
pub use info::*;
pub use path::*;

use crate::{enums::ObsLogLevel, logger::internal_log_global};


pub struct ObsModules {
    info: obs_module_failure_info,
}

impl ObsModules {
    pub fn add_paths(paths: &StartupPaths) {
        unsafe {
            libobs::obs_add_data_path(paths.libobs_data_path().as_ptr());
            libobs::obs_add_module_path(
                paths.plugin_bin_path().as_ptr(),
                paths.plugin_data_path().as_ptr(),
            );
        }
    }

    pub fn load_modules() -> Self {
        unsafe {
            let mut failure_info: obs_module_failure_info = std::mem::zeroed();
            internal_log_global(ObsLogLevel::Info, "---------------------------------".to_string());
            libobs::obs_load_all_modules2(&mut failure_info);
            internal_log_global(ObsLogLevel::Info, "---------------------------------".to_string());
            libobs::obs_log_loaded_modules();
            internal_log_global(ObsLogLevel::Info, "---------------------------------".to_string());
            libobs::obs_post_load_modules();
            Self { info: failure_info }
        }
    }

    pub fn log_if_failed(&self) {
        if self.info.count == 0 {
            return;
        }

        let mut failed_modules = Vec::new();
        for i in 0..self.info.count {
            let module = unsafe { self.info.failed_modules.offset(i as isize) };
            let plugin_name = unsafe { CStr::from_ptr(*module) };
            failed_modules.push(plugin_name.to_string_lossy());
        }

        internal_log_global(ObsLogLevel::Warning, format!("Failed to load modules: {}", failed_modules.join(", ")));
    }
}

pub const ENCODER_HIDE_FLAGS: u32 = libobs::OBS_ENCODER_CAP_DEPRECATED | libobs::OBS_ENCODER_CAP_INTERNAL;