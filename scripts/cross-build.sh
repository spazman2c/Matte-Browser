#!/bin/bash

# Matte Browser Cross-Compilation Build Script
# This script builds the Matte browser for different target architectures

set -e  # Exit on any error

echo "========================================"
echo "Matte Browser - Cross-Compilation Build"
echo "========================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

# Default values
TARGET=${1:-x86_64-unknown-linux-gnu}
BUILD_TYPE=${2:-release}

# Available targets
declare -A TARGETS=(
    ["x86_64-unknown-linux-gnu"]="Linux x86_64"
    ["aarch64-unknown-linux-gnu"]="Linux ARM64"
    ["x86_64-pc-windows-gnu"]="Windows x86_64 (MinGW)"
    ["x86_64-pc-windows-msvc"]="Windows x86_64 (MSVC)"
    ["x86_64-apple-darwin"]="macOS x86_64"
    ["aarch64-apple-darwin"]="macOS ARM64"
    ["aarch64-linux-android"]="Android ARM64"
    ["armv7-linux-androideabi"]="Android ARMv7"
    ["i686-linux-android"]="Android x86"
    ["x86_64-linux-android"]="Android x86_64"
)

# Check if target is valid
if [[ ! ${TARGETS[$TARGET]+_} ]]; then
    echo "Error: Invalid target '$TARGET'"
    echo "Available targets:"
    for target in "${!TARGETS[@]}"; do
        echo "  $target - ${TARGETS[$target]}"
    done
    exit 1
fi

echo "Building for target: $TARGET (${TARGETS[$TARGET]})"
echo "Build type: $BUILD_TYPE"

# Add target if not already added
rustup target add "$TARGET" 2>/dev/null || true

# Set up environment variables for cross-compilation
export CARGO_TARGET_DIR="target-cross"
export RUSTFLAGS="-C target-feature=+crt-static"

# Platform-specific setup
case "$TARGET" in
    *-windows-*)
        # Windows targets
        if [[ "$TARGET" == *-msvc ]]; then
            # MSVC targets
            export CC="cl"
            export CXX="cl"
        else
            # MinGW targets
            export CC="x86_64-w64-mingw32-gcc"
            export CXX="x86_64-w64-mingw32-g++"
            export AR="x86_64-w64-mingw32-ar"
        fi
        ;;
    *-apple-darwin)
        # macOS targets
        export CC="clang"
        export CXX="clang++"
        export AR="ar"
        export MACOSX_DEPLOYMENT_TARGET="13.0"
        ;;
    *-linux-*)
        # Linux targets
        if [[ "$TARGET" == *-android* ]]; then
            # Android targets
            export ANDROID_NDK_ROOT="${ANDROID_NDK_ROOT:-$HOME/Android/Sdk/ndk}"
            if [ ! -d "$ANDROID_NDK_ROOT" ]; then
                echo "Error: ANDROID_NDK_ROOT not set or not found"
                echo "Please set ANDROID_NDK_ROOT to your Android NDK installation"
                exit 1
            fi
            
            # Set up Android toolchain
            case "$TARGET" in
                aarch64-linux-android)
                    export CC="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
                    export CXX="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang++"
                    export AR="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"
                    ;;
                armv7-linux-androideabi)
                    export CC="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang"
                    export CXX="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang++"
                    export AR="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi-ar"
                    ;;
                i686-linux-android)
                    export CC="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android21-clang"
                    export CXX="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android21-clang++"
                    export AR="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar"
                    ;;
                x86_64-linux-android)
                    export CC="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android21-clang"
                    export CXX="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android21-clang++"
                    export AR="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android-ar"
                    ;;
            esac
        else
            # Native Linux targets
            case "$TARGET" in
                x86_64-unknown-linux-gnu)
                    export CC="x86_64-linux-gnu-gcc"
                    export CXX="x86_64-linux-gnu-g++"
                    export AR="x86_64-linux-gnu-ar"
                    ;;
                aarch64-unknown-linux-gnu)
                    export CC="aarch64-linux-gnu-gcc"
                    export CXX="aarch64-linux-gnu-g++"
                    export AR="aarch64-linux-gnu-ar"
                    ;;
            esac
        fi
        ;;
esac

# Build Rust components
echo "Building Rust components for $TARGET..."
cargo build --target "$TARGET" --"$BUILD_TYPE"
if [ $? -ne 0 ]; then
    echo "Error: Rust build failed for $TARGET"
    exit 1
fi

# Run tests for the target
echo "Running tests for $TARGET..."
cargo test --target "$TARGET" --"$BUILD_TYPE"
if [ $? -ne 0 ]; then
    echo "Warning: Some tests failed for $TARGET"
fi

echo "========================================"
echo "Cross-compilation completed successfully!"
echo "========================================"
echo
echo "Build artifacts:"
echo "- Target: $TARGET"
echo "- Location: $CARGO_TARGET_DIR/$TARGET/$BUILD_TYPE/"
echo
echo "To run the browser (if supported on current platform):"
echo "cargo run --target $TARGET --$BUILD_TYPE"
