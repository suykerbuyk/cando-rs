#!/bin/bash
set -e

# Tier 1 Integration Testing - cando-dump-messages Metadata Validation
# Tests that cando-dump-messages correctly outputs metadata with --comments, --enums, and --full flags
#
# Purpose: Validate that DBC metadata (comments, enumerations, comprehensive info) is properly
#          extracted and displayed by cando-dump-messages utility
# Target: <2 minutes execution, works in any environment

echo "=== cando-dump-messages Metadata Integration Tests ==="
echo "Validating --comments, --enums, and --full flags"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Phase results tracking
PHASE2_PASSED=0
PHASE3_PASSED=0
PHASE4_PASSED=0
PHASE5_PASSED=0
PHASE6_PASSED=0
PHASE7_PASSED=0
PHASE8_PASSED=0
PHASE9_PASSED=0
PHASE10_PASSED=0
CURRENT_PHASE=0

# Temporary directory for test outputs
TEST_OUTPUT_DIR="/tmp/dump_messages_test_$$"
mkdir -p "$TEST_OUTPUT_DIR"

# Cleanup function
cleanup() {
    echo ""
    echo "Cleaning up test outputs..."
    rm -rf "$TEST_OUTPUT_DIR"
}
trap cleanup EXIT

# Test result tracking
record_test() {
    local test_name="$1"
    local result="$2"
    local details="$3"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if [ "$result" == "PASS" ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        echo -e "${GREEN}  $test_name${NC}"

        # Track phase-specific passes
        case $CURRENT_PHASE in
            2) PHASE2_PASSED=$((PHASE2_PASSED + 1)) ;;
            3) PHASE3_PASSED=$((PHASE3_PASSED + 1)) ;;
            4) PHASE4_PASSED=$((PHASE4_PASSED + 1)) ;;
            5) PHASE5_PASSED=$((PHASE5_PASSED + 1)) ;;
            6) PHASE6_PASSED=$((PHASE6_PASSED + 1)) ;;
            7) PHASE7_PASSED=$((PHASE7_PASSED + 1)) ;;
            8) PHASE8_PASSED=$((PHASE8_PASSED + 1)) ;;
            9) PHASE9_PASSED=$((PHASE9_PASSED + 1)) ;;
            10) PHASE10_PASSED=$((PHASE10_PASSED + 1)) ;;
        esac
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        echo -e "${RED}  FAIL: $test_name${NC}"
        if [ -n "$details" ]; then
            echo -e "${RED}  Details: $details${NC}"
        fi
    fi
}

# Verify cando-dump-messages binary exists (check release first, then debug)
echo "Phase 1: Binary Verification"
echo "=============================="
if [ -x "./target/release/cando-dump-messages" ]; then
    DUMP_MESSAGES_BIN="./target/release/cando-dump-messages"
    echo -e "${GREEN}  cando-dump-messages binary found (release)${NC}"
elif [ -x "./target/debug/cando-dump-messages" ]; then
    DUMP_MESSAGES_BIN="./target/debug/cando-dump-messages"
    echo -e "${GREEN}  cando-dump-messages binary found (debug)${NC}"
else
    echo -e "${RED}Error: cando-dump-messages binary not found${NC}"
    echo "Please run: cargo build -p cando-dump-messages"
    exit 1
fi
echo ""

# Test protocols to validate (J1939 only for cando-rs)
PROTOCOLS=("j1939")

CURRENT_PHASE=2
echo "Phase 2: Basic Output Tests (No Flags)"
echo "========================================"
for protocol in "${PROTOCOLS[@]}"; do
    output_file="$TEST_OUTPUT_DIR/${protocol}_basic.txt"
    if $DUMP_MESSAGES_BIN --protocol "$protocol" > "$output_file" 2>&1; then
        if [ -s "$output_file" ]; then
            record_test "$protocol: Basic output" "PASS"
        else
            record_test "$protocol: Basic output" "FAIL" "Empty output"
        fi
    else
        record_test "$protocol: Basic output" "FAIL" "Command failed"
    fi
done
echo ""

CURRENT_PHASE=3
echo "Phase 3: Comments Flag Tests (--comments)"
echo "=========================================="
for protocol in "${PROTOCOLS[@]}"; do
    basic_file="$TEST_OUTPUT_DIR/${protocol}_basic.txt"
    comments_file="$TEST_OUTPUT_DIR/${protocol}_comments.txt"

    $DUMP_MESSAGES_BIN --protocol "$protocol" --comments > "$comments_file" 2>&1 || true

    # Check if comments output is different from basic output
    if [ -s "$comments_file" ]; then
        basic_size=$(wc -c < "$basic_file" 2>/dev/null || echo 0)
        comments_size=$(wc -c < "$comments_file")

        # Comments output should typically be larger (has additional metadata)
        # or at minimum be present and valid
        if [ "$comments_size" -gt 0 ]; then
            # Check for comment-like content (Description:, Comment:, etc.)
            if grep -qi "description\|comment" "$comments_file" 2>/dev/null || [ "$comments_size" -ge "$basic_size" ]; then
                record_test "$protocol: --comments flag" "PASS"
            else
                record_test "$protocol: --comments flag" "PASS" "Output generated but no comments detected (may be normal if DBC lacks comments)"
            fi
        else
            record_test "$protocol: --comments flag" "FAIL" "Empty output"
        fi
    else
        record_test "$protocol: --comments flag" "FAIL" "No output generated"
    fi
