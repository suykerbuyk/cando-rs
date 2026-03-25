# Installing Cando-RS from Debian Package

**Version**: 0.1.0  
**Last Updated**: 2024-11-07  
**Package**: cando-rs_0.1.0-1_amd64.deb / cando-rs_0.1.0-1_arm64.deb

---

## 📋 Quick Start

```bash
# Download package (replace with actual GitHub release URL)
wget https://github.com/suykerbuyk/cando-rs/releases/download/v0.1.0/cando-rs_0.1.0-1_amd64.deb

# Install
sudo dpkg -i cando-rs_0.1.0-1_amd64.deb

# Verify
rust-can-util --version
```

---

## 📦 Package Information

**Package Name**: cando-rs  
**Version**: 0.1.0-1  
**Size**: 8.9 MB  
**Dependencies**: libcap2-bin (for CAN interface access without sudo)  
**Runtime Dependencies**: None (all binaries statically linked)  
**Supported Architectures**: 
- amd64 (x86_64)
- arm64 (aarch64 - Raspberry Pi 4/5, ARM servers)

**Contents**:
- 15 CAN bus utilities and simulators
- 15 man pages
- Shell completions (bash, zsh, fish)
- Documentation

**Automatic Configuration**:
- Sets CAP_NET_RAW capability on CAN-related binaries during installation
- Enables unprivileged CAN interface access (no sudo required)
- Supports both physical CAN (can0, can1) and virtual CAN (vcan0, vcan1)

---
</text>

<old_text line=80>
wget https://github.com/suykerbuyk/cando-rs/releases/download/v0.1.0/cando-rs_0.1.0-1_amd64.deb

## 🖥️ System Requirements

### Minimum Requirements
- **Operating System**: Debian 10+ or Ubuntu 20.04+
- **Architecture**: x86_64 (amd64) or ARM64 (aarch64)
- **Disk Space**: 65 MB
- **RAM**: 100 MB
- **Privileges**: sudo/root for installation

### Supported Distributions
✅ **Tested and Supported**:
- Ubuntu 22.04 LTS (Jammy Jellyfish)
- Ubuntu 24.04 LTS (Noble Numbat)
- Debian 11 (Bullseye)
- Debian 12 (Bookworm)
- Raspberry Pi OS (ARM64)

✅ **Should Work** (untested):
- Linux Mint 21+
- Pop!_OS 22.04+
- Elementary OS 7+
- Any Debian-based distribution

❌ **Not Supported**:
- Red Hat / CentOS / Fedora (use RPM packages instead)
- Arch Linux / Manjaro (use AUR or build from source)
- Alpine Linux (use musl build directly)

---

## 📥 Installation Methods

### Method 1: Download from GitHub Releases (Recommended)

**Step 1: Download the Package**

For **x86_64 (Intel/AMD)** systems:
```bash
wget https://github.com/suykerbuyk/cando-rs/releases/download/v0.1.0/cando-rs_0.1.0-1_amd64.deb
```

For **ARM64 (Raspberry Pi 4/5, ARM servers)**:
```bash
wget https://github.com/suykerbuyk/cando-rs/releases/download/v0.1.0/cando-rs_0.1.0-1_arm64.deb
```

**Step 2: Verify Checksum (Recommended)**

```bash
# Download checksum file
wget https://github.com/suykerbuyk/cando-rs/releases/download/v0.1.0/SHA256SUMS

# Verify (should output "OK")
sha256sum -c SHA256SUMS 2>&1 | grep cando-rs
```

Expected output:
```
cando-rs_0.1.0-1_amd64.deb: OK
```

**Step 3: Install**

```bash
sudo dpkg -i cando-rs_0.1.0-1_amd64.deb
```

### Method 2: Direct Download and Install (One-Liner)

**For x86_64**:
```bash
curl -L https://github.com/suykerbuyk/cando-rs/releases/download/v0.1.0/cando-rs_0.1.0-1_amd64.deb -o cando-rs.deb && sudo dpkg -i cando-rs.deb
```

**For ARM64**:
```bash
curl -L https://github.com/suykerbuyk/cando-rs/releases/download/v0.1.0/cando-rs_0.1.0-1_arm64.deb -o cando-rs.deb && sudo dpkg -i cando-rs.deb
```

---

## ✅ Verify Installation

### Check Installed Version

```bash
rust-can-util --version
# Output: rust-can-util 0.1.0
```

### Verify All Binaries

```bash
# CLI Tools
rust-can-util --version
dump-messages --version
monitor-can --version
candump-rs --version
cansend-rs --version
can-log-analyzer --version
count-hvpc-signals --version

# Simulators
emp-simulator --version
hvpc-simulator --version
udc-simulator --version
j1939-simulator --version

# Web Interface
cando-webui --version

# Code Generation
cando-codegen --version
```

All should output: `<tool-name> 0.1.0`

### Check Man Pages

```bash
man rust-can-util
man candump-rs
man cando-webui
```

Press `q` to exit the man page viewer.

### Check Shell Completions

