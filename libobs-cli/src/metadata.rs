use std::env::current_dir;

use anyhow::{anyhow, Result};
use toml::{map::Map, Table, Value};

pub fn get_main_meta() -> Result<Map<String, Value>> {
    let dir = current_dir()?;

    let meta = dir.join("Cargo.toml");
    let meta = std::fs::read_to_string(meta)?;

    let parsed: Table = toml::from_str(&meta)?;
    let val = parsed.get("package")
    .and_then(|package| package.as_table())
    .and_then(|package| package.get("metadata"))
    .and_then(|metadata| metadata.as_table())
    .ok_or_else(|| anyhow::anyhow!("Failed to read `package.metadata` from Cargo.toml"))?
    .clone();

    Ok(val)
}

pub fn read_val_from_meta(m: &Map<String, Value>, key: &str) -> anyhow::Result<String> {
    let tag = m.get(key)
    .and_then(|tag| tag.as_str())
    .ok_or_else(|| anyhow!("Failed to read `{}` from Cargo.toml under `package.metadata`", key))?;

    Ok(tag.to_string())

}