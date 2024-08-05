use git::{clone_repo, fetch_release, ReleaseInfo};
use lock::{acquire_lock, wait_for_lock};
use metadata::{get_main_meta, read_val_from_meta};
use std::{
    env::args,
    fs::{self, File},
    path::{Path, PathBuf},
};
use util::{build_cmake, configure_cmake, copy_to_dir, delete_all_except};

use clap::Parser;
use colored::Colorize;

mod download;
mod git;
mod lock;
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
    #[arg(short = 'o', long, default_value = "obs-build")]
    cache_dir: PathBuf,

    /// The github repository to clone OBS Studio from
    #[arg(long, default_value = "obsproject/obs-studio")]
    repo_id: String,

    /// The build config that obs should be built with (can be Release, Debug, RelWithDebInfo)
    #[arg(short, long, default_value = "RelWithDebInfo")]
    config: String,

    /// Enable to keep the OBS Studio sources after the build
    #[arg(short, long, default_value_t = false)]
    no_remove: bool,

    /// wether the tool should download the OBS Studio binaries (to keep the signature of the win capture plugin)
    #[arg(short, long, default_value_t = true)]
    download_bin: bool,

    /// When this flag is active, the cache will be cleared and a new build will be started
    #[arg(short, long, default_value_t = false)]
    rebuild: bool,
}

fn main() -> anyhow::Result<()> {
    let mut args: Vec<_> = args().collect();
    if args.get(1).is_some_and(|e| e == "obs-build") {
        args.remove(1);
    }

    let args = RunArgs::parse_from(args);

    let RunArgs {
        cache_dir,
        repo_id,
        profile: target_profile,
        config: build_type,
        no_remove,
        download_bin,
        rebuild
    } = args;

    let target_out_dir = PathBuf::new().join("target").join(&target_profile);

    let meta = get_main_meta()?;

    let cache_dir = read_val_from_meta(&meta, "libobs-cache-dir")
        .and_then(|e| Ok(PathBuf::from(&e)))
        .unwrap_or_else(|_e| cache_dir);

    let tag = read_val_from_meta(&meta, "libobs-version")?;
    let mut release = None;

    // Fetching tag name, if it's latest, we fetch the latest release and get the tag name from there
    let tag = if tag.trim() == "latest" {
        // Fetching latest release and setting it as tag
        let tmp = fetch_release(&repo_id, &None)?;
        release = Some(tmp.clone());

        tmp.tag.clone()
    } else {
        tag
    };

    let repo_dir = cache_dir.join(&tag);
    let repo_exists = repo_dir.is_dir();

    if !repo_dir.is_dir() {
        fs::create_dir_all(&repo_dir)?;
    }

    let build_out = repo_dir.join("build_out");
    let lock_file = cache_dir.join(format!("{}.lock", tag));
    let success_file = repo_dir.join(".success");

    wait_for_lock(&lock_file)?;

    if !success_file.is_file() || rebuild {
        let lock = acquire_lock(&lock_file)?;
        if repo_exists || rebuild {
            println!("Cleaning up old build...");
            delete_all_except(&repo_dir, None)?;
        }

        println!("Fetching {} version of OBS Studio...", tag.on_blue());

        let release = release
            .map(|e| Ok(e))
            .unwrap_or_else(|| fetch_release(&repo_id, &Some(tag.clone())))?;

        build_obs(
            release,
            &tag,
            &repo_id,
            &repo_dir,
            &build_out,
            &build_type,
            no_remove,
            download_bin,
        )?;

        File::create(&success_file)?;
        drop(lock);
    }

    println!(
        "Copying files from {} to {}",
        build_out.display().to_string().green(),
        target_out_dir.display().to_string().green()
    );
    copy_to_dir(&build_out, &target_out_dir, None)?;

    println!("Done!");

    Ok(())
}

fn build_obs(
    release: ReleaseInfo,
    tag: &str,
    repo_id: &str,
    repo_dir: &Path,
    build_out: &Path,
    build_type: &str,
    no_remove: bool,
    download_bin: bool,
) -> anyhow::Result<()> {
    clone_repo(&repo_id, &tag, &repo_dir)?;
    fs::create_dir_all(&build_out)?;

    let obs_preset = if cfg!(target_family = "windows") {
        "windows-x64"
    } else {
        "linux-x64"
    };

    configure_cmake(&repo_dir, obs_preset, &build_type)?;
    build_cmake(&repo_dir, &build_out, &build_type)?;


    #[cfg(target_family = "windows")]
    win::process_source(&repo_dir, &build_out, &build_type, download_bin, &release)?;

    #[cfg(not(target_family = "windows"))]
    println!("Unsupported platform");


    if !no_remove {
        delete_all_except(&repo_dir, Some(&build_out))?;
    }

    Ok(())
}
