
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RunArgs {
    /// The directory the OBS Studio binaries should be copied to
    #[arg(short, long)]
    pub out_dir: String,

    /// The location where the OBS Studio sources should be cloned to
    #[arg(short, long, default_value = "obs-build")]
    pub cache_dir: PathBuf,

    /// The github repository to clone OBS Studio from
    #[arg(long, default_value = "obsproject/obs-studio")]
    pub repo_id: String,

    #[arg(long)]
    /// If this is specified, the specified zip file will be used instead of downloading the latest release
    /// This is useful for testing purposes, but it is not recommended to use this in production
    pub override_zip: Option<PathBuf>,

    /// When this flag is active, the cache will be cleared and a new build will be started
    #[arg(short, long, default_value_t = false)]
    pub rebuild: bool,

    /// If the browser should be included in the build
    #[arg(short, long, default_value_t = false)]
    pub browser: bool,

    /// The tag of the OBS Studio release to build.
    /// If none is specified, the matching release for the libobs crate will be used.
    /// Use `latest` for the latest obs release. If a version in the `workspace.metadata` is set, that version will be used.
    #[arg(short, long)]
    pub tag: Option<String>,

    /// If the browser should be included in the build
    #[arg(short, long, default_value_t = false)]
    pub skip_compatibility_check: bool,
}

