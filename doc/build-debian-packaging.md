# Building Debian Packages - Maintainer Guide

**Audience**: Package maintainers, contributors, CI/CD engineers  
**Last Updated**: 2024-11-07  
**Package Version**: 0.1.0-1

---

## 📋 Overview

This guide covers building Cando-RS Debian packages from source. It is intended for:
- Package maintainers
- Contributors preparing releases
- CI/CD pipeline developers
- Anyone building custom packages

**For end users**: See [build-debian-install.md](build-debian-install.md) instead.

---

## 🎯 Quick Start

```bash
# 1. Install prerequisites (amd64 only)
rustup target add x86_64-unknown-linux-musl
cargo install cargo-deb

# For ARM64 builds, also install:
rustup target add aarch64-unknown-linux-musl
cargo install cargo-zigbuild
sudo snap install zig --classic --beta

# 2. Build package
make build-deb        # amd64 only
make build-deb-arm64  # ARM64 only
make deb-all          # Both architectures

# 3. Package created at:
ls -lh target/debian/cando-rs_*.deb
```

---

## 📦 Package Architecture

### Meta-Package Approach

Cando-RS uses a **meta-package** strategy:
- Package name: `cando-meta` (workspace member)
- Output: `cando-rs` Debian package
- Contains binaries from all 13 workspace members
- Single package simplifies distribution and installation

### Directory Structure

```
cando-rs/
├── cando-meta/              # Meta-package for Debian packaging
│   ├── Cargo.toml            # Contains [package.metadata.deb]
│   └── src/lib.rs            # Minimal library (packaging only)
├── scripts/packaging/
│   └── generate-completions.sh   # Shell completion generator
├── man/                       # Man pages (*.1 files)
├── target/
│   ├── debian/               # Output: .deb packages
│   ├── completions/          # Generated shell completions
│   └── x86_64-unknown-linux-musl/release/  # Static binaries
└── Makefile                  # Packaging targets
```

---

## 🔧 Prerequisites

### System Requirements

**Operating System**:
- Linux (Ubuntu 20.04+, Debian 11+)
- macOS (with limitations - see Cross-Compilation)
- WSL2 on Windows (experimental)

**Tools Required**:
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Musl targets for static linking
rustup target add x86_64-unknown-linux-musl       # Required for amd64
rustup target add aarch64-unknown-linux-musl      # Required for ARM64

# cargo-deb for package generation
cargo install cargo-deb

# ARM64 cross-compilation (required for ARM64 builds)
cargo install cargo-zigbuild
sudo snap install zig --classic --beta
# Alternative: Download from https://ziglang.org/download/

# Optional: For validation
sudo apt install lintian dpkg-dev
```

**Why Zig for ARM64?**
- Cross-compiling musl binaries for ARM64 requires a proper aarch64-musl toolchain
- Zig provides excellent cross-compilation support out of the box
- `cargo-zigbuild` integrates Zig seamlessly with Cargo's build process
- This is simpler than setting up `musl-cross-make` or Docker-based `cross`

### Verify Setup

```bash
make check-packaging-deps
```

Expected output:
```
🔍 Checking Debian packaging dependencies...
✅ All packaging dependencies present
```

---

## 🏗️ Building Packages

### Method 1: Using Makefile (Recommended)

**Build amd64 package**:
```bash
make build-deb-amd64
# or
make build-deb
```

**Build arm64 package**:
```bash
make build-deb-arm64
```

**Build both architectures**:
```bash
make deb-all
```

**Output**:
```
target/debian/cando-rs_0.1.0-1_amd64.deb
target/debian/cando-rs_0.1.0-1_arm64.deb
```

### Method 2: Manual cargo-deb

**Step 1: Build workspace**
```bash
cargo build --workspace --release --target x86_64-unknown-linux-musl
```

**Step 2: Generate completions**
```bash
./scripts/packaging/generate-completions.sh x86_64-unknown-linux-musl
```

**Step 3: Create package**
```bash
cargo deb -p cando-meta \
    --target=x86_64-unknown-linux-musl \
    --output=target/debian/
```

### Method 3: Step-by-Step (Understanding the Process)

```bash
# 1. Check dependencies
make check-packaging-deps

# 2. Clean previous builds (optional)
cargo clean
rm -rf target/debian/ target/completions/

