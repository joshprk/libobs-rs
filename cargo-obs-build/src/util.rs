use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::bail;
use colored::Colorize;
use walkdir::WalkDir;

fn add_disabled_features(cmd: &mut Command) {
    cmd.arg("-DCMAKE_BUILD_TYPE=RelWithDebInfo");
    cmd.arg("-DENABLE_BROWSER:BOOL=OFF ");
    cmd.arg("-DENABLE_VLC:BOOL=OFF ");
    cmd.arg("-DENABLE_UI:BOOL=OFF ");
    cmd.arg("-DENABLE_VST:BOOL=OFF ");
    cmd.arg("-DENABLE_SCRIPTING:BOOL=OFF");
    cmd.arg("-DCOPIED_DEPENDENCIES:BOOL=OFF");
    cmd.arg("-DCOPY_DEPENDENCIES:BOOL=ON");
    cmd.arg("-DBUILD_FOR_DISTRIBUTION:BOOL=ON");
    cmd.arg("-DENABLE_WEBSOCKET:BOOL=OFF");
    cmd.arg("-DCMAKE_COMPILE_WARNING_AS_ERROR=OFF");
}

pub fn configure_cmake(dir: &Path, preset: &str, build_type: &str) -> anyhow::Result<()> {
    let mut cmd = Command::new("cmake");

    cmd.arg("-S")
        .arg(".")
        .arg("-B")
        .arg("build")
        .arg("--preset")
        .arg(preset)
        .arg(format!("-DCMAKE_BUILD_TYPE={}", build_type))
        .current_dir(dir);

    add_disabled_features(&mut cmd);
    println!("{}", "Configuring OBS Studio...".blue());
    let res = cmd.status()?;

    if !res.success() {
        bail!("Failed to configure OBS Studio");
    }

    Ok(())
}

pub fn build_cmake(dir: &Path, build_type: &str) -> anyhow::Result<()> {
    println!("{}", "Building OBS studio...".yellow());
    let cmd = Command::new("cmake")
        .arg("--build")
        .arg("build")
        .arg("--config")
        .arg(build_type)
        .current_dir(dir)
        .status()?;

    if !cmd.success() {
        bail!("Failed to build OBS Studio");
    }

    Ok(())
}

pub fn get_build_out(repo_dir: &Path, preset: &str) -> PathBuf {
    repo_dir.join("build").join("rundir").join(preset)
}

pub fn copy_to_dir(src: &Path, out: &Path, except_dir: Option<&Path>) -> anyhow::Result<()> {
    for entry in WalkDir::new(&src) {
        if entry.is_err() {
            continue;
        }

        let entry = entry.unwrap();
        let path = entry.path();

        if except_dir.is_some_and(|e| path.starts_with(e)) {
            continue;
        }

        let copy_to = out.join(path.strip_prefix(src).unwrap());
        if path.is_dir() {
            fs::create_dir_all(&copy_to)?;
            continue;
        }

        fs::copy(entry.path(), copy_to)?;
    }

    Ok(())
}

pub fn delete_all_except(src: &Path, except_dir: Option<&Path>) -> anyhow::Result<()> {
    for entry in fs::read_dir(src)? {
        if entry.is_err() {
            continue;
        }

        let entry = entry.unwrap();
        let path = entry.path();

        if except_dir.is_some_and(|e| path.starts_with(e)) {
            continue;
        }

        if path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
    }

    Ok(())
}