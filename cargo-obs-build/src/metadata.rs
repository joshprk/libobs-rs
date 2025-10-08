use std::{env::current_dir, path::PathBuf};

use crate::git::fetch_release;
use anyhow::{anyhow, Context, Result};
use toml::{map::Map, Table, Value};

pub fn get_main_meta() -> Result<Option<Map<String, Value>>> {
    let dir = current_dir()?;

    let meta = dir.join("Cargo.toml");
    if !meta.is_file() {
        return Ok(None);
    }

    let meta = std::fs::read_to_string(meta).context("Reading Cargo.toml")?;

    let parsed: Table = toml::from_str(&meta)?;
    let val = parsed
        .get("package")
        .or_else(|| parsed.get("workspace"))
        .and_then(|package| package.as_table())
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.as_table())
        .cloned();

    Ok(val)
}

pub fn read_val_from_meta(m: &Map<String, Value>, key: &str) -> anyhow::Result<String> {
    let tag = m.get(key).and_then(|tag| tag.as_str()).ok_or_else(|| {
        anyhow!(
            "Failed to read `{}` from Cargo.toml under `package.metadata` or `workspace.metadata`",
            key
        )
    })?;

    Ok(tag.to_string())
}

pub fn get_meta_info(cache_dir: &mut PathBuf, tag: &mut String) -> anyhow::Result<()> {
    let meta = get_main_meta()?;

    if let Some(meta) = meta {
        if let Ok(dir) = read_val_from_meta(&meta, "libobs-cache-dir").map(PathBuf::from) {
            *cache_dir = dir;
        }

        if let Ok(version) = read_val_from_meta(&meta, "libobs-version") {
            *tag = version;
        }
    }

    Ok(())
}

pub fn fetch_latest_release_tag(repo_id: &str) -> anyhow::Result<String> {
    let release = fetch_release(repo_id, &None)?;
    Ok(release.tag)
}
