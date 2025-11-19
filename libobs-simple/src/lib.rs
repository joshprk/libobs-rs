//! A simplified interface for recording and streaming with libobs

pub mod output;
pub mod quick_start;
pub mod sources;

#[cfg(feature = "auto-bootstrap")]
pub use libobs_bootstrapper;
