# SocketCAN parameters and configuration guide

## Initial setup for configured Linux kernels
### Configuring the `can0` Interface for Cando-RS

Configuring the `can0` interface in Linux (via SocketCAN) is essential for cando-rs utilities like `monitor_can`, `emp_simulator`, or `hvpc_simulator` to interact with physical CAN hardware (e.g., a USB-to-CAN adapter like Peak PCAN or Kvaser Leaf). This assumes you're on a Debian/Ubuntu-like distro (common for Rust dev setups); adjust for others (e.g., Fedora uses `nmcli`).

The process involves:

- Loading kernel modules for your hardware.
- Setting up the interface with `ip` commands.
- Making it persistent across reboots.
- Ensuring unprivileged access (building on our previous `set_can_privileges.sh` script).

#### Step 1: Install SocketCAN Tools and Dependencies

If not already installed (check with `ip link`):

```bash
sudo apt update
sudo apt install can-utils iproute2 libcap2-bin  # can-utils for candump/cansend; libcap2-bin for setcap
```

- `can-utils` provides testing tools like `candump can0` for verification.

#### Step 2: Load Kernel Module for Your Hardware

Identify your CAN adapter (e.g., via `lsusb` for USB devices):

- **Peak PCAN-USB**: `sudo modprobe pcan` (then `dmesg | grep pcan` to confirm detection).
- **Kvaser Leaf**: `sudo modprobe kvascarcom` (or `can4linux` for older).
- **Generic SocketCAN-compatible**: Often auto-loaded, but check `lsmod | grep can`.

Make it persistent: Add to `/etc/modules`:

```bash
echo "pcan" | sudo tee -a /etc/modules  # Replace 'pcan' with your module
sudo depmod -a
```

Reboot or `sudo modprobe <module>` to load.

#### Step 3: Configure and Bring Up the `can0` Interface

Set the bitrate (common values: 125k, 250k, 500k, 1M—match your bus; e.g., 500000 for automotive):

```bash
# Set CAN type, bitrate, and optional params (e.g., loopback for testing, no error frames)
sudo ip link set can0 type can bitrate 500000 loopback off restart-ms 100

# Bring it up
sudo ip link set can0 up

# Verify
ip link show can0  # Should show state UP
ifconfig can0      # Or use ip addr show can0
```

- **Troubleshooting**:
  - "No such device": Hardware not detected—check `dmesg | grep can`.
  - Bitrate mismatch: Leads to silent errors; use an oscilloscope or `candump` to confirm.
  - For multi-interface setups (e.g., can0, can1): Repeat for each.

Test basic connectivity (loopback mode echoes sent frames):

```bash
# Enable loopback if testing solo
sudo ip link set can0 type can bitrate 500000 loopback on

# Send a test frame (ID 0x123, data 0xDEADBEEF)
cansend can0 123#DEADBEEF

# Monitor (in another terminal)
candump can0
```

You should see the echoed frame. Disable loopback for real bus use.

#### Step 4: Make Configuration Persistent Across Reboots
##### For systems that use ifupdown:

Use `/etc/network/interfaces` (traditional; works with ifupdown):

```bash
# Edit with sudo nano /etc/network/interfaces
```

Add:

```
auto can0
iface can0 inet manual
    pre-up ip link set can0 type can bitrate 500000
    up ip link set can0 up
    down ip link set can0 down
```

Apply: `sudo ifdown can0 && sudo ifup can0` (or reboot).

##### For systems that use systemd-networkd:
Create `/etc/systemd/network/20-can0.network`:

```
[Match]
Name=can0

[Link]
RequiredForOnline=no

[CAN]
BitRate=500000
Loopback=false
RestartSec=100ms
```

Then:

```bash
sudo systemctl restart systemd-networkd
sudo systemctl enable systemd-networkd
```

#### Step 5: Grant Unprivileged Access to Cando binaries:

As root, we ensure binaries have `CAP_NET_RAW`:

```bash
cd /home/johns/code/cando-rs
cargo build --workspace --release
./scripts/set_can_privileges.sh caps  # Applies to all 12 binaries
```