# 3. Build all workspace binaries with musl (static linking)
cargo build --workspace --release --target x86_64-unknown-linux-musl

# 4. Generate shell completions
./scripts/packaging/generate-completions.sh x86_64-unknown-linux-musl

# 5. Create Debian package
cargo deb -p cando-meta \
    --target=x86_64-unknown-linux-musl \
    --output=target/debian/ \
    -v  # Verbose output

# 6. Verify package
make deb-test
```

---

## 🔄 Cross-Compilation

### Building ARM64 Packages

**Prerequisites**:
```bash
# Add ARM64 musl target
rustup target add aarch64-unknown-linux-musl

# Optional: ARM64 cross-compilation tools
sudo apt install gcc-aarch64-linux-gnu
```

**Build**:
```bash
make build-deb-arm64
```

**What Happens**:
1. Cargo builds all binaries for `aarch64-unknown-linux-musl`
2. Completions generated for ARM64 binaries
3. cargo-deb creates `cando-rs_0.1.0-1_arm64.deb`

### Cross-Compilation Notes

- **Musl targets** simplify cross-compilation (no system dependencies)
- **Pure Rust** projects compile easily across architectures
- **C dependencies** may require additional setup (not applicable here)
- **Testing ARM64** packages requires ARM64 hardware or QEMU

---

## 🧪 Testing Packages

### Quick Validation

```bash
make deb-test
```

This checks:
- Package info (metadata)
- Package contents (file listing)
- Binary static linking verification

### Full Testing

**Install locally**:
```bash
sudo dpkg -i target/debian/cando-rs_0.1.0-1_amd64.deb
```

**Test binaries**:
```bash
rust-can-util --version
candump-rs --version
# ... test all 15 binaries
```

**Test man pages**:
```bash
man rust-can-util
man candump-rs
```

**Test completions**:
```bash
source /etc/bash_completion
rust-can-util --<TAB>
```

**Uninstall**:
```bash
sudo dpkg -r cando-rs
```

### Lintian Validation

```bash
# Install lintian
sudo apt install lintian

# Validate package
lintian target/debian/cando-rs_0.1.0-1_amd64.deb

# Verbose output
lintian -v target/debian/cando-rs_0.1.0-1_amd64.deb
```

**Expected warnings** (acceptable):
- `no-copyright-file` - We use generated copyright
- `binary-without-manpage` - All binaries have man pages (false positive)

**Errors should be fixed before release.**

---

## 📝 Package Configuration

### Cargo.toml Metadata

Package metadata is in `cando-meta/Cargo.toml`:

```toml
[package.metadata.deb]
name = "cando-rs"
maintainer = "John Suykerbuyk <john@syketech.com>"
copyright = "2024, John Suykerbuyk <john@syketech.com>"
extended-description = """\
Comprehensive CAN bus toolkit with type-safe message handling..."""
section = "utils"
priority = "optional"
depends = "$auto"

