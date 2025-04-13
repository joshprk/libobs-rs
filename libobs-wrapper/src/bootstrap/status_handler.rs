use std::fmt::Debug;

use async_trait::async_trait;

//NOTE: Maybe do not require to implement Debug here?
#[async_trait]
pub trait ObsBootstrapStatusHandler : Debug + Send + Sync {
    async fn handle_downloading(&mut self, progress: f32, message: String) -> anyhow::Result<()>;
    async fn handle_extraction(&mut self, progress: f32, message: String) -> anyhow::Result<()>;
}


#[derive(Debug)]
pub struct ObsBootstrapConsoleHandler;

impl Default for ObsBootstrapConsoleHandler {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl ObsBootstrapStatusHandler for ObsBootstrapConsoleHandler {
    async fn handle_downloading(&mut self, progress: f32, message: String) -> anyhow::Result<()> {
        println!("Downloading: {}% - {}", progress * 100.0, message);
        Ok(())
    }

    async fn handle_extraction(&mut self, progress: f32, message: String) -> anyhow::Result<()> {
        println!("Extracting: {}% - {}", progress * 100.0, message);
        Ok(())
    }
}
