use std::fmt::Debug;

//NOTE: Maybe do not require to implement Debug here?
#[cfg_attr(not(feature="blocking"), async_trait::async_trait)]
pub trait ObsBootstrapStatusHandler : Debug + Send + Sync {
    #[cfg_attr(feature="blocking", remove_async_await::remove_async_await)]
    async fn handle_downloading(&mut self, progress: f32, message: String) -> anyhow::Result<()>;
    #[cfg_attr(feature="blocking", remove_async_await::remove_async_await)]
    async fn handle_extraction(&mut self, progress: f32, message: String) -> anyhow::Result<()>;
}


#[derive(Debug)]
pub struct ObsBootstrapConsoleHandler;

impl Default for ObsBootstrapConsoleHandler {
    fn default() -> Self {
        Self
    }
}

#[cfg_attr(not(feature="blocking"), async_trait::async_trait)]
impl ObsBootstrapStatusHandler for ObsBootstrapConsoleHandler {
    #[cfg_attr(feature="blocking", remove_async_await::remove_async_await)]
    async fn handle_downloading(&mut self, progress: f32, message: String) -> anyhow::Result<()> {
        println!("Downloading: {}% - {}", progress * 100.0, message);
        Ok(())
    }

    #[cfg_attr(feature="blocking", remove_async_await::remove_async_await)]
    async fn handle_extraction(&mut self, progress: f32, message: String) -> anyhow::Result<()> {
        println!("Extracting: {}% - {}", progress * 100.0, message);
        Ok(())
    }
}
