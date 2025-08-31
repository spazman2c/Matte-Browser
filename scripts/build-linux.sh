#!/bin/bash

# Matte Browser Linux Build Script
# This script builds the Matte browser on Linux

set -e  # Exit on any error

echo "========================================"
echo "Matte Browser - Linux Build Script"
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

# Check for required packages
check_package() {
    if ! pkg-config --exists $1; then
        echo "Error: $1 development package not found. Please install $2"
        exit 1
    fi
}

echo "Checking required packages..."
check_package "x11" "libx11-dev"
check_package "xrandr" "libxrandr-dev"
check_package "xfixes" "libxfixes-dev"
check_package "xcursor" "libxcursor-dev"

# Create build directory
BUILD_DIR="build-linux"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Configure CMake
echo "Configuring CMake..."
cmake .. \
    -DCMAKE_BUILD_TYPE="$BUILD_TYPE" \
    -DCMAKE_CXX_FLAGS="-Wall -Wextra -Werror" \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
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
echo "- C++ components: $BUILD_DIR/"
echo "- Rust components: target/$BUILD_TYPE/"
echo
echo "To run the browser:"
echo "cargo run --$BUILD_TYPE"
