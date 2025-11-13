use std::{
    env::current_exe,
    path::{Path, PathBuf},
    pin::Pin,
};

use async_stream::stream;
use futures_core::Stream;
use futures_util::{StreamExt, pin_mut};
use sevenz_rust::{Password, SevenZReader, default_entry_extract_fn};
use tokio::task;
pub enum ExtractStatus {
    Error(anyhow::Error),
    Progress(f32, String),
}

type ExtractStream = Pin<Box<dyn Stream<Item = ExtractStatus> + Send>>;

pub(crate) async fn extract_obs(
    archive_file: &Path,
) -> anyhow::Result<ExtractStream> {
    log::info!("Extracting OBS at {}", archive_file.display());

    let path = PathBuf::from(archive_file);

    let destination = current_exe()?;
    let destination = destination
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Should be able to get parent of exe"))?
        .join("obs_new");

    // Platform-specific extraction
    #[cfg(target_os = "macos")]
    {
        if path.extension().and_then(|s| s.to_str()) == Some("dmg") {
            return extract_dmg(&path, &destination).await;
        }
    }

    //TODO delete old obs dlls and plugins
    let dest = destination.clone();
    let stream = stream! {
        yield Ok((0.0, "Reading file...".to_string()));
        let mut sz = SevenZReader::open(&path, Password::empty())?;
        let (tx, mut rx) = tokio::sync::mpsc::channel(5);

        let total = sz.archive().files.len() as f32;
        if !dest.exists() {
            std::fs::create_dir_all(&dest)?;
        }

        let mut curr = 0;
        let mut r = task::spawn_blocking(move || {
            sz.for_each_entries(|entry, reader| {
                curr += 1;
                tx.blocking_send((curr as f32 / total, format!("Extracting {}", entry.name()))).unwrap();

                let dest_path = dest.join(entry.name());

                default_entry_extract_fn(entry, reader, &dest_path)
            })?;

            Result::<_, anyhow::Error>::Ok((1.0, "Extraction done".to_string()))
        });

        loop {
            tokio::select! {
                m = rx.recv() => {
                    match m {
                        Some(e) => yield Ok(e),
                        None => break
                    }
                },
                res = &mut r => {
                    match res {
                        Ok(e) => yield e,
                        Err(e) => {
                            yield Err(e.into());
                        }
                    }

                    break;
                }
            }
        }

        yield Ok((1.0, "Extraction done".to_string()));
    };

    Ok(Box::pin(stream! {
            pin_mut!(stream);
            while let Some(status) = stream.next().await {
                match status {
                    Ok(e) => yield ExtractStatus::Progress(e.0, e.1),
                    Err(err) => {
                        log::error!("Error extracting OBS: {:?}", err);
                        yield ExtractStatus::Error(err);
                        return;
                    }
                }
            }

    }))
}

#[cfg(target_os = "macos")]
async fn extract_dmg(dmg_path: &Path, output_dir: &Path) -> anyhow::Result<ExtractStream> {
    use tokio::process::Command;
    use uuid::Uuid;
    
    let mount_point = PathBuf::from("/tmp").join(format!("obs-mount-{}", Uuid::new_v4()));
    let dmg_path_buf = dmg_path.to_path_buf();
    let output_dir_buf = output_dir.to_path_buf();
    
    let stream = stream! {
        let dmg_path = &dmg_path_buf;
        let output_dir = &output_dir_buf;
        
        yield Ok((0.0, "Mounting DMG...".to_string()));
        
        // Create mount point
        tokio::fs::create_dir_all(&mount_point).await?;
        
        // Mount the DMG
        let mount_output = Command::new("hdiutil")
            .args(&["attach", "-nobrowse", "-mountpoint"])
            .arg(&mount_point)
            .arg(&dmg_path)
            .output()
            .await?;
        
        if !mount_output.status.success() {
            let error_msg = String::from_utf8_lossy(&mount_output.stderr);
            yield Err(anyhow::anyhow!("Failed to mount DMG: {}", error_msg));
            return;
        }
        
        yield Ok((0.3, "Copying files...".to_string()));
        
        // Copy OBS.app contents
        let app_path = mount_point.join("OBS.app/Contents");
        if !app_path.exists() {
            let _ = Command::new("hdiutil").args(&["detach"]).arg(&mount_point).output().await;
            yield Err(anyhow::anyhow!("OBS.app not found in DMG"));
            return;
        }
        
        // Create output directory
        if let Err(e) = tokio::fs::create_dir_all(&output_dir).await {
            let _ = Command::new("hdiutil").args(&["detach"]).arg(&mount_point).output().await;
            yield Err(e.into());
            return;
        }
        
        yield Ok((0.4, "Copying Frameworks...".to_string()));
        
        // Copy Frameworks (contains libobs.dylib and dependencies)
        let frameworks_path = app_path.join("Frameworks");
        if frameworks_path.exists() {
            if let Err(e) = copy_dir_recursive(&frameworks_path, &output_dir).await {
                let _ = Command::new("hdiutil").args(&["detach"]).arg(&mount_point).output().await;
                yield Err(e);
                return;
            }
        }
        
        yield Ok((0.7, "Copying PlugIns...".to_string()));
        
        // Copy PlugIns
        let plugins_path = app_path.join("PlugIns");
        if plugins_path.exists() {
            let dest_plugins = output_dir.join("obs-plugins");
            if let Err(e) = copy_dir_recursive(&plugins_path, &dest_plugins).await {
                let _ = Command::new("hdiutil").args(&["detach"]).arg(&mount_point).output().await;
                yield Err(e);
                return;
            }
        }
        
        yield Ok((0.9, "Copying Resources...".to_string()));
        
        // Copy Resources/data
        let data_path = app_path.join("Resources/data");
        if data_path.exists() {
            let dest_data = output_dir.join("data");
            if let Err(e) = copy_dir_recursive(&data_path, &dest_data).await {
                let _ = Command::new("hdiutil").args(&["detach"]).arg(&mount_point).output().await;
                yield Err(e);
                return;
            }
        }
        
        yield Ok((0.95, "Unmounting DMG...".to_string()));
        
        // Unmount
        let unmount_output = Command::new("hdiutil")
            .args(&["detach"])
            .arg(&mount_point)
            .output()
            .await?;
        
        if !unmount_output.status.success() {
            log::warn!("Failed to unmount DMG cleanly, but files were copied");
        }
        
        // Clean up mount point
        let _ = tokio::fs::remove_dir(&mount_point).await;
        
        yield Ok((1.0, "Extraction complete".to_string()));
    };
    
    Ok(Box::pin(stream! {
        pin_mut!(stream);
        while let Some(status) = stream.next().await {
            match status {
                Ok(e) => yield ExtractStatus::Progress(e.0, e.1),
                Err(err) => {
                    log::error!("Error extracting DMG: {:?}", err);
                    yield ExtractStatus::Error(err);
                    return;
                }
            }
        }
    }))
}

#[cfg(target_os = "macos")]
async fn copy_dir_recursive(src: &Path, dst: &Path) -> anyhow::Result<()> {
    use tokio::process::Command;
    
    // Use ditto to preserve code signatures and extended attributes on macOS
    tokio::fs::create_dir_all(dst.parent().unwrap_or(dst)).await?;
    
    let status = Command::new("ditto")
        .arg(src)
        .arg(dst)
        .status()
        .await?;
    
    if !status.success() {
        anyhow::bail!("ditto failed copying {:?} to {:?}", src, dst);
    }
    
    Ok(())
}
