
#[cfg(target_family = "windows")]
mod window_capture;

#[cfg(target_family = "windows")]
pub use window_capture::*;