For each binary created in the workspace target directory, the script applies this setcap flag:
```
CAPABILITY="cap_net_raw+eip"
sudo setcap "$CAPABILITY" "$bin_path"
```

Now your can run the workspace tools without `sudo`:

```bash
# Example: Monitor live traffic on can0 with EMP DBC
./target/release/monitor_can can0 --emp-dbc dbc/EMP.dbc --output json

# Simulate EMP device sending to can0
./target/release/emp_simulator --device-type fan can0 --device-id 0x8A
```

- If errors: Check `getcap ./target/release/monitor_can` (should show `cap_net_raw+eip`).

For hardware-specific groups (e.g., serial access): Add user to `dialout`:

```bash
sudo usermod -aG dialout $USER  # Log out/in
```

### Zed Editor Integration

If using the Zed editor, you can automate this in a custom task. Edit `~/.config/zed/tasks.json` (or project-level `.zed/tasks.json`):

```json
{
  "label": "Setup can0",
  "command": "sh",
  "args": [
    "-c",
    "sudo ip link set can0 type can bitrate 500000 && sudo ip link set can0 up && echo 'can0 ready!'"
  ],
  "group": "build",
  "is_continuing": true // Continues even if sudo prompts
}
```

- Run via Cmd/Ctrl+Shift+P > Tasks: Setup can0.
- For full persistence, add a pre-build hook in Zed's `settings.json` (Cmd/Ctrl+,):
  ```json
  {
    "lsp": {
      "rust-analyzer": {
        "checkOnSave": {
          "command": "check"
        }
      }
    },
    "tasks": [
      {
        "label": "Build & CAN Setup",
        "commands": [
          "cargo build --release --workspace",
          "./scripts/set_can_privileges.sh caps",
          "sh -c 'sudo ip link set can0 up || true'"
        ]
      }
    ]
  }
  ```
  This ties into your workspace—hit Cmd+Shift+P > Tasks: Build & CAN Setup after hardware changes.
### Creating CAN udev group rules for named user access.

The following function is a feature of the `cando-rs/scripts/set_can_privileges.sh udev`
```
UDEV_RULE="/etc/udev/rules.d/99-cando-can.rules"
GROUP="can-users"
sudo groupadd -f "$GROUP"         # Create group if needed
sudo usermod -aG "$GROUP" "$USER" # Add current user
echo "KERNEL==\"can*\", SUBSYSTEM==\"net\", ACTION==\"add\", GROUP=\"$GROUP\", MODE=\"0660\"" | sudo tee "$UDEV_RULE"
echo "KERNEL==\"vcan*\", SUBSYSTEM==\"net\", ACTION==\"add\", GROUP=\"$GROUP\", MODE=\"0660\"" | sudo tee -a "$UDEV_RULE"
sudo udevadm control --reload-rules
sudo udevadm trigger
echo "✓ Udev rules applied. Log out/in for group changes. Restart hardware for effect."
```
Any unpriviliged user who is a member of the 'can-users' group should be able configure CAN interfaces without the use of sudo once this rule has been deployed.
### Verification with Cando-RS

- Decode a test message: Use `cansend can0 18FF0010#1122334455667788` (J1939-style), then run `monitor_can`.
- Logs: Check `dmesg | tail` for errors.
- Advanced: For error handling, add `restart` or `triple-sampling` in `ip link` (see `man ip-link`).

If your hardware is specific (e.g., not Peak), share `lsusb` output or model for tailored steps. This should get `can0` humming with cando-rs—let me know how the first `candump` goes!

## SocketCAN Interface Control Parameters on Linux

SocketCAN, the Linux kernel's implementation for Controller Area Network (CAN) bus interfaces, supports a range of control parameters for configuring physical and virtual CAN devices (e.g., via `ip link` commands or systemd-networkd `.network` files). These parameters handle bit-timing (for classical CAN and CAN-FD), operational modes, termination, recovery, and more. The parameters are set using Netlink attributes, and systemd-networkd maps them directly (e.g., `BitRate=` corresponds to `bitrate`, `TimeQuantaNSec=` to `tq` scaled in nanoseconds).

