use anyhow::bail;
use colored::Colorize;
use http_req::{request::Request, response::StatusCode, uri::Uri};
use serde_json::Value;
use std::{path::PathBuf, process::Command};

use clap::Parser;

mod util;

#[cfg(target_family = "windows")]
mod win;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RunArgs {
    /// The tag of OBS Studio to build
    #[arg(short, long, default_value = "latest")]
    tag: String,

    /// The directory to build OBS Studio in
    #[arg(short, long, default_value = "obs-build")]
    build_dir: PathBuf,

    /// The github repository to clone OBS Studio from
    #[arg(short, long, default_value = "obsproject/obs-studio")]
    repo_id: String,
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

fn clone_repo(repo_id: &str, tag: &str, repo_dir: &PathBuf) -> anyhow::Result<()> {
    let repo_url = format!("https://github.com/{}.git", repo_id);

    let res = Command::new("git")
        .arg("clone")
        .arg("--recursive")
        .arg("--depth=1")
        .arg("--branch")
        .arg(&tag)
        .arg(&repo_url)
        .arg(&repo_dir)
        .status()?;

    if !res.success() {
        bail!("Failed to clone OBS Studio");
    }

    Ok(())
}

fn checkout_repo(repo_dir: &PathBuf, tag: &str) -> anyhow::Result<()> {
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

fn main() -> anyhow::Result<()> {
    let args = RunArgs::parse();

    let RunArgs {
        mut tag,
        build_dir: repo_dir,
        repo_id,
    } = args;

    if tag == "latest" {
        tag = fetch_latest(&repo_id)?;
    }

    println!("Fetching {} version of OBS Studio...", tag.on_blue());
    if !repo_dir.exists() {
        clone_repo(&repo_id, &tag, &repo_dir)?;
    } else {
        println!("{}", "Repo already exists, checking out tag...".yellow());
        checkout_repo(&repo_dir, &tag)?;
    }

    #[cfg(target_family = "windows")]
    win::run(&repo_dir)?;

    #[cfg(not(target_family = "windows"))]
    println!("Unsupported platform");

    Ok(())
}
