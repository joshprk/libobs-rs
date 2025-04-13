use libobs_wrapper::bootstrap::status_handler::ObsBootstrapStatusHandler;
use tauri::{AppHandle, Emitter};

#[derive(Debug)]
pub struct ObsTauriStatusHandler {
    pub handle: AppHandle
}

#[async_trait::async_trait]
impl ObsBootstrapStatusHandler for ObsTauriStatusHandler {
    async fn handle_downloading(&mut self, progress: f32, message: String) -> anyhow::Result<()> {
        self.handle.emit("download_status", (progress, message))?;
        Ok(())
    }

    async fn handle_extraction(&mut self, progress: f32, message: String) -> anyhow::Result<()> {
        self.handle.emit("extraction_status", (progress, message))?;
        Ok(())
    }
}