This list is comprehensive for modern Linux kernels (e.g., 6.x series as of 2025), drawn from the official kernel documentation. It includes all documented options for classical CAN, CAN-FD, and general controls. Parameters are shown in `ip link` syntax (e.g., `ip link set can0 type can bitrate 500000`), with equivalents for systemd-networkd where applicable. Always verify hardware/driver support with `ip -details link show can0`.

### Classical CAN Bit-Timing Parameters

These define the bit-time structure per the Bosch CAN 2.0 spec. If `CONFIG_CAN_CALC_BITTIMING=y` (default in most kernels), the kernel auto-calculates timings from `bitrate`; otherwise, manual parameters are required. The bit-time formula is: **1 bit = (1 + prop-seg + phase-seg1 + phase-seg2) × tq**, sampled at the end of phase-seg1.

| Parameter (ip link) | systemd-networkd Equivalent | Description                                                                                                   | Example (ip link)    | Range/Notes                                                                     |
| ------------------- | --------------------------- | ------------------------------------------------------------------------------------------------------------- | -------------------- | ------------------------------------------------------------------------------- |
| `bitrate`           | `BitRate=`                  | Desired nominal bitrate in bits/sec. Triggers auto-calculation of other timings if supported.                 | `bitrate 500000`     | 10k–1M typical; auto-fallback to manual if invalid.                             |
| `tq`                | `TimeQuantaNSec=`           | Basic time quantum in nanoseconds (or raw tq count in ip link). Defines the resolution for segments.          | `tq 20`              | 8–25 tq typical; scaled to ns in systemd (e.g., 500 for 50 ns at 20 MHz clock). |
| `prop-seg`          | `PropagationSegment=`       | Propagation segment: Accounts for signal propagation delay on the bus.                                        | `prop-seg 8`         | 1–8; part of tseg1 (prop-seg + phase-seg1).                                     |
| `phase-seg1`        | `PhaseBufferSegment1=`      | Phase segment 1: First buffer for phase correction and sampling point.                                        | `phase-seg1 5`       | 1–8; influences sample point.                                                   |
| `phase-seg2`        | `PhaseBufferSegment2=`      | Phase segment 2: Second buffer for resynchronization.                                                         | `phase-seg2 3`       | 1–8; typically ≤ phase-seg1.                                                    |
| `sjw`               | `SyncJumpWidth=`            | Synchronization Jump Width: Max adjustment for oscillator drift.                                              | `sjw 4`              | 1–4; ≤ phase-seg1.                                                              |
| `sample-point`      | `SamplePoint=`              | Sample point position as a percentage/fraction (75–90% ideal for noise immunity). Overrides calculated value. | `sample-point 87.5%` | 50–95%; default ~87.5%.                                                         |

### CAN-FD (Flexible Data-Rate) Parameters

CAN-FD enables higher data-phase speeds (up to 8–15 Mbps) while keeping arbitration classical. Requires `fd on`. Data-phase timings follow similar structure but are independent.

| Parameter (ip link) | systemd-networkd Equivalent | Description                                                                 | Example (ip link)   | Range/Notes                                        |
| ------------------- | --------------------------- | --------------------------------------------------------------------------- | ------------------- | -------------------------------------------------- |
| `fd on/off`         | `FDMode=`                   | Enables/disables CAN-FD mode. Changes MTU to 72 bytes for 64-byte payloads. | `fd on`             | Requires hardware support (e.g., mttcan).          |
| `fd-non-iso on/off` | `FDNonISOMode=`             | Non-ISO mode (per 2012 CAN-FD spec; ignores ISO 11898-1:2015 rules).        | `fd-non-iso on`     | Optional; shown as `FD-NON-ISO` in `ip link show`. |
| `dbitrate`          | `DataBitRate=`              | Nominal data-phase bitrate (≥ arbitration bitrate).                         | `dbitrate 2000000`  | 1–15M; auto-calculates if supported.               |
| `dtq`               | `DataTimeQuantaNSec=`       | Data-phase time quantum.                                                    | `dtq 10`            | Similar to `tq`; ns in systemd.                    |
| `dprop-seg`         | `DataPropagationSegment=`   | Data-phase propagation segment.                                             | `dprop-seg 6`       | 1–32; part of dtseg1.                              |
| `dphase-seg1`       | `DataPhaseBufferSegment1=`  | Data-phase phase segment 1.                                                 | `dphase-seg1 5`     | 1–32.                                              |
| `dphase-seg2`       | `DataPhaseBufferSegment2=`  | Data-phase phase segment 2.                                                 | `dphase-seg2 2`     | 1–8.                                               |
| `dsjw`              | `DataSyncJumpWidth=`        | Data-phase synchronization jump width.                                      | `dsjw 1`            | 1–4.                                               |
| `dsample-point`     | `DataSamplePoint=`          | Data-phase sample point (percentage).                                       | `dsample-point 80%` | 50–95%; default ~80%.                              |