**Bash** (restart terminal or run):
```bash
source /etc/bash_completion
rust-can-util --<TAB><TAB>
# Should show: --help --version --device-id --message --fields --send-interface --format
```

**Zsh** (add to ~/.zshrc):
```bash
fpath=(/usr/share/zsh/site-functions $fpath)
autoload -Uz compinit && compinit
```

**Fish** (automatic):
```bash
rust-can-util --<TAB>
# Should show completions
```

---

## 🚀 Quick Start Usage

### Example 1: Encode a CAN Message

```bash
rust-can-util \
    --device-id 0x8A \
    --message MCM_MotorCommandMessage \
    --fields "mcm_percentmotorspeedcommand=75.0,mcm_onoffdirectioncommand=1"
```

### Example 2: Monitor CAN Bus (Virtual Interface)

```bash
# Set up virtual CAN interface
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0

# Monitor CAN traffic
candump-rs vcan0
```

### Example 3: Run EMP Simulator

```bash
# Start EMP motor simulator on vcan0
emp-simulator --interface vcan0 --device-id 0x8A
```

### Example 4: Start Web Interface

```bash
# Start web monitoring interface
cando-webui --port 8080

# Access in browser: http://localhost:8080
```

### Example 5: Get Help

```bash
# Show help for any tool
rust-can-util --help
candump-rs --help
cando-webui --help
```

---

## 🔄 Upgrading

### Upgrade to Newer Version

```bash
# Download new version
wget https://github.com/suykerbuyk/cando-rs/releases/download/v0.2.0/cando-rs_0.2.0-1_amd64.deb

# Install (automatically upgrades)
sudo dpkg -i cando-rs_0.2.0-1_amd64.deb

# Verify new version
rust-can-util --version
```

**Note**: Configuration files and user data are preserved during upgrades.

---

## 🗑️ Uninstallation

### Remove Package

```bash
sudo dpkg -r cando-rs
```

### Remove Package and Configuration

```bash
# dpkg doesn't create config files, so this is equivalent
sudo dpkg --purge cando-rs
```

### Verify Removal

```bash
which rust-can-util
# Should output nothing (command not found)

dpkg -l | grep cando-rs
# Should output nothing (package not installed)
```

---

## 🐛 Troubleshooting

### Issue: Package Won't Install

**Symptom**:
```bash
dpkg: error processing package cando-rs
```

**Solution**:
```bash
# Check for broken packages
sudo dpkg --configure -a

# Try installing again
sudo dpkg -i cando-rs_0.1.0-1_amd64.deb
```

### Issue: "Command not found" After Installation

**Symptom**:
```bash
$ rust-can-util --version
bash: rust-can-util: command not found
```

**Solution**:
```bash
# Ensure /usr/bin is in your PATH
echo $PATH | grep -q /usr/bin || export PATH=$PATH:/usr/bin

# Restart terminal and try again
rust-can-util --version
```

### Issue: Permission Denied When Accessing CAN Interface

**Symptom**:
```bash
$ candump-rs can0
Error: Permission denied (os error 13)
```

