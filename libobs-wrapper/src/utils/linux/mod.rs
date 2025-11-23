//! Contains linux specific bindigns to x11 and wayland

use std::os::raw::c_char;
extern "C" {
    // X11 functions
    pub(crate) fn XOpenDisplay(display_name: *const c_char) -> *mut std::os::raw::c_void;
    pub(crate) fn XCloseDisplay(display: *mut std::os::raw::c_void) -> i32;

    // Wayland functions
    pub(crate) fn wl_display_connect(name: *const c_char) -> *mut std::os::raw::c_void;
    pub(crate) fn wl_display_disconnect(display: *mut std::os::raw::c_void);
}
#[derive(Debug)]
pub struct LinuxGlibLoop {
    glib_loop: glib::MainLoop,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl LinuxGlibLoop {
    pub fn new() -> Self {
        let g_loop = glib::MainLoop::new(None, false);
        let g_loop_clone = g_loop.clone();
        let handle = std::thread::spawn(move || {
            g_loop_clone.run();
        });

        Self {
            glib_loop: g_loop,
            handle: Some(handle),
        }
    }
}

impl Default for LinuxGlibLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LinuxGlibLoop {
    fn drop(&mut self) {
        if self.glib_loop.is_running() {
            self.glib_loop.quit();
        }

        if let Some(handle) = self.handle.take() {
            let r = handle.join();
            if std::thread::panicking() {
                log::error!(
                    "[libobs-wrapper]: Thread panicked while dropping LinuxGlibLoop: {:?}",
                    r.err()
                );
            } else {
                r.unwrap();
            }
        }
    }
}

pub(crate) fn wl_proxy_get_display(
    proxy: *mut std::os::raw::c_void,
) -> Result<*mut std::os::raw::c_void, libloading::Error> {
    unsafe {
        let lib = libloading::Library::new("libwayland-client.so")?;
        let sym: Result<
            libloading::Symbol<
                unsafe extern "C" fn(*mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
            >,
            libloading::Error,
        > = lib.get(b"wl_proxy_get_display\0");

        match sym {
            Ok(f) => Ok(f(proxy)),
            Err(e) => Err(e),
        }
    }
}
