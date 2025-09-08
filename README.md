# libobs-rs

Simple and safe video recording through libobs.

Currently only tested on Windows. Linux and MacOS likely work, but they are untested. These will receive support later down the road.

The API is currently unstable and will definitely have breaking revisions in the future.

To build on Linux, you must install the libobs-dev package, as well as the bindgen dependencies.
```
sudo apt-get libobs-dev llvm-dev libclang-dev clang
```


> [!NOTE]
> The libobs-wrapper API is now fully sendable and async. If you do not want to use the async runtime, enable the feature `blocking` for `libobs-wrapper` & `libobs-sources`


## Prerequisites
Make sure that the OBS binaries are in your target directory. There's even a tool to help you build OBS from source! <br>
Install the tool
```bash
cargo install cargo-obs-build
```

> [!NOTE]
> There is now a new bootstrapper that can download and install OBS binaries at runtime, which is useful for distributing applications without requiring users to install OBS separately. See the [OBS Bootstrapper](./libobs-wrapper/README.md#obs-bootstrapper) section for more details.

Add the following to your `Cargo.toml`
```toml
[package.metadata]
# The libobs version to use (can either be a specific version or "latest")
libobs-version="31.0.3"
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


## Quick Start

Below is an example that will record video-only footage of an exclusive fullscreen application. Note that the API is extremely limited right now, but you can already record both video and audio with full control over the output already. If you need more, libobs is exposed.

Examples are located in the [examples](./examples) directory.
Documentation is also available for [libobs-sources](./libobs-sources/README.md) or [libobs-wrapper](./libobs-wrapper/README.md).
