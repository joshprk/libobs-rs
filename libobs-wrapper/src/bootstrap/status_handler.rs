use std::fmt::Debug;

use async_trait::async_trait;

//NOTE: Maybe do not require to implement Debug here?
#[async_trait]
pub trait ObsBootstrapStatusHandler : Debug {
    async fn handle_downloading(&self, progress: f32, message: String) -> anyhow::Result<()>;
    async fn handle_extraction(&self, progress: f32, message: String) -> anyhow::Result<()>;
}


#[derive(Debug)]
pub struct ObsBootstrapConsoleHandler;

#[async_trait]
impl ObsBootstrapStatusHandler for ObsBootstrapConsoleHandler {
    async fn handle_downloading(&self, progress: f32, message: String) -> anyhow::Result<()> {
        println!("Downloading: {}% - {}", progress * 100.0, message);
        Ok(())
    }

    async fn handle_extraction(&self, progress: f32, message: String) -> anyhow::Result<()> {
        println!("Extracting: {}% - {}", progress * 100.0, message);
        Ok(())
    }
}
