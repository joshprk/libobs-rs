# Example build.rs for using cargo-obs-build as a library

This is an example `build.rs` file that demonstrates how to use the `cargo-obs-build` library in a build script to automatically download and install OBS binaries.

```rust
use cargo_obs_build::{build_obs_binaries, ObsBuildConfig};
use std::env;
use std::path::PathBuf;

fn main() {
    // Set up logging (optional, but recommended)
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Get the target directory from cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(&out_dir)
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.join("obs-binaries"))
        .expect("Failed to determine target directory");

    // Configure the build
    let config = ObsBuildConfig {
        out_dir: target_dir.clone(),
        cache_dir: PathBuf::from("obs-build"), // Cache directory for downloads
        repo_id: "obsproject/obs-studio".to_string(),
        override_zip: None,
        rebuild: false,
        browser: false, // Set to true if you need browser support
        tag: None, // Will auto-detect from libobs crate version
        skip_compatibility_check: false,
    };

    // Build and install OBS binaries
    match build_obs_binaries(config) {
        Ok(_) => {
            println!("cargo:warning=OBS binaries successfully installed to {}", target_dir.display());
            
            // Tell cargo to link to the OBS libraries
            println!("cargo:rustc-link-search=native={}", target_dir.display());
            
            // Rerun if the cache directory changes
            println!("cargo:rerun-if-changed=obs-build");
        }
        Err(e) => {
            eprintln!("Failed to build OBS binaries: {}", e);
            std::process::exit(1);
        }
    }
}
```

## Adding to your Cargo.toml

Add this to your `[build-dependencies]`:

```toml
[build-dependencies]
cargo-obs-build = { version = "1.2.4", default-features = false }
env_logger = "0.11" # Optional, for logging
```

Note: We use `default-features = false` to avoid pulling in CLI-specific dependencies (clap, colored, fern) that aren't needed in a build script.

## Customization

You can customize the build by modifying the `ObsBuildConfig`:

- `out_dir`: Where to copy the final binaries
- `cache_dir`: Where to cache downloaded files (useful for CI caching)
- `repo_id`: Use a different OBS repository
- `browser`: Include browser support in the binaries
- `tag`: Specify a specific OBS version (or "latest")
- `rebuild`: Force a rebuild even if cached
- `skip_compatibility_check`: Skip version compatibility warnings

## Workspace Metadata

You can also configure the library version and cache directory in your `Cargo.toml`:

```toml
[workspace.metadata]
libobs-version = "30.2.2"  # Specific version to use
libobs-cache-dir = "../obs-build"  # Shared cache directory
```

These settings will be automatically picked up by the library.
