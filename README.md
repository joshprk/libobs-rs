# libobs-rs

Simple and safe video recording through libobs.

Currently only tested on Windows. Linux and MacOS likely work, but they are untested. These will receive support later down the road.

The API is currently unstable and will definitely have breaking revisions in the future.

To build on Linux, you must install the libobs-dev package, as well as the bindgen dependencies.
```
sudo apt-get libobs-dev llvm-dev libclang-dev clang
```

Compiled Windows DLLs for libobs can be found at https://github.com/joshprk/libobs-rs/releases/tag/deps

## Quick Start

Below is an example that will record video-only footage of an exclusive fullscreen application. Note that the API is extremely limited right now, but you can already record both video and audio with full control over the output already. If you need more, libobs is exposed.

For examples look at [libobs-sources](./libobs-sources/README.md) or the [wrapper](./libobs-wrapper/README.md).sadf