### Operational and Hardware Control Parameters

These manage runtime behavior, error handling, and physical layer features.

| Parameter (ip link)       | systemd-networkd Equivalent | Description                                                                                                           | Example (ip link)    | Range/Notes                                          |     |                                           |
| ------------------------- | --------------------------- | --------------------------------------------------------------------------------------------------------------------- | -------------------- | ---------------------------------------------------- | --- | ----------------------------------------- |
| `termination <value>`     | `Termination=`              | Enables/disables termination resistor (e.g., 120Ω for bus-end nodes). Query supported values with `ip -details link`. | `termination 120`    | 0 (off), 120 (on); driver-specific (e.g., via GPIO). |     |                                           |
| `restart-ms <ms>`         | `RestartSec=`               | Auto-restart delay after bus-off (error counter overflow). 0 disables. Generates error frame on restart.              | `restart-ms 5000`    | 0–∞ ms; systemd uses seconds (e.g., 5s).             |     |                                           |
| `restart`                 | N/A (manual via ip)         | Manually restarts the controller (clears bus-off).                                                                    | `restart`            | One-shot; use for immediate recovery.                |     |                                           |
| `listen-only on/off`      | `ListenOnly=`               | Receive-only mode: Ignores ACKs, no transmission. Useful for bus monitoring.                                          | `listen-only on`     | Default off; no error frames generated.              |     |                                           |
| `loopback on/off`         | `Loopback=`                 | Loops transmitted frames back to local receive queue (for testing without bus).                                       | `loopback on`        | Default off; ignores remote ACKs.                    |     |                                           |
| `triple-sampling on/off`  | `TripleSampling=`           | Enables three samples per bit (middle vote) for noise reduction.                                                      | `triple-sampling on` | Hardware-dependent; default off.                     |     |                                           |
| `one-shot on/off`         | `OneShot=`                  | Transmitter retries failed sends indefinitely (off) or once (on).                                                     | `one-shot on`        | Default off; for error-passive handling.             |     |                                           |
| `berr-reporting on/off`   | `BusErrorReporting=`        | Enables/disables bus error reporting frames.                                                                          | `berr-reporting on`  | Default on; off silences errors.                     |     |                                           |
| `fd frames {error-warning | error-passive               | bus-off}`                                                                                                             | N/A                  | Filters FD error frames (advanced; kernel-internal). | N/A | Rarely used; see kernel docs for details. |

### Transmitter Delay Compensation (TDC) Parameters

For high-speed CAN-FD (>5 Mbps) to compensate loop delays. Requires controller support.

| Parameter (ip link)            | systemd-networkd Equivalent | Description                                                          | Example (ip link) | Range/Notes                     |
| ------------------------------ | --------------------------- | -------------------------------------------------------------------- | ----------------- | ------------------------------- |
| `tdc-mode {off\|auto\|manual}` | `TDCMode=`                  | TDC mode: off (disabled), auto (kernel measures), manual (user-set). | `tdc-mode auto`   | Requires `TDC-*` kernel config. |
| `tdco <value>`                 | `TDCOffset=`                | TDC offset in time quanta (delay calibration).                       | `tdco 20`         | 0–31; mandatory in manual/auto. |
| `tdcv <value>`                 | `TDCValue=`                 | Measured TDC value (auto-filled in auto mode).                       | `tdcv 10`         | Read-only in auto; 0–127.       |
| `tdcf <value>`                 | `TDCFilter=`                | TDC filter window (if supported).                                    | `tdcf 5`          | Optional; 0–127.                |

### Interface State and Virtual CAN (VCAN) Notes

