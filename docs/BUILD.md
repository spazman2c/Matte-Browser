# Matte Browser - Build Documentation

This document describes how to build the Matte Browser project using various build systems and configurations.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Build Systems](#build-systems)
3. [Hermetic Builds](#hermetic-builds)
4. [Cross-Compilation](#cross-compilation)
5. [Platform-Specific Builds](#platform-specific-builds)
6. [Docker Builds](#docker-builds)
7. [Build Configuration](#build-configuration)
8. [Troubleshooting](#troubleshooting)

## Quick Start

### Prerequisites

- Rust 1.75.0 or later
- Cargo (comes with Rust)
- CMake 3.28.1 or later
- Platform-specific development tools

### Basic Build

```bash
# Clone the repository
git clone https://github.com/your-org/matte-browser.git
cd matte-browser

# Build in release mode
cargo build --release

# Run tests
cargo test --release

# Run the browser
cargo run --release
```

## Build Systems

### Cargo (Primary)

The primary build system is Cargo, Rust's package manager. It handles:
- Rust code compilation
- Dependency management
- Testing
- Documentation generation

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc

# Check code quality
cargo clippy
cargo fmt
```

### CMake (C++ Components)

For any C++ components or native dependencies:

```bash
# Configure CMake
cmake -B build -S .

# Build C++ components
cmake --build build

# Install (if needed)
cmake --install build
```

## Hermetic Builds

Hermetic builds ensure reproducible builds by using isolated toolchains and dependencies.

### Local Hermetic Toolchain

```bash
# Set up hermetic toolchain
./scripts/hermetic-toolchain.sh

# Activate hermetic environment
source toolchains/setup-env.sh

# Verify toolchain
./toolchains/verify-toolchain.sh

# Build with hermetic toolchain
./toolchains/hermetic-build.sh release
```

### Docker Hermetic Builds

```bash
# Build hermetic Docker image
docker build -f Dockerfile.hermetic -t matte-browser-hermetic .

# Run hermetic build
docker run -v $(pwd):/home/builder/matte-browser:ro matte-browser-hermetic

# Or use Docker Compose
docker-compose -f docker-compose.hermetic.yml up hermetic-build
```

### Docker Compose Services

- `hermetic-build`: Release build for x86_64 Linux
- `hermetic-build-debug`: Debug build for x86_64 Linux
- `hermetic-cross-build`: Cross-compilation for ARM64 Linux
- `hermetic-test`: Run tests in hermetic environment

## Cross-Compilation

### Supported Targets

- `x86_64-unknown-linux-gnu`: Linux x86_64
- `aarch64-unknown-linux-gnu`: Linux ARM64
- `x86_64-pc-windows-gnu`: Windows x86_64 (MinGW)
- `x86_64-pc-windows-msvc`: Windows x86_64 (MSVC)
- `x86_64-apple-darwin`: macOS x86_64
- `aarch64-apple-darwin`: macOS ARM64
- `aarch64-linux-android`: Android ARM64
- `armv7-linux-androideabi`: Android ARMv7
- `i686-linux-android`: Android x86
- `x86_64-linux-android`: Android x86_64

### Cross-Compilation Commands

```bash
# Using the cross-build script
./scripts/cross-build.sh x86_64-pc-windows-gnu release

# Using Cargo directly
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release

# Using hermetic toolchain
./toolchains/hermetic-build.sh release x86_64-pc-windows-gnu
```

## Platform-Specific Builds

### macOS

```bash
# Using the macOS build script
./scripts/build-macos.sh release

# Manual build
export MACOSX_DEPLOYMENT_TARGET="13.0"
cargo build --release
```

### Windows

```bash
# Using the Windows build script
scripts\build-windows.bat release

# Manual build (in Developer Command Prompt)
cargo build --release
```

### Linux

```bash
# Using the Linux build script
./scripts/build-linux.sh release

# Manual build
cargo build --release
```

## Docker Builds

### Development Environment

```bash
# Build development container
docker build -t matte-browser-dev .

# Run development container
docker run -it -v $(pwd):/workspace matte-browser-dev
```

### Production Build

```bash
# Multi-stage production build
docker build -f Dockerfile.prod -t matte-browser:latest .

# Run production container
docker run -p 8080:8080 matte-browser:latest
```

## Build Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `BUILD_TYPE` | Build type (debug/release) | `release` |
| `TARGET` | Target triple | Host triple |
| `RUSTFLAGS` | Rust compiler flags | `-C target-feature=+crt-static` |
| `CARGO_INCREMENTAL` | Enable incremental compilation | `0` (release), `1` (debug) |
| `CC` | C compiler | Platform-specific |
| `CXX` | C++ compiler | Platform-specific |
| `AR` | Archiver | Platform-specific |

### Cargo Configuration

The `.cargo/config.toml` file contains:
- Cross-compilation toolchain configurations
- Platform-specific linker settings
- Build optimization flags

### CMake Configuration

The `CMakeLists.txt` file handles:
- C++ component configuration
- Native dependency management
- Platform-specific build settings

## Troubleshooting

### Common Issues

#### Build Failures

1. **Missing Dependencies**
   ```bash
   # Install system dependencies
   sudo apt-get install build-essential cmake pkg-config
   ```

2. **Rust Toolchain Issues**
   ```bash
   # Update Rust
   rustup update
   
   # Check toolchain
   rustup show
   ```

3. **Cross-Compilation Issues**
   ```bash
   # Install cross-compilation tools
   sudo apt-get install gcc-multilib g++-multilib
   
   # Add target
   rustup target add <target-triple>
   ```

#### Platform-Specific Issues

1. **macOS: Xcode Command Line Tools**
   ```bash
   xcode-select --install
   ```

2. **Windows: MSVC Build Tools**
   ```bash
   # Install Visual Studio Build Tools
   # Or use Developer Command Prompt
   ```

3. **Linux: Missing Libraries**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libx11-dev libxrandr-dev libxfixes-dev
   
   # CentOS/RHEL
   sudo yum install libX11-devel libXrandr-devel libXfixes-devel
   ```

### Debug Builds

```bash
# Enable debug symbols
RUSTFLAGS="-g" cargo build

# Enable debug logging
RUST_LOG=debug cargo run

# Profile build
cargo build --profile=release-with-debug
```

### Performance Optimization

```bash
# Enable link-time optimization
RUSTFLAGS="-C lto=fat" cargo build --release

# Enable CPU-specific optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Parallel compilation
cargo build --release -j$(nproc)
```

### Memory Usage

```bash
# Monitor build memory usage
/usr/bin/time -v cargo build --release

# Limit parallel jobs to reduce memory usage
cargo build --release -j2
```

## Build Artifacts

### Output Locations

- **Rust binaries**: `target/<target>/<profile>/`
- **C++ libraries**: `build/lib/`
- **Documentation**: `target/doc/`
- **Test artifacts**: `target/<target>/<profile>/deps/`

### Artifact Types

- **Executables**: `matte-browser`, `matte-browser-tests`
- **Libraries**: `libmatte_browser.rlib`, `libmatte_browser.so`
- **Documentation**: HTML docs in `target/doc/`
- **Debug symbols**: `.dSYM` (macOS), `.pdb` (Windows)

## Continuous Integration

### GitHub Actions

The project includes GitHub Actions workflows for:
- Automated testing on multiple platforms
- Hermetic builds
- Cross-compilation verification
- Performance regression testing

### Local CI Simulation

```bash
# Run CI locally
./scripts/ci-local.sh

# Run specific CI steps
./scripts/test-all-targets.sh
./scripts/build-all-platforms.sh
```

## Contributing

When contributing to the build system:

1. Test builds on multiple platforms
2. Verify hermetic builds work correctly
3. Update documentation for new build options
4. Ensure cross-compilation still works
5. Run the full test suite

For more information, see the [Contributing Guide](CONTRIBUTING.md).
