#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::approx_constant)]
#![allow(clippy::unreadable_literal)]
#![allow(rustdoc::bare_urls)]
//! # LibOBS bindings (and wrapper) for rust
//! This crate provides bindings to the [LibOBS](https://obsproject.com/) library for rust.
//! Furthermore, this crate provides a safe wrapper around the unsafe functions, which can be found in the [`libobs-wrapper`](https://crates.io/crates/libobs-wrapper) crate.

#[cfg(test)]
mod test;

mod bindings {
    #[cfg(not(feature = "skip_bindings"))]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    #[cfg(feature = "skip_bindings")]
    include!("bindings.rs");
}

pub use bindings::*;