- `up` / `down`: Activates/deactivates the link. Bit-timing must be set before `up`. systemd-networkd handles this implicitly.
- For VCAN (virtual): No bit-timing needed; create with `ip link add dev vcan0 type vcan`.
- Driver-Specific Constants: View limits with `ip -details link show <dev>` (e.g., `clock 8000000 tseg1 1..16` for SJA1000). Includes `brp` (baud prescaler), `brp-inc`.

### Usage Notes

- **Configuration Order**: Set timings/modes before `up`. Example full command: `ip link set can0 down type can bitrate 500000 fd on dbitrate 2000000 termination 120 restart-ms 2000 up`.
- **Systemd-Networkd Mapping**: Uses `[CAN]` section; values like `SamplePoint=87.5%` directly translate. See `man systemd.network` for syntax.
- **Verification**: Use `ip -details -statistics link show can0` for stats (e.g., errors, bitrate).
- **Kernel Support**: All parameters require SocketCAN (CONFIG_CAN=y) and drivers (e.g., mttcan for FD). For custom clocks, manual timings are essential.

## Persistent SocketCAN Interface Configurations

### Adding Linux kernel support for CAN and SocketCAN (WSL)

This should not be necessary for any current 6.x Linux kernel from a upstream distro, however, adding in CAN support to a WSL linux kernel shows how it is done.

This will show how to create a Virtual CAN environment on a Windows machine. Socket-can doesn’t work on windows machines and we can’t access the “ **canutils** ” toolkit on Windows machines. This can be frustrating for developers using the Windows operating system or you may need to use Microsoft applications in your company. It will also be perfect for those who do not have real CAN hardware like PCAN & Vector and want to develop. To overcome this situation, we can utilize WSL (Windows Subsystem for Linux). This is not our main topic, but to briefly talk about the WSL a compatibility layer provided by Microsoft to run Linux binary executables natively on Windows machines. It’s a completely virtual machine and you can access the Linux terminal with one click. There are tons of resources online for WSL installation so I assumed that you have installed WSL2 at this point.

At this point, you might think that everything is OK, but unfortunately, WSL does not support CAN networking by default, so we cannot create a virtual CAN interface at first. To avoid this we need to run our custom kernel in WSL2.

First of all, we need to update our environment with this command

```c
sudo apt-get update -y
```

Then we need to clone the WSL project on Github to our local.

```c
git clone https://github.com/microsoft/WSL2-Linux-Kernel
```

When you cloned the project change your directory into it. Then run the command below.

> Note: If you already have WSL2, use **“uname -r”** to find out the kernel version and clone the matching kernel version. In my case the kernel version is **“5.15.146”**

```c
cat /proc/config.gz | gunzip > .config
```

This command retrieves the kernel configuration information from the compressed `/proc/config.gz` file, decompresses it, and saves it to a file named `.config`. According to this configuration file, we will build our custom kernel. You can use **“cat.config”** to check the operation.

Then run the command.

```c
make prepare modules_prepare
```

So, when you run `make prepare modules_prepare`, you're instructing the build system to first prepare the kernel source tree in general (`make prepare`), and then specifically prepare it for building external modules (`modules_prepare`). This ensures that the kernel source is in the appropriate state for building both the kernel itself and any external modules that might be needed.

Once this is done, open the menu config interface with the

```c
make menuconfig
```

When you run this command you are supposed to see this screen.

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*v7wA8CTcunHgKvLBw405YA.png)

Kernel Configuration Display

In this interface, we need to enable the device drivers required for CAN and virtual CAN. To do this go **“Networking Support”** option and press Enter.

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*xy4P5WOcvJDUIzFOXMkGSw.png)

By default “CAN BUS subsystem support” will be empty. Navigate to the line and press M. Once you have done this press Enter to see device drivers.

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*uCar6GejuL5lHfnY7its2w.png)

In this section, select CAN-TP or J-1939 protocols if you need to use them in your project. My recommendation is to choose these protocols because you will need them eventually. But as I said, it is not a mandatory step. Then navigate to the **“CAN Device Drivers”** tab.

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*FQ9W5gkMAuxJx1d23ZFuwg.png)

