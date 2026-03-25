# Linux Capabilities for CAN Interface Access

**Feature**: CAP_NET_RAW Automatic Configuration  
**Version**: 0.1.0  
**Last Updated**: 2024-11-07  
**Applies To**: Debian/Ubuntu package installations

---

## 📋 Overview

Cando-RS Debian packages automatically configure Linux capabilities during installation, enabling unprivileged users to access CAN interfaces without using `sudo`. This feature uses the `CAP_NET_RAW` capability to grant minimal, specific permissions rather than requiring full root access.

---

## 🔐 What Are Linux Capabilities?

### Traditional Approach: Root or Nothing

Historically, Linux had two privilege levels:
- **Unprivileged** (regular users): Cannot access raw network interfaces
- **Root** (superuser): Can do anything

Problem: Many tools only need **one specific permission** but require full root access.

### Modern Approach: Fine-Grained Capabilities

Linux capabilities divide root privileges into distinct units. Instead of "all or nothing," programs can be granted only the specific capabilities they need.

**Example**:
- `CAP_NET_RAW` - Create raw network sockets (CAN, packet capture)
- `CAP_NET_ADMIN` - Configure network interfaces
- `CAP_SYS_TIME` - Set system clock

### How It Works

```bash
# Without capabilities - FAILS
$ candump-rs can0
Error: Permission denied (os error 13)

# Traditional workaround - WORKS but grants too much power
$ sudo candump-rs can0
[works, but running as root]

# With capabilities - WORKS with minimal permissions
$ setcap cap_net_raw+eip /usr/bin/candump-rs
$ candump-rs can0  # Now works as regular user!
[works, no sudo needed]
```

---

## 🎯 Why CAP_NET_RAW for CAN?

### CAN Protocol Requirements

The Controller Area Network (CAN) protocol requires:
1. **Raw socket access** - CAN uses AF_CAN sockets (Linux SocketCAN)
2. **Direct hardware interaction** - Bypasses normal network stack
3. **Low-level frame control** - Send/receive raw CAN frames

### Security Trade-offs

**Without Capabilities** (requiring sudo):
- ❌ Users must have sudo privileges
- ❌ Entire program runs as root
- ❌ Any vulnerability in program = root compromise
- ❌ Inconvenient for development/testing

**With CAP_NET_RAW** (capability-based):
- ✅ Users don't need sudo
- ✅ Only CAN access is privileged
- ✅ Rest of program runs as regular user
- ✅ Reduced attack surface
- ✅ Convenient and safe

### Comparison with Other Network Tools

Many network tools use the same approach:

| Tool       | Capability      | Purpose                     |
|------------|-----------------|----------------------------|
| **tcpdump** | CAP_NET_RAW     | Packet capture             |
| **ping**    | CAP_NET_RAW     | ICMP echo requests         |
| **nmap**    | CAP_NET_RAW     | Network scanning           |
| **candump** | CAP_NET_RAW     | CAN frame capture          |
| **cando-rs** | CAP_NET_RAW  | CAN monitoring/simulation  |

---

## 🔧 Cando-RS Implementation

### Automatic Configuration

When you install the Debian package:

```bash
sudo dpkg -i cando-rs_0.1.0-1_amd64.deb
```

The **postinst** script automatically:
1. Checks if `setcap` is available (from libcap2-bin package)
2. Sets `cap_net_raw+eip` on CAN-related binaries
3. Reports success/failure

Example output:
```
Setting up cando-rs (0.1.0-1) ...
Cando-RS: Configuring CAN capabilities...
✓ Set cap_net_raw+eip on 10 binaries
  Users can now access CAN interfaces without sudo
  Note: Virtual CAN (vcan) interfaces work without capabilities
```

### Which Binaries Get Capabilities?

**✅ Granted CAP_NET_RAW** (10 binaries):

| Binary            | Purpose                          | Needs CAN Access |
|-------------------|----------------------------------|------------------|
| candump-rs        | CAN frame capture                | ✅ Yes           |
| cansend-rs        | CAN frame transmission           | ✅ Yes           |
| monitor-can       | Real-time CAN monitoring         | ✅ Yes           |
| rust-can-util     | Message encoding/decoding        | ✅ Yes           |
| dump-messages     | Message inspection               | ✅ Yes           |
| emp-simulator     | EMP device simulation            | ✅ Yes           |
| hvpc-simulator    | HVPC device simulation           | ✅ Yes           |
| udc-simulator     | UDC device simulation            | ✅ Yes           |
| j1939-simulator   | J1939 device simulation          | ✅ Yes           |
| cando-webui     | Web-based monitoring interface   | ✅ Yes           |

**❌ No Capabilities** (3 binaries):

