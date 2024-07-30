use std::{fs, path::Path};

use crate::{util::{build_cmake, configure_cmake}, RunArgs};

pub fn run(repo_dir: &Path) -> anyhow::Result<()> {
    let build = repo_dir.join("build");
    fs::create_dir_all(&build)?;

    configure_cmake(repo_dir, "windows-x64")?;
    build_cmake(repo_dir)?;

    Ok(())
}
