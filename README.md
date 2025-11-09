# libobs-rs
![Build](https://img.shields.io/github/actions/workflow/status/joshprk/libobs-rs/validation.yml?branch=main&label=build&style=flat)
![Docs](https://img.shields.io/github/actions/workflow/status/joshprk/libobs-rs/build-docs.yml?branch=main&label=docs&style=flat)
![Coverage](https://img.shields.io/badge/coverage-55%25-orange?style=flat)




Simple and safe video recording through libobs.

Currently only tested on Windows. Linux and MacOS likely work, but they are untested. These will receive support later down the road.

The API is currently unstable and will definitely have breaking revisions in the future.

To build on Linux, you must install the libobs-dev package, as well as the bindgen dependencies.
```
sudo apt-get libobs-dev llvm-dev libclang-dev clang
```


> [!NOTE]
> The libobs-wrapper async functionality has been removed because of all kinds of issues ([#32](https://github.com/joshprk/libobs-rs/issues/32))


## Prerequisites
Make sure that the OBS binaries are in your target directory. There's even a tool to help you build OBS from source! <br>
Install the tool
```bash
cargo install cargo-obs-build
```

> [!NOTE]
> There is now a standalone `libobs-bootstrapper` crate that can download and install OBS binaries at runtime, which is useful for distributing applications without requiring users to install OBS separately. See the [libobs-bootstrapper documentation](https://crates.io/crates/libobs-bootstrapper) for more details.

Add the following to your `Cargo.toml`
```toml
[package.metadata]
# The libobs version to use (can either be a specific version or "latest")
# This is optional; if not specified, the version will be selected based on the libobs crate version.
# libobs-version="31.0.3"
# The directory in which to store the OBS build (optional)
# libobs-cache-dir="../obs-build"

```

Install OBS in your target directory. This uses the original signed OBS binaries.
```bash
# for debugging
cargo obs-build --out-dir target/debug
# for release
cargo obs-build --out-dir target/release
# for testing
cargo obs-build --out-dir target/(debug|release)/deps
```

> [!NOTE]
> You can specify a `GITHUB_TOKEN` environment variable to increase the rate limit when downloading releases from GitHub. This is especially useful for CI environments.


## Quick Start

Below is an example that will record video-only footage of an exclusive fullscreen application. Note that the API is extremely limited right now, but you can already record both video and audio with full control over the output already. If you need more, libobs is exposed.

Examples are located in the [examples](./examples) directory.
Documentation is also available for [libobs-sources](./libobs-sources/README.md) or [libobs-wrapper](./libobs-wrapper/README.md).
