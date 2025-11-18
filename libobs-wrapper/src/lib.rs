#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

#[cfg(not(windows))]
compiler_error!("libobs-wrapper can only be used in windows");

pub mod context;
pub mod crash_handler;
pub mod data;
pub mod display;
pub mod encoders;
pub mod enums;
pub mod logger;
pub mod runtime;
pub mod scenes;
pub mod signals;
pub mod sources;
pub mod unsafe_send;
pub mod utils;

// Add the macros module to the public exports
pub mod graphics;
#[cfg_attr(coverage_nightly, coverage(off))]
mod macros;

#[deprecated = "Use graphics::Vec2 instead."]
pub type Vec2 = graphics::Vec2;
