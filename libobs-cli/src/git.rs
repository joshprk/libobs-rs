use anyhow::bail;
use http_req::{request::Request, response::StatusCode, uri::Uri};
use serde_json::Value;
use std::{
    path::Path,
    process::Command,
};

pub fn fetch_latest(repo_id: &str) -> anyhow::Result<String> {
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

pub fn clone_repo(repo_id: &str, tag: &str, repo_dir: &Path) -> anyhow::Result<()> {
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

pub fn checkout_repo(repo_dir: &Path, tag: &str) -> anyhow::Result<()> {
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