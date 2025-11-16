use std::{env::temp_dir, path::PathBuf};

use anyhow::Context;
use async_stream::stream;
use futures_core::Stream;
use futures_util::StreamExt;
use libobs::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER};
use semver::Version;
use sha2::{Digest, Sha256};
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

use super::{LIBRARY_OBS_VERSION, github_types};

pub enum DownloadStatus {
    Error(anyhow::Error),
    Progress(f32, String),
    Done(PathBuf),
}

pub(crate) async fn download_obs(repo: &str) -> anyhow::Result<impl Stream<Item = DownloadStatus>> {
    // Fetch latest OBS release
    let client = reqwest::ClientBuilder::new()
        .user_agent("libobs-rs")
        .build()?;

    let releases_url = format!("https://api.github.com/repos/{}/releases", repo);
    let releases: github_types::Root = client.get(&releases_url).send().await?.json().await?;

    let mut possible_versions = vec![];
    for release in releases {
        // For macOS, use official OBS releases (tag without "obs-build-" prefix)
        let tag = if cfg!(target_os = "macos") {
            release.tag_name.clone()
        } else {
            release.tag_name.replace("obs-build-", "")
        };
        
        // Remove leading 'v' if present for version parsing
        let tag_for_parse = tag.trim_start_matches('v');
        let version = match Version::parse(tag_for_parse) {
            Ok(v) => v,
            Err(_) => {
                log::debug!("Skipping release with unparseable version: {}", tag);
                continue;
            }
        };

        // The minor and major version must be the same, patches shouldn't have braking changes
        if version.major == LIBOBS_API_MAJOR_VER as u64
            && version.minor == LIBOBS_API_MINOR_VER as u64
        {
            possible_versions.push(release);
        }
    }

    let latest_version = possible_versions
        .iter()
        .max_by_key(|r| &r.published_at)
        .context(format!(
            "Finding a matching obs version for {}",
            *LIBRARY_OBS_VERSION
        ))?;

    // Platform-specific asset selection (use target platform for cross-compilation)
    let target_os = std::env::var("CARGO_CFG_TARGET_OS")
        .unwrap_or_else(|_| std::env::consts::OS.to_string());
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_else(|_| std::env::consts::ARCH.to_string());
    
    let (asset_extension, file_extension) = if target_os == "macos" {
        let arch = if target_arch == "x86_64" { "Intel" } else { "Apple" };
        (format!("macOS-{}.dmg", arch), "dmg")
    } else {
        (".7z".to_string(), "7z")
    };

    let archive_url = latest_version
        .assets
        .iter()
        .find(|a| a.name.contains(&asset_extension) && !a.name.contains("dSYM"))
        .context(format!("Finding {} asset with pattern: {}", file_extension, asset_extension))?
        .browser_download_url
        .clone();

    // Hash verification is optional for macOS (DMG has built-in verification)
    let hash_url = if cfg!(target_os = "macos") {
        None
    } else {
        Some(
            latest_version
                .assets
                .iter()
                .find(|a| a.name.ends_with(".sha256"))
                .context("Finding sha256 asset")?
                .browser_download_url
                .clone(),
        )
    };

    let res = client.get(archive_url).send().await?;
    let length = res.content_length().unwrap_or(0);

    let mut bytes_stream = res.bytes_stream();

    let path = PathBuf::new()
        .join(temp_dir())
        .join(format!("{}.{}", Uuid::new_v4(), file_extension));
    let mut tmp_file = File::create_new(&path)
        .await
        .context("Creating temporary file")?;

    let mut curr_len = 0;
    let mut hasher = Sha256::new();
    Ok(stream! {
        yield DownloadStatus::Progress(0.0, "Downloading OBS".to_string());
        while let Some(chunk) = bytes_stream.next().await {
            let chunk = chunk.context("Retrieving data from stream");
            if let Err(e) = chunk {
                yield DownloadStatus::Error(e);
                return;
            }

            let chunk = chunk.unwrap();
            hasher.update(&chunk);
            let r = tmp_file.write_all(&chunk).await.context("Writing to temporary file");
            if let Err(e) = r {
                yield DownloadStatus::Error(e);
                return;
            }

            curr_len = std::cmp::min(curr_len + chunk.len() as u64, length);
            yield DownloadStatus::Progress(curr_len as  f32 / length as f32, "Downloading OBS".to_string());
        }

        // Hash verification (only for non-macOS platforms)
        if let Some(hash_url) = hash_url {
            // Getting remote hash
            let remote_hash = client.get(hash_url).send().await.context("Fetching hash");
            if let Err(e) = remote_hash {
                yield DownloadStatus::Error(e);
                return;
            }

            let remote_hash = remote_hash.unwrap().text().await.context("Reading hash");
            if let Err(e) = remote_hash {
                yield DownloadStatus::Error(e);
                return;
            }

            let remote_hash = remote_hash.unwrap();
            let remote_hash = hex::decode(remote_hash.trim()).context("Decoding hash");
            if let Err(e) = remote_hash {
                yield DownloadStatus::Error(e);
                return;
            }

            let remote_hash = remote_hash.unwrap();

            // Calculating local hash
            let local_hash = hasher.finalize();
            if local_hash.to_vec() != remote_hash {
                yield DownloadStatus::Error(anyhow::anyhow!("Hash mismatch"));
                return;
            }

            log::info!("Hashes match");
        } else {
            log::info!("Skipping hash verification for macOS DMG (has built-in verification)");
        }

        yield DownloadStatus::Done(path);
    })
}
