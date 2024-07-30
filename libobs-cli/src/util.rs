use std::{path::Path, process::Command};

use colored::Colorize;
use anyhow::bail;

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

pub fn configure_cmake(dir: &Path, preset: &str) -> anyhow::Result<()> {
    let mut cmd = Command::new("cmake");

    cmd.arg("-S")
        .arg(".")
        .arg("-B")
        .arg("build")
        .arg("--preset")
        .arg(preset)
        .current_dir(dir);

    add_disabled_features(&mut cmd);
    println!("{}", "Configuring OBS Studio...".blue());
    let res = cmd.status()?;

    if !res.success() {
        bail!("Failed to configure OBS Studio");
    }

    Ok(())
}

pub fn build_cmake(dir: &Path) -> anyhow::Result<()> {
    println!("{}", "Building OBS studio...".yellow());
    let cmd = Command::new("cmake")
        .arg("--build")
        .arg("build")
        .arg("--config")
        .arg("RelWithDebInfo")
        .current_dir(dir)
        .status()?;

    if !cmd.success() {
        bail!("Failed to build OBS Studio");
    }

    Ok(())
}
