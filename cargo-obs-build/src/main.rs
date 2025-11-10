use std::env::{self, args};
use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

use cargo_obs_build::{build_obs_binaries, ObsBuildConfig};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RunArgs {
    /// The directory the OBS Studio binaries should be copied to
    #[arg(short, long)]
    out_dir: String,

    /// The location where the OBS Studio sources should be cloned to
    #[arg(short, long, default_value = "obs-build")]
    cache_dir: PathBuf,

    /// The github repository to clone OBS Studio from
    #[arg(long, default_value = "obsproject/obs-studio")]
    repo_id: String,

    #[arg(long)]
    /// If this is specified, the specified zip file will be used instead of downloading the latest release
    /// This is useful for testing purposes, but it is not recommended to use this in production
    override_zip: Option<PathBuf>,

    /// When this flag is active, the cache will be cleared and a new build will be started
    #[arg(short, long, default_value_t = false)]
    rebuild: bool,

    /// If the browser should be included in the build
    #[arg(short, long, default_value_t = false)]
    browser: bool,

    /// The tag of the OBS Studio release to build.
    /// If none is specified, the matching release for the libobs crate will be used.
    /// Use `latest` for the latest obs release. If a version in the `workspace.metadata` is set, that version will be used.
    #[arg(short, long)]
    tag: Option<String>,

    /// If the browser should be included in the build
    #[arg(short, long, default_value_t = false)]
    skip_compatibility_check: bool,
}

fn setup_logger() -> Result<(), fern::InitError> {
    let level = env::var("RUST_LOG")
        .ok()
        .and_then(|val| val.parse().ok()) // Try parsing e.g. "debug", "warn", etc.
        .unwrap_or(log::LevelFilter::Info); // Default if not set

    fern::Dispatch::new()
        .format(|out, message, record| {
            let level_color = match record.level() {
                log::Level::Error => "red",
                log::Level::Warn => "yellow",
                log::Level::Info => "green",
                log::Level::Debug => "blue",
                log::Level::Trace => "bright_black",
            };
            out.finish(format_args!(
                "[{}] {}",
                record.level().to_string().color(level_color),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    setup_logger()?;

    let mut args: Vec<_> = args().collect();
    if args.get(1).is_some_and(|e| e == "obs-build") {
        args.remove(1);
    }

    let args = RunArgs::parse_from(args);

    let config = ObsBuildConfig {
        out_dir: PathBuf::from(args.out_dir),
        cache_dir: args.cache_dir,
        repo_id: args.repo_id,
        override_zip: args.override_zip,
        rebuild: args.rebuild,
        browser: args.browser,
        tag: args.tag,
        skip_compatibility_check: args.skip_compatibility_check,
    };

    build_obs_binaries(config)?;

    Ok(())
}
