use libobs::{gs_init_data, gs_window};
use num_traits::ToPrimitive;

use crate::unsafe_send::Sendable;

use super::{GsColorFormat, GsZstencilFormat};


#[derive(Clone)]
pub struct ObsDisplayCreationData {
    #[cfg(target_family = "windows")]
    pub(super) window_handle: Sendable<windows::Win32::Foundation::HWND>,
    pub(super) create_child: bool,
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) format: GsColorFormat,
    pub(super) zsformat: GsZstencilFormat,
    pub(super) adapter: u32,
    pub(super) backbuffers: u32,
    pub(super) background_color: u32,
}

impl ObsDisplayCreationData {
    #[cfg(target_family = "windows")]
    pub fn new(window_handle: isize, create_child: bool, x: i32, y: i32, width: u32, height: u32) -> Self {
        use std::os::raw::c_void;
        use windows::Win32::Foundation::HWND;

        Self {
            window_handle: Sendable(HWND(window_handle as *mut c_void)),
            create_child,
            format: GsColorFormat::BGRA,
            zsformat: GsZstencilFormat::ZSNone,
            x,
            y,
            width,
            height,
            adapter: 0,
            backbuffers: 0,
            background_color: 0
        }
    }

    pub fn set_format(mut self, format: GsColorFormat) -> Self {
        self.format = format;
        self
    }

    pub fn set_zsformat(mut self, zsformat: GsZstencilFormat) -> Self {
        self.zsformat = zsformat;
        self
    }

    pub fn set_adapter(mut self, adapter: u32) -> Self {
        self.adapter = adapter;
        self
    }

    pub fn set_backbuffers(mut self, backbuffers: u32) -> Self {
        self.backbuffers = backbuffers;
        self
    }

    pub fn set_background_color(mut self, background_color: u32) -> Self {
        self.background_color = background_color;
        self
    }

    pub(super) fn build(self, window: gs_window) -> gs_init_data {
        let data = gs_init_data {
            cx: self.width,
            cy: self.height,
            format: self.format.to_i32().unwrap(),
            zsformat: self.zsformat.to_i32().unwrap(),
            window,
            adapter: self.adapter,
            num_backbuffers: self.backbuffers,
        };

        data
    }
}