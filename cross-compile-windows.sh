#!/bin/bash

# Cross-compile libobs-rs record example to Windows x64 from macOS
# Prerequisites:
#   brew install mingw-w64
#   rustup target add x86_64-pc-windows-gnu

set -e

echo "=== Cross-Compiling to Windows x64 ==="
echo ""

# Check prerequisites
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "ERROR: mingw-w64 is not installed"
    echo ""
    echo "Install with:"
    echo "  brew install mingw-w64"
    exit 1
fi

if ! rustup target list --installed | grep -q x86_64-pc-windows-gnu; then
    echo "ERROR: Windows GNU target not installed"
    echo ""
    echo "Add with:"
    echo "  rustup target add x86_64-pc-windows-gnu"
    exit 1
fi

echo "Prerequisites installed"
echo ""

# Step 1: Download Windows OBS binaries
echo "==> Downloading Windows OBS binaries..."
OBS_BUILD_TARGET_OS=windows OBS_BUILD_TARGET_ARCH=x86_64 \
    cargo run -p cargo-obs-build -- --out-dir target/x86_64-pc-windows-gnu/debug --tag 32.0.2 --rebuild

echo ""
echo "==> Cross-compiling record binary to Windows..."
cargo build --target x86_64-pc-windows-gnu --bin record

echo ""
echo "Build complete!"
echo ""
echo "Windows binary:"
echo "    target/x86_64-pc-windows-gnu/debug/record.exe"
echo ""
echo "To test on Windows:"
echo "  1. Copy target/x86_64-pc-windows-gnu/debug/ directory to Windows"
echo "  2. Run: record.exe"
echo ""
echo "The directory includes all required OBS DLLs and plugins."

