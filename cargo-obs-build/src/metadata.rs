use std::{env::current_dir, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use toml::{map::Map, Table, Value};
use crate::git::fetch_release;

pub fn get_main_meta() -> Result<Option<Map<String, Value>>> {
    let dir = current_dir()?;

    let meta = dir.join("Cargo.toml");
    if !meta.is_file() {
        return Ok(None);
    }

    let meta = std::fs::read_to_string(meta).context("Reading Cargo.toml")?;

    let parsed: Table = toml::from_str(&meta)?;
    let val = parsed.get("package")
    .and_then(|package| package.as_table())
    .and_then(|package| package.get("metadata"))
    .and_then(|metadata| metadata.as_table())
    .ok_or_else(|| anyhow::anyhow!("Failed to read `package.metadata` from Cargo.toml"))?
    .clone();

    Ok(Some(val))
}

pub fn read_val_from_meta(m: &Map<String, Value>, key: &str) -> anyhow::Result<String> {
    let tag = m.get(key)
    .and_then(|tag| tag.as_str())
    .ok_or_else(|| anyhow!("Failed to read `{}` from Cargo.toml under `package.metadata`", key))?;

    Ok(tag.to_string())
}

pub fn get_meta_info(cache_dir: &mut PathBuf) -> anyhow::Result<(Option<PathBuf>, String)> {
    let meta = get_main_meta()?;
    let mut tag = "latest".to_string();

    if let Some(meta) = meta {
        if let Ok(dir) = read_val_from_meta(&meta, "libobs-cache-dir").map(PathBuf::from) {
            *cache_dir = dir;
        }

        if let Ok(version) = read_val_from_meta(&meta, "libobs-version") {
            tag = version;
        }
    }

    Ok((Some(cache_dir.clone()), tag))
}

pub fn fetch_latest_release_tag(repo_id: &str) -> anyhow::Result<String> {
    let release = fetch_release(repo_id, &None)?;
    Ok(release.tag)
}