pub(crate) mod async_sync;
mod error;
mod info;
pub(crate) mod initialization;
mod obs_string;
mod path;
pub mod traits;

use std::ffi::CStr;

pub use error::*;
pub use info::*;
use libobs::obs_module_failure_info;
pub use obs_string::*;
pub use path::*;

use crate::{
    enums::ObsLogLevel, logger::internal_log_global, run_with_obs, runtime::ObsRuntime,
    unsafe_send::Sendable,
};

#[derive(Debug)]
pub struct ObsModules {
    paths: StartupPaths,

    /// A pointer to the module failure info structure.
    info: Option<Sendable<obs_module_failure_info>>,
    pub(crate) runtime: Option<ObsRuntime>,
}

impl ObsModules {
    pub fn add_paths(paths: &StartupPaths) -> Self {
        unsafe {
            libobs::obs_add_data_path(paths.libobs_data_path().as_ptr().0);
            libobs::obs_add_module_path(
                paths.plugin_bin_path().as_ptr().0,
                paths.plugin_data_path().as_ptr().0,
            );
        }

        Self {
            paths: paths.clone(),
            info: None,
            runtime: None,
        }
    }

    pub fn load_modules(&mut self) {
        unsafe {
            let mut failure_info: obs_module_failure_info = std::mem::zeroed();
            internal_log_global(
                ObsLogLevel::Info,
                "---------------------------------".to_string(),
            );
            libobs::obs_load_all_modules2(&mut failure_info);
            internal_log_global(
                ObsLogLevel::Info,
                "---------------------------------".to_string(),
            );
            libobs::obs_log_loaded_modules();
            internal_log_global(
                ObsLogLevel::Info,
                "---------------------------------".to_string(),
            );
            libobs::obs_post_load_modules();
            self.info = Some(Sendable(failure_info));
        }

        self.log_if_failed();
    }

    pub fn log_if_failed(&self) {
        if self.info.as_ref().is_none_or(|x| &x.0.count == &0) {
            return;
        }

        let info = &self.info.as_ref().unwrap().0;
        let mut failed_modules = Vec::new();
        for i in 0..info.count {
            let module = unsafe { info.failed_modules.offset(i as isize) };
            let plugin_name = unsafe { CStr::from_ptr(*module) };
            failed_modules.push(plugin_name.to_string_lossy());
        }

        internal_log_global(
            ObsLogLevel::Warning,
            format!("Failed to load modules: {}", failed_modules.join(", ")),
        );
    }
}

impl Drop for ObsModules {
    fn drop(&mut self) {
        let paths = self.paths.clone();
        let runtime = self.runtime.take().unwrap();

        #[cfg(not(feature = "blocking"))]
        tokio::task::spawn_blocking(move || {
            use crate::run_with_obs_blocking;

            run_with_obs_blocking!(runtime, move || unsafe {
                libobs::obs_remove_data_path(paths.libobs_data_path().as_ptr().0);
            })
            .unwrap();

            //TODO error handling
        });

        #[cfg(feature = "blocking")]
        {
            let r = run_with_obs!(runtime, move || unsafe {
                libobs::obs_remove_data_path(paths.libobs_data_path().as_ptr().0);
            });

            if std::thread::panicking() {
                return;
            }

            r.unwrap();
        }
    }
}

pub const ENCODER_HIDE_FLAGS: u32 =
    libobs::OBS_ENCODER_CAP_DEPRECATED | libobs::OBS_ENCODER_CAP_INTERNAL;
