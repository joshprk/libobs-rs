//! # OBS Window Helper
//! This crate provides necessary information about windows that could be used
//! so they can be captured with the `window_capture` or `game_capture` source in OBS.
//! <br> The function you probably want to use is `get_all_windows` which returns a list of `WindowInfo` structs.


#[cfg(not(windows))]
compile_error!("This library only supports windows!");

#[cfg(not(target_pointer_width = "64"))]
compile_error!("compilation is only allowed for 64-bit targets");

mod game;
mod util;
mod monitor;
mod window;

pub use util::*;
#[cfg(test)]
mod test;

pub use game::*;
pub use helper::*;
use win_iterator::{first_window, next_window};
use windows::Win32::{Foundation::HWND, System::Console::GetConsoleWindow};

/// Retrieves information about all windows based on the specified search mode and game check flag.
///
/// # Arguments
///
/// * `mode` - The search mode to use for window enumeration.
/// * `check_game` - A flag indicating wether a `game_capture` or a `window_capture` is used
///
/// # Returns
///
/// A `Result` containing a vector of `WindowInfo` structs representing the retrieved window information, or an `anyhow::Error` if an error occurs.
pub fn get_all_windows(mode: WindowSearchMode) -> anyhow::Result<Vec<WindowInfo>> {
    let mut use_find_window_ex = false;

    let mut parent = None as Option<HWND>;
    let window = unsafe { first_window(mode, &mut parent, &mut use_find_window_ex)? };
    let mut window = Some(window);

    let curr = unsafe { GetConsoleWindow() };

    let mut out = Vec::new();
    while window.is_some_and(|e| !e.is_invalid()) {
        let w = window.unwrap();
        if curr != w {
            let res = get_window_info(w);
            if let Ok(info) = res {
                out.push(info);
            } else {
                //eprintln!("Error: {:?}", res.err().unwrap());
            }
        }

        unsafe {
            window = next_window(window, mode, &mut parent, use_find_window_ex)?;
        }
    }

    Ok(out)
}

