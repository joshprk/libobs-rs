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
                        libobs::obs_set_nix_platform(libobs::obs_nix_platform_type_OBS_NIX_PLATFORM_X11_EGL);
                        
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
                        libobs::obs_set_nix_platform(libobs::obs_nix_platform_type_OBS_NIX_PLATFORM_WAYLAND);
                        
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
    unsafe {
        // First check environment variables for explicit session type
        if let Some(session_type) = get_env_var("XDG_SESSION_TYPE") {
            let session_lower = session_type.to_lowercase();
            if session_lower.contains("wayland") {
                return Some(PlatformType::Wayland);
            } else if session_lower.contains("x11") {
                return Some(PlatformType::X11);
            }
        }
        
        // Check for Wayland-specific environment variables
        if get_env_var("WAYLAND_DISPLAY").is_some() || get_env_var("WAYLAND_SERVER").is_some() {
            // Try to connect to Wayland to verify it's actually available
            let display = wl_display_connect(ptr::null());
            if !display.is_null() {
                wl_display_disconnect(display);
                return Some(PlatformType::Wayland);
            }
            // Even if we can't connect, if env vars suggest Wayland, prefer it
            return Some(PlatformType::Wayland);
        }
        
        // Check for X11 display environment variable
        if get_env_var("DISPLAY").is_some() {
            // Try to connect to X11 to verify it's actually available  
            let display = XOpenDisplay(ptr::null());
            if !display.is_null() {
                XCloseDisplay(display);
                return Some(PlatformType::X11);
            }
            // Even if we can't connect, if DISPLAY is set, prefer X11
            return Some(PlatformType::X11);
        }
        
        // Check for desktop environment hints
        if let Some(desktop) = get_env_var("XDG_CURRENT_DESKTOP") {
            let desktop_lower = desktop.to_lowercase();
            // Some DEs that typically use Wayland
            if desktop_lower.contains("sway") || desktop_lower.contains("river") || 
               desktop_lower.contains("hyprland") || desktop_lower.contains("weston") {
                return Some(PlatformType::Wayland);
            }
        }
        
        // Default fallback to X11 if nothing else is detected
        // This is the most compatible choice for most Linux systems
        Some(PlatformType::X11)
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
unsafe fn get_env_var(name: &str) -> Option<String> {
    let name_cstr = CString::new(name).ok()?;
    let value_ptr = getenv(name_cstr.as_ptr());
    
    if value_ptr.is_null() {
        None
    } else {
        let value_cstr = std::ffi::CStr::from_ptr(value_ptr);
        value_cstr.to_str().ok().map(|s| s.to_string())
    }
}
