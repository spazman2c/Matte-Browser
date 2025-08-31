#!/bin/bash

# Matte Browser - Hermetic Toolchain Setup
# This script sets up hermetic toolchains for reproducible builds

set -e  # Exit on any error

echo "========================================"
echo "Matte Browser - Hermetic Toolchain Setup"
echo "========================================"

# Configuration
TOOLCHAIN_DIR="${TOOLCHAIN_DIR:-./toolchains}"
RUST_VERSION="1.75.0"
LLVM_VERSION="17.0.6"
CMAKE_VERSION="3.28.1"

# Create toolchain directory
mkdir -p "$TOOLCHAIN_DIR"

# Function to download and extract toolchain
download_toolchain() {
    local url="$1"
    local filename="$2"
    local extract_dir="$3"
    
    echo "Downloading $filename..."
    if [ ! -f "$TOOLCHAIN_DIR/$filename" ]; then
        curl -L -o "$TOOLCHAIN_DIR/$filename" "$url"
    fi
    
    echo "Extracting $filename..."
    if [ ! -d "$TOOLCHAIN_DIR/$extract_dir" ]; then
        case "$filename" in
            *.tar.gz|*.tgz)
                tar -xzf "$TOOLCHAIN_DIR/$filename" -C "$TOOLCHAIN_DIR"
                ;;
            *.tar.xz)
                tar -xJf "$TOOLCHAIN_DIR/$filename" -C "$TOOLCHAIN_DIR"
                ;;
            *.zip)
                unzip -q "$TOOLCHAIN_DIR/$filename" -d "$TOOLCHAIN_DIR"
                ;;
        esac
    fi
}

# Detect platform
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

echo "Platform: $PLATFORM"
echo "Architecture: $ARCH"

# Set up Rust toolchain
echo "Setting up Rust toolchain..."
RUSTUP_HOME="$TOOLCHAIN_DIR/rust" CARGO_HOME="$TOOLCHAIN_DIR/cargo" rustup-init --default-toolchain "$RUST_VERSION" --no-modify-path -y

# Set up LLVM toolchain
echo "Setting up LLVM toolchain..."
case "$PLATFORM" in
    darwin)
        if [[ "$ARCH" == "arm64" ]]; then
            LLVM_URL="https://github.com/llvm/llvm-project/releases/download/llvmorg-$LLVM_VERSION/clang+llvm-$LLVM_VERSION-arm64-apple-darwin.tar.xz"
            LLVM_FILE="clang+llvm-$LLVM_VERSION-arm64-apple-darwin.tar.xz"
            LLVM_DIR="clang+llvm-$LLVM_VERSION-arm64-apple-darwin"
        else
            LLVM_URL="https://github.com/llvm/llvm-project/releases/download/llvmorg-$LLVM_VERSION/clang+llvm-$LLVM_VERSION-x86_64-apple-darwin.tar.xz"
            LLVM_FILE="clang+llvm-$LLVM_VERSION-x86_64-apple-darwin.tar.xz"
            LLVM_DIR="clang+llvm-$LLVM_VERSION-x86_64-apple-darwin"
        fi
        ;;
    linux)
        if [[ "$ARCH" == "aarch64" ]]; then
            LLVM_URL="https://github.com/llvm/llvm-project/releases/download/llvmorg-$LLVM_VERSION/clang+llvm-$LLVM_VERSION-aarch64-linux-gnu.tar.xz"
            LLVM_FILE="clang+llvm-$LLVM_VERSION-aarch64-linux-gnu.tar.xz"
            LLVM_DIR="clang+llvm-$LLVM_VERSION-aarch64-linux-gnu"
        else
            LLVM_URL="https://github.com/llvm/llvm-project/releases/download/llvmorg-$LLVM_VERSION/clang+llvm-$LLVM_VERSION-x86_64-linux-gnu-ubuntu-20.04.tar.xz"
            LLVM_FILE="clang+llvm-$LLVM_VERSION-x86_64-linux-gnu-ubuntu-20.04.tar.xz"
            LLVM_DIR="clang+llvm-$LLVM_VERSION-x86_64-linux-gnu-ubuntu-20.04"
        fi
        ;;
    msys_nt*|cygwin*)
        LLVM_URL="https://github.com/llvm/llvm-project/releases/download/llvmorg-$LLVM_VERSION/LLVM-$LLVM_VERSION-win64.exe"
        LLVM_FILE="LLVM-$LLVM_VERSION-win64.exe"
        LLVM_DIR="LLVM-$LLVM_VERSION-win64"
        ;;
