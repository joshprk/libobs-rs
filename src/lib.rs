use std::{sync::Mutex, thread::{self, ThreadId}};

pub use types::ObsPath;

#[cfg(feature = "unsafe")]
pub mod libobs;
#[cfg(not(feature = "unsafe"))]
mod libobs;

mod types;

static OBS_THREAD_ID: Mutex<Option<ThreadId>> = Mutex::new(None);

pub enum ObsError {
    Generic,
    ThreadFailure,
}

/// Interface to the OBS context. 
/// 
/// Only one context can exist across all threads.
/// Any attempt to create a new context while there 
/// is an existing one will error.
#[derive(Debug)]
pub struct ObsContext {
    _obs_context_shutdown_zst: _ObsContextShutdownZST
}

impl ObsContext {
    pub fn new(info: StartupInfo) -> Result<Self, ObsError> {
        todo!()
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, PartialOrd)]
pub struct StartupInfo {
    libobs_data_path: ObsPath,
    plugin_bin_path: ObsPath,
    plugin_data_path: ObsPath,
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
        if !is_obs_thread() {
            return
        }

        unsafe { libobs::obs_shutdown() }
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