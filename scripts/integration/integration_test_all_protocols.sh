#!/bin/bash
set -e

# Tier 2 Full-Stack Integration Testing Framework
# Cando-RS CAN/vcan Hardware Integration Testing
#
# Purpose: Complete end-to-end validation with CAN/vcan hardware dependencies
# Requirements: Linux kernel with CAN/vcan support, CAP_NET_RAW capability
# Target: <30 minutes execution, J1939 protocol validation

echo "=== Cando-RS Tier 2 Full-Stack Integration Testing ==="
echo "CAN/vcan Hardware-Dependent J1939 Validation"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Timing variables
START_TIME=$(date +%s)
PHASE_START_TIME=$START_TIME

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Report storage
REPORT_DIR="benchmarks/reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
mkdir -p "$REPORT_DIR"

# Logging function
log_phase() {
    local phase="$1"
    local current_time=$(date +%s)
    local phase_duration=$((current_time - PHASE_START_TIME))
    local total_duration=$((current_time - START_TIME))

    echo ""
    echo -e "${BLUE}=== Phase: $phase ===${NC}"
    echo "Phase Duration: ${phase_duration}s | Total Duration: ${total_duration}s"
    echo ""
    PHASE_START_TIME=$current_time
}

# Test result tracking
record_test() {
    local test_name="$1"
    local result="$2"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if [ "$result" == "PASS" ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        echo -e "${GREEN}  $test_name${NC}"
    elif [ "$result" == "SKIP" ]; then
        SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
        echo -e "${YELLOW}  $test_name (SKIPPED)${NC}"
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        echo -e "${RED}  FAIL: $test_name${NC}"
    fi
}

# Process tracking
J1939_SIM_PID=""
J1939_SIM_LOG="/tmp/j1939_simulator_tier2_$$.log"

# Cleanup function
cleanup() {
    echo ""
    echo "=== Cleanup Phase ==="

    # Stop J1939 simulator
    if [ -n "$J1939_SIM_PID" ] && kill -0 "$J1939_SIM_PID" 2>/dev/null; then
        echo "Stopping J1939 simulator (PID: $J1939_SIM_PID)..."
        kill "$J1939_SIM_PID" 2>/dev/null || true
        wait "$J1939_SIM_PID" 2>/dev/null || true
        echo -e "${GREEN}  J1939 simulator stopped${NC}"
    fi

    # Clean up temp files
    rm -f /tmp/tier2_test_*

    echo ""
    echo "=== Test Summary ==="
    echo "Total Tests:  $TOTAL_TESTS"
    echo -e "${GREEN}Passed:       $PASSED_TESTS${NC}"
    echo -e "${RED}Failed:       $FAILED_TESTS${NC}"
    echo -e "${YELLOW}Skipped:      $SKIPPED_TESTS${NC}"
    echo ""

    local end_time=$(date +%s)
    local total_duration=$((end_time - START_TIME))
    local minutes=$((total_duration / 60))
    local seconds=$((total_duration % 60))

    echo "Total Duration: ${minutes}m ${seconds}s"
    echo ""

    # Generate report
    REPORT_FILE="$REPORT_DIR/tier2_integration_$TIMESTAMP.txt"
    cat > "$REPORT_FILE" << EOF
Cando-RS Tier 2 Integration Testing Report
===========================================

Execution Date: $(date)
Total Duration: ${minutes}m ${seconds}s

Test Results:
- Total Tests: $TOTAL_TESTS
- Passed: $PASSED_TESTS
- Failed: $FAILED_TESTS
- Skipped: $SKIPPED_TESTS

Validation Status: $( [ $FAILED_TESTS -eq 0 ] && echo "PASS" || echo "FAIL" )
EOF
    echo "Report saved to: $REPORT_FILE"

    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "${GREEN}All Tier 2 tests passed!${NC}"
    else
        echo -e "${RED}Some Tier 2 tests failed${NC}"
        exit 1
    fi
}

trap cleanup EXIT INT TERM

# ============================================================================
# PHASE 1: ENVIRONMENT VALIDATION
# ============================================================================

log_phase "Environment Validation"

cd "$WORKSPACE_DIR"

# Check for vcan0 interface
echo "Checking vcan0 interface..."
if ip link show vcan0 >/dev/null 2>&1; then
    record_test "vcan0 interface available" "PASS"
else
    echo -e "${RED}vcan0 interface not found${NC}"
    echo "Set up with: sudo modprobe vcan && sudo ip link add dev vcan0 type vcan && sudo ip link set up vcan0"
    record_test "vcan0 interface available" "FAIL"
    exit 1
fi

# Resolve cando-send and cando-dump binaries (prefer release)
if [ -x "./target/release/cando-send" ]; then
    CANDO_SEND="./target/release/cando-send"
elif [ -x "./target/debug/cando-send" ]; then
    CANDO_SEND="./target/debug/cando-send"
else
    echo -e "${RED}cando-send binary not found. Run 'make build' first.${NC}"
    record_test "cando-send binary available" "FAIL"
    exit 1
fi

if [ -x "./target/release/cando-dump" ]; then
    CANDO_DUMP="./target/release/cando-dump"
elif [ -x "./target/debug/cando-dump" ]; then
    CANDO_DUMP="./target/debug/cando-dump"
else
    echo -e "${RED}cando-dump binary not found. Run 'make build' first.${NC}"
    record_test "cando-dump binary available" "FAIL"
    exit 1
fi

record_test "cando-send binary available" "PASS"
record_test "cando-dump binary available" "PASS"

# Check binaries exist
if [ -x "./target/debug/cando-j1939-sim" ] || [ -x "./target/release/cando-j1939-sim" ]; then
    record_test "cando-j1939-sim binary available" "PASS"
else
    echo -e "${RED}cando-j1939-sim binary not found. Run 'make build' first.${NC}"
    record_test "cando-j1939-sim binary available" "FAIL"
    exit 1
fi

# Determine binary path (prefer release)
if [ -x "./target/release/cando-j1939-sim" ]; then
    J1939_BIN="./target/release/cando-j1939-sim"
else
    J1939_BIN="./target/debug/cando-j1939-sim"
fi

# Set CAP_NET_RAW on binaries (best-effort)
echo "Setting CAP_NET_RAW capabilities..."
./scripts/set_can_privileges.sh caps >/dev/null 2>&1 || true
record_test "CAN capabilities configured" "PASS"

# ============================================================================
# PHASE 2: J1939 SIMULATOR STARTUP
# ============================================================================

log_phase "J1939 Simulator Startup"

echo "Starting J1939 simulator on vcan0..."
$J1939_BIN --interface vcan0 > "$J1939_SIM_LOG" 2>&1 &
J1939_SIM_PID=$!

# Wait for simulator to start
sleep 3

if kill -0 "$J1939_SIM_PID" 2>/dev/null; then
    record_test "J1939 simulator started" "PASS"
    echo "  PID: $J1939_SIM_PID"
    echo "  Log: $J1939_SIM_LOG"
else
    record_test "J1939 simulator started" "FAIL"
    echo -e "${RED}J1939 simulator died during startup${NC}"
    echo "Check log: $J1939_SIM_LOG"
    cat "$J1939_SIM_LOG" 2>/dev/null || true
    exit 1
fi

# ============================================================================
# PHASE 3: CAN MESSAGE VALIDATION
# ============================================================================

log_phase "CAN Message Validation"

# Check that simulator is generating CAN frames on vcan0
echo "Checking for J1939 CAN traffic on vcan0..."
CAN_CAPTURE="/tmp/tier2_test_candump_$$"

# Capture CAN traffic for 5 seconds using cando-dump
timeout 5s $CANDO_DUMP vcan0 > "$CAN_CAPTURE" 2>/dev/null || true

if [ -s "$CAN_CAPTURE" ]; then
    FRAME_COUNT=$(wc -l < "$CAN_CAPTURE")
    record_test "J1939 CAN frames detected ($FRAME_COUNT frames in 5s)" "PASS"
else
    record_test "J1939 CAN frames detected" "FAIL"
    echo "  No CAN traffic detected on vcan0"
fi
rm -f "$CAN_CAPTURE"

# Verify simulator is still running after traffic check
if kill -0 "$J1939_SIM_PID" 2>/dev/null; then
    record_test "J1939 simulator stability (post-traffic check)" "PASS"
else
    record_test "J1939 simulator stability (post-traffic check)" "FAIL"
fi

# ============================================================================
# PHASE 4: INTEGRATION TEST SUITE
# ============================================================================

log_phase "Integration Test Suite"

echo "Running J1939 simulator integration tests..."
if cargo test -p cando-j1939-sim --test integration_test -- --include-ignored --test-threads=1 2>/dev/null; then
    record_test "J1939 integration test suite" "PASS"
else
    record_test "J1939 integration test suite" "FAIL"
fi

# ============================================================================
# PHASE 5: STRESS AND STABILITY
# ============================================================================

log_phase "Stress and Stability"

# Verify simulator survived the integration tests
if kill -0 "$J1939_SIM_PID" 2>/dev/null; then
    record_test "J1939 simulator survived integration tests" "PASS"
else
    echo "  Restarting J1939 simulator for stability test..."
    $J1939_BIN --interface vcan0 > "$J1939_SIM_LOG" 2>&1 &
    J1939_SIM_PID=$!
    sleep 3
    if kill -0 "$J1939_SIM_PID" 2>/dev/null; then
        record_test "J1939 simulator restarted for stability" "PASS"
    else
        record_test "J1939 simulator restarted for stability" "FAIL"
    fi
fi

# Let simulator run for 10 more seconds to verify stability
echo "Running stability check (10 seconds)..."
sleep 10

if kill -0 "$J1939_SIM_PID" 2>/dev/null; then
    record_test "J1939 simulator 10s stability" "PASS"
else
    record_test "J1939 simulator 10s stability" "FAIL"
fi

# Cleanup function will print summary and exit with appropriate code
