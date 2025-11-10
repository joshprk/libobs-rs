use std::{env, path::PathBuf};

use cargo_obs_build::{ObsBuildConfig, build_obs_binaries};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    cargo_obs_build::install().unwrap();
}