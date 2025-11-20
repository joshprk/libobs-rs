# libobs-rs Documentation

Welcome to the documentation for `libobs-rs`, a safe and idiomatic Rust wrapper for the OBS Studio API (`libobs`).

## Library Structure

The library is split into two main crates:

- **`libobs-wrapper`**: A low-level, safe wrapper around the raw C API. It handles thread safety, memory management, and provides Rust-native types for OBS objects. Use this if you need fine-grained control or are building complex plugins.
- **`libobs-simple`**: A high-level, opinionated layer built on top of `libobs-wrapper`. It provides easy-to-use builders and managers for common tasks like creating sources, scenes, and encoders. This is the recommended starting point for most users.

## Guides

- [Getting Started](getting_started.md): Prerequisites, installation, and your first "Hello World" with OBS.
- [Creating Custom Sources](custom_sources.md): Learn how to create your own OBS sources using the powerful `libobs-simple-macro` system.

## Examples

You can find fully working examples in the `examples/` directory of the repository.

- **`simple_recording.rs`**: A basic example showing how to record a scene.
- **`window_capture.rs`**: Demonstrates how to capture a specific window.