| Binary              | Purpose                       | Needs CAN Access |
|---------------------|-------------------------------|------------------|
| count-hvpc-signals  | Static DBC analysis           | ❌ No            |
| can-log-analyzer    | Offline log analysis          | ❌ No            |
| cando-codegen     | Code generation from DBC      | ❌ No            |

These tools don't interact with CAN interfaces, so they don't need elevated privileges.

### Capability Flags Explained

The capability string `cap_net_raw+eip` has three parts:

- **cap_net_raw** - The specific capability (raw network socket access)
- **e** - Effective (capability is active when program runs)
- **i** - Inheritable (child processes can inherit this capability)
- **p** - Permitted (capability is in the permitted set)

---

## 🔍 Verification

### Check If Capabilities Are Set

```bash
# Check single binary
getcap /usr/bin/candump-rs
# Expected: /usr/bin/candump-rs cap_net_raw=eip

# Check all CAN binaries
getcap /usr/bin/candump-rs \
       /usr/bin/cansend-rs \
       /usr/bin/monitor-can \
       /usr/bin/rust-can-util \
       /usr/bin/*-simulator \
       /usr/bin/cando-webui

# Check binaries that should NOT have capabilities
getcap /usr/bin/can-log-analyzer \
       /usr/bin/count-hvpc-signals \
       /usr/bin/cando-codegen
# Expected: No output (no capabilities)
```

### Test CAN Access Without Sudo

```bash
# Create virtual CAN interface (requires sudo once)
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0

# Test without sudo - should work!
candump-rs vcan0
# Press Ctrl+C to stop

# If it works, capabilities are properly configured
```

---

## 🛠️ Manual Management

### When Capabilities Are Lost

