//! This

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use lazy_static::lazy_static;
use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Dwm::DwmIsCompositionEnabled,
        System::{
            LibraryLoader::{GetModuleHandleA, GetModuleHandleW},
            SystemInformation::{GetVersionExW, OSVERSIONINFOW},
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW,
            GetWindowLongPtrW, LoadCursorW, RegisterClassExW, SetLayeredWindowAttributes,
            SetParent, SetWindowLongPtrW, TranslateMessage, CS_HREDRAW, CS_NOCLOSE, CS_OWNDC,
            CS_VREDRAW, GWL_EXSTYLE, GWL_STYLE, HTTRANSPARENT, IDC_ARROW, LWA_ALPHA, MSG,
            WM_NCHITTEST, WNDCLASSEXW, WS_CHILD, WS_EX_COMPOSITED, WS_EX_LAYERED,
            WS_EX_TRANSPARENT, WS_POPUP, WS_VISIBLE,
        },
    },
};

use crate::unsafe_send::{WrappedHWND, WrappedObsDisplay};

mod position_trait;
pub use position_trait::WindowPositionTrait;

extern "system" fn wndproc(
    window: HWND,
    message: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    unsafe {
        match message {
            WM_NCHITTEST => {
                return LRESULT(HTTRANSPARENT as _);
            }
            _ => {
                return DefWindowProcW(window, message, w_param, l_param);
            }
        }
    }
}

//TODO generated by AI, check later
fn is_windows8_or_greater() -> windows::core::Result<bool> {
    let mut os_info: OSVERSIONINFOW = unsafe { std::mem::zeroed() };
    os_info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;

    unsafe {
        GetVersionExW(&mut os_info)?;
    }

    let r = (os_info.dwMajorVersion > 6)
        || (os_info.dwMajorVersion == 6 && os_info.dwMinorVersion >= 2);
    Ok(r)
}

lazy_static! {
    static ref REGISTERED_CLASS: AtomicBool = AtomicBool::new(false);
}

fn try_register_class() -> windows::core::Result<()> {
    if REGISTERED_CLASS.load(Ordering::Relaxed) {
        return Ok(());
    }

    unsafe {
        let instance = GetModuleHandleA(None)?;
        let cursor = LoadCursorW(None, IDC_ARROW)?;

        let mut style = CS_HREDRAW | CS_VREDRAW | CS_NOCLOSE;

        let enabled = DwmIsCompositionEnabled()?.as_bool();
        if is_windows8_or_greater()? || !enabled {
            style |= CS_OWNDC;
        }

        let window_class = w!("Win32DisplayClass");
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            hCursor: cursor,
            hInstance: instance.into(),
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            ..Default::default()
        };

        let atom = RegisterClassExW(&wc as *const _);
        if atom == 0 {
            return Err(std::io::Error::last_os_error().into());
        }
    }

    REGISTERED_CLASS.store(true, Ordering::Relaxed);
    Ok(())
}

#[derive(Debug)]
pub struct DisplayWindowManager {
    // Shouldn't really be needed
    _message_thread: std::thread::JoinHandle<()>,
    should_exit: Arc<AtomicBool>,
    hwnd: WrappedHWND,

    x: i32,
    y: i32,

    width: u32,
    height: u32,

    render_at_bottom: bool,

    pub(super) obs_display: Option<WrappedObsDisplay>
}

impl DisplayWindowManager {
    pub fn new(
        parent: HWND,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> windows::core::Result<Self> {
        log::trace!("Registering class...");
        try_register_class()?;
        let win8 = is_windows8_or_greater()?;
        let enabled = unsafe { DwmIsCompositionEnabled()?.as_bool() };

        let mut window_style = WS_EX_TRANSPARENT;
        if win8 && enabled {
            window_style |= WS_EX_COMPOSITED;
        }

        let instance = unsafe { GetModuleHandleW(PCWSTR::null())? };

        let class_name = HSTRING::from("Win32DisplayClass");
        let window_name = HSTRING::from("LibObsChildWindowPreview");
        log::trace!("Creating window...");

        log::debug!(
            "Creating window with x: {}, y: {}, width: {}, height: {}",
            x,
            y,
            width,
            height
        );
        let window = unsafe {
            // More at https://github.com/stream-labs/obs-studio-node/blob/4e19d8a61a4dd7744e75ce77624c664e371cbfcf/obs-studio-server/source/nodeobs_display.cpp#L170
            CreateWindowExW(
                WS_EX_LAYERED,
                &class_name,
                &window_name,
                WS_POPUP | WS_VISIBLE, //WS_POPUP,
                x,
                y,
                width as i32,
                height as i32,
                parent,
                None,
                instance,
                None,
            )?
        };

        log::trace!("HWND is {:?}", window);
        if win8 || !enabled {
            log::trace!("Setting attributes alpha...");
            unsafe {
                SetLayeredWindowAttributes(window, COLORREF(0), 255, LWA_ALPHA)?;
            }
        }

        unsafe {
            log::trace!("Setting parent...");
            SetParent(window, parent)?;
            log::trace!("Setting styles...");
            let mut style = GetWindowLongPtrW(window, GWL_STYLE);
            //TODO Check casts here
            style &= !(WS_POPUP.0 as isize);
            style |= WS_CHILD.0 as isize;

            SetWindowLongPtrW(window, GWL_STYLE, style);

            let mut ex_style = GetWindowLongPtrW(window, GWL_EXSTYLE);
            ex_style |= window_style.0 as isize;

            SetWindowLongPtrW(window, GWL_EXSTYLE, ex_style);
        }

        let should_exit = Arc::new(AtomicBool::new(false));

        let tmp = should_exit.clone();
        let message_thread = std::thread::spawn(move || unsafe {
            log::trace!("Starting up message thread...");
            let mut msg = MSG::default();
            while !tmp.load(Ordering::Relaxed) && GetMessageW(&mut msg, None, 0, 0).as_bool() {
                //TODO check if this can really be ignored
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            log::trace!("Exiting message thread...");
        });

        Ok(Self {
            x,
            y,
            width,
            height,
            hwnd: WrappedHWND(window),
            should_exit,
            _message_thread: message_thread,
            render_at_bottom: false,
            obs_display: None
        })
    }

    pub fn get_child_handle(&self) -> HWND {
        self.hwnd.0.clone()
    }
}

impl Drop for DisplayWindowManager {
    fn drop(&mut self) {
        unsafe {
            self.should_exit.store(true, Ordering::Relaxed);

            log::trace!("Destroying window...");
            let res = DestroyWindow(self.hwnd.0);
            if let Err(err) = res {
                log::error!("Failed to destroy window: {:?}", err);
            }
        }
    }
}