# Bootstrap Options

`libobs-rs` offers two main ways to handle OBS binaries:

## 1. Runtime Bootstrapping (Recommended)

Using `libobs-bootstrapper` (integrated into `libobs-simple`), your application can download and install OBS binaries at runtime.

### Pros:
- Smaller application size (binaries downloaded on demand).
- Automatic updates.
- Easy distribution (just ship your exe).

### Cons:
- Requires internet connection on first run.
- Startup time is longer on first run.

[Example here](../examples/download-at-runtime)

## 2. Build-time Setup

Using `cargo-obs-build`, you can download OBS binaries during development or build time and bundle them.

### Pros:
- No internet required at runtime.
- Faster startup.

### Cons:
- Larger distribution size.
- Manual update management.

[Docs here](../cargo-obs-build/README.md)

## How to choose?

For most users, **Runtime Bootstrapping** is the easiest and best choice. If you are deploying to an offline environment or need instant startup, use **Build-time Setup**.
