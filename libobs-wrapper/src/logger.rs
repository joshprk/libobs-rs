use std::{
    os::raw::c_void,
    sync::{
        atomic::{AtomicBool, Ordering},
        RwLock,
    },
};

use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use vsprintf::vsprintf;

use crate::{enums::ObsLogLevel, utils::StartupInfo};

pub type LogCallback = fn(ObsLogLevel, String) -> ();

pub fn default_log_callback(level: ObsLogLevel, message: String) {
    println!("[{:?}] {}", level, message);
}

lazy_static! {
    /// We are using this as global variable because there can only be one obs context
    pub static ref LOG_CALLBACK: RwLock<LogCallback> = RwLock::new(default_log_callback);

    pub static ref HANDLER_SET: AtomicBool = AtomicBool::new(false);
}

unsafe extern "C" fn extern_log_callback(
    log_level: i32,
    msg: *const i8,
    args: *mut i8,
    _params: *mut c_void,
) {
    let level = ObsLogLevel::from_i32(log_level);
    if level.is_none() {
        eprintln!("Couldn't find log level {}", log_level);
        return;
    }

    let level = level.unwrap();

    let formatted = vsprintf(msg, args);
    if formatted.is_err() {
        eprintln!("Failed to format log message");
        return;
    }

    let callback = LOG_CALLBACK.read().unwrap();

    callback(level, formatted.unwrap());
}

pub trait ObsLogger {
    /// Sets the log callback for the obs context (can be used to write logs to a file)
    fn set_log_callback(self, callback: LogCallback) -> anyhow::Result<StartupInfo>;
}

impl ObsLogger for StartupInfo {
    fn set_log_callback(self, callback: LogCallback) -> anyhow::Result<StartupInfo> {
        if !HANDLER_SET.load(Ordering::Relaxed) {
            unsafe {
                libobs::base_set_log_handler(Some(extern_log_callback), std::ptr::null_mut());
            }
            HANDLER_SET.store(true, Ordering::Relaxed);
        }

        let mut log_callback = LOG_CALLBACK.write()
            .map_err(|e| anyhow::anyhow!("Failed to lock log callback: {}", e))?;
        *log_callback = callback;

        Ok(self)
    }
}
