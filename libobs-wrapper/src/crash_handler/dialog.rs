use arboard::Clipboard;
use dialog::{Choice, DialogBox};

use super::ObsCrashHandler;

pub struct DialogCrashHandler {
    _private: (),
}

impl DialogCrashHandler {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl ObsCrashHandler for DialogCrashHandler {
    fn handle_crash(&self, message: String) {
        eprintln!("OBS crashed: {}", message);
        let res =
            dialog::Question::new("OBS has crashed. Do you want to copy the error to clipboard?")
                .title("OBS Crash Handler")
                .show();

        if let Err(e) = res {
            eprintln!("Failed to show crash handler dialog: {e:?}");
            return;
        }

        let res = res.unwrap();
        if res == Choice::No {
            return;
        }

        let clipboard = Clipboard::new();
        if let Err(e) = clipboard {
            eprintln!("Failed to create clipboard: {e:?}");
            return;
        }

        let mut clipboard = clipboard.unwrap();
        if let Err(e) = clipboard.set_text(message.clone()) {
            eprintln!("Failed to copy crash message to clipboard: {e:?}");
            return;
        }
    }
}
