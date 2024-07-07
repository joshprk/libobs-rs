#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(clippy::approx_constant)]
#![allow(clippy::unreadable_literal)]
#![allow(rustdoc::bare_urls)]

pub mod wrapper;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));