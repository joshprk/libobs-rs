use std::path::Path;

use crate::util::{copy_to_dir, get_build_out};
use colored::Colorize;

pub fn copy_files(
    repo_dir: &Path,
    copy_dir: &Path,
    obs_preset: &str
) -> anyhow::Result<()> {
    println!("Copying files from to {}", copy_dir.display());
    let build_out = get_build_out(repo_dir, obs_preset);

    println!(
        "Copying files from {} to {}",
        build_out.display(),
        copy_dir.display()
    );

    let bin_dir = build_out.join("bin");
    let bin_arch_dir = bin_dir.join("64bit");

    copy_to_dir(&bin_arch_dir, &copy_dir, None)?;
    copy_to_dir(&build_out, &copy_dir, Some(&bin_dir))?;

    Ok(())
}