assets = [
    # Binaries (13)
    ["target/release/rust-can-util", "usr/bin/", "755"],
    # ... more binaries
    
    # Man pages
    ["../man/*.1", "usr/share/man/man1/", "644"],
    
    # Completions
    ["../target/completions/bash/*", "usr/share/bash-completion/completions/", "644"],
    ["../target/completions/zsh/*", "usr/share/zsh/site-functions/", "644"],
    ["../target/completions/fish/*", "usr/share/fish/vendor_completions.d/", "644"],
    
    # Documentation
    ["../README.md", "usr/share/doc/cando-rs/README", "644"],
    ["../CHANGELOG.md", "usr/share/doc/cando-rs/CHANGELOG", "644"],
]
```

### Key Fields Explained

- **name**: Output package name (`cando-rs`)
- **maintainer**: Package maintainer contact
- **copyright**: Copyright holder and year
- **extended-description**: Long description (shown in apt)
- **section**: Debian archive section (utils, devel, etc.)
- **priority**: Installation priority (optional, required, etc.)
- **depends**: Dependencies (`$auto` = detect automatically)
- **assets**: Files to include in package

### Asset Path Rules

**Important**: cargo-deb automatically translates paths:
- `target/release/*` → `target/x86_64-unknown-linux-musl/release/*` (when building with --target)
- Relative paths from `cando-meta/` require `../` prefix

**Examples**:
```toml
# ✅ Correct - binary from target/release (auto-translated)
["target/release/rust-can-util", "usr/bin/", "755"]

# ✅ Correct - man pages from workspace root
["../man/*.1", "usr/share/man/man1/", "644"]

# ❌ Wrong - missing ../ for workspace root files
["man/*.1", "usr/share/man/man1/", "644"]
```

---

## 🔍 Troubleshooting

### Issue: cargo-deb Not Found

```bash
error: cargo-deb: command not found
```

**Solution**:
```bash
cargo install cargo-deb
```

### Issue: Musl Target Missing

```bash
error: target 'x86_64-unknown-linux-musl' not found
```

**Solution**:
```bash
rustup target add x86_64-unknown-linux-musl
```

### Issue: Asset Not Found

```bash
error: Can't resolve asset: cando-meta/man/*.1
```

**Solution**: Check asset path - needs `../` prefix for workspace root files:
```toml
# Change from:
["man/*.1", "usr/share/man/man1/", "644"]
# To:
["../man/*.1", "usr/share/man/man1/", "644"]
```

### Issue: Package Size Too Large

**Check uncompressed size**:
```bash
dpkg-deb --info target/debian/cando-rs_*.deb | grep "Installed-Size"
```

**Solutions**:
- Verify `strip = true` in Cargo.toml release profile
- Check for debug symbols: `file target/.../release/rust-can-util`
- Use `--separate-debug-symbols` flag

### Issue: Completions Not Generated

```bash
error: Can't resolve asset: ../target/completions/bash/*
```

**Solution**: Run completion generation before cargo-deb:
```bash
./scripts/packaging/generate-completions.sh x86_64-unknown-linux-musl
```

### Issue: Permission Denied During Build

```bash
error: Permission denied (os error 13)
```

**Solution**: Don't use sudo for building:
```bash
# ❌ Wrong
sudo make build-deb

# ✅ Correct
make build-deb
```

Only use sudo for installation: `sudo dpkg -i ...`

---

## 🚀 CI/CD Integration

### GitHub Actions Example

```yaml
name: Build Debian Packages

on:
  push:
    tags:
      - 'v*'

jobs:
  build-deb:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target:
          - x86_64-unknown-linux-musl
          - aarch64-unknown-linux-musl
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Install cargo-deb
        run: cargo install cargo-deb
      
      - name: Build package
        run: |
          cargo build --workspace --release --target ${{ matrix.target }}
          ./scripts/packaging/generate-completions.sh ${{ matrix.target }}
          cargo deb -p cando-meta --target=${{ matrix.target }} --output=target/debian/
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: debian-packages
          path: target/debian/*.deb
```

### GitLab CI Example

```yaml
build-deb:
  image: rust:latest
  script:
    - rustup target add x86_64-unknown-linux-musl
    - cargo install cargo-deb
    - make build-deb
  artifacts:
    paths:
      - target/debian/*.deb
    expire_in: 1 week
```

---

## 📦 Release Process

### Step 1: Update Version

Update version in `Cargo.toml`:
```toml
[workspace.package]
version = "0.2.0"
```

### Step 2: Update Changelog

Add entry to `CHANGELOG.md`:
```markdown
## [0.2.0] - 2024-11-15

### Added
- New feature X
- New binary Y

### Fixed
- Bug fix Z
```

### Step 3: Build Packages

```bash
# Build both architectures
make deb-all

# Verify packages
ls -lh target/debian/
```

### Step 4: Test Packages

```bash
# Test amd64
sudo dpkg -i target/debian/cando-rs_0.2.0-1_amd64.deb
rust-can-util --version
sudo dpkg -r cando-rs

# Test arm64 (if ARM64 available)
sudo dpkg -i target/debian/cando-rs_0.2.0-1_arm64.deb
# ... test
```

### Step 5: Generate Checksums

```bash
cd target/debian/
sha256sum *.deb > SHA256SUMS
cat SHA256SUMS
```

### Step 6: Create Git Tag

```bash
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0
```

### Step 7: Create GitHub Release

1. Go to GitHub → Releases → "Draft a new release"
2. Select tag: `v0.2.0`
3. Title: `Cando-RS v0.2.0`
4. Description: Copy from CHANGELOG.md
5. Attach files:
   - `cando-rs_0.2.0-1_amd64.deb`
   - `cando-rs_0.2.0-1_arm64.deb`
   - `SHA256SUMS`
6. Publish release

---

## 🔧 Advanced Topics

### Custom Package Version

Override version with `--deb-version`:
```bash
cargo deb -p cando-meta \
    --target=x86_64-unknown-linux-musl \
    --deb-version 0.1.0-2ubuntu1
```

### Separate Debug Symbols

Create separate debug package:
```bash
cargo deb -p cando-meta \
    --target=x86_64-unknown-linux-musl \
    --separate-debug-symbols
```

Creates:
- `cando-rs_0.1.0-1_amd64.deb`
- `cando-rs-dbgsym_0.1.0-1_amd64.deb`

### Custom Maintainer Scripts

Create `debian/` directory with scripts:
```bash
mkdir -p debian/maintainer-scripts
```

Files:
- `preinst` - Before installation
- `postinst` - After installation
- `prerm` - Before removal
- `postrm` - After removal

Add to Cargo.toml:
```toml
[package.metadata.deb]
maintainer-scripts = "debian/maintainer-scripts"
```

### Package Variants

Create variants for different configurations:
```toml
[package.metadata.deb.variants.minimal]
name = "cando-rs-minimal"
assets = [
    ["target/release/rust-can-util", "usr/bin/", "755"],
    # Only include core tools
]
```

Build variant:
```bash
cargo deb -p cando-meta --variant=minimal
```

---

## 📊 Package Metrics

### Current Package Stats

| Metric | Value |
|--------|-------|
| Package size (compressed) | 8.9 MB |
| Installed size | 63 MB |
| Number of binaries | 13 |
| Number of man pages | 13 |
| Number of completions | 39 (13×3) |
| Dependencies | 0 (static) |
| Build time (amd64) | ~1.5 minutes |
| Build time (arm64) | ~2 minutes |

### Size Breakdown

```bash
# Check individual binary sizes
ls -lh target/x86_64-unknown-linux-musl/release/ | grep -E '^-rwx'

# Check compressed package
du -h target/debian/*.deb

# Check installed size
dpkg-deb --info target/debian/*.deb | grep Installed-Size
```

---

## 🎓 Best Practices

### DO ✅

1. **Always build with musl** for static linking
2. **Generate completions** before packaging
3. **Test packages** before releasing
4. **Verify checksums** after building
5. **Update CHANGELOG.md** with each release
6. **Use semantic versioning** (0.1.0, 0.2.0, 1.0.0)
7. **Tag releases** in Git
8. **Keep logs** of builds for debugging

### DON'T ❌

1. **Don't use sudo** for building (only for installation)
2. **Don't hardcode paths** (use relative paths with ../)
3. **Don't include debug symbols** in release packages
4. **Don't skip testing** before release
5. **Don't modify version manually** in multiple places
6. **Don't build on macOS** for Linux distribution (use Linux or CI)
7. **Don't forget** to update man pages when changing CLI

---

## 📚 Additional Resources

### cargo-deb Documentation

- GitHub: https://github.com/kornelski/cargo-deb
- All configuration options: https://github.com/kornelski/cargo-deb#configuration

### Debian Packaging

- Debian Policy Manual: https://www.debian.org/doc/debian-policy/
- New Maintainer's Guide: https://www.debian.org/doc/manuals/maint-guide/

### Rust Cross-Compilation

- Rust Platform Support: https://doc.rust-lang.org/nightly/rustc/platform-support.html
- Musl libc: https://www.musl-libc.org/

---

## 🆘 Getting Help

### Questions

- GitHub Discussions: https://github.com/suykerbuyk/cando-rs/discussions
- Open an issue: https://github.com/suykerbuyk/cando-rs/issues

### Reporting Build Issues

When reporting build problems, include:
1. Operating system and version
2. Rust version (`rustc --version`)
3. cargo-deb version (`cargo deb --version`)
4. Full build log
5. Output of `make check-packaging-deps`

---

**Document Version**: 1.0  
**Last Updated**: 2024-11-07  
**Maintainer**: Cando-RS Team  
**Status**: Production