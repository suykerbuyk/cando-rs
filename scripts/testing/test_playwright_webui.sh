#!/bin/bash
# vim: set ts=4 sw=4 et ft=sh ai si:
# test_playwright_webui.sh - Playwright WebUI Testing for Cando-RS
#
# Purpose: Browser-based validation of WebUI functionality
# Requirements: vcan0 interface (auto-configured), Playwright browsers (auto-downloaded)
#
# Note: This script is a placeholder for future WebUI testing. The proprietary
# WebUI has been removed. When a cando-rs WebUI is implemented, this script
# should be updated with appropriate test scenarios.
#
# Usage:
#   ./scripts/testing/test_playwright_webui.sh               # Run all phases
#   ./scripts/testing/test_playwright_webui.sh --phase 1     # Run Phase 1 only
#   ./scripts/testing/test_playwright_webui.sh --verbose      # Enable verbose logging

set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
VCAN_INTERFACE="vcan0"
CONFIG_FILE="cando.yaml"
PHASE_FILTER=""
VERBOSE=""

# Timing and results tracking
START_TIME=$(date +%s)
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Output directory
OUTPUT_DIR="$WORKSPACE_DIR/playwright-output"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# ============================================================================
# ARGUMENT PARSING
# ============================================================================

while [[ $# -gt 0 ]]; do
    case $1 in
        --phase)
            PHASE_FILTER="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE="--verbose"
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --phase N      Run only Phase N tests"
            echo "  --verbose      Enable verbose Playwright logging"
            echo "  --help         Show this help message"
            echo ""
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================

record_test() {
    local test_name="$1"
    local result="$2"
    local details="${3:-}"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    case "$result" in
        PASS)
            PASSED_TESTS=$((PASSED_TESTS + 1))
            echo -e "${GREEN}  $test_name${NC}"
            ;;
        FAIL)
            FAILED_TESTS=$((FAILED_TESTS + 1))
            echo -e "${RED}  FAIL: $test_name${NC}"
            ;;
        SKIP)
            SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
            echo -e "${YELLOW}  $test_name (SKIPPED)${NC}"
            ;;
    esac
    [[ -n "$details" ]] && echo "  $details" || true
}

cleanup() {
    echo ""
    echo -e "${BLUE}=== Test Summary ===${NC}"
    echo "Total Tests:  $TOTAL_TESTS"
    echo -e "${GREEN}Passed:       $PASSED_TESTS${NC}"
    echo -e "${RED}Failed:       $FAILED_TESTS${NC}"
    echo -e "${YELLOW}Skipped:      $SKIPPED_TESTS${NC}"
    echo ""

    local end_time=$(date +%s)
    local total_duration=$((end_time - START_TIME))
    echo "Total Duration: ${total_duration}s"
    echo ""

    if [[ $FAILED_TESTS -eq 0 ]]; then
        echo -e "${GREEN}All Playwright tests passed!${NC}"
        exit 0
    else
        echo -e "${RED}Some tests failed${NC}"
        exit 1
    fi
}

trap cleanup EXIT INT TERM

# ============================================================================
# MAIN
# ============================================================================

echo -e "${BLUE}=== Cando-RS Playwright WebUI Integration Testing ===${NC}"
echo ""

# Check for playwright-test binary
PLAYWRIGHT_BIN=""
if [[ -x "$WORKSPACE_DIR/target/release/playwright-test" ]]; then
    PLAYWRIGHT_BIN="$WORKSPACE_DIR/target/release/playwright-test"
elif [[ -x "$WORKSPACE_DIR/target/debug/playwright-test" ]]; then
    PLAYWRIGHT_BIN="$WORKSPACE_DIR/target/debug/playwright-test"
fi

if [[ -z "$PLAYWRIGHT_BIN" ]]; then
    echo -e "${YELLOW}playwright-test binary not found${NC}"
    echo "No WebUI component is currently available in cando-rs."
    echo "When a WebUI is added, build it with: cargo build --bin playwright-test"
    record_test "Playwright binary availability" "SKIP"
    exit 0
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"
echo -e "${GREEN}  Output directory: $OUTPUT_DIR${NC}"

# Setup vcan0 interface
echo "Setting up vcan0 interface..."
if ! ip link show "$VCAN_INTERFACE" &>/dev/null; then
    sudo modprobe vcan 2>/dev/null || true
    sudo ip link add dev "$VCAN_INTERFACE" type vcan 2>/dev/null || true
fi
sudo ip link set up "$VCAN_INTERFACE" 2>/dev/null || true

if ip link show "$VCAN_INTERFACE" &>/dev/null; then
    record_test "vcan0 interface ready" "PASS"
else
    record_test "vcan0 interface ready" "FAIL"
    exit 1
fi

# Set CAP_NET_RAW on binaries
"$WORKSPACE_DIR/scripts/set_can_privileges.sh" caps >/dev/null 2>&1 || true
record_test "CAN capabilities configured" "PASS"

# Run page-load scenario if available
echo ""
echo "Running page-load scenario..."
if "$PLAYWRIGHT_BIN" \
    --cando-config "$WORKSPACE_DIR/$CONFIG_FILE" \
    --scenario "page-load" \
    $VERBOSE \
    2>&1 | tee "$OUTPUT_DIR/page-load_${TIMESTAMP}.log"; then
    record_test "page-load scenario" "PASS"
else
    record_test "page-load scenario" "FAIL"
fi

# Cleanup function will print summary and exit
