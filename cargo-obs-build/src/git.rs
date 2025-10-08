use anyhow::{anyhow, bail};
use http_req::{request::Request, response::StatusCode, uri::Uri};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ReleaseInfo {
    pub tag: String,
    pub assets: Vec<Value>,
    pub checksums: HashMap<String, String>
}

pub fn fetch_release(repo_id: &str, tag: &Option<String>) -> anyhow::Result<ReleaseInfo> {
    let tag = tag.clone();
    let tag = if tag.is_none() { "latest" } else { &format!("tags/{}", tag.unwrap()) };
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
    let assets = body["assets"].as_array().ok_or(anyhow!("No assets found"))?;

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

        checksums.insert(split[0].trim().to_lowercase().to_string(), split[1].trim().to_string());
    }

    return Ok(ReleaseInfo {
        tag: tag.to_string(),
        assets: assets.clone(),
        checksums
    });
}
