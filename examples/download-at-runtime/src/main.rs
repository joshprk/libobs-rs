use std::time::Duration;

use futures_util::{StreamExt, pin_mut};
use indicatif::{ProgressBar, ProgressStyle};
use libobs_wrapper::{
    bootstrap::{BootstrapStatus, ObsBootstrap},
    context::ObsContext,
};

#[tokio::main]
async fn main() {
    let bar = ProgressBar::new(200).with_style(ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    ).unwrap());

    bar.set_message("Initializing bootstrapper...");
    bar.enable_steady_tick(Duration::from_millis(50));
    let stream = ObsContext::bootstrap(Default::default())
        .await
        .expect("Failed to download or initialize OBS");

    pin_mut!(stream);
    while let Some(status) = stream.next().await {
        match status {
            BootstrapStatus::Downloading(prog, msg) => {
                bar.set_message(msg);
                bar.set_position((prog * 100.0) as u64);
            }
            BootstrapStatus::Extracting(prog, msg) => {
                bar.set_message(msg);
                bar.set_position(100 + (prog * 100.0) as u64);
            }
            BootstrapStatus::Error(err) => eprintln!("Bootstrap error: {}", err),
            BootstrapStatus::RestartRequired => {
                bar.set_message("Restart required");
                bar.finish();
                println!("OBS has been downloaded and extracted. Please restart the application.");
                ObsContext::spawn_updater().await.unwrap();
                return;
            },
        }
    }

    bar.finish();
    println!("Done");
    let _context = ObsContext::new(Default::default()).expect("Failed to create OBS context");
}
