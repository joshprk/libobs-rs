use std::os::raw::c_void;
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, GetWindowLongPtrW, IsIconic, IsWindowVisible, GWL_EXSTYLE, GWL_STYLE, WS_CHILD, WS_EX_TOOLWINDOW};
use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED},
};

use windows_result::Result;

const INTERNAL_MICROSOFT_EXES_EXACT: &'static [&'static str] = &[
    "startmenuexperiencehost.exe",
    "applicationframehost.exe",
    "peopleexperiencehost.exe",
    "shellexperiencehost.exe",
    "microsoft.notes.exe",
    "systemsettings.exe",
    "textinputhost.exe",
    "searchapp.exe",
    "video.ui.exe",
    "searchui.exe",
    "lockapp.exe",
    "cortana.exe",
    "gamebar.exe",
    "tabtip.exe",
    "time.exe",
];

const INTERNAL_MICROSOFT_EXES_PARTIAL: &'static [&'static str] = &["windowsinternal"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSearchMode {
    ExcludeMinimized,
    IncludeMinimized,
}

type DWORD = u32;

pub(crate) fn is_window_cloaked(handle: HWND) -> bool {
    let cloaked: DWORD = 0;
    let res = unsafe {
        DwmGetWindowAttribute(
            handle,
            DWMWA_CLOAKED,
            cloaked as *mut c_void,
            size_of::<DWORD>() as u32,
        )
    };

    return res.is_ok() && cloaked != 0;
}

pub fn is_window_valid(handle: HWND, mode: WindowSearchMode) -> Result<bool> {
    let is_visible = unsafe { IsWindowVisible(handle) };
    if !is_visible.as_bool() {
        return Ok(false);
    }

    if mode == WindowSearchMode::ExcludeMinimized {
        let is_minimized = unsafe { IsIconic(handle).as_bool() } || is_window_cloaked(handle);
        if is_minimized {
            return Ok(false);
        }
    }

    let mut rect = RECT::default();
    let styles;
    let ex_styles;

    unsafe {
        GetClientRect(handle, &mut rect)?;

        // Use the W function because obs can only be compiled for 64-bit
        styles = GetWindowLongPtrW(handle, GWL_STYLE) as DWORD;
        ex_styles = GetWindowLongPtrW(handle, GWL_EXSTYLE) as DWORD;
    }

    if ex_styles & WS_EX_TOOLWINDOW.0 > 0 {
        return Ok(false);
    }
    if styles & WS_CHILD.0 > 0 {
        return Ok(false);
    }

    if mode == WindowSearchMode::ExcludeMinimized && (rect.bottom == 0 || rect.right == 0) {
        return Ok(false);
    }

    return Ok(true);
}

pub fn is_microsoft_internal_exe(exe: &str) -> bool {
    let exact = INTERNAL_MICROSOFT_EXES_EXACT.iter().any(|e| *e == exe);
    let partial = INTERNAL_MICROSOFT_EXES_PARTIAL
        .iter()
        .any(|e| exe.contains(e));

    return exact || partial;
}
