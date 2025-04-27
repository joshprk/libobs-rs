#[cfg(not(feature="blocking"))]
mod common;

#[cfg(all(target_family = "windows", not(feature="blocking")))]
mod windows;
