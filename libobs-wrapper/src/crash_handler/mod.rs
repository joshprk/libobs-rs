use std::{ffi::c_void, sync::Mutex};

use lazy_static::lazy_static;

#[cfg(feature = "dialog_crash_handler")]
pub mod dialog;

pub trait ObsCrashHandler: Send {
    fn handle_crash(&self, message: String);
}

pub struct ConsoleCrashHandler {
    _private: (),
}

impl ConsoleCrashHandler {
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl ObsCrashHandler for ConsoleCrashHandler {
    fn handle_crash(&self, message: String) {
        eprintln!("OBS crashed: {}", message);
    }
}

lazy_static! {
    /// We are using this as global variable because there can only be one obs context
    pub static ref CRASH_HANDLER: Mutex<Box<dyn ObsCrashHandler>> = {
        #[cfg(feature="dialog_crash_handler")]
        {
            Mutex::new(Box::new(dialog::DialogCrashHandler::new()))
        }
        #[cfg(not(feature="dialog_crash_handler"))]
        {
            Mutex::new(Box::new(ConsoleCrashHandler {}))
        }
    };
}

pub(crate) unsafe extern "C" fn main_crash_handler(
    format: *const i8,
    args: *mut i8,
    _params: *mut c_void,
) {
    let res = vsprintf::vsprintf(format, args);
    if res.is_err() {
        eprintln!("Failed to format crash handler message");
        return;
    }

    let res = res.unwrap();
    CRASH_HANDLER.lock().unwrap().handle_crash(res);
}
