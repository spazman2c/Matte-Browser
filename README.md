# Matte Browser

A fast, private, and developer-friendly web browser built from scratch with original engines.

## Overview

Matte is a custom web browser that prioritizes:
- **Safety first**: Memory-safe engines with strong sandboxing
- **Performance & power**: Predictable frame pacing, low power consumption
- **Privacy by default**: Strict tracking protection, storage partitioning
- **Developer joy**: Modern DevTools, clean debugging protocol
- **Clarity over scope**: Ship a subset well, expand based on telemetry

## Architecture

Matte uses a multi-process architecture with original engines:

```
+-----------------+     +----------------------+     +------------------+
|   Browser UI    |<--->|  Browser Process     |<--->|  GPU/Compositor  |
+-----------------+     |  (privileged)        |     +------------------+
        ^               |  - Tab mgr            |              ^
        |               |  - Profile/Settings   |              |
        v               |  - Extension host     |              v
+-----------------+     +----------------------+     +------------------+
|  Renderer Proc  |<--->|  Network Process     |<--->| OS / Drivers     |
|  (per-site)     |     |  (TLS, cache, HTTP)  |     +------------------+
|  - Parser/DOM   |     +----------------------+
|  - Style/Layout |
|  - JS VM        |
|  - Scheduler    |
+-----------------+
```

## Prerequisites

### System Requirements
- **Windows**: Windows 11+ with Visual Studio 2022 or later
- **macOS**: macOS 13+ with Xcode 15+ 
- **Linux**: Ubuntu 22.04+ or equivalent with GCC 11+

### Required Tools
- **Rust**: 1.75+ (install via [rustup](https://rustup.rs/))
- **CMake**: 3.20+
- **Git**: 2.30+
- **Python**: 3.8+ (for build scripts)

### Platform-Specific Dependencies

#### Windows
```powershell
# Install Visual Studio 2022 with C++ workload
# Install Windows 11 SDK
# Install Rust via rustup
rustup default stable
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install additional tools
brew install cmake pkg-config
```

#### Linux (Ubuntu/Debian)
```bash
# Install system dependencies
sudo apt update
sudo apt install build-essential cmake pkg-config \
    libssl-dev libx11-dev libxrandr-dev libxfixes-dev \
    libxcursor-dev libxcomposite-dev libxdamage-dev \
    libxss-dev libxtst-dev libxrender-dev libxext-dev \
    libxi-dev libgl1-mesa-dev libglu1-mesa-dev \
    libasound2-dev libpulse-dev libdbus-1-dev \
    libudev-dev libevdev-dev libinput-dev libwayland-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Building

### Quick Start
```bash
# Clone the repository
git clone https://github.com/matte-browser/matte.git
cd matte

# Build the project
cargo build --release

# Run the browser
cargo run --release --bin matte-browser
```

### Development Build
```bash
# Debug build with full logging
RUST_LOG=debug cargo build

# Run with debug logging
RUST_LOG=debug cargo run --bin matte-browser
```

### Platform-Specific Builds

#### Windows
```powershell
# Using Visual Studio Developer Command Prompt
cargo build --release

# Or using PowerShell
cargo build --release --target x86_64-pc-windows-msvc
```

#### macOS
```bash
# Universal build (Intel + Apple Silicon)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Or let Rust auto-detect
cargo build --release
```

#### Linux
```bash
# Standard build
cargo build --release

# With specific target
cargo build --release --target x86_64-unknown-linux-gnu
```

## Project Structure

```
matte/
├── browser/          # Main browser process
├── renderer/         # Renderer process (per-site)
├── network/          # Network process
├── gpu/              # GPU/compositor process
├── dom/              # DOM implementation
├── css/              # CSS parser and engine
├── js/               # JavaScript engine (MatteJS)
├── net/              # Networking stack (MatteNet)
├── graphics/         # Graphics and compositing
├── platform/         # Platform-specific code
├── common/           # Shared utilities
├── tools/            # Development tools
└── docs/             # Documentation
```

## Development

### Code Style
- **Rust**: Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- **C++**: Follow [Google C++ Style Guide](https://google.github.io/styleguide/cppguide.html)
- Use `cargo fmt` and `cargo clippy` for Rust code
- Use `clang-format` for C++ code

### Testing
```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --package dom
cargo test --package css

# Run integration tests
cargo test --test integration

# Run performance benchmarks
cargo bench
```

### Debugging
```bash
# Run with debug symbols
cargo build --debug

# Run with specific log level
RUST_LOG=matte::dom=debug cargo run

# Run with crash reporting enabled
RUST_BACKTRACE=1 cargo run
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following the coding standards
4. Add tests for new functionality
5. Run the test suite (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Development Workflow
1. **Create an issue** for bugs or feature requests
2. **Assign yourself** to the issue
3. **Create a branch** from `main`
4. **Implement** the feature or fix
5. **Add tests** and ensure they pass
6. **Update documentation** as needed
7. **Submit a PR** with a clear description

## Architecture Decision Records (ADRs)

We use ADRs to document significant architectural decisions. See the [docs/adr/](docs/adr/) directory for existing ADRs.

## Roadmap

See [PLAN.md](PLAN.md) for the detailed project roadmap and milestones.

### Current Phase: Foundation (Phase 0)
- [x] Project infrastructure setup
- [ ] Core process architecture
- [ ] Platform integration
- [ ] Basic infrastructure

### Next Phase: First Pixels (Phase 1)
- [ ] HTML parser & DOM
- [ ] CSS & style system
- [ ] JavaScript engine
- [ ] Networking stack
- [ ] Graphics & compositor

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Security

Please report security vulnerabilities to security@matte-browser.com. Do not disclose security issues publicly until they have been addressed.

## Support

- **Documentation**: [docs.matte-browser.com](https://docs.matte-browser.com)
- **Issues**: [GitHub Issues](https://github.com/matte-browser/matte/issues)
- **Discussions**: [GitHub Discussions](https://github.com/matte-browser/matte/discussions)
- **Discord**: [Matte Browser Community](https://discord.gg/matte-browser)

## Acknowledgments

- Inspired by the need for a truly privacy-focused browser
- Built with modern Rust and C++ for safety and performance
- Thanks to the open source community for foundational libraries
