# Cross-Compilation Build System Design

**Created**: 2024-11-06  
**Status**: Planning  
**Priority**: HIGH - Production Deployment Support

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Requirements](#requirements)
3. [Target Platforms](#target-platforms)
4. [Prerequisites](#prerequisites)
5. [Solution Architecture](#solution-architecture)
6. [Makefile Design](#makefile-design)
7. [Prerequisite Detection](#prerequisite-detection)
8. [Build Matrix](#build-matrix)
9. [Implementation Phases](#implementation-phases)
10. [Testing Strategy](#testing-strategy)
11. [Deployment Workflow](#deployment-workflow)
12. [Troubleshooting](#troubleshooting)

---

## Overview

### Problem Statement

The Cando-RS project needs to build binaries for multiple platforms:
- **Build Platform**: x86_64 Debian Linux (development workstation)
- **Target Platform 1**: x86_64 Debian Linux (native, production servers)
- **Target Platform 2**: aarch64 Linux (Raspberry Pi 5, edge deployment)
- **Target Platform 3**: x86_64 musl Linux (Alpine Linux, static binaries)

Each platform requires both **debug** and **release** builds for different purposes:
- Debug: Development, testing, verbose logging
- Release: Production, optimized performance

### Goals

1. **Simple Interface**: `make build-all-targets` should build everything
2. **Target-Specific Builds**: `make build-aarch64-release`, `make build-musl-release` for specific needs
3. **Prerequisite Checking**: Automatic detection of missing tools with install instructions
4. **CI/CD Ready**: Easy integration into automated pipelines
5. **Clear Output**: Organized binaries by target/profile in `target/` directory

---

## Requirements

### Functional Requirements

- ✅ Build for x86_64-unknown-linux-gnu (native glibc)
- ✅ Cross-compile for aarch64-unknown-linux-gnu (Raspberry Pi 5)
- ✅ Cross-compile for x86_64-unknown-linux-musl (Alpine Linux, static)
- ✅ Support debug and release profiles
- ✅ Build entire workspace (all binaries, libraries)
- ✅ Detect missing prerequisites before build
- ✅ Provide clear installation instructions
- ✅ Organize output binaries by target

### Non-Functional Requirements

- ✅ Fast iteration (don't rebuild unnecessarily)
- ✅ Clear error messages
- ✅ Compatible with existing Makefile targets
- ✅ No breaking changes to current workflow
- ✅ Documentation for each target

---

## Target Platforms

### Platform 1: x86_64 Linux (Native)

**Target Triple**: `x86_64-unknown-linux-gnu`

**Use Cases**:
- Development workstations
- CI/CD servers (GitHub Actions)
- Production x86_64 servers
- Testing infrastructure

**Build Method**: Native compilation (no cross-compilation needed)

**Prerequisites**:
- Rust toolchain (already installed)
- Standard Debian build tools

### Platform 2: aarch64 Linux (Raspberry Pi 5)

**Target Triple**: `aarch64-unknown-linux-gnu`

**Use Cases**:
- Edge devices (Raspberry Pi 5)
- Embedded industrial controllers
- Field deployment units
- ARM-based test hardware

**Build Method**: Cross-compilation from x86_64

**Prerequisites**:
- Rust target: `rustup target add aarch64-unknown-linux-gnu`
- Cross-compiler: `gcc-aarch64-linux-gnu`
- Cross-linker configuration in `.cargo/config.toml`

### Platform 3: x86_64 musl Linux (Alpine Linux)

**Target Triple**: `x86_64-unknown-linux-musl`

**Use Cases**:
- Alpine Linux containers (Docker)
- Static binaries (no dynamic library dependencies)
- Portable binaries (works on any x86_64 Linux)
- Minimal container images
- Air-gapped systems

**Build Method**: Static compilation using Zig toolchain

**Prerequisites**:
- Rust target: `rustup target add x86_64-unknown-linux-musl`
- Zig toolchain: `zig` package
- cargo-zigbuild: `cargo install cargo-zigbuild`
- No linker configuration needed (Zig handles it)

**Key Benefits**:
- **Static linking**: No runtime library dependencies
- **Portability**: Single binary works on any x86_64 Linux
- **Security**: Smaller attack surface (no dynamic libraries)
- **Container-friendly**: Minimal Alpine images (~5MB base)

---

## Prerequisites

### Native x86_64 Build Prerequisites

**Already Installed** (from existing project):
```bash
# Core Rust toolchain
rustc, cargo, rustup

# Standard development tools
gcc, make, git

# CAN utilities (for testing)
can-utils
```

**No Additional Requirements**: Native builds work out of the box.

### Cross-Compilation Prerequisites

**Rust Targets** (install once):
```bash
# For ARM64 dynamic builds (glibc)
rustup target add aarch64-unknown-linux-gnu

# For musl static builds (Zig-based)
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
```

**Cross-Compilation Toolchains**:
```bash
# ARM64 cross-compiler (for dynamic glibc builds)
sudo apt install gcc-aarch64-linux-gnu

# Zig toolchain (for static musl builds)
# Arch: sudo pacman -S zig
# Debian/Ubuntu: sudo snap install zig --classic --beta
# See doc/BUILD-SYSTEM.md for detailed installation

# cargo-zigbuild (for static musl builds)
cargo install cargo-zigbuild
```

**Cargo Configuration** (`.cargo/config.toml`):
```toml
# ARM64 dynamic builds (glibc)
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

# Note: musl targets use Zig via cargo-zigbuild
# No linker configuration needed
```

---

## Solution Architecture

### Directory Structure

```
cando-rs/
├── .cargo/
│   └── config.toml          # Cross-compilation linker configuration
├── target/
│   ├── x86_64-unknown-linux-gnu/
│   │   ├── debug/           # Native debug builds
│   │   └── release/         # Native release builds
│   ├── aarch64-unknown-linux-gnu/
│   │   ├── debug/           # ARM64 debug builds
│   │   └── release/         # ARM64 release builds
│   └── x86_64-unknown-linux-musl/
│       ├── debug/           # musl debug builds (static)
│       └── release/         # musl release builds (static)
├── Makefile                 # Enhanced with cross-compilation targets
└── doc/build-system/
    ├── CROSS-COMPILATION-DESIGN.md    # This file
    └── CROSS-COMPILATION-GUIDE.md     # User guide (to be created)
```

### Build Matrix

| Target | Profile | Command | Output Path |
|--------|---------|---------|-------------|
| x86_64 (glibc) | debug | `make build-x86_64-debug` | `target/x86_64-unknown-linux-gnu/debug/` |
| x86_64 (glibc) | release | `make build-x86_64-release` | `target/x86_64-unknown-linux-gnu/release/` |
| aarch64 | debug | `make build-aarch64-debug` | `target/aarch64-unknown-linux-gnu/debug/` |
| aarch64 | release | `make build-aarch64-release` | `target/aarch64-unknown-linux-gnu/release/` |
| x86_64 (musl) | debug | `make build-musl-debug` | `target/x86_64-unknown-linux-musl/debug/` |
| x86_64 (musl) | release | `make build-musl-release` | `target/x86_64-unknown-linux-musl/release/` |

### Cargo Commands

**Native x86_64**:
```bash
# Debug (native, no --target needed)
cargo build --workspace

# Release (native)
cargo build --workspace --release
```

**Cross-compile aarch64**:
```bash
# Debug
cargo build --workspace --target aarch64-unknown-linux-gnu

# Release
cargo build --workspace --target aarch64-unknown-linux-gnu --release
```

**Cross-compile musl (static)**:
```bash
# Debug
cargo build --workspace --target x86_64-unknown-linux-musl

# Release
cargo build --workspace --target x86_64-unknown-linux-musl --release
```

---

## Makefile Design

### New Targets Hierarchy

```
Cross-Compilation Targets
├── build-all-targets        # Build all platforms/profiles (x86_64, aarch64, musl)
├── build-all-x86_64         # Build both debug + release for x86_64 (glibc)
├── build-all-aarch64        # Build both debug + release for aarch64
├── build-all-musl           # Build both debug + release for musl
├── build-x86_64-debug       # Native x86_64 debug
├── build-x86_64-release     # Native x86_64 release
├── build-aarch64-debug      # Cross-compile aarch64 debug
├── build-aarch64-release    # Cross-compile aarch64 release
├── build-musl-debug         # Cross-compile musl debug (static)
├── build-musl-release       # Cross-compile musl release (static)
└── check-cross-prereqs      # Verify prerequisites installed
```

### Integration with Existing Targets

**Preserve Existing Behavior**:
- `make build` → Native debug (unchanged)
- `make build-release` → Native release (unchanged)
- `make build-all` → Enhanced to include cross-compilation

**New Default for CI/CD**:
- `make build-all` → Build all targets and profiles

### Prerequisite Checking Target

```makefile
check-cross-prereqs:
	@echo "🔍 Checking cross-compilation prerequisites..."
	@echo "   Note: musl builds use Zig toolchain (see check-packaging-deps)"
	@# Check Rust aarch64 target
	@rustup target list --installed | grep -q aarch64-unknown-linux-gnu || { \
		echo "❌ Missing: Rust aarch64 target"; \
		echo "   Install: rustup target add aarch64-unknown-linux-gnu"; \
		MISSING=1; \
	}
	@# Check GCC cross-compiler for ARM
	@command -v aarch64-linux-gnu-gcc >/dev/null 2>&1 || { \
		echo "❌ Missing: aarch64 cross-compiler"; \
		echo "   Install: sudo apt install gcc-aarch64-linux-gnu"; \
		MISSING=1; \
	}
	@# Check .cargo/config.toml
	@[ -f .cargo/config.toml ] || { \
		echo "⚠️  Missing: .cargo/config.toml"; \
		echo "   Creating with cross-compilation linker configuration..."; \
		mkdir -p .cargo; \
		echo '[target.aarch64-unknown-linux-gnu]' > .cargo/config.toml; \
		echo 'linker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml; \
	}
	@# Exit if any missing
	@[ -z "$$MISSING" ] || exit 1
	@echo "✅ All cross-compilation prerequisites installed"
```

### Example Build Target

```makefile
build-aarch64-release: check-cross-prereqs
	@echo "🚀 Building for aarch64-unknown-linux-gnu (release)"
	cargo build --workspace --target aarch64-unknown-linux-gnu --release
	@echo "✅ Build complete: target/aarch64-unknown-linux-gnu/release/"

build-musl-release: check-packaging-deps
	@echo "🚀 Building for x86_64-unknown-linux-musl (release, static)"
	@echo "   Note: Using Zig toolchain for static linking"
	cargo zigbuild --workspace --target x86_64-unknown-linux-musl --release
	@echo "✅ Build complete: target/x86_64-unknown-linux-musl/release/"
	@echo "📦 Static binaries - no dynamic library dependencies"
```
</text>

<old_text line=288>
**Missing Prerequisites**:
```bash
$ make build-aarch64-release
🔍 Checking cross-compilation prerequisites...
   Note: musl builds use Zig toolchain (see check-packaging-deps)
❌ Missing: Rust aarch64 target
   Install: rustup target add aarch64-unknown-linux-gnu
❌ Missing: aarch64 cross-compiler
   Install: sudo apt install gcc-aarch64-linux-gnu

To install all prerequisites, run:
  rustup target add aarch64-unknown-linux-gnu
  sudo apt install gcc-aarch64-linux-gnu

make: *** [Makefile:XXX: check-cross-prereqs] Error 1
```

---

## Prerequisite Detection

### Detection Strategy

**Three-Level Checking**:

1. **Critical Prerequisites** (block build):
   - Rust toolchain (`rustc`, `cargo`)
   - Target architecture support
   - Cross-compiler (for non-native targets)

2. **Build Prerequisites** (auto-install if possible):
   - `.cargo/config.toml` linker configuration (auto-create)
   - Cargo targets (prompt for `rustup target add`)

3. **Optional Prerequisites** (warn but continue):
   - `strip` for binary size optimization
   - `upx` for compression

### User Experience

**Happy Path** (all prerequisites installed):
```bash
$ make build-aarch64-release
🔍 Checking cross-compilation prerequisites...
   Note: musl builds use Zig toolchain (see check-packaging-deps)
✅ All cross-compilation prerequisites installed
🚀 Building for aarch64-unknown-linux-gnu (release)
[cargo build output...]
✅ Build complete: target/aarch64-unknown-linux-gnu/release/
```

**Missing Prerequisites**:
```bash
$ make build-aarch64-release
🔍 Checking cross-compilation prerequisites...
   Note: musl builds use Zig toolchain (see check-packaging-deps)
❌ Missing: Rust aarch64 target
   Install: rustup target add aarch64-unknown-linux-gnu
❌ Missing: aarch64 cross-compiler
   Install: sudo apt install gcc-aarch64-linux-gnu

To install all prerequisites, run:
  rustup target add aarch64-unknown-linux-gnu
  sudo apt install gcc-aarch64-linux-gnu

make: *** [Makefile:XXX: check-cross-prereqs] Error 1
```

---

## Build Matrix

### Complete Build Matrix

| Target | Profile | Time Est. | Output Size | Use Case |
|--------|---------|-----------|-------------|----------|
| x86_64 (glibc) | debug | ~2 min | ~150 MB | Development, debugging |
| x86_64 (glibc) | release | ~5 min | ~50 MB | Production x86_64 servers |
| aarch64 | debug | ~3 min | ~200 MB | ARM development, debug |
| aarch64 | release | ~6 min | ~60 MB | Raspberry Pi deployment |
| x86_64 (musl) | debug | ~3 min | ~160 MB | Alpine debug, static debug |
| x86_64 (musl) | release | ~6 min | ~55 MB | Alpine prod, portable static |

**Total Time**: ~25 minutes for full matrix build (all 6 configurations)
</text>

<old_text line=323>
**Sequential** (current):
```bash
make build-all-targets  # 16 minutes total
```

**Parallel** (optional optimization):
```bash
make -j4 build-all-targets  # ~8 minutes (if parallelized)
```

### Parallelization

**Sequential** (current):
```bash
make build-all-targets  # 16 minutes total
```

**Parallel** (optional optimization):
```bash
make -j4 build-all-targets  # ~8 minutes (if parallelized)
```

---

## Implementation Phases

### Phase 1: Prerequisite Detection (45 minutes)

**Tasks**:
1. Create `check-cross-prereqs` target
2. Detect installed Rust targets (aarch64, musl)
3. Detect cross-compilers (aarch64, musl)
4. Check/create `.cargo/config.toml` with all linkers
5. Test on clean Debian system

**Deliverables**:
- Working prerequisite checker for all targets
- Clear error messages with installation commands
- Auto-generation of complete `.cargo/config.toml`
- Documentation
</text>

<old_text line=352>
### Phase 2: Cross-Compilation Targets (1 hour)

**Tasks**:
1. Implement `build-aarch64-debug` target
2. Implement `build-aarch64-release` target
3. Implement `build-all-aarch64` target
4. Test actual cross-compilation
5. Verify binary architecture with `file` command

**Deliverables**:
- Working cross-compilation targets
- Verified ARM64 binaries
</text>
<new_text>
### Phase 2: Cross-Compilation Targets (1.5 hours)

**Tasks**:
1. Implement `build-aarch64-debug` target
2. Implement `build-aarch64-release` target
3. Implement `build-all-aarch64` target
4. Implement `build-musl-debug` target
5. Implement `build-musl-release` target
6. Implement `build-all-musl` target
7. Test actual cross-compilation for both targets
8. Verify binary architectures with `file` command
9. Verify musl static linking with `ldd` command

**Deliverables**:
- Working aarch64 cross-compilation targets
- Working musl cross-compilation targets
- Verified ARM64 binaries
- Verified static musl binaries
</text>

<old_text line=378>
### Phase 4: Unified Build Target (30 minutes)

**Tasks**:
1. Implement `build-all-targets` master target
2. Update `build-all` to optionally include cross-compilation
3. Add progress indicators
4. Add summary report at end

**Deliverables**:
- One-command full build
- Build summary with sizes and paths
</text>
<new_text>
### Phase 4: Unified Build Target (45 minutes)

**Tasks**:
1. Implement `build-all-targets` master target (all 3 platforms)
2. Keep `build-all` as native-only (backward compatible)
3. Add progress indicators
4. Add summary report at end with all targets
5. Test full build matrix

**Deliverables**:
- One-command full build (`build-all-targets`)
- Native-only `build-all` (unchanged behavior)
- Build summary with sizes and paths for all 6 configs
</text>

<old_text line=394>
### Phase 5: Documentation & Testing (1 hour)

**Tasks**:
1. Create `CROSS-COMPILATION-GUIDE.md`
2. Update `README.md` with cross-compilation section
3. Update `resume.md` with implementation status
4. Test on clean Debian system
5. Verify all binaries work on target hardware

**Deliverables**:
- Complete user documentation
- Tested on Raspberry Pi 5
- CI/CD integration guide

**Total Estimated Time**: 3.5 hours
</text>
<new_text>
### Phase 5: Documentation & Testing (1.5 hours)

**Tasks**:
1. Create `CROSS-COMPILATION-GUIDE.md`
2. Update `README.md` with cross-compilation section
3. Update `resume.md` with implementation status
4. Test on clean Debian system
5. Verify ARM binaries work on Raspberry Pi 5
6. Verify musl binaries work on Alpine Linux
7. Test static binary portability
8. Create Alpine Docker test

**Deliverables**:
- Complete user documentation
- Tested on Raspberry Pi 5
- Tested on Alpine Linux
- Docker example for musl binaries
- CI/CD integration guide

**Total Estimated Time**: 4.5 hours
</text>

<old_text line=418>
**Test 2: Cross-Compilation**
```bash
# Build ARM64 binary
make build-aarch64-release

# Verify architecture
file target/aarch64-unknown-linux-gnu/release/rust-can-util
# Should output: "ELF 64-bit LSB executable, ARM aarch64, ..."
```
</text>
<new_text>
**Test 2: Cross-Compilation (ARM64)**
```bash
# Build ARM64 binary
make build-aarch64-release

# Verify architecture
file target/aarch64-unknown-linux-gnu/release/rust-can-util
# Should output: "ELF 64-bit LSB executable, ARM aarch64, ..."
```

**Test 2b: Cross-Compilation (musl static)**
```bash
# Build musl binary (uses Zig toolchain)
make build-musl-release

# Verify architecture (should be x86_64)
file target/x86_64-unknown-linux-musl/release/rust-can-util
# Should output: "ELF 64-bit LSB executable, x86-64, ..."

# Verify static linking (no dynamic dependencies)
ldd target/x86_64-unknown-linux-musl/release/rust-can-util
# Should output: "not a dynamic executable" or "statically linked"
```
</text>

<old_text line=436>
**Test 3: Binary Functionality**
```bash
# Copy to Raspberry Pi 5
scp target/aarch64-unknown-linux-gnu/release/rust-can-util pi@rpi5:~/

# SSH to Raspberry Pi and test
ssh pi@rpi5
./rust-can-util --help  # Should work
./rust-can-util --version  # Should show correct version
```
</text>
<new_text>
**Test 3: Binary Functionality (ARM64)**
```bash
# Copy to Raspberry Pi 5
scp target/aarch64-unknown-linux-gnu/release/rust-can-util pi@rpi5:~/

# SSH to Raspberry Pi and test
ssh pi@rpi5
./rust-can-util --help  # Should work
./rust-can-util --version  # Should show correct version
```

**Test 3b: Binary Functionality (musl / Alpine)**
```bash
# Test on Alpine Linux container
docker run --rm -v $(pwd)/target/x86_64-unknown-linux-musl/release:/bin:ro \
  alpine:latest /bin/rust-can-util --version
# Should work with no library errors

# Or copy to Alpine system
scp target/x86_64-unknown-linux-musl/release/rust-can-util alpine@host:~/
ssh alpine@host
./rust-can-util --help  # Should work
```
</text>

<old_text line=452>
**Test 4: Full Build Matrix**
```bash
make build-all-targets  # Should complete successfully
ls -lh target/*/release/rust-can-util  # Verify all binaries exist
```
</text>
<new_text>
**Test 4: Full Build Matrix**
```bash
make build-all-targets  # Should complete successfully (all 6 configs)

# Verify all binaries exist
ls -lh target/x86_64-unknown-linux-gnu/release/rust-can-util
ls -lh target/aarch64-unknown-linux-gnu/release/rust-can-util  
ls -lh target/x86_64-unknown-linux-musl/release/rust-can-util

# Compare sizes (musl might be slightly larger due to static linking)
du -h target/*/release/rust-can-util
```
</text>

<old_text line=478>
### Development Workflow

```bash
# 1. Develop on x86_64 workstation
make build-x86_64-debug
make tier1  # Test on native

# 2. Prepare for ARM deployment
make build-aarch64-release

# 3. Deploy to Raspberry Pi
./scripts/deploy-to-rpi.sh target/aarch64-unknown-linux-gnu/release/
```

### Phase 2: Cross-Compilation Targets (1 hour)

**Tasks**:
1. Implement `build-aarch64-debug` target
2. Implement `build-aarch64-release` target
3. Implement `build-all-aarch64` target
4. Test actual cross-compilation
5. Verify binary architecture with `file` command

**Deliverables**:
- Working cross-compilation targets
- Verified ARM64 binaries

### Phase 3: Native Build Targets (30 minutes)

**Tasks**:
1. Implement `build-x86_64-debug` target (wrapper for existing)
2. Implement `build-x86_64-release` target (wrapper for existing)
3. Implement `build-all-x86_64` target
4. Ensure backward compatibility

**Deliverables**:
- Consistent naming across targets
- No breaking changes to existing workflow

### Phase 4: Unified Build Target (30 minutes)

**Tasks**:
1. Implement `build-all-targets` master target
2. Update `build-all` to optionally include cross-compilation
3. Add progress indicators
4. Add summary report at end

**Deliverables**:
- One-command full build
- Build summary with sizes and paths

### Phase 5: Documentation & Testing (1 hour)

**Tasks**:
1. Create `CROSS-COMPILATION-GUIDE.md`
2. Update `README.md` with cross-compilation section
3. Update `resume.md` with implementation status
4. Test on clean Debian system
5. Verify all binaries work on target hardware

**Deliverables**:
- Complete user documentation
- Tested on Raspberry Pi 5
- CI/CD integration guide

**Total Estimated Time**: 3.5 hours

---

## Testing Strategy

### Build Testing

**Test 1: Prerequisite Detection**
```bash
# Remove prerequisites
rustup target remove aarch64-unknown-linux-gnu
sudo apt remove gcc-aarch64-linux-gnu

# Test detection
make check-cross-prereqs  # Should fail with clear instructions

# Reinstall
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu

# Test again
make check-cross-prereqs  # Should succeed

# For musl builds, check packaging dependencies
make check-packaging-deps  # Verifies Zig and cargo-zigbuild
```

**Test 2: Cross-Compilation**
```bash
# Build ARM64 binary
make build-aarch64-release

# Verify architecture
file target/aarch64-unknown-linux-gnu/release/rust-can-util
# Should output: "ELF 64-bit LSB executable, ARM aarch64, ..."
```

**Test 3: Binary Functionality**
```bash
# Copy to Raspberry Pi 5
scp target/aarch64-unknown-linux-gnu/release/rust-can-util pi@rpi5:~/

# SSH to Raspberry Pi and test
ssh pi@rpi5
./rust-can-util --help  # Should work
./rust-can-util --version  # Should show correct version
```

### Integration Testing

**Test 4: Full Build Matrix**
```bash
make build-all-targets  # Should complete successfully
ls -lh target/*/release/rust-can-util  # Verify all binaries exist
```

**Test 5: CI/CD Simulation**
```bash
# Clean environment
make clean-all
rm -rf .cargo/config.toml

# Simulate CI/CD build
make check-cross-prereqs  # Should fail or auto-fix
make build-all-targets    # Should complete end-to-end
```

---

## Deployment Workflow

### Development Workflow

```bash
# 1. Develop on x86_64 workstation
make build-x86_64-debug
make tier1  # Test on native

# 2. Prepare for ARM deployment
make build-aarch64-release

# 3. Deploy to Raspberry Pi
./scripts/deploy-to-rpi.sh target/aarch64-unknown-linux-gnu/release/
```

### CI/CD Workflow

```yaml
# GitHub Actions example
- name: Check Cross-Compilation Prerequisites
  run: make check-cross-prereqs

- name: Build All Targets
  run: make build-all-targets

- name: Archive Artifacts
  uses: actions/upload-artifact@v3
  with:
    name: cando-binaries
    path: |
      target/x86_64-unknown-linux-gnu/release/
      target/aarch64-unknown-linux-gnu/release/
```

### Release Workflow

```bash
# 1. Build release binaries for all targets
make build-all-targets

# 2. Create tarball for each target
./scripts/package-release.sh x86_64
./scripts/package-release.sh aarch64

# 3. Upload to releases
gh release create v1.0.0 \
  cando-x86_64-v1.0.0.tar.gz \
  cando-aarch64-v1.0.0.tar.gz
```

---

## Troubleshooting

### Common Issues

**Issue 1: "linker `aarch64-linux-gnu-gcc` not found"**
```bash
# Solution:
sudo apt install gcc-aarch64-linux-gnu

# Verify:
which aarch64-linux-gnu-gcc
```

**Issue 2: "can't find crate for `std`"**
```bash
# Solution: Install Rust target
rustup target add aarch64-unknown-linux-gnu

# Verify:
rustup target list --installed | grep aarch64
```

**Issue 3: ".cargo/config.toml not found"**
```bash
# Auto-created by check-cross-prereqs
# Or manually create:
mkdir -p .cargo
cat > .cargo/config.toml << 'EOF'
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF
```

**Issue 4: Binary doesn't run on Raspberry Pi**
```bash
# Check binary architecture:
file target/aarch64-unknown-linux-gnu/release/rust-can-util

# Should show: ARM aarch64
# If shows x86_64, cross-compilation failed

# Verify on Raspberry Pi:
ssh pi@rpi5 'uname -m'  # Should show: aarch64
```

---

## Best Practices

### 1. Always Check Prerequisites First
```bash
make check-cross-prereqs && make build-aarch64-release
```

### 2. Use Explicit Targets for Production
```bash
# Clear and explicit
make build-aarch64-release

# Avoid generic
cargo build --release  # Builds for native only
```

### 3. Verify Binary Architecture
```bash
file target/aarch64-unknown-linux-gnu/release/rust-can-util
```

### 4. Test on Actual Hardware
Don't assume cross-compiled binary works—always test on target device.

### 5. Keep .cargo/config.toml in Git
```bash
# Commit the linker configuration
git add .cargo/config.toml
git commit -m "Add cross-compilation linker configuration"
```

---

## Future Enhancements

### Potential Additions

1. **Additional Targets**:
   - `armv7-unknown-linux-gnueabihf` (Raspberry Pi 3/4)
   - `x86_64-unknown-linux-musl` (Alpine Linux, static binaries)

2. **Build Optimization**:
   - Binary stripping (`strip` command)
   - Compression with `upx`
   - Link-time optimization (LTO)

3. **Deployment Automation**:
   - SCP deployment scripts
   - Ansible playbooks
   - Docker images for each target

4. **Testing Automation**:
   - QEMU user-mode emulation for testing ARM binaries on x86_64
   - Automated testing on Raspberry Pi test rack
   - Performance benchmarking per target

---

## Success Criteria

### Phase 1-4 Complete When:

- ✅ `make build-all-targets` builds for x86_64 and aarch64
- ✅ `make check-cross-prereqs` detects missing tools
- ✅ Clear installation instructions provided
- ✅ ARM64 binaries verified with `file` command
- ✅ Existing Makefile targets still work
- ✅ No breaking changes to current workflow

### Phase 5 Complete When:

- ✅ Binaries tested on Raspberry Pi 5
- ✅ Documentation complete (user guide)
- ✅ `README.md` updated with cross-compilation section
- ✅ CI/CD integration tested

---

## References

### External Documentation

- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Cargo Configuration](https://doc.rust-lang.org/cargo/reference/config.html)
- [Cross-Compilation Toolchains](https://wiki.debian.org/CrossCompiling)

### Internal Documentation

- `Makefile` - Current build system
- `doc/AI-WORKFLOW-GUIDE.md` - File organization
- `.cargo/config.toml` - Linker configuration (to be created)

---

**Status**: Design Complete, Ready for Implementation  
**Estimated Implementation Time**: 3.5 hours  
**Next Step**: Implement Phase 1 (Prerequisite Detection)