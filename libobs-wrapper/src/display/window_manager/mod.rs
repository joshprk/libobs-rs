#[cfg(windows)]
pub(crate) mod windows;

mod traits;
pub use traits::*;

mod misc_trait;
mod position_trait;
mod show_hide;
