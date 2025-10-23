use num_derive::{FromPrimitive, ToPrimitive};

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the priority of the window capture source.
/// Used in `WindowCaptureSourceBuilder`
pub enum ObsWindowPriority {
    /// The window class names must be the same. This means that windows are of the same type.
    Class = libobs::window_priority_WINDOW_PRIORITY_CLASS,
    /// Window titles must match otherwise, find window with the same class
    Title = libobs::window_priority_WINDOW_PRIORITY_TITLE,
    /// Match title, otherwise find window with the same executable
    Executable = libobs::window_priority_WINDOW_PRIORITY_EXE,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the capture method of the window capture source.
/// Used in `WindowCaptureSourceBuilder`
pub enum ObsWindowCaptureMethod {
    /// Automatically selects the best method based on the window.
    MethodAuto = libobs::window_capture_method_METHOD_AUTO,
    /// Uses BitBlt to capture the window. BitBlt (Windows 7 and up)
    MethodBitBlt = libobs::window_capture_method_METHOD_BITBLT,
    /// Uses Windows Graphics Capture to capture the window. Windows 10 (1903 and up)
    MethodWgc = libobs::window_capture_method_METHOD_WGC,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the capture method of the monitor capture source.
/// Used in `MonitorCaptureSourceBuilder`
pub enum ObsDisplayCaptureMethod {
    /// Automatically selects the best method based on the monitor.
    MethodAuto = libobs::display_capture_method_DISPLAY_METHOD_AUTO,
    /// Uses DXGI to capture the monitor.
    MethodDXGI = libobs::display_capture_method_DISPLAY_METHOD_DXGI,
    /// Uses Windows Graphics Capture to capture the monitor.
    MethodWgc = libobs::display_capture_method_DISPLAY_METHOD_WGC,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
/// Describes the capture method of the window capture source.
/// Used in `WindowCaptureSourceBuilder`
pub enum ObsHookRate {
    Slow = libobs::hook_rate_HOOK_RATE_SLOW,
    Normal = libobs::hook_rate_HOOK_RATE_NORMAL,
    Fast = libobs::hook_rate_HOOK_RATE_FAST,
    Fastest = libobs::hook_rate_HOOK_RATE_FASTEST,
}
