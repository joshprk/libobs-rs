use std::path::PathBuf;

use anyhow::Context;
use cargo_metadata::MetadataCommand;
use log::{debug, info};
use regex::Regex;

pub fn get_lib_obs_version() -> anyhow::Result<(u32, u32, u32)> {
    let re = Regex::new(r"^(#define LIBOBS_API_(MAJOR|MINOR|PATCH)_VER\s*)(\d+)$").unwrap();

    info!("Getting libobs version from bindings...");
    let meta = MetadataCommand::new().exec()?;

    let manifest = if let Some(root_pkg) = meta.root_package() {
        let root_pkg = meta
            .workspace_packages()
            .into_iter()
            .find(|p| p.name == root_pkg.name)
            .ok_or(anyhow::anyhow!(
                "root package not found in workspace packages"
            ))?;

        let libobs_workspace_dep = root_pkg
            .dependencies
            .iter()
            .find(|e| e.name == "libobs")
            .ok_or(anyhow::anyhow!("libobs dependency not found"))?;

        let req = libobs_workspace_dep.req.clone();
        let pkg = meta
            .packages
            .iter()
            .find(|p| p.name == "libobs" && req.matches(&p.version))
            .ok_or(anyhow::anyhow!("libobs package not found in metadata"))?;

        PathBuf::from(pkg.manifest_path.clone())
    } else {
        let manifest_path = meta
            .workspace_packages()
            .iter()
            .find(|p| p.name == "libobs")
            .ok_or(anyhow::anyhow!(
                "libobs package not found in workspace packages"
            ))
            .map(|e| &e.manifest_path)?;

        PathBuf::from(manifest_path.clone())
    };

    let dir = manifest
        .parent()
        .context("manifest path has no parent directory")?;

    let bindings_file = dir.join("headers").join("obs").join("obs-config.h");
    let bindings = std::fs::read_to_string(&bindings_file)
        .with_context(|| format!("failed to read bindings file: {}", bindings_file.display()))?;

    debug!("bindings file: {}", bindings_file.display());
    let version_parts = bindings
        .lines()
        .filter_map(|line| {
            // use the Option result directly and propagate missing groups via `?`
            re.captures(line).and_then(|captures| {
                let name = captures.get(2)?.as_str();
                let version = captures.get(3)?.as_str();
                Some((name, version))
            })
        })
        .collect::<Vec<_>>();

    let major_version: Option<u32> = version_parts.iter().find_map(|(name, version)| {
        if name.contains("MAJOR") {
            version.parse().ok()
        } else {
            None
        }
    });
    let minor_version: Option<u32> = version_parts.iter().find_map(|(name, version)| {
        if name.contains("MINOR") {
            version.parse().ok()
        } else {
            None
        }
    });

    let patch_version: Option<u32> = version_parts.iter().find_map(|(name, version)| {
        if name.contains("PATCH") {
            version.parse().ok()
        } else {
            None
        }
    });

    if major_version.is_none() || minor_version.is_none() || patch_version.is_none() {
        anyhow::bail!("failed to find version parts in bindings");
    }

    let obs_version = (
        major_version.unwrap(),
        minor_version.unwrap(),
        patch_version.unwrap(),
    );

    Ok(obs_version)
}
