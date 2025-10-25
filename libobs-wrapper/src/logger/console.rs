use crate::enums::ObsLogLevel;

use super::ObsLogger;

#[derive(Debug)]
pub struct ConsoleLogger {
    _private: (),
}

impl ConsoleLogger {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl ObsLogger for ConsoleLogger {
    fn log(&mut self, level: ObsLogLevel, msg: String) {
        let level_str = format!("{:?}", level);

        #[cfg(feature = "color-logger")]
        let level_str = level.colorize(&level_str);

        println!("[{}] {}", level_str, msg);
    }
}