Here, scroll to the lines marked in the screenshot above and press the “M” key. If you will be working with interfaces such as PCAN or Vector, it is useful to select them here as well. When you select the relative device drivers save and exit.

So, we have modified our “.config” configuration file. To build our customized kernel run these commands

```c
make -j4
sudo make module_install
```

This process will take a while because it compiles the Linux source files. Patiently wait for all files to compile.

After the compile process is finished, copy the built custom kernel to a location you can access. Also, this kernel should be in a Windows environment as WSL will use it.

```c
cp vmlinux /mnt/c/
```

The above command will copy our custom kernel to the top directory.

After that, we need to give our custom kernel path to WSL2. To do that you can use the following command.

```c
cat >> /mnt/c/.wslconfig << "ENDL"
[wsl2]
kernel=C:\\vmlinux
ENDL
```

When you have done, restart the WSL and wait a little bit.

```c
wsl --shutdown

wsl --list -v #Make sure everything stopped correctly
```

Now we will manually add the modules we built to WSL. We can use the following commands for this.

```c
sudo modprobe can
sudo modprobe can-raw
sudo modprobe vcan
```

If at this point you get such an error as “ `modprobe: ERROR: could not insert 'vcan': Invalid argument` ”, something is missing in the previous steps. You need to repeat the steps and correct this problem. If the terminal does not give any output when you execute these commands, you have successfully activated the modules or you can use the **“lsmod”** command.

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*UZ6XaP8fvaIxdHuiIJP3yA.png)

After doing this, create a virtual CAN interface called `**vcan0**` **with** these commands.

```c
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

You can see the interface created using the “ **ifconfig** ” command.

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*k29oW03_lsOfkuhOGQnSJA.png)

From this point, you can install and run the toolkit.

```c
sudo apt install can-utils
```

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*-nlW4YFZJhE5SxAudKPVXg.png)

### Setting up systemd network files

```
config_can_devs() {
    NETWORK_DIR="/etc/systemd/network"
    CANX_NETWORK_FILE="$NETWORK_DIR/50-all-can.network"
    # Generate the config file
    cat >"$CANX_NETWORK_FILE" <<-'END_OF_CANX'
    [Match]
    Name=can*

    [Link]
    # ConfigureWithoutCarrier=yes

    [Network]
    DHCP=no

    [CAN]
    BitRate=500000
    # Optional: Add Termination=yes if at bus end
END_OF_CANX
    VCAN_NETDEV_FILE="$NETWORK_DIR/50-vcan0.netdev"
    cat >"$VCAN_NETDEV_FILE" <<-'END_OF_VCAN0_NETDEV'
    [NetDev]
    Name=vcan0
    Kind=vcan
END_OF_VCAN0_NETDEV

    VCAN_NETDEV_FILE="$NETWORK_DIR/50-vcan1.netdev"
    cat >"$VCAN_NETDEV_FILE" <<-'END_OF_VCAN1_NETDEV'
    [NetDev]
    Name=vcan1
    Kind=vcan
END_OF_VCAN1_NETDEV
    VCAN_NETWORK_FILE="$NETWORK_DIR/50-vcan.network"
    cat >"$VCAN_NETWORK_FILE" <<-'VCAN_NETWORK_FILE'
    [Match]
    Name=vcan*

    [Network]
    DHCP=no
VCAN_NETWORK_FILE
    # Set permissions (readable by root, standard for systemd configs)
    chmod 644 "$CANX_NETWORK_FILE"
    chmod 644 "$VCAN_NETDEV_FILE"
    chmod 644 "$VCAN_NETWORK_FILE"

    echo "Generated CAN network configs at:"
    echo "  $CANX_NETWORK_FILE"
    echo "  $VCAN_NETDEV_FILE"
    echo "  $VCAN_NETWORK_FILE"
    echo "Reload systemd-networkd: sudo networkctl reload"
    echo "Bring up interfaces: sudo networkctl up can*"
}
```

## vcan loop back

modprobe can-gw

cangw -A -s vcan1 -d vcan0 -e
cangw -A -s vcan0 -d vcan1 -e

Test:
candump vcan1
cansend vcan0 123#DEADBEEFDEADBEEF