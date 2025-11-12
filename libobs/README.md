# libobs-rs

This is currently just a wrapper around the C API of OBS Studio.
The stable API is available in the [`libobs-wrapper`](https://crates.io/crates/libobs-wrapper) crate.

## Building on Linux

### Prerequisites

Before building this crate on Linux, you need to install the following system dependencies:

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y clang libclang-dev libsimde-dev libxcb1-dev libx11-dev
```

#### Fedora/RHEL/CentOS
```bash
sudo dnf install clang clang-devel simde-devel libxcb-devel libX11-devel
# or on older versions:
# sudo yum install clang clang-devel simde-devel
```

#### Arch Linux
```bash
sudo pacman -S clang simde libxcb libx11
```

### Build Instructions

Once the dependencies are installed, you can build the crate:

```bash
cargo build
```

For a release build:

```bash
cargo build --release
```

### Dependencies Explained

- **clang**: Required by bindgen to parse C headers and generate Rust bindings
- **libclang-dev**: Development headers for libclang
- **libsimde-dev**: SIMD Everywhere library headers, required by OBS headers for cross-platform SIMD operations

### Troubleshooting

If you encounter build errors:

1. Make sure all system dependencies are installed
2. Verify that clang is in your PATH: `clang --version`
3. If you have a custom OBS installation, set the `LIBOBS_PATH` environment variable to point to your OBS library directory