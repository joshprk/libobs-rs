use std::{fs::File, path::Path};

use chrono::Local;

use super::ObsLogger;

/// A logger that writes logs to a file
pub struct FileLogger {
    file: File,
}

impl FileLogger {
    pub fn from_dir(dir: &Path) -> anyhow::Result<Self> {
        let current_local = Local::now();
        let custom_format = current_local.format("%Y-%m-%d %H:%M:%S");

        Ok(Self {
            file: File::create(dir.join(format!("obs-{}.log", custom_format)))?,
        })
    }

    pub fn from_file(file: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            file: File::create(file)?,
        })
    }
}

impl ObsLogger for FileLogger {
    fn log(&mut self, level: crate::enums::ObsLogLevel, msg: String) {
        use std::io::Write;
        writeln!(self.file, "[{:?}] {}", level, msg).unwrap();
    }
}
