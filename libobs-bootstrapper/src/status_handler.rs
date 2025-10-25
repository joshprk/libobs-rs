use std::fmt::Debug;

//NOTE: Maybe do not require to implement Debug here?
pub trait ObsBootstrapStatusHandler: Debug + Send + Sync {
    fn handle_downloading(&mut self, progress: f32, message: String) -> anyhow::Result<()>;
    fn handle_extraction(&mut self, progress: f32, message: String) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct ObsBootstrapConsoleHandler;

impl Default for ObsBootstrapConsoleHandler {
    fn default() -> Self {
        Self
    }
}

impl ObsBootstrapStatusHandler for ObsBootstrapConsoleHandler {
    fn handle_downloading(&mut self, progress: f32, message: String) -> anyhow::Result<()> {
        println!("Downloading: {}% - {}", progress * 100.0, message);
        Ok(())
    }

    fn handle_extraction(&mut self, progress: f32, message: String) -> anyhow::Result<()> {
        println!("Extracting: {}% - {}", progress * 100.0, message);
        Ok(())
    }
}
