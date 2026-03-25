#!/bin/bash
# vim: set ts=4 sw=4 et ft=sh ai si:
# set_can_privileges.sh - Permanently grant CAP_NET_RAW to cando-rs binaries for unprivileged CAN access
# Usage: ./scripts/set_can_privileges.sh [setup|caps|udev|can|all]
# Requires: libcap2-bin (sudo apt install libcap2-bin)
# Run after cargo build --release

set -euo pipefail # Strict mode

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
TARGET_DIR="$WORKSPACE_DIR/target"

mapfile -d $'\0' BINARIES < <(find ${WORKSPACE_DIR}/target/*/ -maxdepth 1 -type f -executable -print0)
CAPABILITY="cap_net_raw+eip"

echo "Cando-RS CAN Privilege Setup Utility"
echo "Workspace: $WORKSPACE_DIR"

if [[ ! -d "${TARGET_DIR}" ]]; then
    echo "This script needs to run from the workspace/project root"
    exit 1
fi

check_networkd() {
    echo "Checking systemd-networkd status..."

    # Check if systemd-networkd is installed
    if ! systemctl list-unit-files systemd-networkd.service 2>/dev/null | grep -q systemd-networkd.service; then
        echo "systemd-networkd not installed"
        echo "Install: sudo apt install systemd-networkd"
        return 1
    fi

    # Check if enabled and running
    local needs_enable=false
    local needs_start=false

    if ! systemctl is-enabled systemd-networkd >/dev/null 2>&1; then
        needs_enable=true
    fi

    if ! systemctl is-active systemd-networkd >/dev/null 2>&1; then
        needs_start=true
    fi

    # Handle not enabled (also implies not running)
    if [[ "$needs_enable" == "true" ]]; then
        echo "systemd-networkd is installed but NOT enabled"
        echo ""
        echo "systemd-networkd is required to manage CAN and vcan interfaces."
        echo ""
        read -p "Would you like to enable and start systemd-networkd now? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "Running: sudo systemctl enable --now systemd-networkd"
            if sudo systemctl enable --now systemd-networkd; then
                echo "systemd-networkd enabled and started successfully"
            else
                echo "Failed to enable systemd-networkd"
                return 1
            fi
        else
            echo "Skipped. To enable manually, run:"
            echo "  sudo systemctl enable --now systemd-networkd"
            return 1
        fi
    # Handle enabled but not running
    elif [[ "$needs_start" == "true" ]]; then
        echo "systemd-networkd is enabled but NOT running"
        echo ""
        read -p "Would you like to start systemd-networkd now? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "Running: sudo systemctl start systemd-networkd"
            if sudo systemctl start systemd-networkd; then
                echo "systemd-networkd started successfully"
            else
                echo "Failed to start systemd-networkd"
                return 1
            fi
        else
            echo "Skipped. To start manually, run:"
            echo "  sudo systemctl start systemd-networkd"
            return 1
        fi
    fi

    echo "systemd-networkd is installed, enabled, and running"

    # Warn about NetworkManager conflict
    if systemctl is-active NetworkManager >/dev/null 2>&1; then
        echo ""
        echo "WARNING: NetworkManager is also running"
        echo "  Both NetworkManager and systemd-networkd are active."
        echo "  To prevent conflicts, ensure CAN interfaces are excluded from NetworkManager:"
        echo ""
        echo "  Create /etc/NetworkManager/conf.d/unmanaged-can.conf with:"
        echo "    [keyfile]"
        echo "    unmanaged-devices=interface-name:can*;interface-name:vcan*"
        echo ""
        echo "  Then reload NetworkManager:"
        echo "    sudo systemctl reload NetworkManager"
        echo ""
    fi

    return 0
}

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
    BitRate=250000
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

case "${1:-all}" in
"caps")
    echo "Applying $CAPABILITY to binaries in $TARGET_DIR..."
    if [[ ! -d "$TARGET_DIR" ]]; then
        echo "Error: Run 'cargo build --release --workspace' first!"
        exit 1
    fi
    for bin_path in "${BINARIES[@]}"; do
        bin="$(basename $bin_path)"
        build="$(basename $(dirname $bin_path))"
        if [[ -x "$bin_path" ]]; then
            sudo -n setcap "$CAPABILITY" "$bin_path" 2>/dev/null
            echo "Set $CAPABILITY on $build $bin"
        else
            echo "$build $bin not found in $bin_path (skipped)"
        fi
    done
    echo "Caps applied! Test with: ./target/release/cando-monitor vcan0"
    ;;
"udev")
    echo "Setting up udev rules for CAN devices (group: can-users)..."
    # Create udev rule for /sys/class/net/can* (hardware discovery/permissions)
    UDEV_RULE="/etc/udev/rules.d/99-cando-can.rules"
    GROUP="can-users"
    sudo groupadd -f "$GROUP"         # Create group if needed
    sudo usermod -aG "$GROUP" "$USER" # Add current user
    echo "KERNEL==\"can*\", SUBSYSTEM==\"net\", ACTION==\"add\", GROUP=\"$GROUP\", MODE=\"0660\"" | sudo tee "$UDEV_RULE"
    echo "KERNEL==\"vcan*\", SUBSYSTEM==\"net\", ACTION==\"add\", GROUP=\"$GROUP\", MODE=\"0660\"" | sudo tee -a "$UDEV_RULE"
    sudo udevadm control --reload-rules
    sudo udevadm trigger
    echo "Udev rules applied. Log out/in for group changes. Restart hardware for effect."
    ;;
"setup")
    echo "One-time vcan0 setup..."
    sudo modprobe vcan
    sudo ip link add dev vcan0 type vcan
    sudo ip link set up vcan0
    echo "vcan0 created. Add to /etc/network/interfaces for persistence."
    ;;
"can")
    echo "systemd-networkd for all can devices"
    echo ""
    if ! check_networkd; then
        echo ""
        echo "ERROR: systemd-networkd must be enabled and running to manage CAN interfaces."
        echo "Please enable and start systemd-networkd, then re-run this script."
        exit 1
    fi
    echo ""
    sudo bash -c "$(declare -f config_can_devs); config_can_devs"
    ;;
"all" | "")
    # Check networkd first before doing anything
    if ! check_networkd; then
        echo ""
        echo "ERROR: systemd-networkd must be enabled and running to manage CAN interfaces."
        echo "Please enable and start systemd-networkd, then re-run this script."
        exit 1
    fi
    "$0" can
    "$0" setup
    "$0" udev
    "$0" caps
    ;;
*)
    echo "Invalid mode: $1. Use: setup|caps|udev|can|all"
    exit 1
    ;;
esac

echo "Done! Re-run 'caps' after future builds."
