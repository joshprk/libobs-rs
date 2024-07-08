mod types;
#[cfg(feature = "unsafe")]
#[allow(warnings, unused)]
pub mod libobs;
#[cfg(not(feature = "unsafe"))]
#[allow(warnings, unused)]
mod libobs;

use std::{ptr, thread};
use std::sync::Mutex;
use std::thread::ThreadId;

pub use types::ObsPath;
use types::ObsString;

use libobs as obs;

static OBS_THREAD_ID: Mutex<Option<ThreadId>> = Mutex::new(None);

pub enum ObsError {
    Generic,
    ThreadFailure,
    ThreadMutexFailure,
}

/// Interface to the OBS context. 
/// 
/// Only one context can exist across all threads.
/// Any attempt to create a new context while there 
/// is an existing one will error.
#[allow(unused)]
#[derive(Debug)]
pub struct ObsContext {
    locale: ObsString,
    libobs_data_path: ObsString,
    plugin_bin_path: ObsString,
    plugin_data_path: ObsString,
    module_config_path: Option<ObsString>,
    _obs_context_shutdown_zst: _ObsContextShutdownZST,
}

impl ObsContext {
    pub fn new(info: StartupInfo) -> Result<Self, ObsError> {
        if let Ok(mut thread_id) = OBS_THREAD_ID.lock() {
            if *thread_id != None {
                return Err(ObsError::ThreadFailure)
            }

            *thread_id = Some(thread::current().id());
        } else {
            return Err(ObsError::ThreadMutexFailure)
        }

        let locale_str = ObsString::new("en-US");
        let libobs_data_path_str = info.libobs_data_path.into_obs_string();
        let plugin_bin_data_str = info.plugin_bin_path.into_obs_string();
        let plugin_data_path_str = info.plugin_data_path.into_obs_string();
        let module_config_path_str = match info.module_config_path {
            Some(x) => Some(x.into_obs_string()),
            None    => None,
        };

        unsafe {
            obs::obs_startup(
                locale_str.as_ptr(), 
                match module_config_path_str {
                    Some(ref x) => x.as_ptr(),
                    None    => ptr::null(),
                },
                ptr::null_mut(),
            );
        }

        Ok(Self {
            locale: locale_str,
            libobs_data_path: libobs_data_path_str,
            plugin_bin_path: plugin_bin_data_str,
            plugin_data_path: plugin_data_path_str,
            module_config_path: module_config_path_str,
            _obs_context_shutdown_zst: _ObsContextShutdownZST::new(),
        })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct StartupInfo {
    libobs_data_path: ObsPath,
    plugin_bin_path: ObsPath,
    plugin_data_path: ObsPath,
    module_config_path: Option<ObsPath>,
}

/// Internal hack to ensure that OBS frees memory
/// in the correct order. The library crashes if
/// you free its members before shutting down
/// the OBS context.
#[derive(Debug)]
struct _ObsContextShutdownZST {}

impl _ObsContextShutdownZST {
    /// Creates a new `_ObsContextShutdownZST`.
    /// 
    /// Creating this struct outside of 
    /// `ObsContext` is fine because libobs 
    /// checks if the library was started before
    /// attempting to shut down, but why would
    /// you want to?
    pub fn new() -> Self {
        Self {}
    }
}

impl Drop for _ObsContextShutdownZST {
    fn drop(&mut self) {
        if let Ok(mut thread_id) = OBS_THREAD_ID.lock() {
            if *thread_id != Some(thread::current().id()) {
                return
            }

            unsafe { libobs::obs_shutdown() }
            
            *thread_id = None;
        } else {
            return
        }
    }
}

/// Checks whether or not the current thread has
/// created a `ObsContext` and is the OBS thread.
/// 
/// If the mutex is poisoned, it will only return
/// false.
pub fn is_obs_thread() -> bool {
    if let Ok(thread_id) = OBS_THREAD_ID.lock() {
        return *thread_id == Some(thread::current().id())
    }

    return false
}

/// Gets the number of active memory allocations 
/// that OBS is using.
pub fn get_obs_allocs() -> i64 {
    // windows binaries return bnum_allocs() as
    // i32, cross-compatibility into
    unsafe { obs::bnum_allocs().into() }
}