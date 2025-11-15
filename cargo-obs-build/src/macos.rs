use anyhow::bail;
use log::{debug, info, warn};
use std::fs;
use std::path::Path;
use std::process::Command;
use crate::util::copy_to_dir;

/// Extract macOS DMG file
pub fn extract_dmg(dmg_path: &Path, output_dir: &Path) -> anyhow::Result<()> {
    info!("Mounting DMG...");
    let mount_output = Command::new("hdiutil")
        .args(["attach", "-nobrowse", "-mountpoint", "/tmp/obs-mount"])
        .arg(dmg_path)
        .output()?;
    
    if !mount_output.status.success() {
        bail!("Failed to mount DMG: {}", String::from_utf8_lossy(&mount_output.stderr));
    }
    
    // Copy OBS.app contents
    let app_path = Path::new("/tmp/obs-mount/OBS.app/Contents");
    if app_path.exists() {
        // Copy MacOS directory (contains obs-ffmpeg-mux and other helpers)
        let macos_path = app_path.join("MacOS");
        if macos_path.exists() {
            info!("Copying helper binaries...");
            for entry in fs::read_dir(&macos_path)? {
                let entry = entry?;
                let path = entry.path();
                let file_name = entry.file_name();
                
                // Skip the main OBS binary, only copy helpers
                if file_name != "OBS" {
                    let dest = output_dir.join(&file_name);
                    Command::new("ditto").arg(&path).arg(&dest).status()?;
                }
            }
        }
        
        // Copy Frameworks (contains libobs.dylib)
        let frameworks_path = app_path.join("Frameworks");
        if frameworks_path.exists() {
            info!("Copying Frameworks...");
            copy_to_dir(&frameworks_path, output_dir, None)?;
            
            // Extract libobs framework Resources (effect files)
            let libobs_resources = frameworks_path.join("libobs.framework/Versions/A/Resources");
            if libobs_resources.exists() {
                info!("Extracting libobs data...");
                let dest_libobs_data = output_dir.join("data/libobs");
                copy_to_dir(&libobs_resources, &dest_libobs_data, None)?;
            }
        }
        
        // Copy PlugIns
        let plugins_path = app_path.join("PlugIns");
        if plugins_path.exists() {
            info!("Copying PlugIns...");
            let dest_plugins = output_dir.join("obs-plugins");
            copy_to_dir(&plugins_path, &dest_plugins, None)?;
            
            // Extract plugin data from .plugin bundles
            info!("Extracting plugin data...");
            let dest_plugin_data = output_dir.join("data/obs-plugins");
            fs::create_dir_all(&dest_plugin_data)?;
            
            for entry in fs::read_dir(&plugins_path)? {
                let entry = entry?;
                let path = entry.path();
                let file_name = entry.file_name();
                
                if file_name.to_string_lossy().ends_with(".plugin") {
                    let plugin_resources = path.join("Contents/Resources");
                    if plugin_resources.exists() {
                        let plugin_name = file_name.to_string_lossy().replace(".plugin", "");
                        let dest = dest_plugin_data.join(&plugin_name);
                        copy_to_dir(&plugin_resources, &dest, None)?;
                    }
                }
            }
        }
        
        // Copy Resources directory contents
        let resources_path = app_path.join("Resources");
        if resources_path.exists() {
            info!("Copying Resources...");
            let dest_data = output_dir.join("data");
            fs::create_dir_all(&dest_data)?;
            
            for entry in fs::read_dir(&resources_path)? {
                let entry = entry?;
                let path = entry.path();
                let file_name = entry.file_name();
                
                if file_name.to_string_lossy().ends_with(".plugin") {
                    continue;
                }
                
                if path.is_dir() {
                    let dest = dest_data.join(&file_name);
                    copy_to_dir(&path, &dest, None)?;
                } else {
                    let dest = dest_data.join(&file_name);
                    Command::new("ditto").arg(&path).arg(&dest).status()?;
                }
            }
        } else {
            warn!("Resources directory not found at {:?}", resources_path);
        }
    }
    
    // Unmount
    info!("Unmounting DMG...");
    let _unmount = Command::new("hdiutil")
        .args(["detach", "/tmp/obs-mount"])
        .output()?;
    
    Ok(())
}

/// Fix helper binaries on macOS to find dylibs properly
pub fn fix_helper_binaries_macos(output_dir: &Path) -> anyhow::Result<()> {
    let helper_binaries = ["obs-ffmpeg-mux"];
    
    for helper_name in &helper_binaries {
        let helper_path = output_dir.join(helper_name);
        if !helper_path.exists() {
            debug!("Helper binary {} not found, skipping", helper_name);
            continue;
        }
        
        debug!("Fixing rpath for {}", helper_name);
        
        // Add rpaths for finding dylibs
        let rpaths = [
            "@executable_path",
            "@executable_path/..",
            "@loader_path",
            "@loader_path/..",
        ];
        
        for rpath in &rpaths {
            let status = Command::new("install_name_tool")
                .arg("-add_rpath")
                .arg(rpath)
                .arg(&helper_path)
                .status();
            
            if let Ok(s) = status {
                if !s.success() {
                    debug!("Note: Could not add rpath {} (may already exist)", rpath);
                }
            }
        }
        
        // Re-sign with ad-hoc signature
        let sign_status = Command::new("codesign")
            .args(["--force", "--sign", "-"])
            .arg(&helper_path)
            .status()?;
        
        if !sign_status.success() {
            bail!("Failed to sign helper binary: {}", helper_name);
        }
        
        info!("Fixed and signed: {}", helper_name);
    }
    
    Ok(())
}

