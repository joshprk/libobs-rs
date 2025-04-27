use std::{sync::Arc, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use libobs_wrapper::{
    bootstrap::{
        ObsBootstrapperOptions,
        status_handler::ObsBootstrapStatusHandler,
    },
    context::ObsContext,
};

#[derive(Debug, Clone)]
struct ObsBootstrapProgress(Arc<ProgressBar>);

impl ObsBootstrapProgress {
    pub fn new() -> Self {
        let bar = ProgressBar::new(200).with_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap(),
        );

        bar.set_message("Initializing bootstrapper...");
        bar.enable_steady_tick(Duration::from_millis(50));

        Self(Arc::new(bar))
    }

    pub fn done(&self) {
        self.0.finish();
    }
}

#[async_trait::async_trait]
impl ObsBootstrapStatusHandler for ObsBootstrapProgress {
    async fn handle_downloading(&mut self, prog: f32, msg: String) -> anyhow::Result<()> {
        self.0.set_message(msg);
        self.0.set_position((prog * 100.0) as u64);
        Ok(())
    }
    async fn handle_extraction(&mut self, prog: f32, msg: String) -> anyhow::Result<()> {
        self.0.set_message(msg);
        self.0.set_position(100 + (prog * 100.0) as u64);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let handler = ObsBootstrapProgress::new();

    let context = ObsContext::builder()
        .enable_bootstrapper(handler.clone(), ObsBootstrapperOptions::default())
        .start()
        .await
        .unwrap();

    let context = match context {
        libobs_wrapper::context::ObsContextReturn::Done(c) => c,
        libobs_wrapper::context::ObsContextReturn::Restart => {
            println!("OBS has been downloaded and extracted. The application will now restart.");
            return;
        }
    };

    handler.done();

    println!("Done");
    // Use the context here
    // For example creating new obs data
    context.data().await.unwrap();
}
