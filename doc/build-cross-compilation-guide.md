# Cross-Compilation User Guide

**Last Updated**: 2024-11-06  
**Status**: Production Ready  
**Applies To**: Cando-RS v0.1.0+

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Prerequisites](#prerequisites)
4. [Basic Usage](#basic-usage)
5. [Platform Details](#platform-details)
6. [Verification](#verification)
7. [Common Workflows](#common-workflows)
8. [Troubleshooting](#troubleshooting)
9. [CI/CD Integration](#cicd-integration)
10. [FAQ](#faq)

---

## Overview

### What is Cross-Compilation?

Cross-compilation allows you to build binaries for different platforms from your development machine. For example, building ARM64 binaries for Raspberry Pi 5 on your x86_64 workstation.

### Supported Platforms

Cando-RS supports building for three target platforms:

| Platform | Target Triple | Use Case | Binary Type |
|----------|--------------|----------|-------------|
| **x86_64 (glibc)** | `x86_64-unknown-linux-gnu` | Development, production servers | Dynamic |
| **ARM64** | `aarch64-unknown-linux-gnu` | Raspberry Pi 5, ARM servers | Dynamic |
| **Alpine (musl)** | `x86_64-unknown-linux-musl` | Alpine Linux, containers | Static |

### Why Use This?

- ✅ **Raspberry Pi Deployment**: Build ARM binaries without needing a Pi
- ✅ **Alpine Containers**: Create minimal Docker images with static binaries
- ✅ **Portable Binaries**: musl binaries work on any x86_64 Linux
- ✅ **CI/CD**: Build all platforms in one pipeline
- ✅ **Faster**: Cross-compile on fast x86_64 hardware instead of slow ARM boards

---

## Quick Start

### 5-Minute Setup

```bash
# 1. Install prerequisites
rustup target add aarch64-unknown-linux-gnu x86_64-unknown-linux-musl
sudo apt install gcc-aarch64-linux-gnu musl-tools

# 2. Verify setup
make check-cross-prereqs

# 3. Build for all platforms
make build-all-targets
```

That's it! Binaries will be in:
- `target/release/` (native x86_64)
- `target/aarch64-unknown-linux-gnu/release/` (Raspberry Pi)
- `target/x86_64-unknown-linux-musl/release/` (Alpine)

---

## Prerequisites

### Step 1: Install Rust Targets

```bash
# Add ARM64 support
rustup target add aarch64-unknown-linux-gnu

# Add musl support (static binaries)
rustup target add x86_64-unknown-linux-musl
```

**Verify**:
```bash
rustup target list --installed | grep -E "(aarch64|musl)"
```

Expected output:
```
aarch64-unknown-linux-gnu
x86_64-unknown-linux-musl
```

### Step 2: Install Cross-Compilers

**On Debian/Ubuntu**:
```bash
sudo apt install gcc-aarch64-linux-gnu musl-tools
```

**On Arch Linux**:
```bash
sudo pacman -S aarch64-linux-gnu-gcc musl
```

**On Fedora**:
```bash
sudo dnf install gcc-aarch64-linux-gnu musl-gcc
```

**Verify**:
```bash
aarch64-linux-gnu-gcc --version
musl-gcc --version
```

### Step 3: Verify Configuration

The `.cargo/config.toml` file should already exist in the repository. If not, it will be auto-created.

**Check it exists**:
```bash
cat .cargo/config.toml
```

**Expected content**:
```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[target.x86_64-unknown-linux-musl]
linker = "musl-gcc"
```

### Step 4: Automated Verification

Run the automated checker:
```bash
make check-cross-prereqs
```

**Success looks like**:
```
🔍 Checking cross-compilation prerequisites...
✅ All cross-compilation prerequisites installed
```

**If something is missing**:
```
❌ Missing: Rust aarch64 target
   Install: rustup target add aarch64-unknown-linux-gnu
❌ Missing: musl toolchain
   Install: sudo apt install musl-tools

To install all prerequisites, run:
  rustup target add aarch64-unknown-linux-gnu x86_64-unknown-linux-musl
  sudo apt install gcc-aarch64-linux-gnu musl-tools
```

---

## Basic Usage

### Build Single Target/Profile

```bash
# Native x86_64 builds
make build-x86_64-debug      # Fast, for development
make build-x86_64-release    # Optimized, for production

# Raspberry Pi 5 (ARM64)
make build-aarch64-debug
make build-aarch64-release

# Alpine Linux (static binaries)
make build-musl-debug
make build-musl-release
```

### Build All Configurations for One Platform

```bash
make build-all-x86_64    # Both debug + release
make build-all-aarch64   # Both debug + release
make build-all-musl      # Both debug + release
```

### Build Everything (Complete Matrix)

```bash
make build-all-targets
```

This builds **6 configurations**:
- x86_64 debug + release
- aarch64 debug + release  
- musl debug + release

**Expected time**: ~25 minutes for full matrix on typical hardware

### Existing Commands Still Work

The new targets don't break existing workflows:

```bash
make build          # Same as before (native debug)
make build-release  # Same as before (native release)
make build-all      # Native builds only (unchanged)
make tier1          # Still uses native binaries
```

---

## Platform Details

### x86_64 (Native - glibc)

**Target**: `x86_64-unknown-linux-gnu`

**Best For**:
- Development and testing
- Production servers running Debian/Ubuntu/RHEL
- Fastest build times (native compilation)

**Commands**:
```bash
make build-x86_64-release
```

**Output**:
```
target/release/rust-can-util
target/release/dump-messages
target/release/cando-webui
# ... all binaries
```

**Verify**:
```bash
file target/release/rust-can-util
# Output: ELF 64-bit LSB pie executable, x86-64, ...

./target/release/rust-can-util --version
# Should run immediately
```

---

### ARM64 (Raspberry Pi 5)

**Target**: `aarch64-unknown-linux-gnu`

**Best For**:
- Raspberry Pi 5 (and Pi 4 with 64-bit OS)
- ARM-based servers
- Edge computing devices
- Industrial ARM controllers

**Commands**:
```bash
make build-aarch64-release
```

**Output**:
```
target/aarch64-unknown-linux-gnu/release/rust-can-util
target/aarch64-unknown-linux-gnu/release/dump-messages
target/aarch64-unknown-linux-gnu/release/cando-webui
# ... all binaries
```

**Verify**:
```bash
file target/aarch64-unknown-linux-gnu/release/rust-can-util
# Output: ELF 64-bit LSB pie executable, ARM aarch64, ...
```

**Deploy to Raspberry Pi**:
```bash
# Copy to Pi
scp target/aarch64-unknown-linux-gnu/release/rust-can-util pi@raspberrypi:~/

# SSH and test
ssh pi@raspberrypi
./rust-can-util --version
```

**Size**: ~9.2 MB (release, not stripped)

---

### musl (Alpine Linux - Static)

**Target**: `x86_64-unknown-linux-musl`

**Best For**:
- Alpine Linux systems
- Docker containers (minimal images)
- Portable binaries (works on any x86_64 Linux)
- Air-gapped systems
- Static deployment

**Commands**:
```bash
make build-musl-release
```

**Output**:
```
target/x86_64-unknown-linux-musl/release/rust-can-util
target/x86_64-unknown-linux-musl/release/dump-messages
target/x86_64-unknown-linux-musl/release/cando-webui
# ... all binaries
```

**Verify Static Linking**:
```bash
ldd target/x86_64-unknown-linux-musl/release/rust-can-util
# Output: statically linked
```

**Test in Alpine Container**:
```bash
docker run --rm -v $(pwd)/target/x86_64-unknown-linux-musl/release:/bin:ro \
  alpine:latest /bin/rust-can-util --version
```

**Size**: ~8.4 MB (release, static)

**Key Benefits**:
- ✅ No library dependencies
- ✅ Works on any Linux distribution
- ✅ Perfect for containers (FROM scratch possible)
- ✅ Simpler deployment (single file)

---

## Verification

### Verify Architecture

**x86_64 (native)**:
```bash
file target/release/rust-can-util
# Expected: x86-64, version 1 (SYSV), dynamically linked
```

**ARM64**:
```bash
file target/aarch64-unknown-linux-gnu/release/rust-can-util
# Expected: ARM aarch64, version 1 (SYSV), dynamically linked
```

**musl**:
```bash
file target/x86_64-unknown-linux-musl/release/rust-can-util
# Expected: x86-64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl
```

### Verify Static Linking (musl)

```bash
ldd target/x86_64-unknown-linux-musl/release/rust-can-util
# Expected: "statically linked" or "not a dynamic executable"
```

### Verify Functionality

**Test Binary Runs**:
```bash
# Native (should work immediately)
./target/release/rust-can-util --version

# musl (should work on any Linux)
./target/x86_64-unknown-linux-musl/release/rust-can-util --version

# ARM64 (requires ARM hardware or QEMU)
# Copy to Raspberry Pi first, then:
ssh pi@raspberrypi './rust-can-util --version'
```

### Compare Binary Sizes

```bash
ls -lh target/*/release/rust-can-util target/release/rust-can-util
```

Expected sizes:
- Native x86_64: ~8.3 MB
- ARM64: ~9.2 MB
- musl: ~8.4 MB

---

## Common Workflows

### Development Workflow

```bash
# 1. Develop on native platform (fast iteration)
make build-x86_64-debug
make tier1

# 2. Before committing: verify all platforms
make check-cross-prereqs
make build-all-targets
```

### Release Workflow

```bash
# 1. Build release binaries for all platforms
make build-all-targets

# 2. Test native
./target/release/rust-can-util --version
make tier1

# 3. Package for distribution
tar -czf cando-x86_64-v1.0.0.tar.gz -C target/release .
tar -czf cando-aarch64-v1.0.0.tar.gz -C target/aarch64-unknown-linux-gnu/release .
tar -czf cando-musl-v1.0.0.tar.gz -C target/x86_64-unknown-linux-musl/release .
```

### Raspberry Pi Deployment

```bash
# 1. Build ARM64 release
make build-aarch64-release

# 2. Create deployment directory
ssh pi@raspberrypi 'mkdir -p ~/cando/bin'

# 3. Copy binaries
scp target/aarch64-unknown-linux-gnu/release/rust-can-util pi@raspberrypi:~/cando/bin/
scp target/aarch64-unknown-linux-gnu/release/cando-webui pi@raspberrypi:~/cando/bin/
scp target/aarch64-unknown-linux-gnu/release/*-simulator pi@raspberrypi:~/cando/bin/

# 4. Test on Pi
ssh pi@raspberrypi 'cd ~/cando/bin && ./rust-can-util --version'
```

### Docker/Alpine Workflow

```bash
# 1. Build static musl binary
make build-musl-release

# 2. Create minimal Dockerfile
cat > Dockerfile << 'EOF'
FROM scratch
COPY target/x86_64-unknown-linux-musl/release/rust-can-util /bin/
ENTRYPOINT ["/bin/rust-can-util"]
EOF

# 3. Build container (~8MB total!)
docker build -t cando-rust-can-util .

# 4. Run
docker run --rm cando-rust-can-util --version
```

### CI/CD Workflow

```bash
# In your CI pipeline:
# 1. Check prerequisites
make check-cross-prereqs || exit 1

# 2. Build all targets
make build-all-targets

# 3. Archive artifacts
tar -czf artifacts.tar.gz target/*/release/

# 4. Upload for release
gh release create v1.0.0 artifacts.tar.gz
```

---

## Troubleshooting

### Problem: "linker `aarch64-linux-gnu-gcc` not found"

**Solution**:
```bash
sudo apt install gcc-aarch64-linux-gnu
```

**Verify**:
```bash
which aarch64-linux-gnu-gcc
aarch64-linux-gnu-gcc --version
```

---

### Problem: "can't find crate for `std`"

**Cause**: Rust target not installed

**Solution**:
```bash
rustup target add aarch64-unknown-linux-gnu
# or
rustup target add x86_64-unknown-linux-musl
```

**Verify**:
```bash
rustup target list --installed
```

---

### Problem: Binary doesn't run on Raspberry Pi

**Check 1: Verify architecture**:
```bash
file target/aarch64-unknown-linux-gnu/release/rust-can-util
# Should show: ARM aarch64
```

**Check 2: Verify Pi is 64-bit**:
```bash
ssh pi@raspberrypi 'uname -m'
# Should show: aarch64 (not armv7l)
```

If it shows `armv7l`, your Pi is running 32-bit OS. Either:
- Install 64-bit Raspberry Pi OS, or
- Build for `armv7-unknown-linux-gnueabihf` target

---

### Problem: musl binary fails with "No such file"

**Cause**: Missing musl dynamic linker (rare)

**Check**:
```bash
ls -l /lib/ld-musl-x86_64.so.1
```

**Solution** (Alpine):
```bash
apk add musl
```

**Solution** (Debian/Ubuntu):
```bash
sudo apt install musl
```

---

### Problem: "make check-cross-prereqs" passes but build fails

**Debug**:
```bash
# Test manual cargo build
cargo build --target aarch64-unknown-linux-gnu --release

# Check .cargo/config.toml exists
cat .cargo/config.toml

# Check linker configuration
grep -A 1 "aarch64" .cargo/config.toml
```

**Recreate config**:
```bash
rm .cargo/config.toml
make check-cross-prereqs  # Will recreate it
```

---

### Problem: Build is very slow

**Optimization 1: Use release build** (slower to build, but you only build once):
```bash
make build-aarch64-release  # Use release, not debug
```

**Optimization 2: Build specific binaries**:
```bash
cargo build --bin rust-can-util --target aarch64-unknown-linux-gnu --release
```

**Optimization 3: Enable LTO** (link-time optimization):
```toml
# In Cargo.toml [profile.release]:
lto = true
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Cross-Platform Build

on: [push, pull_request]

jobs:
  build-matrix:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
      
      - name: Install Cross-Compilation Prerequisites
        run: |
          rustup target add aarch64-unknown-linux-gnu x86_64-unknown-linux-musl
          sudo apt-get update
          sudo apt-get install -y gcc-aarch64-linux-gnu musl-tools
      
      - name: Verify Prerequisites
        run: make check-cross-prereqs
      
      - name: Build All Targets
        run: make build-all-targets
      
      - name: Run Tests (native)
        run: make tier1
      
      - name: Archive Binaries
        uses: actions/upload-artifact@v3
        with:
          name: cando-binaries
          path: |
            target/release/rust-can-util
            target/aarch64-unknown-linux-gnu/release/rust-can-util
            target/x86_64-unknown-linux-musl/release/rust-can-util
```

### GitLab CI Example

```yaml
build:cross-compile:
  image: rust:latest
  before_script:
    - rustup target add aarch64-unknown-linux-gnu x86_64-unknown-linux-musl
    - apt-get update && apt-get install -y gcc-aarch64-linux-gnu musl-tools
  script:
    - make check-cross-prereqs
    - make build-all-targets
  artifacts:
    paths:
      - target/*/release/
    expire_in: 1 week
```

---

## FAQ

### Q: Do I need to cross-compile for development?

**A**: No! For development, use native builds:
```bash
make build          # Fast, native debug
make build-release  # Native release
```

Only use cross-compilation when deploying to other platforms.

---

### Q: Can I cross-compile on macOS or Windows?

**A**: This guide is for Linux→Linux cross-compilation. For other host platforms:
- **macOS**: Use Docker with Linux container
- **Windows**: Use WSL2 and follow this guide

---

### Q: Which musl binary works on Alpine Linux?

**A**: The `x86_64-unknown-linux-musl` target:
```bash
make build-musl-release
```

This creates a static binary that works on Alpine and any x86_64 Linux.

---

### Q: Can I strip binaries to reduce size?

**A**: Yes!
```bash
strip target/release/rust-can-util                    # ~8.3 MB → ~2.5 MB
strip target/aarch64-unknown-linux-gnu/release/rust-can-util  # ~9.2 MB → ~3.1 MB
```

Add to Makefile:
```makefile
build-aarch64-release-stripped: build-aarch64-release
	aarch64-linux-gnu-strip target/aarch64-unknown-linux-gnu/release/rust-can-util
```

---

### Q: How do I test ARM binaries without ARM hardware?

**A**: Use QEMU user-mode emulation:
```bash
# Install QEMU
sudo apt install qemu-user-static

# Run ARM binary on x86_64
qemu-aarch64-static target/aarch64-unknown-linux-gnu/release/rust-can-util --version
```

Note: Some features (like CAN sockets) may not work in emulation.

---

### Q: What about 32-bit ARM (Raspberry Pi 3)?

**A**: Add support for `armv7-unknown-linux-gnueabihf`:
```bash
rustup target add armv7-unknown-linux-gnueabihf
sudo apt install gcc-arm-linux-gnueabihf
```

Then build:
```bash
cargo build --target armv7-unknown-linux-gnueabihf --release
```

---

### Q: Can I build all targets in parallel?

**A**: Yes, with GNU make:
```bash
make -j4 build-all-targets  # Parallel builds
```

Note: This uses more memory and CPU. Adjust `-j` value for your system.

---

## Additional Resources

### Documentation
- [build-cross-compilation-design.md](build-cross-compilation-design.md) - Complete design and architecture
- [Rust Cross-Compilation](https://rust-lang.github.io/rustup/cross-compilation.html) - Official Rust docs
- [Cargo Configuration](https://doc.rust-lang.org/cargo/reference/config.html) - Cargo reference

### Tools
- [cross](https://github.com/cross-rs/cross) - Docker-based cross-compilation (alternative approach)
- [cargo-strip](https://crates.io/crates/cargo-strip) - Automatic binary stripping

---

## Summary Commands

```bash
# One-time setup
rustup target add aarch64-unknown-linux-gnu x86_64-unknown-linux-musl
sudo apt install gcc-aarch64-linux-gnu musl-tools
make check-cross-prereqs

# Development (fast)
make build

# Release for Raspberry Pi
make build-aarch64-release

# Release for Alpine
make build-musl-release

# Everything
make build-all-targets
```

---

**Questions or issues?** Check the troubleshooting section or refer to the design document.

**Happy cross-compiling!** 🚀