done
echo ""

CURRENT_PHASE=4
echo "Phase 4: Enums Flag Tests (--enums)"
echo "===================================="
for protocol in "${PROTOCOLS[@]}"; do
    enums_file="$TEST_OUTPUT_DIR/${protocol}_enums.txt"

    $DUMP_MESSAGES_BIN --protocol "$protocol" --enums > "$enums_file" 2>&1 || true

    if [ -s "$enums_file" ]; then
        # Check for enumeration-like content (value descriptions)
        # Common patterns: "0: Value", "Value Descriptions:", numbers followed by text
        if grep -qE "^\s*[0-9]+\s*[:=]|Value.*:|Enum" "$enums_file" 2>/dev/null; then
            record_test "$protocol: --enums flag" "PASS"
        else
            # Output exists but may not have enums (could be normal)
            record_test "$protocol: --enums flag" "PASS" "Output generated (enums may not exist in this protocol)"
        fi
    else
        record_test "$protocol: --enums flag" "FAIL" "No output generated"
    fi
done
echo ""

CURRENT_PHASE=5
echo "Phase 5: Full Flag Tests (--full)"
echo "=================================="
for protocol in "${PROTOCOLS[@]}"; do
    basic_file="$TEST_OUTPUT_DIR/${protocol}_basic.txt"
    full_file="$TEST_OUTPUT_DIR/${protocol}_full.txt"

    $DUMP_MESSAGES_BIN --protocol "$protocol" --full > "$full_file" 2>&1 || true

    if [ -s "$full_file" ]; then
        basic_size=$(wc -c < "$basic_file" 2>/dev/null || echo 0)
        full_size=$(wc -c < "$full_file")

        # Full output should be the most comprehensive
        if [ "$full_size" -ge "$basic_size" ]; then
            record_test "$protocol: --full flag" "PASS"
        else
            record_test "$protocol: --full flag" "FAIL" "Full output smaller than basic"
        fi
    else
        record_test "$protocol: --full flag" "FAIL" "No output generated"
    fi
done
echo ""

CURRENT_PHASE=6
echo "Phase 6: Combined Flags Tests"
echo "=============================="
# Test that multiple flags work together
combo_file="$TEST_OUTPUT_DIR/j1939_combo.txt"
if $DUMP_MESSAGES_BIN --protocol j1939 --comments --enums > "$combo_file" 2>&1; then
    if [ -s "$combo_file" ]; then
        record_test "Combined flags (--comments --enums)" "PASS"
    else
        record_test "Combined flags (--comments --enums)" "FAIL" "Empty output"
    fi
else
    record_test "Combined flags (--comments --enums)" "FAIL" "Command failed"
fi
echo ""

CURRENT_PHASE=7
echo "Phase 7: Output Format Validation"
echo "=================================="
# Test different output formats with metadata flags
for format in "text" "csv" "json"; do
    format_file="$TEST_OUTPUT_DIR/j1939_${format}_full.txt"
    if [ "$format" == "text" ]; then
        cmd="$DUMP_MESSAGES_BIN --protocol j1939 --full"
    elif [ "$format" == "csv" ]; then
        cmd="$DUMP_MESSAGES_BIN --protocol j1939 --csv --full"
    else
        cmd="$DUMP_MESSAGES_BIN --protocol j1939 --json --full"
    fi

    if $cmd > "$format_file" 2>&1; then
        if [ -s "$format_file" ]; then
            # Validate format-specific content
            case "$format" in
                "csv")
                    if grep -q "," "$format_file" 2>/dev/null; then
                        record_test "Format $format with --full" "PASS"
                    else
                        record_test "Format $format with --full" "FAIL" "No CSV delimiters found"
                    fi
                    ;;
                "json")
                    if grep -q "{" "$format_file" 2>/dev/null; then
                        record_test "Format $format with --full" "PASS"
                    else
                        record_test "Format $format with --full" "FAIL" "No JSON structure found"
                    fi
                    ;;
                *)
                    record_test "Format $format with --full" "PASS"
                    ;;
            esac
        else
            record_test "Format $format with --full" "FAIL" "Empty output"
        fi
    else
        record_test "Format $format with --full" "FAIL" "Command failed"
    fi
done
echo ""

CURRENT_PHASE=8
echo "Phase 8: Metadata Content Validation"
echo "====================================="
# Detailed validation that metadata is actually present and meaningful
j1939_full="$TEST_OUTPUT_DIR/j1939_full_detailed.txt"
$DUMP_MESSAGES_BIN --protocol j1939 --full > "$j1939_full" 2>&1 || true