esac

download_toolchain "$LLVM_URL" "$LLVM_FILE" "$LLVM_DIR"

# Set up CMake
echo "Setting up CMake..."
case "$PLATFORM" in
    darwin)
        if [[ "$ARCH" == "arm64" ]]; then
            CMAKE_URL="https://github.com/Kitware/CMake/releases/download/v$CMAKE_VERSION/cmake-$CMAKE_VERSION-macos-universal.tar.gz"
            CMAKE_FILE="cmake-$CMAKE_VERSION-macos-universal.tar.gz"
            CMAKE_DIR="cmake-$CMAKE_VERSION-macos-universal"
        else
            CMAKE_URL="https://github.com/Kitware/CMake/releases/download/v$CMAKE_VERSION/cmake-$CMAKE_VERSION-macos-universal.tar.gz"
            CMAKE_FILE="cmake-$CMAKE_VERSION-macos-universal.tar.gz"
            CMAKE_DIR="cmake-$CMAKE_VERSION-macos-universal"
        fi
        ;;
    linux)
        CMAKE_URL="https://github.com/Kitware/CMake/releases/download/v$CMAKE_VERSION/cmake-$CMAKE_VERSION-linux-x86_64.tar.gz"
        CMAKE_FILE="cmake-$CMAKE_VERSION-linux-x86_64.tar.gz"
        CMAKE_DIR="cmake-$CMAKE_VERSION-linux-x86_64"
        ;;
    msys_nt*|cygwin*)
        CMAKE_URL="https://github.com/Kitware/CMake/releases/download/v$CMAKE_VERSION/cmake-$CMAKE_VERSION-windows-x86_64.zip"
        CMAKE_FILE="cmake-$CMAKE_VERSION-windows-x86_64.zip"
        CMAKE_DIR="cmake-$CMAKE_VERSION-windows-x86_64"
        ;;
esac

download_toolchain "$CMAKE_URL" "$CMAKE_FILE" "$CMAKE_DIR"

# Create environment setup script
echo "Creating environment setup script..."
cat > "$TOOLCHAIN_DIR/setup-env.sh" << 'EOF'
#!/bin/bash
# Matte Browser Hermetic Toolchain Environment Setup

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Set up Rust environment
export RUSTUP_HOME="$SCRIPT_DIR/rust"
export CARGO_HOME="$SCRIPT_DIR/cargo"
export PATH="$CARGO_HOME/bin:$PATH"

# Set up LLVM environment
export LLVM_ROOT="$SCRIPT_DIR/clang+llvm-17.0.6-$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')"
if [[ "$(uname -s)" == "Darwin" ]]; then
    export LLVM_ROOT="$SCRIPT_DIR/cmake-3.28.1-macos-universal/CMake.app/Contents/bin"
fi
export PATH="$LLVM_ROOT/bin:$PATH"

# Set up CMake environment
export CMAKE_ROOT="$SCRIPT_DIR/cmake-3.28.1-$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')"
if [[ "$(uname -s)" == "Darwin" ]]; then
    export CMAKE_ROOT="$SCRIPT_DIR/cmake-3.28.1-macos-universal/CMake.app/Contents"
fi
export PATH="$CMAKE_ROOT/bin:$PATH"

# Set compiler environment variables
export CC="$LLVM_ROOT/bin/clang"
export CXX="$LLVM_ROOT/bin/clang++"
export AR="$LLVM_ROOT/bin/llvm-ar"
export LD="$LLVM_ROOT/bin/ld64.lld"
export RANLIB="$LLVM_ROOT/bin/llvm-ranlib"
export STRIP="$LLVM_ROOT/bin/llvm-strip"

