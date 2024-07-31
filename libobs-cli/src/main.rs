use anyhow::bail;
use colored::Colorize;
use http_req::{request::Request, response::StatusCode, uri::Uri};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use util::{build_cmake, configure_cmake};

use clap::Parser;

mod util;

#[cfg(target_family = "windows")]
mod win;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RunArgs {
    /// The target where the OBS Studio sources should be copied to. Example: "debug", "release"
    #[arg(short, long)]
    profile: String,

    /// The tag of OBS Studio to build
    #[arg(short, long, default_value = "latest")]
    tag: String,

    /// The directory to build OBS Studio in
    #[arg(short, long, default_value = "obs-build")]
    build_dir: PathBuf,

    /// The github repository to clone OBS Studio from
    #[arg(short, long, default_value = "obsproject/obs-studio")]
    repo_id: String,

    /// The build config that obs should be built with (can be Release, Debug, RelWithDebInfo)
    #[arg(short, long, default_value = "RelWithDebInfo")]
    config: String,
}

fn fetch_latest(repo_id: &str) -> anyhow::Result<String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo_id);
    let url = Uri::try_from(url.as_str())?;

    let mut body = Vec::new(); //Container for body of a response.
    let res = Request::new(&url)
        .header("User-Agent", "libobs-cli")
        .send(&mut body)?;

    if res.status_code() != StatusCode::new(200) {
        bail!(
            "Failed to fetch latest release: {} with {}",
            res.status_code(),
            String::from_utf8(body).unwrap_or("Couldn't parse".to_string())
        );
    }

    let body = String::from_utf8(body)?;
    let body: Value = serde_json::from_str(&body)?;
    let body = body["tag_name"].as_str();

    if let Some(tag) = body {
        return Ok(tag.to_string());
    }

    bail!("Failed to fetch latest release")
}

fn clone_repo(repo_id: &str, tag: &str, repo_dir: &Path) -> anyhow::Result<()> {
    let repo_url = format!("https://github.com/{}.git", repo_id);

    let res = Command::new("git")
        .arg("clone")
        .arg("--recursive")
        .arg(&repo_url)
        .arg(&repo_dir)
        .status()?;

    if !res.success() {
        bail!("Failed to clone OBS Studio");
    }

    checkout_repo(repo_dir, tag)?;

    Ok(())
}

fn checkout_repo(repo_dir: &Path, tag: &str) -> anyhow::Result<()> {
    let res = Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg(tag)
        .current_dir(&repo_dir)
        .status()?;

    if !res.success() {
        bail!("Failed to fetch tag");
    }

    let res = Command::new("git")
        .arg("checkout")
        .arg(tag)
        .current_dir(&repo_dir)
        .status()?;

    if !res.success() {
        bail!("Failed to checkout tag");
    }

    let res = Command::new("git")
        .arg("submodule")
        .arg("update")
        .current_dir(&repo_dir)
        .status()?;

    if !res.success() {
        bail!("Failed to update submodules");
    }

    Ok(())
}

fn has_changed(repo_dir: &Path, tag: &str) -> anyhow::Result<bool> {
    let res = Command::new("git")
        .arg("describe")
        .arg("--exact-match")
        .arg("--tags")
        .current_dir(&repo_dir)
        .output()?;

    let res = String::from_utf8(res.stdout)?;
    println!("Out '{}' tag '{}'", res.trim(), tag);

    Ok(res.trim() != tag)
}

fn main() -> anyhow::Result<()> {
    let args = RunArgs::parse();

    let RunArgs {
        mut tag,
        build_dir: repo_dir,
        repo_id,
        profile: target_profile,
        config: build_type,
    } = args;

    let copy_dir = PathBuf::new().join("target").join(&target_profile);

    let mut changed = true;

    if tag == "latest" {
        tag = fetch_latest(&repo_id)?;
    }

    println!("Fetching {} version of OBS Studio...", tag.on_blue());
    if !repo_dir.exists() {
        clone_repo(&repo_id, &tag, &repo_dir)?;
    } else {
        let c = has_changed(&repo_dir, &tag);
        if let Err(e) = &c {
            eprintln!("Error checking changed: {}", e);
        }

        changed = c.unwrap_or(false);

        if changed {
            println!("{}", "Repo already exists, checking out tag...".yellow());
            checkout_repo(&repo_dir, &tag)?;
        }
    }

    let obs_preset = if cfg!(target_family = "windows") {
        "windows-x64"
    } else {
        "linux-x64"
    };

    let build = repo_dir.join("build");
    fs::create_dir_all(&build)?;

    if changed {
        configure_cmake(&repo_dir, obs_preset, &build_type)?;
        build_cmake(&repo_dir, &build_type)?;
    }

    #[cfg(target_family = "windows")]
    win::run(&repo_dir, &copy_dir, &build_type)?;

    #[cfg(not(target_family = "windows"))]
    println!("Unsupported platform");

    Ok(())
}