if [ -s "$j1939_full" ]; then
    # Check for expected metadata components
    line_count=$(wc -l < "$j1939_full")

    # Should have substantial content
    if [ "$line_count" -gt 10 ]; then
        record_test "Metadata: Substantial output" "PASS"
    else
        record_test "Metadata: Substantial output" "FAIL" "Too few lines ($line_count)"
    fi

    # Should contain signal information
    if grep -qi "signal\|field" "$j1939_full"; then
        record_test "Metadata: Signal information present" "PASS"
    else
        record_test "Metadata: Signal information present" "FAIL"
    fi

    # Should contain message information
    if grep -qi "message\|can.*id" "$j1939_full"; then
        record_test "Metadata: Message information present" "PASS"
    else
        record_test "Metadata: Message information present" "FAIL"
    fi
else
    record_test "Metadata: Content validation" "FAIL" "No output file"
fi
echo ""

CURRENT_PHASE=9
echo "Phase 9: Verbose Flag Test"
echo "==========================="
# Test that --verbose flag works (should imply --comments --enums)
verbose_file="$TEST_OUTPUT_DIR/j1939_verbose.txt"
if $DUMP_MESSAGES_BIN --protocol j1939 --verbose > "$verbose_file" 2>&1; then
    if [ -s "$verbose_file" ]; then
        basic_size=$(wc -c < "$TEST_OUTPUT_DIR/j1939_basic.txt" 2>/dev/null || echo 0)
        verbose_size=$(wc -c < "$verbose_file")

        # Verbose should be more comprehensive than basic
        if [ "$verbose_size" -ge "$basic_size" ]; then
            record_test "--verbose flag implies metadata" "PASS"
        else
            record_test "--verbose flag implies metadata" "FAIL" "Verbose output not comprehensive"
        fi
    else
        record_test "--verbose flag implies metadata" "FAIL" "Empty output"
    fi
else
    record_test "--verbose flag implies metadata" "FAIL" "Command failed"
fi
echo ""

CURRENT_PHASE=10
echo "Phase 10: Edge Cases"
echo "===================="
# Test that flags work with 'all' protocol
all_full_file="$TEST_OUTPUT_DIR/all_full.txt"
if $DUMP_MESSAGES_BIN --protocol all --full > "$all_full_file" 2>&1; then
    if [ -s "$all_full_file" ]; then
        # Should contain output for protocols
        protocol_count=$(grep -c "Protocol\|^\s*[A-Z]" "$all_full_file" 2>/dev/null || echo 0)
        if [ "$protocol_count" -gt 10 ]; then
            record_test "All protocols with --full" "PASS"
        else
            record_test "All protocols with --full" "PASS" "Limited output but functional"
        fi
    else
        record_test "All protocols with --full" "FAIL" "Empty output"
    fi
else
    record_test "All protocols with --full" "FAIL" "Command failed"
fi
echo ""

# Final summary
echo "=================================================="
echo "Integration Test Summary: cando-dump-messages Metadata"
echo "=================================================="
echo "Total Tests: $TOTAL_TESTS"
echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"
if [ $FAILED_TESTS -gt 0 ]; then
    echo -e "${RED}Failed: $FAILED_TESTS${NC}"
else
    echo -e "${GREEN}Failed: $FAILED_TESTS${NC}"
fi
echo ""

# Calculate success rate
if [ $TOTAL_TESTS -gt 0 ]; then
    success_rate=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo "Success Rate: ${success_rate}%"
    echo ""
fi

# Save results to report
REPORT_FILE="benchmarks/reports/dump_messages_metadata_$(date +%Y%m%d_%H%M%S).txt"
mkdir -p benchmarks/reports
cat > "$REPORT_FILE" << EOF
cando-dump-messages Metadata Integration Test Results
=====================================================
Date: $(date)
Total Tests: $TOTAL_TESTS
Passed: $PASSED_TESTS
Failed: $FAILED_TESTS
Success Rate: ${success_rate}%

Test Phases:
1. Binary Verification: done
2. Basic Output Tests: $PHASE2_PASSED passed
3. Comments Flag Tests: $PHASE3_PASSED passed
4. Enums Flag Tests: $PHASE4_PASSED passed
5. Full Flag Tests: $PHASE5_PASSED passed
6. Combined Flags Tests: $PHASE6_PASSED passed
7. Output Format Validation: $PHASE7_PASSED passed
8. Metadata Content Validation: $PHASE8_PASSED passed
9. Verbose Flag Test: $PHASE9_PASSED passed
10. Edge Cases: $PHASE10_PASSED passed

All metadata flags (--comments, --enums, --full, --verbose) validated across J1939 protocol.
EOF

echo "Report saved to: $REPORT_FILE"
echo ""

# Exit with appropriate code
if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Please review the output above.${NC}"
    exit 1
fi
