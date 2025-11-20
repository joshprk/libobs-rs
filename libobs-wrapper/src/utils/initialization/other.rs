use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
extern "C" {
    fn getenv(name: *const c_char) -> *mut c_char;

    // X11 functions
    fn XOpenDisplay(display_name: *const c_char) -> *mut std::os::raw::c_void;
    fn XCloseDisplay(display: *mut std::os::raw::c_void) -> i32;

    // Wayland functions
    fn wl_display_connect(name: *const c_char) -> *mut std::os::raw::c_void;
    fn wl_display_disconnect(display: *mut std::os::raw::c_void);
}

/// Detects the current display server and initializes OBS platform accordingly
pub(crate) fn load_debug_privilege() {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        // Try to detect and initialize the platform
        if let Some(platform_type) = detect_platform() {
            unsafe {
                match platform_type {
                    PlatformType::X11 => {
                        libobs::obs_set_nix_platform(
                            libobs::obs_nix_platform_type_OBS_NIX_PLATFORM_X11_EGL,
                        );

                        // Try to get X11 display - note: this may fail in headless environments
                        let display = XOpenDisplay(ptr::null());
                        if !display.is_null() {
                            libobs::obs_set_nix_platform_display(display);
                        } else {
                            // Set a null display - OBS can handle this case
                            libobs::obs_set_nix_platform_display(ptr::null_mut());
                        }

                        let message = CString::new("Using EGL/X11").unwrap();
                        libobs::blog(libobs::LOG_INFO as i32, message.as_ptr());
                    }
                    PlatformType::Wayland => {
                        libobs::obs_set_nix_platform(
                            libobs::obs_nix_platform_type_OBS_NIX_PLATFORM_WAYLAND,
                        );

                        // Try to get Wayland display - note: this may fail in headless environments
                        let display = wl_display_connect(ptr::null());
                        if !display.is_null() {
                            libobs::obs_set_nix_platform_display(display);
                        } else {
                            // Set a null display - OBS can handle this case
                            libobs::obs_set_nix_platform_display(ptr::null_mut());
                        }

                        let message = CString::new("Platform: Wayland").unwrap();
                        libobs::blog(libobs::LOG_INFO as i32, message.as_ptr());
                    }
                }
            }
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
enum PlatformType {
    X11,
    Wayland,
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn detect_platform() -> Option<PlatformType> {
    // Check for Wayland first
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        return Some(PlatformType::Wayland);
    }

    // Check for X11
    if std::env::var("DISPLAY").is_ok() {
        // Could be XWayland, check XDG_SESSION_TYPE for more accuracy
        if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
            return match session_type.as_str() {
                "wayland" => Some(PlatformType::Wayland),
                "x11" => Some(PlatformType::X11),
                _ => None,
            };
        }
        return Some(PlatformType::X11);
    }

    None
}
