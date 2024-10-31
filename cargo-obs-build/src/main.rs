use download::download_binaries;
use git::{fetch_release, ReleaseInfo};
use lock::{acquire_lock, wait_for_lock};
use metadata::{get_main_meta, read_val_from_meta};
use std::{
    env::args,
    fs::{self, File},
    path::{Path, PathBuf},
};
use util::{copy_to_dir, delete_all_except};
use walkdir::WalkDir;
use zip::ZipArchive;

use clap::Parser;
use colored::Colorize;

mod download;
mod git;
mod lock;
mod metadata;
mod util;

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

    /// When this flag is active, the cache will be cleared and a new build will be started
    #[arg(short, long, default_value_t = false)]
    rebuild: bool,


    /// If the browser should be included in the build
    #[arg(short, long, default_value_t = false)]
    browser: bool
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
        rebuild,
        browser
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

        build_obs(release, &build_out, browser)?;

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

fn build_obs(release: ReleaseInfo, build_out: &Path, include_browser: bool) -> anyhow::Result<()> {
    #[cfg(not(target_family = "windows"))]
    panic!("Unsupported platform");

    fs::create_dir_all(&build_out)?;

    let obs_path = download_binaries(build_out, &release)?;
    let obs_archive = File::open(&obs_path)?;
    let mut archive = ZipArchive::new(&obs_archive)?;

    println!("{} OBS Studio binaries...", "Extracting".on_blue());
    archive.extract(&build_out)?;
    let bin_path = build_out.join("bin").join("64bit");
    copy_to_dir(&bin_path, &build_out, None)?;
    fs::remove_dir_all(build_out.join("bin"))?;

    let mut to_exclude = vec![
        "obs64",
        "frontend",
        "obs-webrtc",
        "obs-websocket",
        "decklink",
        "obs-scripting",
        "qt6",
        "qminimal",
        "qwindows",
        "imageformats",
        "obs-studio"
    ];

    if !include_browser {
        to_exclude.append(&mut vec![
            "obs-browser",
            "obs-browser-page",
            "chrome_",
            "resources",
            "cef",
            "snapshot",
            "locales"
        ]);
    }

    println!("{} unnecessary files...", "Cleaning up".red());
    for entry in WalkDir::new(&build_out) {
        if entry.is_err() {
            continue;
        }

        let entry = entry.unwrap();
        let path = entry.path();

        if to_exclude.iter().any(|e| path.file_name().is_some_and(|x|{
            let x_l = x.to_string_lossy().to_lowercase();

            x_l.contains(e) || x_l == *e
        })) {
            println!("Deleting: {}", path.display().to_string().red());
            if path.is_dir() {
                fs::remove_dir_all(path).unwrap();
            } else {
                fs::remove_file(path).unwrap();
            }
        }
    }

    fs::remove_file(&obs_path)?;

    Ok(())
}
