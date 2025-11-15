#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub(crate) use windows::*;

#[cfg(not(windows))]
mod linux;

#[cfg(not(windows))]
pub(crate) use linux::*;
