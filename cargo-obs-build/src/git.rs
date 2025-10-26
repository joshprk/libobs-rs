use anyhow::{anyhow, bail};
use http_req::{request::Request, response::StatusCode, uri::Uri};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ReleaseInfo {
    pub tag: String,
    pub assets: Vec<Value>,
    pub checksums: HashMap<String, String>,
}

pub fn fetch_release(repo_id: &str, tag: &Option<String>) -> anyhow::Result<ReleaseInfo> {
    let tag = tag.clone();
    let tag = if tag.is_none() {
        "latest"
    } else {
        &format!("tags/{}", tag.unwrap())
    };
    let url = format!("https://api.github.com/repos/{}/releases/{}", repo_id, tag);
    let url = Uri::try_from(url.as_str())?;

    let mut body = Vec::new(); //Container for body of a response.
    let res = Request::new(&url)
        .header("User-Agent", "cargo-obs-build")
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
    let tag_name = body["tag_name"].as_str();

    if tag_name.is_none() {
        bail!("Tag name in release is none");
    }

    let tag = tag_name.unwrap();
    let assets = body["assets"]
        .as_array()
        .ok_or(anyhow!("No assets found"))?;

    let mut checksums = HashMap::new();
    let note = body["body"].as_str().unwrap_or("");

    let split = note.replace("\r", "");
    let split = split.split("\n");

    let mut is_checksums = false;
    for line in split {
        if line.to_lowercase().contains("checksums") {
            is_checksums = true;
            continue;
        }

        if !is_checksums {
            continue;
        }

        let split: Vec<&str> = line.trim().split(":").collect();
        if split.len() != 2 {
            continue;
        }

        checksums.insert(
            split[0].trim().to_lowercase().to_string(),
            split[1].trim().to_string(),
        );
    }

    Ok(ReleaseInfo {
        tag: tag.to_string(),
        assets: assets.clone(),
        checksums,
    })
}

pub fn fetch_latest_patch_release(
    repo_id: &str,
    major: u32,
    minor: u32,
) -> anyhow::Result<Option<String>> {
    let url = format!("https://api.github.com/repos/{}/releases", repo_id);
    let url = Uri::try_from(url.as_str())?;

    let mut body = Vec::new();
    let res = Request::new(&url)
        .header("User-Agent", "cargo-obs-build")
        .send(&mut body)?;

    if res.status_code() != StatusCode::new(200) {
        bail!(
            "Failed to fetch releases: {} with {}",
            res.status_code(),
            String::from_utf8(body).unwrap_or("Couldn't parse".to_string())
        );
    }

    let body = String::from_utf8(body)?;
    let arr: Vec<Value> = serde_json::from_str(&body)?;

    let mut best_patch: Option<u32> = None;
    let mut best_tag: Option<String> = None;

    for rel in arr.iter() {
        // skip drafts and prereleases
        if rel["draft"].as_bool().unwrap_or(false) || rel["prerelease"].as_bool().unwrap_or(false) {
            continue;
        }
        let tag_name = rel["tag_name"].as_str().unwrap_or("").to_string();
        if tag_name.is_empty() {
            continue;
        }
        let parts: Vec<&str> = tag_name.trim_start_matches('v').split('.').collect();
        if parts.len() < 3 {
            continue;
        }
        let r_major = parts[0].parse::<u32>().unwrap_or(0);
        let r_minor = parts[1].parse::<u32>().unwrap_or(0);
        let r_patch = parts[2].parse::<u32>().unwrap_or(0);

        if r_major == major
            && r_minor == minor
            && (best_patch.is_none() || r_patch > best_patch.unwrap())
        {
            best_patch = Some(r_patch);
            best_tag = Some(tag_name);
        }
    }

    Ok(best_tag)
}
