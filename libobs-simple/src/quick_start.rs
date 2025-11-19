#[cfg(feature = "auto-bootstrap")]
use libobs_bootstrapper::{ObsBootstrapper, ObsBootstrapperOptions, ObsBootstrapperResult};
use libobs_wrapper::{context::ObsContext, utils::StartupInfo};

/// Quickly start the OBS context with default settings.
///
/// This function will:
/// 1. If the `auto-bootstrap` feature is enabled (default), it will check for an OBS installation
///    and download/update it if necessary.
/// 2. Initialize the `ObsContext` with default `StartupInfo`.
///
/// # Returns
///
/// * `Ok(ObsContext)` - The initialized OBS context.
/// * `Err(anyhow::Error)` - If bootstrapping fails or context initialization fails.
///
/// # Example
///
/// ```rust,no_run
/// use libobs_simple::quick_start::quick_start;
/// let context = quick_start().await.unwrap();
/// ```
pub async fn quick_start() -> anyhow::Result<ObsContext> {
    #[cfg(feature = "auto-bootstrap")]
    {
        let options = ObsBootstrapperOptions::default();
        // We use the console handler by default for quick start
        let result = ObsBootstrapper::bootstrap(&options).await?;

        if let ObsBootstrapperResult::Restart = result {
            println!("OBS has been updated. Restarting application...");
            std::process::exit(0);
        }
    }

    let context = ObsContext::new(StartupInfo::default())?;
    Ok(context)
}
