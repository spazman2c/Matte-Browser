#!/bin/bash

# Matte Browser macOS Build Script
# This script builds the Matte browser on macOS using Xcode

set -e  # Exit on any error

echo "========================================"
echo "Matte Browser - macOS Build Script"
echo "========================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

# Set build configuration
BUILD_TYPE=${1:-release}
TARGET_ARCH=${2:-x86_64}

# Check for required tools
if ! command -v cmake &> /dev/null; then
    echo "Error: cmake not found. Please install CMake."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found. Please install Rust."
    exit 1
fi

# Check for Xcode
if ! xcode-select -p &> /dev/null; then
    echo "Error: Xcode not found. Please install Xcode Command Line Tools."
    exit 1
fi

# Set macOS deployment target
export MACOSX_DEPLOYMENT_TARGET="13.0"

# Create build directory
BUILD_DIR="build-macos"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Configure CMake
echo "Configuring CMake..."
cmake .. \
    -G "Xcode" \
    -DCMAKE_BUILD_TYPE="$BUILD_TYPE" \
    -DCMAKE_OSX_ARCHITECTURES="$TARGET_ARCH" \
    -DCMAKE_OSX_DEPLOYMENT_TARGET="13.0" \
    -DCMAKE_CXX_FLAGS="-Wall -Wextra -Werror"
if [ $? -ne 0 ]; then
    echo "Error: CMake configuration failed."
    exit 1
fi

# Build the project
echo "Building Matte browser..."
cmake --build . --config "$BUILD_TYPE" --parallel
if [ $? -ne 0 ]; then
    echo "Error: Build failed."
    exit 1
fi

# Build Rust components
echo "Building Rust components..."
cd ..
cargo build --"$BUILD_TYPE"
if [ $? -ne 0 ]; then
    echo "Error: Rust build failed."
    exit 1
fi

# Run tests
echo "Running tests..."
cargo test --"$BUILD_TYPE"
if [ $? -ne 0 ]; then
    echo "Warning: Some tests failed."
fi

echo "========================================"
echo "Build completed successfully!"
echo "========================================"
echo
echo "Build artifacts:"
echo "- C++ components: $BUILD_DIR/build/$BUILD_TYPE/"
echo "- Rust components: target/$BUILD_TYPE/"
echo
echo "To run the browser:"
echo "cargo run --$BUILD_TYPE"
