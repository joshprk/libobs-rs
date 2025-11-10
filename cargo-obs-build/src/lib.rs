use git::{fetch_latest_patch_release, fetch_release, ReleaseInfo};
use lock::{acquire_lock, wait_for_lock};
use log::{debug, info, warn};
use metadata::{fetch_latest_release_tag, get_meta_info};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};
use util::{copy_to_dir, delete_all_except};
#[cfg(target_family = "windows")]
use walkdir::WalkDir;

use lib_version::get_lib_obs_version;

#[cfg(target_family = "windows")]
use download::download_binaries;
#[cfg(target_family = "windows")]
use zip::ZipArchive;

mod download;
mod git;
mod lib_version;
mod lock;
mod metadata;
mod util;

/// Configuration options for building OBS binaries
#[derive(Debug, Clone)]
pub struct ObsBuildConfig {
    /// The directory the OBS Studio binaries should be copied to
    pub out_dir: PathBuf,
    
    /// The location where the OBS Studio sources should be cloned to
    pub cache_dir: PathBuf,
    
    /// The github repository to clone OBS Studio from
    pub repo_id: String,
    
    /// If this is specified, the specified zip file will be used instead of downloading the latest release
    /// This is useful for testing purposes, but it is not recommended to use this in production
    pub override_zip: Option<PathBuf>,
    
    /// When this flag is active, the cache will be cleared and a new build will be started
    pub rebuild: bool,
    
    /// If the browser should be included in the build
    pub browser: bool,
    
    /// The tag of the OBS Studio release to build.
    /// If none is specified, the matching release for the libobs crate will be used.
    /// Use `latest` for the latest obs release. If a version in the `workspace.metadata` is set, that version will be used.
    pub tag: Option<String>,
    
    /// If the compatibility check should be skipped
    pub skip_compatibility_check: bool,
}

impl Default for ObsBuildConfig {
    fn default() -> Self {
        Self {
            out_dir: PathBuf::from("obs-out"),
            cache_dir: PathBuf::from("obs-build"),
            repo_id: "obsproject/obs-studio".to_string(),
            override_zip: None,
            rebuild: false,
            browser: false,
            tag: None,
            skip_compatibility_check: false,
        }
    }
}

/// Simple installation method for use in build scripts.
///
/// This automatically:
/// - Determines the target directory from the OUT_DIR environment variable
/// - Uses default cache directory ("obs-build")
/// - Auto-detects the OBS version from the libobs crate
/// - Handles all caching and locking
///
/// # Example
///
/// ```rust,no_run
/// fn main() {
///     cargo_obs_build::install().expect("Failed to install OBS binaries");
/// }
/// ```
///
/// This is equivalent to calling `build_obs_binaries()` with default configuration
/// and the out_dir set to `$OUT_DIR/../../obs-binaries`.
pub fn install() -> anyhow::Result<()> {
    use std::env;
    
    let out_dir = env::var("OUT_DIR")
        .map_err(|_| anyhow::anyhow!("OUT_DIR environment variable not set. This function should only be called from a build script."))?;
    
    let target_dir = PathBuf::from(&out_dir)
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.join("obs-binaries"))
        .ok_or_else(|| anyhow::anyhow!("Failed to determine target directory from OUT_DIR"))?;
    
    let config = ObsBuildConfig {
        out_dir: target_dir,
        ..Default::default()
    };
    
    build_obs_binaries(config)
}

