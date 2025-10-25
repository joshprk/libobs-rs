//! This is derived from the frontend/obs-main.cpp.

use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, HANDLE, LUID},
        Security::{
            AdjustTokenPrivileges, LookupPrivilegeValueW, SE_DEBUG_NAME, SE_INC_BASE_PRIORITY_NAME,
            SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
        },
        System::Threading::{GetCurrentProcess, OpenProcessToken},
    },
};

pub fn load_debug_privilege() {
    unsafe {
        let flags = TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY;
        let mut tp = TOKEN_PRIVILEGES::default();
        let mut token = HANDLE::default();
        let mut val = LUID::default();

        if OpenProcessToken(GetCurrentProcess(), flags, &mut token).is_err() {
            return;
        }

        if LookupPrivilegeValueW(PCWSTR::null(), SE_DEBUG_NAME, &mut val).is_ok() {
            tp.PrivilegeCount = 1;
            tp.Privileges[0].Luid = val;
            tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

            let res = AdjustTokenPrivileges(
                token,
                false,
                Some(&tp),
                std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
                None,
                None,
            );
            if let Err(e) = res {
                // Use a logging mechanism compatible with your Rust application
                eprintln!("Could not set privilege to debug process: {e:?}");
            }
        }

        if LookupPrivilegeValueW(PCWSTR::null(), SE_INC_BASE_PRIORITY_NAME, &mut val).is_ok() {
            tp.PrivilegeCount = 1;
            tp.Privileges[0].Luid = val;
            tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

            let res = AdjustTokenPrivileges(
                token,
                false,
                Some(&tp),
                std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
                None,
                None,
            );

            if let Err(e) = res {
                // Use a logging mechanism compatible with your Rust application
                eprintln!("Could not set privilege to increase GPU priority {e:?}");
            }
        }

        let _ = CloseHandle(token);
    }
}
