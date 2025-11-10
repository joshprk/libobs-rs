use std::{env, path::PathBuf};

use cargo_obs_build::{ObsBuildConfig, build_obs_binaries};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR")
        .unwrap();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .unwrap();
    let root_dir = PathBuf::from(manifest_dir);
    let root_dir = root_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

        let target_dir = PathBuf::from(&out_dir);
    let target_dir = target_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap();

    let config = ObsBuildConfig {
        out_dir: target_dir.to_path_buf(),
        cache_dir: root_dir.join("obs-build"),
        ..Default::default()
    };

    build_obs_binaries(config).unwrap();
}