**Cause**: Capabilities may not have been set during installation (filesystem doesn't support xattr, or installation error).

**Solution 1 - Check Capabilities**:
```bash
# Verify capabilities are set
getcap /usr/bin/candump-rs
# Expected output: /usr/bin/candump-rs cap_net_raw=eip

# If no output, manually set capabilities
sudo setcap cap_net_raw+eip /usr/bin/candump-rs
sudo setcap cap_net_raw+eip /usr/bin/monitor-can
sudo setcap cap_net_raw+eip /usr/bin/rust-can-util
# ... repeat for all CAN tools
```

**Solution 2 - Use sudo (temporary)**:
```bash
# Run with sudo as a workaround
sudo candump-rs can0
```

**Solution 3 - Reinstall with libcap2-bin**:
```bash
# Ensure libcap2-bin is installed
sudo apt update
sudo apt install libcap2-bin

# Reinstall package to reapply capabilities
sudo dpkg -r cando-rs
sudo dpkg -i cando-rs_0.1.0-1_amd64.deb
```

### Issue: Virtual CAN (vcan0) Not Working

**Symptom**:
```bash
$ candump-rs vcan0
Error: No such device
```

**Solution**:
```bash
# Verify package is installed
dpkg -l | grep cando-rs

# Check if binary exists
ls -la /usr/bin/rust-can-util

# Update PATH (should not be necessary)
export PATH="/usr/bin:$PATH"

# Restart terminal
```

### Issue: Wrong Architecture

**Symptom**:
```bash
dpkg: error: package architecture (amd64) does not match system (arm64)
```

**Solution**:
Download the correct package for your architecture:
- Use `amd64` package for Intel/AMD processors
- Use `arm64` package for ARM processors (Raspberry Pi, etc.)

Check your architecture:
```bash
dpkg --print-architecture
# Output: amd64 or arm64
```

### Issue: Permission Denied

**Symptom**:
```bash
dpkg: error: requested operation requires superuser privilege
```

**Solution**:
Use `sudo`:
```bash
sudo dpkg -i cando-rs_0.1.0-1_amd64.deb
```

### Issue: "Cannot Find CAN Interface"

**Symptom**:
```bash
Error: No such device (os error 19)
```

**Solution**:
```bash
# For virtual CAN (testing):
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0

# For physical CAN:
sudo ip link set can0 type can bitrate 250000
sudo ip link set up can0

# Verify interface exists:
ip link show | grep can
```

### Issue: Man Pages Not Found

**Symptom**:
```bash
$ man rust-can-util
No manual entry for rust-can-util
```

**Solution**:
```bash
# Update man database
sudo mandb

# Try again
man rust-can-util
```

### Issue: Completions Not Working

**Symptom**:
Tab completion doesn't work after installation.

**Solution**:

**Bash**:
```bash
# Restart terminal or source completion
source /etc/bash_completion
```

**Zsh**:
```bash
# Add to ~/.zshrc
fpath=(/usr/share/zsh/site-functions $fpath)
autoload -Uz compinit && compinit

# Then restart terminal
```

**Fish**:
```bash
# Restart fish shell
exec fish
```

---

## 📚 Additional Documentation

### Man Pages

All tools have comprehensive man pages:
```bash
man rust-can-util      # CAN message encoder/decoder
man candump-rs         # CAN frame dumper
man cansend-rs         # CAN frame sender
man monitor-can        # Real-time CAN monitor
man dump-messages      # Message inspection
man can-log-analyzer   # Log file analysis
man count-hvpc-signals # Signal counter
man emp-simulator      # EMP device simulator
man hvpc-simulator     # HVPC device simulator
man udc-simulator      # UDC device simulator
man j1939-simulator    # J1939 device simulator
man cando-webui      # Web monitoring interface
man cando-codegen    # DBC code generator
```

### Online Documentation

- **GitHub Repository**: https://github.com/suykerbuyk/cando-rs
- **Issue Tracker**: https://github.com/suykerbuyk/cando-rs/issues
- **Discussions**: https://github.com/suykerbuyk/cando-rs/discussions

### Getting Help

```bash
# Command-line help
<tool-name> --help

# Examples:
rust-can-util --help
candump-rs --help
cando-webui --help
```

---

## 🔒 Security Notes

### Package Integrity

Always verify checksums before installation:
```bash
sha256sum -c SHA256SUMS
```

### Permissions

Cando-RS binaries:
- Do NOT require root privileges for most operations
- May require root for CAN interface configuration
- Never run simulators or tools as root unless necessary

### Network Security

`cando-webui`:
- Binds to localhost (127.0.0.1) by default
- Use `--bind 0.0.0.0` carefully (exposes to network)
- No authentication by default - use firewall rules
- Recommended: Use behind nginx/apache reverse proxy

---

## 📊 Package Details

### Installed Files

**Binaries** (13 files in `/usr/bin/`):
- rust-can-util
- dump-messages
- monitor-can
- candump-rs
- cansend-rs
- can-log-analyzer
- count-hvpc-signals
- emp-simulator
- hvpc-simulator
- udc-simulator
- j1939-simulator
- cando-webui
- cando-codegen

**Man Pages** (13 files in `/usr/share/man/man1/`):
- All binaries have corresponding .1.gz man pages

**Shell Completions**:
- Bash: `/usr/share/bash-completion/completions/` (13 files)
- Zsh: `/usr/share/zsh/site-functions/` (13 files, _prefixed)
- Fish: `/usr/share/fish/vendor_completions.d/` (13 .fish files)

**Documentation**:
- `/usr/share/doc/cando-rs/README`
- `/usr/share/doc/cando-rs/CHANGELOG`
- `/usr/share/doc/cando-rs/copyright`

### Disk Space

- **Package size**: 8.9 MB
- **Installed size**: ~63 MB
- **Dependencies**: None (statically linked)

### List All Package Files

```bash
dpkg -L cando-rs
```

---

## 🆘 Support

### Report Issues

Found a bug? Please report it:
1. Check existing issues: https://github.com/suykerbuyk/cando-rs/issues
2. Create new issue with:
   - Operating system and version
   - Package version (`rust-can-util --version`)
   - Steps to reproduce
   - Error messages

### Feature Requests

Have an idea? Open a discussion:
https://github.com/suykerbuyk/cando-rs/discussions

### Community

- GitHub Discussions for questions and ideas
- Issue tracker for bugs and feature requests

---

## 📝 Changelog

### Version 0.1.0-1 (2024-11-07)
- Initial Debian package release
- 15 binaries included
- Full man page documentation
- Shell completions for bash, zsh, fish
- Static linking (zero dependencies)

---

## 📄 License

Cando-RS is dual-licensed under MIT OR Apache-2.0.

See `/usr/share/doc/cando-rs/copyright` for details.

---

**Document Version**: 1.0  
**Last Updated**: 2024-11-07  
**Package Version**: 0.1.0-1  
**Status**: Production Release