# Platform-specific settings
case "$(uname -s)" in
    Darwin)
        export MACOSX_DEPLOYMENT_TARGET="13.0"
        export SDKROOT="$(xcrun --show-sdk-path)"
        ;;
    Linux)
        export CFLAGS="-fPIC"
        export CXXFLAGS="-fPIC"
        ;;
esac

echo "Hermetic toolchain environment activated:"
echo "  Rust: $RUSTUP_HOME"
echo "  LLVM: $LLVM_ROOT"
echo "  CMake: $CMAKE_ROOT"
echo "  CC: $CC"
echo "  CXX: $CXX"
EOF

chmod +x "$TOOLCHAIN_DIR/setup-env.sh"

# Create build configuration
echo "Creating hermetic build configuration..."
cat > "$TOOLCHAIN_DIR/hermetic-build.sh" << 'EOF'
#!/bin/bash
# Matte Browser Hermetic Build Script

set -e

# Source the hermetic environment
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/setup-env.sh"

# Build configuration
BUILD_TYPE=${1:-release}
TARGET=${2:-$(rustc --print target-list | grep -E "(x86_64|aarch64).*$(uname -s | tr '[:upper:]' '[:lower:]')" | head -1)}

echo "========================================"
echo "Matte Browser - Hermetic Build"
echo "========================================"
echo "Build type: $BUILD_TYPE"
echo "Target: $TARGET"
echo "Using hermetic toolchain:"
echo "  Rust: $(rustc --version)"
echo "  Clang: $(clang --version | head -1)"
echo "  CMake: $(cmake --version | head -1)"

# Add target if needed
rustup target add "$TARGET" 2>/dev/null || true

# Build the project
echo "Building Matte Browser..."
cargo build --target "$TARGET" --"$BUILD_TYPE"

# Run tests
echo "Running tests..."
cargo test --target "$TARGET" --"$BUILD_TYPE"

echo "========================================"
echo "Hermetic build completed successfully!"
echo "========================================"
EOF

chmod +x "$TOOLCHAIN_DIR/hermetic-build.sh"

# Create toolchain verification script
echo "Creating toolchain verification script..."
cat > "$TOOLCHAIN_DIR/verify-toolchain.sh" << 'EOF'
#!/bin/bash
# Matte Browser Toolchain Verification Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/setup-env.sh"

echo "========================================"
echo "Matte Browser - Toolchain Verification"
echo "========================================"

# Check Rust
echo "Checking Rust toolchain..."
if command -v rustc >/dev/null 2>&1; then
    echo "✓ Rust: $(rustc --version)"
    echo "✓ Cargo: $(cargo --version)"
else
    echo "✗ Rust not found"
    exit 1
fi

# Check LLVM/Clang
echo "Checking LLVM/Clang toolchain..."
if command -v clang >/dev/null 2>&1; then
    echo "✓ Clang: $(clang --version | head -1)"
    echo "✓ Clang++: $(clang++ --version | head -1)"
else
    echo "✗ Clang not found"
    exit 1
fi

# Check CMake
echo "Checking CMake..."
if command -v cmake >/dev/null 2>&1; then
    echo "✓ CMake: $(cmake --version | head -1)"
else
    echo "✗ CMake not found"
    exit 1
fi

# Check target support
echo "Checking target support..."
TARGETS=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin")
for target in "${TARGETS[@]}"; do
    if rustup target list | grep -q "$target"; then
        echo "✓ Target $target available"
    else
        echo "✗ Target $target not available"
    fi
done

echo "========================================"
echo "Toolchain verification completed!"
echo "========================================"
EOF

chmod +x "$TOOLCHAIN_DIR/verify-toolchain.sh"

echo "========================================"
echo "Hermetic toolchain setup completed!"
echo "========================================"
echo
echo "Next steps:"
echo "1. Source the environment: source $TOOLCHAIN_DIR/setup-env.sh"
echo "2. Verify toolchain: $TOOLCHAIN_DIR/verify-toolchain.sh"
echo "3. Build with hermetic toolchain: $TOOLCHAIN_DIR/hermetic-build.sh"
echo
echo "Toolchain location: $TOOLCHAIN_DIR"
