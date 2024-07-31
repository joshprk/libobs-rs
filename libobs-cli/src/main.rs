use git::{clone_repo, fetch_latest};
use metadata::{get_main_meta, read_val_from_meta};
use std::{
    fs,
    path::PathBuf,
};
use util::{build_cmake, configure_cmake, copy_to_dir, delete_all_except};

use clap::Parser;
use colored::Colorize;

mod git;
mod metadata;
mod util;

#[cfg(target_family = "windows")]
mod win;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RunArgs {
    /// The target where the OBS Studio sources should be copied to. Example: "debug", "release"
    #[arg(short, long)]
    profile: String,

    /// The location where the OBS Studio sources should be cloned to
    #[arg(short='o', long, default_value = "obs-build")]
    cache_dir: PathBuf,

    /// The github repository to clone OBS Studio from
    #[arg(short, long, default_value = "obsproject/obs-studio")]
    repo_id: String,

    /// The build config that obs should be built with (can be Release, Debug, RelWithDebInfo)
    #[arg(short, long, default_value = "RelWithDebInfo")]
    config: String,
}

fn main() -> anyhow::Result<()> {
    let args = RunArgs::parse();

    let RunArgs {
        cache_dir,
        repo_id,
        profile: target_profile,
        config: build_type,
    } = args;

    let target_out_dir = PathBuf::new().join("target").join(&target_profile);

    let meta = get_main_meta()?;

    let mut tag = read_val_from_meta(&meta, "libobs-version")?;
    let cache_dir = read_val_from_meta(&meta, "libobs-cache-dir")
        .and_then(|e| Ok(PathBuf::from(&e)))
        .unwrap_or_else(|_e| cache_dir);

    if tag == "latest" {
        tag = fetch_latest(&repo_id)?;
    }

    let repo_dir = cache_dir.join(&tag);
    let exists = repo_dir.is_dir();
    if !repo_dir.is_dir() {
        fs::create_dir_all(&repo_dir)?;
    }

    println!("Fetching {} version of OBS Studio...", tag.on_blue());
    let build_out = repo_dir.join("build_out");
    let build = repo_dir.join("build");

    if !exists {
        clone_repo(&repo_id, &tag, &repo_dir)?;

        let obs_preset = if cfg!(target_family = "windows") {
            "windows-x64"
        } else {
            "linux-x64"
        };

        fs::create_dir_all(&build)?;

        configure_cmake(&repo_dir, obs_preset, &build_type)?;
        build_cmake(&repo_dir, &build_type)?;

        fs::create_dir_all(&build_out)?;

        #[cfg(target_family = "windows")]
        win::copy_files(&repo_dir, &build_out, &build_type)?;

        delete_all_except(&repo_dir, Some(&build_out))?;

        #[cfg(not(target_family = "windows"))]
        println!("Unsupported platform");
    }

    copy_to_dir(&build_out, &target_out_dir, None)?;

    Ok(())
}
