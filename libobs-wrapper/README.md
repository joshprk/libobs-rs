# LibOBS Wrapper

This is a simple wrapper for the `libobs-new` crate.

## Prerequisites
Make sure that the OBS binaries are in your target directory. There's even a tool to help you build OBS from source! <br>
Install the tool
```bash
cargo install cargo-obs-build
```

Add the following to your `Cargo.toml`
```toml
[package.metadata]
# The libobs version to use (can either be a specific version or "latest")
libobs-version="31.0.3"
# The directory in which to store the OBS build (optional)
# libobs-cache-dir="../obs-build"

```

Install OBS in your target directory
```bash
# for debugging
cargo obs-build target/debug
# for release
cargo obs-build target/release
# for testing
cargo obs-build target/(debug|release)/deps
```

## OBS Bootstrapper

The library includes a bootstrapper that can download and install OBS binaries at runtime, which is useful for distributing applications without requiring users to install OBS separately.

### Usage

1. Add a dummy `obs.dll` file to your executable directory. This file will be replaced by the bootstrapper.
   - You can download a dummy DLL from [https://github.com/sshcrack/libobs-builds/releases](https://github.com/sshcrack/libobs-builds/releases)
   - Make sure to use the version that matches your target OBS version
   - Rename the downloaded file to `obs.dll`

2. Initialize the bootstrapper at application startup:

```rust
use libobs_wrapper::context::ObsContext;
use libobs_wrapper::bootstrap::{ObsBootstrap, ObsDownloaderOptions, BootstrapStatus};
use futures_util::StreamExt;

async fn bootstrap() -> anyhow::Result<()> {
    // Configure the download options
    let options = ObsDownloaderOptions::default();
    
    // Start the bootstrap process
    let bootstrap_stream = ObsContext::bootstrap(options).await?;
    pin_mut!(bootstrap_stream);
    
    // Process bootstrap status updates
    while let Some(status) = bootstrap_stream.next().await {
        match status {
            BootstrapStatus::Downloading(progress, message) => {
                println!("Downloading: {}% - {}", progress * 100.0, message);
            },
            BootstrapStatus::Extracting(progress, message) => {
                println!("Extracting: {}% - {}", progress * 100.0, message);
            },
            BootstrapStatus::Error(err) => {
                return Err(err);
            },
            BootstrapStatus::RestartRequired => {
                println!("OBS has been updated. Application needs to restart.");
                // Spawn the updater and exit
                ObsContext::spawn_updater().await?;
                std::process::exit(0);
            }
        }
    }

    // You can use ObsContext now

    Ok(())
}
```

3. When the bootstrapper returns `BootstrapStatus::RestartRequired`, call `ObsContext::spawn_updater()` and exit your application. The updater will:
   - Wait for your application to exit
   - Replace `obs.dll` with the newly downloaded version
   - Restart your application with the same command-line arguments

## Usage

Note: This is the usage without using the [libobs-sources](https://crates.io/crates/libobs-sources) crate (which makes it significantly easier to create sources)
### [Example](https://github.com/joshprk/libobs-rs/blob/main/libobs-wrapper/tests/record_test.rs)