/// Build and install OBS binaries according to the provided configuration
///
/// This is the main entry point for the library. It handles:
/// - Version detection from the libobs crate
/// - Downloading and extracting OBS binaries
/// - Caching to avoid re-downloads
/// - Locking to prevent concurrent builds
/// - Copying binaries to the target directory
pub fn build_obs_binaries(config: ObsBuildConfig) -> anyhow::Result<()> {
    let ObsBuildConfig {
        mut cache_dir,
        repo_id,
        out_dir,
        rebuild,
        browser,
        mut tag,
        override_zip,
        skip_compatibility_check,
    } = config;

    let mut obs_ver = None;
    if tag.is_none() {
        obs_ver = Some(get_lib_obs_version()?);
        let (major, minor, patch) = obs_ver.as_ref().unwrap();
        let lib_tag = format!("{}.{}.{}", major, minor, patch);

        // Check if a newer version of libobs (same major/minor, higher patch) exists in releases.
        // If found, use that tag; otherwise fall back to the crate version tag.
        match fetch_latest_patch_release(&repo_id, *major, *minor) {
            Ok(Some(found_tag)) => {
                let parts: Vec<&str> = found_tag.trim_start_matches('v').split('.').collect();
                let found_patch = parts
                    .get(2)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0);
                if found_patch > *patch {
                    info!(
                        "Found newer libobs binaries release {} (crate: {}). Using {}",
                        found_tag, lib_tag, found_tag
                    );
                    tag = Some(found_tag);
                } else {
                    // no newer patch found -> use crate version
                    tag = Some(lib_tag);
                }
            }
            Ok(None) => {
                // none found -> use crate version
                tag = Some(lib_tag);
            }
            Err(e) => {
                // On error, log debug and fall back to crate version
                warn!("Failed to check for newer compatible libobs release: {}", e);
                tag = Some(lib_tag);
            }
        }
    }

    let mut tag = tag.unwrap();
    let target_out_dir = PathBuf::new().join(&out_dir);
    get_meta_info(&mut cache_dir, &mut tag)?;

    let tag = if tag.trim() == "latest" {
        fetch_latest_release_tag(&repo_id)?
    } else {
        tag
    };

    if !skip_compatibility_check {
        let (major, minor, patch) = if let Some(v) = obs_ver {
            v
        } else {
            get_lib_obs_version()?
        };

        info!(
            "Detected libobs crate version: {}.{}.{}",
            major, minor, patch
        );
        let tag_parts: Vec<&str> = tag.trim_start_matches('v').split('.').collect();
        let tag_parts = tag_parts
            .iter()
            .map(|e| e.parse::<u32>().unwrap_or(0))
            .collect::<Vec<u32>>();

        if tag_parts.len() < 3 {
            info!(
                "Warning: Could not determine libobs compatibility, tag does not have 3 parts"
            );
        } else {
            let (tag_major, tag_minor, tag_patch) = (tag_parts[0], tag_parts[1], tag_parts[2]);
            if major != tag_major || minor != tag_minor {
                use log::warn;

                warn!(
                    "libobs (crate) version {}.{}.{} may not be compatible with libobs (binaries) {}.{}.{}",
                    major, minor, patch, tag_major, tag_minor, tag_patch
                );
                warn!(
                    "Set the `libobs-version` in `[workspace.metadata]` to {}.{}.{} to avoid runtime issues",
                    major, minor, patch
                );
            } else {
                info!(
                    "libobs (crate) version {}.{}.{} should be compatible with libobs (binaries) {}.{}.{}",
                    major, minor, patch, tag_major, tag_minor, tag_patch
                );
            }
        }
    }

    let repo_dir = cache_dir.join(&tag);
    let repo_exists = repo_dir.is_dir();

    if !repo_exists {
        fs::create_dir_all(&repo_dir)?;
    }

    let build_out = repo_dir.join("build_out");
    let lock_file = cache_dir.join(format!("{}.lock", tag));
    let success_file = repo_dir.join(".success");

    wait_for_lock(&lock_file)?;

    if !success_file.is_file() || rebuild {
        let lock = acquire_lock(&lock_file)?;
        if repo_exists || rebuild {
            debug!("Cleaning up old build...");
            delete_all_except(&repo_dir, None)?;
        }

        debug!("Fetching {} version of OBS Studio...", tag);

        let release = fetch_release(&repo_id, &Some(tag.clone()))?;
        build_obs(release, &build_out, browser, override_zip)?;

        File::create(&success_file)?;
        drop(lock);
    }

    info!(
        "Copying files from {} to {}",
        build_out.display(),
        target_out_dir.display()
    );
    copy_to_dir(&build_out, &target_out_dir, None)?;

    info!("Done!");

    Ok(())
}

fn build_obs(
    _release: ReleaseInfo,
    _build_out: &Path,
    _include_browser: bool,
    _override_zip: Option<PathBuf>,
) -> anyhow::Result<()> {
    #[cfg(not(target_family = "windows"))]
    {
        anyhow::bail!("Unsupported platform: OBS binaries are only available for Windows");
    }

    #[cfg(target_family = "windows")]
    {
        fs::create_dir_all(_build_out)?;

        let obs_path = if let Some(e) = _override_zip {
            e
        } else {
            download_binaries(_build_out, &_release)?
        };

        let obs_archive = File::open(&obs_path)?;
        let mut archive = ZipArchive::new(&obs_archive)?;

        info!("Extracting OBS Studio binaries...");
        archive.extract(_build_out)?;
        let bin_path = _build_out.join("bin").join("64bit");
        copy_to_dir(&bin_path, _build_out, None)?;
        fs::remove_dir_all(_build_out.join("bin"))?;

        clean_up_files(_build_out, _include_browser)?;

        fs::remove_file(&obs_path)?;

        Ok(())
    }
}

#[cfg(target_family = "windows")]
fn clean_up_files(build_out: &Path, include_browser: bool) -> anyhow::Result<()> {
    let mut to_exclude = vec![
        "obs64",
        "frontend",
        "obs-webrtc",
        "obs-websocket",
        "decklink",
        "obs-scripting",
        "qt6",
        "qminimal",
        "qwindows",
        "imageformats",
        "obs-studio",
        "aja-output-ui",
        "obs-vst",
    ];

    if !include_browser {
        to_exclude.append(&mut vec![
            "obs-browser",
            "obs-browser-page",
            "chrome_",
            "resources",
            "cef",
            "snapshot",
            "locales",
        ]);
    }

    info!("Cleaning up unnecessary files...");
    for entry in WalkDir::new(build_out).into_iter().flatten() {
        let path = entry.path();
        if to_exclude.iter().any(|e| {
            path.file_name().is_some_and(|x| {
                let x_l = x.to_string_lossy().to_lowercase();
                x_l.contains(e) || x_l == *e
            })
        }) {
            debug!("Deleting: {}", path.display());
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    }

    Ok(())
}
