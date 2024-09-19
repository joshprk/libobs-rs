use std::ptr;
use std::thread;
use std::sync::Mutex;
use std::thread::ThreadId;

use crate::errors::ContextError;
use crate::ffi;
use crate::types::ObsString;
use crate::types::ObsPath;

static OBS_THREAD_ID: Mutex<Option<ThreadId>> = Mutex::new(None);

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
    pub fn new(config: Config) -> Result<Self, ContextError> {
        if let Ok(mut thread_id) = OBS_THREAD_ID.lock() {
            if *thread_id != None {
                return Err(ContextError::ContextExists)
            }

            *thread_id = Some(thread::current().id());
        } else {
            return Err(ContextError::MutexPoisoned)
        }

        let locale_str = ObsString::new("en-US");

        let libobs_data_path_str = config.libobs_data_path.into();
        let plugin_bin_data_str = config.plugin_bin_path.into();
        let plugin_data_path_str = config.plugin_data_path.into();
        let module_config_path_str = match config.module_config_path {
            Some(x) => Some(x.into_obs_string()),
            None    => None,
        };

        unsafe {
            if ffi::obs_startup(
                locale_str.as_ptr(), 
                match module_config_path_str {
                    Some(ref x) => x.as_ptr(),
                    None        => ptr::null(),
                },
                ptr::null_mut(),
            ) {
                Ok(())
            } else {
                Err(ContextError::StartupFailure)
            }
        }?;

        Ok(Self {
            locale: locale_str,
            libobs_data_path: libobs_data_path_str,
            plugin_bin_path: plugin_bin_data_str,
            plugin_data_path: plugin_data_path_str,
            module_config_path: module_config_path_str,
            _obs_context_shutdown_zst: _ObsContextShutdownZST::new(),
        })
    }

    pub fn reset_video(&mut self) {
        
    }
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
    fn new() -> Self {
        Self {}
    }
}

impl Drop for _ObsContextShutdownZST {
    fn drop(&mut self) {
        if let Ok(mut thread_id) = OBS_THREAD_ID.lock() {
            if *thread_id != Some(thread::current().id()) {
                return
            }

            unsafe { ffi::obs_shutdown() }
            
            *thread_id = None;
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Config {
    libobs_data_path: ObsPath,
    plugin_bin_path: ObsPath,
    plugin_data_path: ObsPath,
    module_config_path: Option<ObsPath>,
}

impl Config {
    pub fn new(
        libobs_data_path: ObsPath,
        plugin_bin_path: ObsPath,
        plugin_data_path: ObsPath,
        module_config_path: Option<ObsPath>,
    ) -> Self {
        Self {
            libobs_data_path,
            plugin_bin_path,
            plugin_data_path,
            module_config_path,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            libobs_data_path: "data/libobs".into(),
            plugin_bin_path: "obs-plugins/64bit".into(),
            plugin_data_path: "data/obs-plugins/%module%".into(),
            module_config_path: None,
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
/// that OBS is using. Useful for testing.
pub fn get_obs_allocs() -> i64 {
    // non-windows binaries return bnum_allocs() 
    // as i64, cross-compatibility into
    unsafe { ffi::bnum_allocs().into() }
}
