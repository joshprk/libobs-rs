#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    unnecessary_transmutes,
    non_snake_case,
    clippy::all
)]

//! # LibOBS bindings (and wrapper) for rust
//! This crate provides bindings to the [LibOBS](https://obsproject.com/) library for rust.
//! Furthermore, this crate provides a safe wrapper around the unsafe functions, which can be found in the [`libobs-wrapper`](https://crates.io/crates/libobs-wrapper) crate.

#[cfg_attr(coverage_nightly, coverage(off))]
mod bindings {
    #[cfg(feature = "generate_bindings")]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[cfg(all(not(feature = "generate_bindings"), target_family = "windows"))]
    include!("bindings_win.rs");

    #[cfg(all(not(feature = "generate_bindings"), target_os = "linux"))]
    include!("bindings_linux.rs");

    #[cfg(all(not(feature = "generate_bindings"), target_os = "macos"))]
    include!("bindings_linux.rs");  // Use Linux bindings for macOS (both Unix-like)
}

pub use bindings::*;