Capabilities can be lost when:
- **Filesystem doesn't support xattr** (FAT32, some NFS mounts)
- **Binary is modified** (capabilities removed on file change)
- **Binary is copied** (cp doesn't preserve capabilities by default)
- **Package is reinstalled without libcap2-bin**

### Manually Set Capabilities

If capabilities are lost, you can restore them manually:

```bash
# Single binary
sudo setcap cap_net_raw+eip /usr/bin/candump-rs

# All CAN binaries (from workspace root)
for binary in candump-rs cansend-rs monitor-can rust-can-util \
              dump-messages emp-simulator hvpc-simulator \
              udc-simulator j1939-simulator cando-webui; do
    sudo setcap cap_net_raw+eip /usr/bin/$binary
    echo "✓ Set capability on $binary"
done
```

### Remove Capabilities

If you want to require sudo again:

```bash
# Remove from single binary
sudo setcap -r /usr/bin/candump-rs

# Remove from all binaries
sudo setcap -r /usr/bin/candump-rs \
              /usr/bin/cansend-rs \
              /usr/bin/monitor-can \
              /usr/bin/rust-can-util \
              /usr/bin/*-simulator \
              /usr/bin/cando-webui
```

After removal, you'll need to use `sudo` to access CAN interfaces:
```bash
sudo candump-rs can0
```

### Preserving Capabilities When Copying

```bash
# Wrong - loses capabilities
cp /usr/bin/candump-rs /tmp/candump-rs

# Correct - preserves capabilities
cp --preserve=all /usr/bin/candump-rs /tmp/candump-rs

# Verify
getcap /tmp/candump-rs
```

---

## 🔒 Security Considerations

### Is This Safe?

**Yes, when used properly**. Here's why:

1. **Minimal Privilege**: Only grants raw socket access, not full root
2. **Industry Standard**: Same approach used by tcpdump, ping, wireshark
3. **Auditable**: Capabilities visible via `getcap`, logged by audit subsystem
4. **Revocable**: Can be removed without uninstalling package

### What Can CAP_NET_RAW Do?

**Allowed**:
- Create raw sockets (CAN, ICMP, packet capture)
- Send/receive raw network frames
- Bypass normal socket restrictions

**NOT Allowed**:
- Read/write arbitrary files
- Install software
- Modify system configuration
- Access other users' data
- Change network settings
- Execute arbitrary code as root

### Attack Surface Analysis

**Potential Risks**:
- Program with CAP_NET_RAW could sniff network traffic
- Malicious code could inject crafted CAN frames
- Buffer overflow in program could be exploited

**Mitigations in Cando-RS**:
- ✅ Rust memory safety (no buffer overflows)
- ✅ Static linking (no dependency vulnerabilities)
- ✅ Minimal attack surface
- ✅ Open source (auditable code)
- ✅ Limited to CAN interfaces (not general network)

### Comparison with sudo

| Aspect              | With sudo      | With Capabilities |
|---------------------|----------------|-------------------|
| Privilege Level     | Full root      | CAN access only   |
| Audit Trail         | sudo log       | Audit subsystem   |
| User Management     | sudoers file   | File permissions  |
| Vulnerability Impact| System-wide    | Limited scope     |
| Convenience         | Password prompt| No prompt needed  |

**Verdict**: Capabilities are safer than sudo for this use case.

---

## 🐛 Troubleshooting

### Issue 1: Capabilities Not Set During Install

**Symptom**:
```bash
$ getcap /usr/bin/candump-rs
# No output
```

**Cause**: libcap2-bin not installed, or filesystem doesn't support xattr

**Solution**:
```bash
# Install libcap2-bin
sudo apt update
sudo apt install libcap2-bin

# Reinstall package
sudo dpkg --purge cando-rs
sudo dpkg -i cando-rs_0.1.0-1_amd64.deb

# Manually set if still failing
sudo setcap cap_net_raw+eip /usr/bin/candump-rs
```

### Issue 2: "Operation not permitted" When Setting Capabilities

**Symptom**:
```bash
$ sudo setcap cap_net_raw+eip /usr/bin/candump-rs
Failed to set capabilities on file `/usr/bin/candump-rs' (Operation not permitted)
```

**Cause**: Filesystem doesn't support extended attributes (xattr)

**Check filesystem**:
```bash
df -T /usr/bin
# Filesystem should be ext4, xfs, btrfs (not FAT32, vfat, some NFS)
```

**Solutions**:
1. **Use sudo instead**: `sudo candump-rs can0`
2. **Copy to supported filesystem**: Install on ext4 partition
3. **Add user to can group**: See "Alternative: Group-Based Access" below

### Issue 3: Capabilities Work But Still Get Permission Denied

**Symptom**:
```bash
$ getcap /usr/bin/candump-rs
/usr/bin/candump-rs cap_net_raw=eip

$ candump-rs can0
Error: Permission denied (os error 13)
```

**Cause**: CAN interface permissions or kernel module not loaded

**Solution**:
```bash
# Check if interface exists
ip link show can0

# Check if SocketCAN module is loaded
lsmod | grep can

# Load module if needed
sudo modprobe can
sudo modprobe vcan  # For virtual CAN

# For virtual CAN, create interface
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

### Issue 4: Works on Development Machine But Not Production

**Cause**: Different filesystem, kernel options, or security modules (SELinux, AppArmor)

**Check**:
```bash
# Check if SELinux is enforcing
getenforce  # Should be "Permissive" or "Disabled"

# Check AppArmor status
sudo aa-status | grep candump-rs

# Check filesystem capabilities support
getfattr -d -m - /usr/bin/candump-rs
```

---

## 🔄 Alternative: Group-Based Access

If capabilities don't work on your system, use group-based permissions:

### Setup

```bash
# Create can-users group
sudo groupadd can-users

# Add your user to the group
sudo usermod -aG can-users $USER

# Create udev rule
sudo tee /etc/udev/rules.d/99-can.rules <<EOF
KERNEL=="can*", SUBSYSTEM=="net", ACTION=="add", GROUP="can-users", MODE="0660"
KERNEL=="vcan*", SUBSYSTEM=="net", ACTION=="add", GROUP="can-users", MODE="0660"
EOF

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Log out and back in for group changes to take effect
```

### Trade-offs

| Method        | Pros                          | Cons                              |
|---------------|-------------------------------|-----------------------------------|
| Capabilities  | Per-binary control, no logout | Filesystem support required       |
| Groups        | Works on all filesystems      | Requires logout, affects all CAN  |

---

## 📚 References

### Documentation
- **Linux capabilities(7)**: `man 7 capabilities`
- **setcap(8)**: `man 8 setcap`
- **getcap(8)**: `man 8 getcap`
- **Kernel Documentation**: https://www.kernel.org/doc/html/latest/security/capabilities.html

### Standards
- **POSIX.1e Capabilities**: Draft standard for fine-grained privileges
- **Linux Security Modules**: Framework for implementing access control

### Related Cando-RS Documentation
- `scripts/set_can_privileges.sh` - Development environment capability setup
- `doc/debian-packaging/INSTALL.md` - Installation guide
- `doc/debian-packaging/PACKAGING.md` - Package building guide

### Similar Implementations
- **tcpdump**: Uses CAP_NET_RAW for packet capture
- **Wireshark**: Uses CAP_NET_RAW via dumpcap helper
- **nmap**: Uses CAP_NET_RAW for network scanning
- **can-utils**: Traditional approach (requires sudo or setuid)

---

## 📝 Summary

**What**: Automatic CAP_NET_RAW capability configuration during package installation

**Why**: Enable CAN interface access without sudo, improving security and convenience

**How**: postinst script calls `setcap cap_net_raw+eip` on 10 CAN-related binaries

**When**: Applied during `dpkg -i`, can be manually managed anytime

**Who**: All users can access CAN interfaces after installation (no group/sudo needed)

**Security**: Safer than sudo (minimal privilege), industry-standard approach

---

**Last Updated**: 2024-11-07  
**Package Version**: 0.1.0-1  
**Applies To**: Debian/Ubuntu installations