use crate::enums::ObsLogLevel;

use super::ObsLogger;

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
        println!("[{:?}] {}", level, msg);
    }
}
