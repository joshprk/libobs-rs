use std::{fs, path::Path};

use walkdir::WalkDir;

pub fn copy_to_dir(src: &Path, out: &Path, except_dir: Option<&Path>) -> anyhow::Result<()> {
    // On macOS, use ditto to preserve code signatures and extended attributes
    #[cfg(target_os = "macos")]
    {
        copy_to_dir_macos(src, out, except_dir)
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        for entry in WalkDir::new(src) {
            if entry.is_err() {
                continue;
            }

            let entry = entry?;
            let path = entry.path();

            if except_dir.is_some_and(|e| path.starts_with(e)) {
                continue;
            }

            let copy_to = out.join(path.strip_prefix(src)?);
            if path.is_dir() {
                fs::create_dir_all(&copy_to)?;
                continue;
            }

            fs::copy(entry.path(), copy_to)?;
        }

        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn copy_to_dir_macos(src: &Path, out: &Path, except_dir: Option<&Path>) -> anyhow::Result<()> {
    use std::process::Command;
    
    // If no except_dir, use ditto for the entire directory (preserves signatures)
    if except_dir.is_none() {
        fs::create_dir_all(out)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let file_name = entry.file_name();
            let dest_path = out.join(&file_name);
            
            log::debug!("Copying {:?} to {:?}", src_path, dest_path);
            
            // Remove destination if it exists (ditto can have issues with existing directories)
            if dest_path.exists() {
                if dest_path.is_dir() {
                    fs::remove_dir_all(&dest_path)?;
                } else {
                    fs::remove_file(&dest_path)?;
                }
            }
            
            // Use ditto with -V for verbose output and better error reporting
            let output = Command::new("ditto")
                .arg("-V")  // Verbose mode
                .arg(&src_path)
                .arg(&dest_path)
                .output()?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                anyhow::bail!("ditto failed for {:?}\nstdout: {}\nstderr: {}", src_path, stdout, stderr);
            }
            
            // Log what was copied
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                log::debug!("ditto output for {:?}:\n{}", file_name, stderr);
            }
        }
        
        return Ok(());
    }
    
    // If except_dir is specified, fall back to manual copy (but use ditto per-file)
    for entry in WalkDir::new(src) {
        if entry.is_err() {
            continue;
        }

        let entry = entry?;
        let path = entry.path();

        if except_dir.is_some_and(|e| path.starts_with(e)) {
            continue;
        }

        let copy_to = out.join(path.strip_prefix(src)?);
        if path.is_dir() {
            fs::create_dir_all(&copy_to)?;
            continue;
        }

        // Use ditto for files to preserve signatures
        let status = Command::new("ditto")
            .arg(path)
            .arg(&copy_to)
            .status()?;
        
        if !status.success() {
            // Fallback to regular copy if ditto fails
            fs::copy(path, copy_to)?;
        }
    }

    Ok(())
}

pub fn delete_all_except(src: &Path, except_dir: Option<&Path>) -> anyhow::Result<()> {
    for entry in fs::read_dir(src)? {
        if entry.is_err() {
            continue;
        }

        let entry = entry?;
        let path = entry.path();

        if except_dir.is_some_and(|e| path.starts_with(e)) {
            continue;
        }

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
