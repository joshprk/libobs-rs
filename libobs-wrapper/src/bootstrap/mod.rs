use std::{env, path::PathBuf, process};

use anyhow::Context;
use async_stream::stream;
use async_trait::async_trait;
use download::DownloadStatus;
use extract::ExtractStatus;
use futures_core::Stream;
use futures_util::{pin_mut, StreamExt};
use lazy_static::lazy_static;
use libobs::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER};
use tokio::{fs::File, io::AsyncWriteExt, process::Command};

use crate::context::ObsContext;

mod download;
mod extract;
mod github_types;
mod options;
pub mod status_handler;
mod version;

pub use options::ObsBootstrapperOptions;

pub enum BootstrapStatus {
    /// Downloading status (first is progress from 0.0 to 1.0 and second is message)
    Downloading(f32, String),

    /// Extracting status (first is progress from 0.0 to 1.0 and second is message)
    Extracting(f32, String),
    Error(anyhow::Error),
    /// The application must be restarted to use the new version of OBS.
    /// This is because the obs.dll file is in use by the application and can not be replaced while running.
    /// Therefore the "updater" is spawned to watch for the application to exit and rename the "obs_new.dll" file to "obs.dll".
    /// The updater will start the application again with the same arguments as the original application.
    /// Call `ObsContext::spawn_updater()`
    RestartRequired,
}

#[async_trait]
/// A trait for bootstrapping OBS Studio.
///
/// This trait provides functionality to download, extract, and set up OBS Studio
/// for use with libobs-rs. It also handles updates to OBS when necessary.
///
/// If you want to use this bootstrapper to also install required OBS binaries at runtime,
/// do the following:
/// - Add a `obs.dll` file to your executable directory. This file will be replaced by the obs installer.
/// Recommended to use is the a dll dummy (found [here](https://github.com/sshcrack/libobs-builds/releases), make sure you use the correct OBS version)
/// and rename it to `obs.dll`.
/// - Call `ObsRuntime::new()` at the start of your application. Options must be configured. For more documentation look at the [tauri example app](https://github.com/joshprk/libobs-rs/tree/main/examples/tauri-app). This will download the latest version of OBS and extract it in the executable directory.
/// - If BootstrapStatus::RestartRequired is returned, call `ObsContext::spawn_updater()` to spawn the updater process.
/// - Exit the application. The updater process will wait for the application to exit and rename the `obs_new.dll` file to `obs.dll` and restart your application with the same arguments as before.
///
/// [Example project](https://github.com/joshprk/libobs-rs/tree/main/examples/download-at-runtime)
pub trait ObsBootstrap {
    fn is_valid_installation() -> anyhow::Result<bool>;
    fn is_update_available() -> anyhow::Result<bool>;
}

lazy_static! {
    pub(crate) static ref LIBRARY_OBS_VERSION: String = format!(
        "{}.{}.{}",
        LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER
    );
}

pub const UPDATER_SCRIPT: &'static str = include_str!("./updater.ps1");

fn get_obs_dll_path() -> anyhow::Result<PathBuf> {
    let executable = env::current_exe()?;
    let obs_dll = executable
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory"))?
        .join("obs.dll");

    Ok(obs_dll)
}

#[cfg_attr(feature="blocking", remove_async_await::remove_async_await)]
pub(crate) async fn bootstrap(
    options: &options::ObsBootstrapperOptions,
) -> anyhow::Result<Option<impl Stream<Item = BootstrapStatus>>> {
    let repo = options.repository.to_string();

    log::trace!("Checking for update...");
    let update = if options.update {
        ObsContext::is_update_available()?
    } else {
        ObsContext::is_valid_installation()?
    };

    if !update {
        log::debug!("No update needed.");
        return Ok(None);
    }

    let options = options.clone();
    Ok(Some(stream! {
        log::debug!("Downloading OBS from {}", repo);
        let download_stream = download::download_obs(&repo).await;
        if let Err(err) = download_stream {
            yield BootstrapStatus::Error(err);
            return;
        }

        let download_stream = download_stream.unwrap();
        pin_mut!(download_stream);

        let mut file = None;
        while let Some(item) = download_stream.next().await {
            match item {
                DownloadStatus::Error(err) => {
                    yield BootstrapStatus::Error(err);
                    return;
                }
                DownloadStatus::Progress(progress, message) => {
                    yield BootstrapStatus::Downloading(progress, message);
                }
                DownloadStatus::Done(path) => {
                    file = Some(path)
                }
            }
        }

        let archive_file = file.ok_or_else(|| anyhow::anyhow!("OBS Archive could not be downloaded."));
        if let Err(err) = archive_file {
            yield BootstrapStatus::Error(err);
            return;
        }

        log::debug!("Extracting OBS to {:?}", archive_file);
        let archive_file = archive_file.unwrap();
        let extract_stream = extract::extract_obs(&archive_file).await;
        if let Err(err) = extract_stream {
            yield BootstrapStatus::Error(err);
            return;
        }

        let extract_stream = extract_stream.unwrap();
        pin_mut!(extract_stream);

        while let Some(item) = extract_stream.next().await {
            match item {
                ExtractStatus::Error(err) => {
                    yield BootstrapStatus::Error(err);
                    return;
                }
                ExtractStatus::Progress(progress, message) => {
                    yield BootstrapStatus::Extracting(progress, message);
                }
            }
        }

        let r = spawn_updater(options).await;
        if let Err(err) = r {
            yield BootstrapStatus::Error(err);
            return;
        }

        yield BootstrapStatus::RestartRequired;
    }))
}

pub(crate) async fn spawn_updater(options: options::ObsBootstrapperOptions) -> anyhow::Result<()> {
    let pid = process::id();
    let args = env::args().collect::<Vec<_>>();
    // Skip the first argument which is the executable path
    let args = args.into_iter().skip(1).collect::<Vec<_>>();

    let updater_path = env::temp_dir().join("libobs_updater.ps1");
    let mut updater_file = File::create(&updater_path)
        .await
        .context("Creating updater script")?;

    updater_file
        .write_all(UPDATER_SCRIPT.as_bytes())
        .await
        .context("Writing updater script")?;

    let mut command = Command::new("powershell");
    command
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-NoProfile")
        .arg("-WindowStyle")
        .arg("Hidden")
        .arg("-File")
        .arg(updater_path)
        .arg("-processPid")
        .arg(pid.to_string())
        .arg("-binary")
        .arg(env::current_exe()?.to_string_lossy().to_string());

    if options.restart_after_update {
        command.arg("-restart");
    }

    // Add arguments as an array
    if !args.is_empty() {
        command.arg("-arguments");
        command.arg(format!("({})", args.join(",").replace("\"", "`\"")));
    }

    command.spawn().context("Spawning updater process")?;

    Ok(())
}

#[async_trait]
impl ObsBootstrap for ObsContext {
    fn is_valid_installation() -> anyhow::Result<bool> {
        let installed = version::get_installed_version(&get_obs_dll_path()?)?;

        Ok(installed.is_some())
    }

    fn is_update_available() -> anyhow::Result<bool> {
        let installed = version::get_installed_version(&get_obs_dll_path()?)?;
        if installed.is_none() {
            return Ok(true);
        }

        let installed = installed.unwrap();
        Ok(version::should_update(&installed)?)
    }
}
