pub(crate) mod helper;
pub(crate) mod string_conv;
pub(crate) mod validators;
pub(crate) mod win_iterator;

pub use helper::{get_window_info, WindowInfo};
pub use validators::{WindowSearchMode, is_microsoft_internal_exe, is_window_valid};