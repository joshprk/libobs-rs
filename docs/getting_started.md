# Getting Started with libobs-rs

This guide will help you set up a new project using `libobs-rs` and record your first video.

## Prerequisites

- Rust installed (stable toolchain)
- Windows (currently the only supported platform for easy setup)

## Step 1: Create a new project

```bash
cargo new my-obs-recorder
cd my-obs-recorder
```

## Step 2: Add dependencies

Add `libobs-simple` to your `Cargo.toml`.

```toml
[dependencies]
libobs-simple = "0.1" # Replace with actual version
tokio = { version = "1", features = ["full"] }
```

## Step 3: Write the code

Replace `src/main.rs` with the following:

```rust
use libobs_simple::quick_start::quick_start;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize OBS context with auto-download
    let context = quick_start().await?;
    
    println!("OBS initialized successfully!");
    
    // Your recording logic here...
    
    Ok(())
}
```

## Step 4: Run it

```bash
cargo run
```

The first run will automatically download and install the necessary OBS binaries.
