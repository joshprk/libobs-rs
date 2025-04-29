#[cfg(not(windows))]
compiler_error!("libobs-wrapper can only be used in windows");

pub mod unsafe_send;
pub mod crash_handler;
pub mod data;
pub mod sources;
pub mod encoders;
pub mod context;
pub mod logger;
pub mod signals;
pub mod display;
pub mod scenes;
#[cfg(feature="bootstrapper")]
pub mod bootstrap;
pub mod runtime;

pub mod utils;
pub mod enums;

// Add the macros module to the public exports
mod macros;