use std::fmt::Debug;

//NOTE: Maybe do not require to implement Debug here?
pub trait ObsBootstrapStatusHandler: Debug + Send + Sync {
    fn handle_downloading(&mut self, progress: f32, message: String) -> anyhow::Result<()>;
    fn handle_extraction(&mut self, progress: f32, message: String) -> anyhow::Result<()>;
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[derive(Debug)]
pub struct ObsBootstrapConsoleHandler;

#[cfg_attr(coverage_nightly, coverage(off))]
impl Default for ObsBootstrapConsoleHandler {
    fn default() -> Self {
        Self
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
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
