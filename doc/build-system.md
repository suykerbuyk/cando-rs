# Cando-RS Build System Documentation

**Version**: 2.0 (Zig-Exclusive Architecture)  
**Last Updated**: November 8, 2024  
**Status**: Production Ready

---

## 🤖 AI Context Restoration

**Purpose**: This document serves as a complete, standalone AI context restoration guide for the Cando-RS build system.

**When to Read This**:
- Adding new build architectures (e.g., RISC-V, PowerPC)
- Troubleshooting build system issues
- Understanding toolchain decisions
- Onboarding new maintainers
- Restoring AI context for build system work

**What You'll Learn**:
- Complete build system architecture and philosophy
- Why we chose Zig exclusively for static builds
- How to add new architectures step-by-step
- Platform-specific installation guidance
- Historical context and lessons learned
- Troubleshooting common build issues

**Quick Navigation**:
- 🎯 [Architecture Philosophy](#architecture-philosophy) - Why the system is designed this way
- 🔧 [Dependencies](#dependencies) - Platform-specific installation with native packages
- ➕ [Adding New Architectures](#adding-new-architectures) - Step-by-step guide
- 🔍 [Troubleshooting](#troubleshooting) - Common issues and solutions
- 📚 [Historical Context](#historical-context) - Evolution and lessons learned

---
</text>

<old_text line=56>
## Architecture Philosophy

### Core Design Principles

#### 1. Separation of Concerns

## Table of Contents

1. [Overview](#overview)
2. [Architecture Philosophy](#architecture-philosophy)
3. [Build System Types](#build-system-types)
4. [Toolchain Architecture](#toolchain-architecture)
5. [Build Targets Reference](#build-targets-reference)
6. [Dependencies](#dependencies)
7. [Adding New Architectures](#adding-new-architectures)
8. [Troubleshooting](#troubleshooting)
9. [Historical Context](#historical-context)
10. [Technical Details](#technical-details)

---

## Overview

Cando-RS uses a dual-purpose build system optimized for two distinct workflows:

- **Development Builds**: Fast, dynamic linking for rapid iteration and testing
- **Distribution Builds**: Static, zero-dependency packages for deployment

**Key Principle**: Use the right tool for each job.

### Quick Reference

```bash
# Development (fast iteration)
cargo build              # Debug with system libraries
cargo build --release    # Release with system libraries
cargo test               # Run tests

# Distribution (static packages)
make deb-all            # Build AMD64 + ARM64 Debian packages
make build-deb-amd64    # Build AMD64 only
make build-deb-arm64    # Build ARM64 only
```

### The Golden Rule

```
Development builds  →  Native toolchain (GCC/Clang)  →  Fast iteration
Distribution builds →  Zig toolchain              →  Static, portable packages
```

---

## Architecture Philosophy

### Core Design Principles

#### 1. Separation of Concerns

**Development** and **Distribution** are fundamentally different use cases requiring different toolchains:

| Aspect | Development | Distribution |
|--------|------------|--------------|
| **Goal** | Fast iteration | Maximum portability |
| **Linking** | Dynamic | Static |
| **Dependencies** | System libraries | Zero dependencies |
| **Toolchain** | Native (GCC/Clang) | Zig (LLVM) |
| **Compilation** | Fast (incremental) | Slower (full static) |
| **Testing** | Quick feedback | Cross-platform validation |
| **Debugging** | Easy (symbols) | Limited (stripped) |

#### 2. Single Toolchain for Static Builds

**Decision**: Use Zig exclusively for all static/distribution builds.

**Rationale**:
- Zig is designed specifically for reliable cross-compilation
- Works identically on all platforms (Linux, macOS, Windows)
- Already required for ARM64 cross-compilation
- No platform-specific quirks or wrapper issues
- LLVM-based: modern, well-maintained, future-proof

**Previous Approach** (deprecated):
- Used musl-gcc for AMD64 static builds
- ❌ Broke on Arch Linux (wrapper incompatibility)
- ❌ Required platform-specific workarounds
- ❌ Different behavior across distributions
- ❌ Maintained dual toolchains unnecessarily

#### 3. Zero Dependencies for Distribution

All distribution packages are statically linked with musl libc:
- No runtime dependencies (not even libc)
- Works on any Linux distribution
- Consistent behavior regardless of host system
- Predictable, deterministic builds

#### 4. Explicit Over Implicit

Build targets clearly indicate their purpose:
- `cargo build` = development (obvious, standard Rust)
- `make build-deb-*` = distribution (explicit, clear intent)
- No magic or hidden behavior

#### 5. Platform-Native Package Management

**Decision**: Use native package managers for installing build dependencies.

**Philosophy Alignment**:
```
Build System Philosophy:
  Development → Native toolchain   → Fast iteration
  Distribution → Zig exclusively   → Reliable packages
  
Package Management Philosophy:
  Platform packages → Native managers → Integration, updates
  Universal tools → When native unavailable → Fallback only
```

**Platform-Specific Approach**:

| Platform | Method | Why |
|----------|--------|-----|
| **Arch Linux** | pacman | Native, maintained, simple |
| **Debian** | APT repository | Native, auto-updates, no snap needed |
| **Ubuntu** | APT repository (or snap) | Native preferred, snap as option |
| **Fedora/RHEL** | dnf | Native, integrated |
| **macOS** | Homebrew | Standard macOS practice |

**Rationale**:
- Native packages integrate better with system updates
- Follow platform conventions and user expectations
- Avoid universal installers (snap, flatpak) unless necessary
- Provide clear, platform-specific guidance
- Users get security updates through normal system channels

---
</text>

<old_text line=455>
#### 1. Zig (Required)

The core static build toolchain.

```bash
# Arch Linux
sudo pacman -S zig

# Debian 12
# Add official Zig repository
wget -qO- https://pkg.machengine.org/zig/gpg.key | \
    sudo gpg --dearmor -o /usr/share/keyrings/zig-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/zig-archive-keyring.gpg] \
    https://pkg.machengine.org/zig/debian stable main" | \
    sudo tee /etc/apt/sources.list.d/zig.list
sudo apt update && sudo apt install zig

# Ubuntu
# Option 1 (recommended): Use official Zig repository (same as Debian)
# Option 2: Use snap
sudo snap install zig --classic --beta

# Fedora/RHEL
sudo dnf install zig

# macOS
brew install zig

# Verify installation
zig version
```

## Build System Types

### Type 1: Development Builds

**Purpose**: Fast iteration during development and testing.

**Characteristics**:
- Dynamic linking to system libraries
- Fast incremental compilation
- Easy debugging with native tools
- Uses standard Rust toolchain
- Platform-specific binaries

**Commands**:
```bash
# Debug build (fastest)
cargo build

# Release build (optimized)
cargo build --release

# Build specific package
cargo build -p rust-can-util

# Run tests
cargo test

# Run binary
cargo run --bin rust-can-util
```

**Toolchain**: Native (provided by rustup)
- Linux: Uses system GCC/Clang
- macOS: Uses Xcode CLI tools
- Windows: Uses MSVC or GNU

**Dependencies**: System libraries (glibc, etc.)

**Output**: `target/debug/` or `target/release/`

### Type 2: Distribution Builds (Static)

**Purpose**: Create deployable packages for end users.

**Characteristics**:
- Static linking (musl libc)
- Zero runtime dependencies
- Cross-compilation support
- Stripped binaries (smaller size)
- Platform-independent

**Commands**:
```bash
# Build Debian packages for all architectures
make deb-all

# Build specific architecture
make build-deb-amd64    # x86_64
make build-deb-arm64    # aarch64

# Build and install locally
make deb-install

# Test package
make deb-test
```

**Toolchain**: Zig (via cargo-zigbuild)

**Dependencies**: None (static musl)

**Output**: `target/debian/*.deb`

---

## Toolchain Architecture

### Zig Toolchain (Distribution Builds)

**Why Zig?**

Zig provides a LLVM-based C/C++ cross-compilation toolchain that:
1. Works reliably across all platforms
2. Supports musl libc out of the box
3. Handles cross-compilation transparently
4. Requires no platform-specific configuration
5. Produces bit-identical builds

**How It Works**:

```
┌─────────────────────────────────────────────────────────┐
│ cargo zigbuild --target x86_64-unknown-linux-musl      │
│                                                         │
│  ┌──────────────┐                                      │
│  │ Rust Compiler│ → LLVM IR                            │
│  └──────────────┘                                      │
│         ↓                                               │
│  ┌──────────────┐                                      │
│  │ Zig Linker   │ → Links with musl (static)           │
│  └──────────────┘                                      │
│         ↓                                               │
│  ┌──────────────┐                                      │
│  │ Static Binary│ (zero dependencies)                  │
│  └──────────────┘                                      │
└─────────────────────────────────────────────────────────┘
```

**Advantages**:
- Consistent across macOS, Linux, Windows host systems
- No musl-gcc wrapper issues
- Clean, simple toolchain
- One-time setup, works everywhere

**Installation**:

```bash
# Install Zig binary
# Arch Linux:
sudo pacman -S zig

# Debian/Ubuntu (snap - officially supported by Zig):
sudo snap install zig --classic --beta

# Debian/Ubuntu (alternative - manual binary install):
# Download from https://ziglang.org/download/
# Extract to /usr/local or ~/.local and add to PATH
# Example:
# wget https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz
# sudo tar -C /usr/local -xf zig-linux-x86_64-0.13.0.tar.xz
# sudo ln -s /usr/local/zig-linux-x86_64-0.13.0/zig /usr/local/bin/zig
</text>

<old_text line=444>
# Arch Linux
sudo pacman -S zig

# Debian 12
# Add official Zig repository
wget -qO- https://pkg.machengine.org/zig/gpg.key | \
    sudo gpg --dearmor -o /usr/share/keyrings/zig-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/zig-archive-keyring.gpg] \
    https://pkg.machengine.org/zig/debian stable main" | \
    sudo tee /etc/apt/sources.list.d/zig.list
sudo apt update && sudo apt install zig

# Ubuntu
# Option 1 (recommended): Use official Zig repository (same as Debian)
# Option 2: Use snap
sudo snap install zig --classic --beta

# Fedora/RHEL:
sudo dnf install zig

# macOS:
brew install zig

# Install cargo-zigbuild
cargo install cargo-zigbuild

# Add Rust musl targets
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl
```

### Native Toolchain (Development Builds)

**What It Is**: The standard Rust toolchain using system linker and libraries.

**Provided By**: rustup (standard Rust installation)

**Platform-Specific**:
- Linux: Uses system GCC/Clang and glibc
- macOS: Uses system Clang and platform libraries
- Windows: Uses MSVC or MinGW

**No Special Setup**: Works out of the box with `cargo build`

---

## Build Targets Reference

### Debian Package Targets

All Debian package targets produce static binaries using the Zig toolchain.

#### `make deb-all`

Builds Debian packages for all supported architectures.

**Process**:
1. Validates dependencies
2. Builds AMD64 package
3. Validates AMD64 binaries
4. Builds ARM64 package
5. Lists generated packages

**Output**:
- `target/debian/cando-rs_VERSION_amd64.deb`
- `target/debian/cando-rs_VERSION_arm64.deb`

**Example**:
```bash
$ make deb-all
🔍 Checking Debian packaging dependencies...
✅ All packaging dependencies present
📦 Building Debian package for amd64...
  ✓ rust-can-util validated
  ✓ candump-rs validated
  ✓ cando-codegen validated
✅ Debian package built successfully!
📦 Building Debian package for arm64...
✅ Debian package built successfully!
🎉 All Debian packages built successfully!
```

#### `make build-deb-amd64`

Builds Debian package for x86_64 (AMD64) architecture.

**Process**:
1. Checks dependencies
2. Auto-installs x86_64-musl target if needed
3. Builds all workspace binaries with Zig
4. Validates binaries (runs --version on samples)
5. Generates shell completions (bash, zsh, fish)
6. Creates .deb package with cargo-deb

**Binary Validation**: Tests that compiled binaries actually execute (catches broken builds)

**Command Details**:
```bash
cargo zigbuild --workspace --release --target x86_64-unknown-linux-musl
```

**Output**: `target/debian/cando-rs_VERSION_amd64.deb`

**Package Contents**:
- 15 binaries in `/usr/bin/`
- 15 man pages in `/usr/share/man/man1/`
- 45 shell completions (15 binaries × 3 shells)
- postinst script for capabilities setup

#### `make build-deb-arm64`

Builds Debian package for ARM64 (aarch64) architecture.

**Process**:
1. Checks dependencies
2. Auto-installs aarch64-musl target if needed
3. Cross-compiles all workspace binaries with Zig
4. Uses x86_64 completions (architecture-independent)
5. Creates .deb package with cargo-deb

**Note**: Binary validation skipped (can't run ARM64 on x86_64)

**Command Details**:
```bash
cargo zigbuild --workspace --release --target aarch64-unknown-linux-musl
```

**Output**: `target/debian/cando-rs_VERSION_arm64.deb`

#### `make build-deb`

Alias for `make build-deb-amd64`.

#### `make deb-install`

Builds AMD64 package and installs it locally.

**Process**:
1. Builds AMD64 package
2. Runs `sudo dpkg -i` to install
3. Prompts for password

**Use Case**: Testing package installation on development machine.

#### `make deb-test`

Tests the AMD64 package without installing.

**Process**:
1. Builds AMD64 package
2. Extracts package metadata
3. Lists package contents
4. Verifies file structure

#### `make deb-clean`

Removes all generated packages and build artifacts.

```bash
rm -rf target/debian/*.deb
cargo clean
```

### Utility Targets

#### `make check-packaging-deps`

Validates that all packaging dependencies are installed.

**Checks**:
- ✅ `zig` binary available
- ✅ `cargo-deb` installed
- ✅ `cargo-zigbuild` installed
- ✅ `x86_64-unknown-linux-musl` target installed
- ✅ `aarch64-unknown-linux-musl` target installed

**On Failure**: Provides platform-specific installation commands.

**Example Output**:
```bash
🔍 Checking Debian packaging dependencies...
   Note: Static builds use Zig exclusively (no musl-gcc required)
✅ All packaging dependencies present
```

#### `make clean`

Removes all build artifacts (Rust standard).

```bash
cargo clean
```

---

## Dependencies

### Development Dependencies

**Minimal - Standard Rust toolchain**:

```bash
# Install Rust (if not already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# That's it! cargo build works out of the box
```

### Distribution Dependencies

**Required for building Debian packages**:

#### 1. Zig (Required)

The core static build toolchain.

```bash
# Arch Linux
sudo pacman -S zig

# Debian 12
# Add official Zig repository
wget -qO- https://pkg.machengine.org/zig/gpg.key | \
    sudo gpg --dearmor -o /usr/share/keyrings/zig-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/zig-archive-keyring.gpg] \
    https://pkg.machengine.org/zig/debian stable main" | \
    sudo tee /etc/apt/sources.list.d/zig.list
sudo apt update && sudo apt install zig

# Ubuntu
# Option 1 (recommended): Use official Zig repository (same as Debian)
# Option 2: Use snap
sudo snap install zig --classic --beta

# Fedora/RHEL
sudo dnf install zig

# macOS
brew install zig

# Verify installation
zig version
```

#### 2. cargo-zigbuild (Required)

Rust integration for Zig toolchain.

```bash
cargo install cargo-zigbuild

# Verify installation
cargo zigbuild --version
```

#### 3. cargo-deb (Required)

Creates Debian packages from Rust projects.

```bash
cargo install cargo-deb

# Verify installation
cargo deb --version
```

#### 4. Rust musl targets (Required)

Cross-compilation targets.

```bash
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl

# Verify installation
rustup target list | grep musl
```

### Quick Install (All Platforms)

**Step 1: Install Zig (platform-specific, use native packages)**

```bash
# Arch Linux
sudo pacman -S zig

# Debian 12 - Add official repository, then install
wget -qO- https://pkg.machengine.org/zig/gpg.key | \
    sudo gpg --dearmor -o /usr/share/keyrings/zig-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/zig-archive-keyring.gpg] \
    https://pkg.machengine.org/zig/debian stable main" | \
    sudo tee /etc/apt/sources.list.d/zig.list
sudo apt update && sudo apt install zig

# Ubuntu - Use official repository (recommended) or snap
# Recommended: Same as Debian above
# Alternative: sudo snap install zig --classic --beta

# Fedora/RHEL
sudo dnf install zig

# macOS
brew install zig
```

**Step 2: Install Rust tools**

```bash
cargo install cargo-deb cargo-zigbuild
```

**Step 3: Add musl targets**

```bash
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
```

**Step 4: Verify everything**

```bash
make check-packaging-deps
```

**Expected Output** (all dependencies satisfied):
```
✅ All packaging dependencies satisfied
   zig: 0.11.0 (or later)
   cargo-deb: installed
   cargo-zigbuild: installed
   x86_64-unknown-linux-musl target: installed
   aarch64-unknown-linux-musl target: installed

Ready to build Debian packages!
```

### Platform Installation Guide

This section provides detailed, step-by-step installation instructions for each supported platform, emphasizing native package management.

#### Arch Linux

**Best Method**: Native `pacman` package (officially maintained)

```bash
# Install Zig
sudo pacman -S zig

# Verify
zig version  # Should show 0.11.0 or later

# Install Rust tools
cargo install cargo-deb cargo-zigbuild

# Add musl targets
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

# Verify complete setup
make check-packaging-deps
```

**Why this method**:
- Official Arch package, regularly updated
- Integrates with system package manager
- No additional repositories needed
- Receives security updates via `pacman -Syu`

---

#### Debian 12 (Bookworm)

**Best Method**: Official Zig APT repository (no snap required)

```bash
# Step 1: Add Zig GPG key
wget -qO- https://pkg.machengine.org/zig/gpg.key | \
    sudo gpg --dearmor -o /usr/share/keyrings/zig-archive-keyring.gpg

# Step 2: Add Zig repository
echo "deb [signed-by=/usr/share/keyrings/zig-archive-keyring.gpg] \
    https://pkg.machengine.org/zig/debian stable main" | \
    sudo tee /etc/apt/sources.list.d/zig.list

# Step 3: Install Zig
sudo apt update
sudo apt install zig

# Verify
zig version  # Should show 0.11.0 or later

# Step 4: Install Rust tools
cargo install cargo-deb cargo-zigbuild

# Step 5: Add musl targets
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

# Verify complete setup
make check-packaging-deps
```

**Why this method**:
- Native APT package management (Debian's standard)
- No snap dependency required
- Automatic updates via `apt upgrade`
- Follows Debian conventions

**References**:
- Official Zig packages: https://pkg.machengine.org/zig/
- Installation guide: https://dario.griffo.io/posts/how-to-install-zig-debian

---

#### Ubuntu (20.04+)

**Method 1 (Recommended)**: Official Zig APT repository

```bash
# Same as Debian 12 above
wget -qO- https://pkg.machengine.org/zig/gpg.key | \
    sudo gpg --dearmor -o /usr/share/keyrings/zig-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/zig-archive-keyring.gpg] \
    https://pkg.machengine.org/zig/debian stable main" | \
    sudo tee /etc/apt/sources.list.d/zig.list
sudo apt update && sudo apt install zig

# Then continue with Rust tools...
```

**Method 2 (Alternative)**: Snap package

```bash
# Install Zig via snap
sudo snap install zig --classic --beta

# Verify
zig version

# Then install Rust tools and targets as above
```

**Why Method 1 is preferred**:
- Native APT integration
- Consistent with Debian methodology
- No snap daemon required

**When to use Method 2**:
- User preference for snap
- Already using snap for other tools
- Simpler one-command install

---

#### Fedora / RHEL / CentOS

**Best Method**: Native `dnf` package

```bash
# Install Zig
sudo dnf install zig

# Verify
zig version

# Install Rust tools
cargo install cargo-deb cargo-zigbuild

# Add musl targets
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

# Verify
make check-packaging-deps
```

**Why this method**:
- Official Fedora package
- Native dnf/yum integration
- System updates include Zig

---

#### macOS

**Best Method**: Homebrew (standard macOS package manager)

```bash
# Install Zig
brew install zig

# Verify
zig version

# Install Rust tools
cargo install cargo-deb cargo-zigbuild

# Add musl targets
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

# Verify
make check-packaging-deps
```

**Why this method**:
- Standard macOS development practice
- Homebrew is the de facto package manager
- Simple updates via `brew upgrade`

---

#### Platform Support Matrix

| Platform | Primary Method | Package Manager | Auto-Updates | Notes |
|----------|---------------|-----------------|--------------|-------|
| **Arch Linux** | Native package | pacman | ✅ Yes | Official, well-maintained |
| **Debian 12** | APT repository | apt | ✅ Yes | No snap needed |
| **Ubuntu** | APT repository | apt | ✅ Yes | Snap also available |
| **Fedora/RHEL** | Native package | dnf/yum | ✅ Yes | Official repo |
| **macOS** | Homebrew | brew | ✅ Yes | Standard practice |

**Key Takeaway**: Every supported platform has a native package manager solution. Universal installers (snap, manual binaries) are available but not recommended as primary methods.

---

</text>

<old_text line=900>
### Lessons Learned

1. **Right Tool for Job**: Don't force one toolchain for all purposes
2. **Platform Testing**: Test on diverse platforms early (Arch revealed musl-gcc issues)
3. **Binary Validation**: Always validate compiled binaries actually work
4. **Simplicity**: One toolchain better than multiple with workarounds
5. **Documentation**: Clear rationale helps future maintainers

---
</text>

<old_text line=911>
| Platform | Zig Source | Install Method | Notes |
|----------|------------|----------------|-------|
| **Arch Linux** | Native package | `sudo pacman -S zig` | Official native package |
| **Debian/Ubuntu** | Snap (official) | `sudo snap install zig --classic --beta` | Officially supported by Zig |
| **Debian/Ubuntu** | Manual binary | Download from ziglang.org | For snap-averse users |
| **Fedora/RHEL** | Native package | `sudo dnf install zig` | Official native package |
| **macOS** | Homebrew | `brew install zig` | Standard macOS approach |

---

## Adding New Architectures

The Zig-exclusive architecture makes adding new platforms straightforward.

### Example: Adding RISC-V Support

#### Step 1: Add Rust Target

```bash
# Check if target is available
rustup target list | grep riscv

# Add target
rustup target add riscv64gc-unknown-linux-musl
```

#### Step 2: Create Makefile Target

Add to `Makefile`:

```makefile
# Build Debian package for RISC-V (riscv64gc) using static musl linking via Zig
build-deb-riscv64: check-packaging-deps
	@echo "📦 Building Debian package for riscv64 (riscv64gc-unknown-linux-musl)"
	@echo "   Note: Using Zig toolchain for static linking (zero dependencies)"
	@echo ""
	@rustup target list | grep -q "riscv64gc-unknown-linux-musl (installed)" || { \
		echo "Installing riscv64gc-musl target..."; \
		rustup target add riscv64gc-unknown-linux-musl; \
	}
	@mkdir -p logs
	@echo "Note: RISC-V binaries cannot be validated on x86_64 host (cross-compilation)"
	@echo ""
	@echo "Step 1: Building all workspace binaries with Zig..."
	@if ! cargo zigbuild --workspace --release --target riscv64gc-unknown-linux-musl 2>&1 | tee logs/deb-build-riscv64_$$(date +%Y%m%d_%H%M%S).log | tail -20; then \
		echo ""; \
		echo "❌ RISC-V build failed! Common causes:"; \
		echo "   • Zig not installed (run 'make check-packaging-deps' for help)"; \
		echo "   • cargo-zigbuild not installed (cargo install cargo-zigbuild)"; \
		echo "   • Missing musl target (rustup target add riscv64gc-unknown-linux-musl)"; \
		exit 1; \
	fi
	@echo ""
	@echo "Step 2: Shell completions (using x86_64 completions - architecture-independent)..."
	@if [ ! -d target/completions ]; then \
		echo "⚠ Warning: No completions found. Building x86_64 first..."; \
		$(MAKE) build-deb-amd64 > /dev/null 2>&1 || true; \
	fi
	@if [ -d target/completions ]; then \
		echo "✓ Using existing completions"; \
	else \
		echo "⚠ Warning: Completions not available (package will have reduced functionality)"; \
	fi
	@echo ""
	@echo "Step 3: Creating Debian package with cargo-deb..."
	@if ! cargo deb -p cando-meta --target=riscv64gc-unknown-linux-musl --no-build \
		--output=target/debian/ -v 2>&1 | tee -a logs/deb-build-riscv64_$$(date +%Y%m%d_%H%M%S).log | tail -30; then \
		echo ""; \
		echo "❌ Debian package creation failed!"; \
		echo "   • Check the cargo-deb output above for details"; \
		exit 1; \
	fi
	@echo ""
	@echo "✅ Debian package built successfully!"
	@echo ""
	@echo "📦 Package details:"
	@ls -lh target/debian/cando-rs_*_riscv64.deb 2>/dev/null || echo "Package location: target/debian/"
	@echo ""
	@echo "To test on RISC-V: Transfer .deb to RISC-V system and run 'sudo dpkg -i cando-rs_*_riscv64.deb'"
```

#### Step 3: Update Help Text

Add to help target in `Makefile`:

```makefile
@echo "  make build-deb-riscv64 - Build Debian package for RISC-V 64-bit"
```

#### Step 4: Optional - Add to deb-all

If RISC-V should be included in `deb-all`:

```makefile
deb-all: check-packaging-deps
	# ... existing code ...
	@$(MAKE) build-deb-riscv64 || { \
		echo ""; \
		echo "❌ RISC-V package build failed!"; \
		exit 1; \
	}
```

#### Step 5: Test

```bash
# Build RISC-V package
make build-deb-riscv64

# Verify output
ls -lh target/debian/*_riscv64.deb
```

### Architecture Support Matrix

| Architecture | Rust Target | Zig Support | Status |
|--------------|-------------|-------------|--------|
| **AMD64** (x86_64) | `x86_64-unknown-linux-musl` | ✅ Yes | ✅ Production |
| **ARM64** (aarch64) | `aarch64-unknown-linux-musl` | ✅ Yes | ✅ Production |
| **RISC-V 64** | `riscv64gc-unknown-linux-musl` | ✅ Yes | 📋 Template above |
| **ARM32** (armv7) | `armv7-unknown-linux-musleabihf` | ✅ Yes | 📋 Similar to ARM64 |
| **PowerPC64** | `powerpc64le-unknown-linux-musl` | ✅ Yes | 📋 Similar to ARM64 |
| **MIPS64** | `mips64-unknown-linux-muslabi64` | ✅ Yes | 📋 Similar to ARM64 |

**Note**: Zig supports virtually all architectures. Adding support is primarily a matter of:
1. Verifying Rust target exists
2. Creating Makefile target (copy existing pattern)
3. Testing on target hardware

---

## Troubleshooting

### Build Failures

#### "zig not found"

**Cause**: Zig not installed or not in PATH.

**Solution**:
```bash
# Verify Zig installation
which zig
zig version

# If missing, install (see Dependencies section)

# Verify installation
make check-packaging-deps
```

#### "cargo-zigbuild not found"

**Cause**: cargo-zigbuild not installed.

**Solution**:
```bash
cargo install cargo-zigbuild

# Verify
cargo zigbuild --version
```

#### "target 'x86_64-unknown-linux-musl' not found"

**Cause**: Rust musl target not installed.

**Solution**:
```bash
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl

# Verify
rustup target list | grep musl
```

#### "Binary validation failed"

**Cause**: Compiled binaries can't execute.

**Symptoms**:
```
Step 1.5: Validating binaries...
  ❌ rust-can-util failed to execute
```

**Common Causes**:
1. Zig installation corrupted
2. cargo-zigbuild outdated
3. System configuration issue

**Solution**:
```bash
# Update cargo-zigbuild
cargo install cargo-zigbuild --force

# Verify Zig works
zig version

# Clean and rebuild
make clean
make build-deb-amd64

# If still fails, check Zig installation
which zig
zig cc --version
```

### Package Installation Failures

#### "Package has unmet dependencies"

**Cause**: Should not happen with static builds, but if it does:

**Check**:
```bash
# Verify package is truly static
ldd target/x86_64-unknown-linux-musl/release/rust-can-util

# Should output: "not a dynamic executable"
```

**If dynamically linked**: Build system misconfigured, check that Zig is actually being used.

#### "setcap failed" during installation

**Cause**: Installing without sudo or on system without capability support.

**Solution**:
```bash
# Install with sudo
sudo dpkg -i target/debian/cando-rs_*.deb

# Or skip capabilities (manual setup required)
sudo dpkg -i --force-depends target/debian/cando-rs_*.deb
```

### Cross-Compilation Issues

#### ARM64 package doesn't work on target

**Check**:
1. Verify target is actually ARM64: `uname -m` should show `aarch64`
2. Verify package architecture: `dpkg --info cando-rs_*_arm64.deb`
3. Check for architecture mismatch

**Note**: AMD64 packages won't work on ARM64 and vice versa.

---

## Historical Context

### Evolution of the Build System

#### Phase 1: Initial Implementation (May-October 2024)

**Approach**: Used musl-gcc for static AMD64 builds

**Architecture**:
```
AMD64 → musl-gcc → Static binary
ARM64 → Zig (cross-compile) → Static binary
```

**Problems**:
- musl-gcc broken on Arch Linux (wrapper incompatibility)
- Platform-specific build failures
- Different toolchains for different architectures
- Maintenance burden for platform workarounds

#### Phase 2: Dual Toolchain (October 2024)

**Approach**: Added binary validation and Zig alternative

**Architecture**:
```
AMD64 → musl-gcc (primary) → Validate → If fails, use Zig
AMD64 → Zig (alternative) → Validate → Always works
ARM64 → Zig → Static binary
```

**Improvements**:
- Binary validation caught broken musl-gcc builds
- Zig alternative worked on Arch Linux
- Clear error messages guided users

**Problems**:
- Maintained two toolchains unnecessarily
- Complex validation logic
- Platform-specific workarounds (~300 lines)
- User confusion about which target to use
- Dual build paths for AMD64

#### Phase 3: Zig-Exclusive (November 2024) - Current

**Approach**: Use Zig exclusively for all static builds

**Architecture**:
```
AMD64 → Zig → Static binary
ARM64 → Zig → Static binary
All Future Architectures → Zig → Static binary
```

**Benefits**:
- ✅ Single, consistent toolchain
- ✅ Works reliably on all platforms
- ✅ No platform-specific workarounds
- ✅ Simplified codebase (~200 lines removed)
- ✅ Clear mental model
- ✅ Easy to add new architectures

**Result**: Production-ready, maintainable, future-proof build system

### Key Decision Points

#### Decision 1: Static vs Dynamic Linking

**Question**: Should distribution packages be statically or dynamically linked?

**Decision**: Static (musl)

**Rationale**:
- Zero dependencies = works everywhere
- Predictable behavior
- No glibc version issues
- Easier support
- Slightly larger size acceptable trade-off

#### Decision 2: Which Toolchain for Static Builds?

**Question**: musl-gcc or Zig?

**Initial**: musl-gcc (standard, familiar)

**Final**: Zig (after discovering musl-gcc issues)

**Rationale**:
- Zig designed for cross-compilation
- Works reliably everywhere
- No platform quirks
- Already required for ARM64
- Modern, well-maintained
- Future-proof

#### Decision 3: Separate or Unified Build System?

**Question**: Should dev and dist builds use same toolchain?

**Decision**: Separate (native for dev, Zig for dist)

**Rationale**:
- Different goals require different tools
- Dev builds prioritize speed
- Dist builds prioritize portability
- Clear separation of concerns
- Each optimized for purpose

### Lessons Learned

1. **Right Tool for Right Job**: Don't force one toolchain for all purposes. Development and distribution have different needs and benefit from different toolchains.

2. **Platform Testing is Critical**: Test on diverse platforms early in development. Arch Linux revealed musl-gcc wrapper incompatibilities that weren't visible on Debian/Ubuntu.

3. **Binary Validation is Essential**: Always validate that compiled binaries actually work, not just that compilation succeeds. A binary that compiles but segfaults is worse than a compilation error.

4. **Simplicity Over Cleverness**: One consistent toolchain with clear behavior is better than multiple toolchains with platform-specific workarounds. The ~200 lines of complexity we removed were not worth maintaining.

5. **Documentation Saves Future Pain**: Clear rationale and historical context helps future maintainers understand *why* decisions were made, not just *what* was decided.

6. **Native Package Managers Win**: Users expect and prefer platform-native package management. Providing clear, platform-specific guidance reduces confusion and improves the user experience.

7. **Cross-Platform Design**: When a tool (like Zig) works identically on all platforms, it's a strong signal it's the right choice for cross-platform builds.

8. **Fail Fast, Fail Clear**: When things go wrong, provide clear, actionable error messages with copy-paste ready commands. See `make check-packaging-deps` for examples.

---

## Technical Details

### Makefile Organization

The build system is implemented in the root `Makefile` with logical sections:

```makefile
# 1. Phony target declarations
.PHONY: deb-all build-deb-amd64 build-deb-arm64 ...

# 2. Help system
help:
    # Lists all targets with descriptions

# 3. Dependency checking
check-packaging-deps:
    # Validates Zig, cargo-zigbuild, cargo-deb, targets

# 4. Build targets
build-deb-amd64:
    # AMD64 package build
    
build-deb-arm64:
    # ARM64 package build

# 5. Utility targets
deb-clean:
    # Cleanup
```

### Binary Validation Logic

AMD64 builds include runtime validation to ensure binaries work:

```bash
# Test sample binaries
for binary in rust-can-util candump-rs cando-codegen; do
    timeout 2s "$binary" --version >/dev/null 2>&1 || FAIL
done

# If any fail, stop build with helpful message
```

**Purpose**: Catch broken toolchain issues before packaging

**Why Only AMD64**: Can't run ARM64/RISC-V binaries on x86_64 host

### Shell Completion Generation

Completions generated once and reused:

1. AMD64 build generates completions (architecture-independent)
2. ARM64/other architectures reuse existing completions
3. Falls back gracefully if unavailable

**Script**: `scripts/packaging/generate-completions.sh`

### Package Metadata

Defined in `cando-meta/Cargo.toml`:

```toml
[package.metadata.deb]
maintainer = "..."
depends = ""  # Empty = no dependencies (static)
section = "utils"
priority = "optional"
assets = [
    # 15 binaries
    # 15 man pages  
    # 45 shell completions
]
```

### Build Logs

All builds create timestamped logs:

- AMD64: `logs/deb-build-amd64_YYYYMMDD_HHMMSS.log`
- ARM64: `logs/deb-build-arm64_YYYYMMDD_HHMMSS.log`

Useful for debugging build failures.

---

## Summary

The Cando-RS build system provides:

1. **Fast development iteration** with native toolchain
2. **Reliable distribution packages** with Zig static builds
3. **Cross-platform consistency** (works everywhere)
4. **Zero dependencies** for end users
5. **Easy architecture additions** (simple pattern to copy)
6. **Clear separation of concerns** (dev vs dist)
7. **Comprehensive error handling** (helpful messages)
8. **Production-ready packages** (validated binaries)

**Key Takeaway**: Use Zig for distribution, native for development. It's that simple.

---

## Quick Command Reference

```bash
# Development
cargo build                    # Fast dev build
cargo test                     # Run tests
cargo build --release          # Optimized dev build

# Distribution
make check-packaging-deps      # Verify dependencies
make deb-all                   # Build all packages
make build-deb-amd64          # AMD64 only
make build-deb-arm64          # ARM64 only
make deb-install               # Build and install locally

# Maintenance
make clean                     # Remove build artifacts
make deb-clean                 # Remove packages
```

---

**For questions or issues, refer to this document first. It contains the complete rationale and implementation details for the build system.**

---

## References and Additional Resources

### Official Documentation

**Zig Toolchain**:
- Official website: https://ziglang.org/
- Download page: https://ziglang.org/download/
- Documentation: https://ziglang.org/documentation/master/

**Zig Package Repositories**:
- Mach Engine (Debian/Ubuntu): https://pkg.machengine.org/zig/
- Installation guide for Debian: https://dario.griffo.io/posts/how-to-install-zig-debian

**Rust Cross-Compilation**:
- cargo-zigbuild: https://github.com/rust-cross/cargo-zigbuild
- cargo-deb: https://github.com/kornelski/cargo-deb
- Platform Support: https://doc.rust-lang.org/nightly/rustc/platform-support.html

### Related Cando-RS Documentation

**Primary AI Context Documents**:
- `doc/AI-WORKFLOW-GUIDE.md` - Mandatory AI behavioral rules
- `doc/AI-WORKFLOW-GUIDE.md` - Project standards and workflows
- `resume-main.md` - Main project context restoration

**Build System History**:
- `doc/build-system/NATIVE-PKG-UPDATE.md` - Native package manager update record
- `doc/build-system/resume-debian-packaging.md` - Debian packaging feature context

**Makefile Targets**:
- Root `Makefile` - Contains all build targets and validation logic

### Key Decisions Archive

**Why Zig-Exclusive** (November 2024):
- Eliminated musl-gcc due to Arch Linux wrapper incompatibility
- Reduced build system complexity by ~200 lines
- Unified toolchain for all static builds (AMD64, ARM64, future architectures)
- See "Phase 3: Zig-Exclusive" in Historical Context section

**Why Native Package Managers** (November 2024):
- Prioritize platform conventions over universal installers
- Better system integration and automatic updates
- Reduced user confusion with clear platform-specific guidance
- See "Platform-Native Package Management" in Architecture Philosophy

### Community Resources

**Issue Tracking**:
- Arch Linux musl-gcc issue: https://bbs.archlinux.org/viewtopic.php?id=239533
- Report build system issues in project repository

**Getting Help**:
1. Check this document's Troubleshooting section first
2. Run `make check-packaging-deps` for dependency validation
3. Review build logs in `logs/` directory
4. Consult platform-specific installation guides above

### For AI Assistants

**Context Restoration Priority**:
1. Read this entire document (BUILD-SYSTEM.md) for build system work
2. Understand the "Right Tool for Right Job" philosophy
3. Review "Adding New Architectures" for extending the system
4. Check "Lessons Learned" for historical pitfalls to avoid

**Key Principles to Remember**:
- Development builds use native toolchain (fast iteration)
- Distribution builds use Zig exclusively (static, portable)
- Native package managers are preferred for dependencies
- Every architectural decision has documented rationale
- Binary validation is essential (compilation success ≠ working binary)

---

**Document Status**: Complete and production-ready  
**Last Major Update**: November 2024 (Zig-exclusive architecture + native package management)  
**Maintainer Note**: This is the canonical reference for the Cando-RS build system