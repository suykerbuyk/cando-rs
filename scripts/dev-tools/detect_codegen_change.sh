#!/bin/bash
# Detect codegen algorithm changes
#
# This script wraps the cando-codegen detect-changes command to provide
# CI/CD friendly exit codes and actionable output.
#
# Exit Codes:
#   0 - All protocols clean (no changes)
#   1 - Algorithm evolution detected (breaking changes possible)
#   2 - Script error (tool not found, build failure, etc.)
#
# Usage:
#   ./scripts/dev-tools/detect_codegen_change.sh          # Check all protocols
#   ./scripts/dev-tools/detect_codegen_change.sh --strict # Exit 1 on any changes
#   ./scripts/dev-tools/detect_codegen_change.sh --ci     # CI-friendly output

set -euo pipefail

# Colors for output (disabled in CI)
if [ -t 1 ] && [ "${CI:-false}" != "true" ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    NC='\033[0m' # No Color
else
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    NC=''
fi

# Script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Configuration
STRICT_MODE=false
CI_MODE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --strict)
            STRICT_MODE=true
            shift
            ;;
        --ci)
            CI_MODE=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Detect codegen algorithm changes across all protocols."
            echo ""
            echo "Options:"
            echo "  --strict    Exit with error on any changes (not just algorithm evolution)"
            echo "  --ci        CI-friendly output (no colors, structured logging)"
            echo "  --help, -h  Show this help message"
            echo ""
            echo "Exit Codes:"
            echo "  0 - All protocols clean"
            echo "  1 - Algorithm evolution detected (or any changes in strict mode)"
            echo "  2 - Script error"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 2
            ;;
    esac
done

# Change to project root
cd "$PROJECT_ROOT"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}ERROR: cargo not found${NC}"
    echo "Please install Rust: https://rustup.rs/"
    exit 2
fi

# Check if cando-codegen binary exists or can be built
if [ "$CI_MODE" = true ]; then
    echo "::group::Building cando-codegen"
fi

if ! cargo build --bin cando-codegen &> /dev/null; then
    echo -e "${RED}ERROR: Failed to build cando-codegen${NC}"
    exit 2
fi

if [ "$CI_MODE" = true ]; then
    echo "::endgroup::"
fi

# Run the detection
if [ "$CI_MODE" = true ]; then
    echo "::group::Running codegen change detection"
fi

# Capture output and exit code
set +e
OUTPUT=$(cargo run --bin cando-codegen -- detect-changes 2>&1)
CODEGEN_EXIT=$?
set -e

# Display output
echo "$OUTPUT"

if [ "$CI_MODE" = true ]; then
    echo "::endgroup::"
fi

# Parse output for algorithm evolution
if echo "$OUTPUT" | grep -q "Algorithm Evolution: 0"; then
    ALGO_EVOLUTION=false
else
    ALGO_EVOLUTION=true
fi

# Check if any protocols have normal updates
if echo "$OUTPUT" | grep -q "Normal Updates: 0"; then
    NORMAL_UPDATES=false
else
    NORMAL_UPDATES=true
fi

# Determine exit code based on mode and results
EXIT_CODE=0

if [ $CODEGEN_EXIT -ne 0 ]; then
    # Tool failed
    EXIT_CODE=2
elif [ "$ALGO_EVOLUTION" = true ]; then
    # Algorithm evolution detected (breaking changes possible)
    echo ""
    echo -e "${YELLOW}ACTION REQUIRED: Algorithm evolution detected${NC}"
    echo ""
    echo "This means the codegen algorithm changed but DBC files did not."
    echo "Dependent code (test scripts) may be broken."
    echo ""
    echo "Next steps:"
    echo "  1. Review generated code changes"
    echo "  2. Update test scripts with new field names if needed"
    echo "  3. Commit changes together"
    echo ""

    if [ "$CI_MODE" = true ]; then
        echo "::error::Codegen algorithm evolution detected - validation required"
    fi

    EXIT_CODE=1
elif [ "$STRICT_MODE" = true ] && [ "$NORMAL_UPDATES" = true ]; then
    # Strict mode: any changes are an error
    echo ""
    echo -e "${YELLOW}Changes detected in strict mode${NC}"
    echo ""

    if [ "$CI_MODE" = true ]; then
        echo "::error::Codegen changes detected in strict mode"
    fi

    EXIT_CODE=1
else
    # All clean or normal updates
    if [ "$CI_MODE" = true ]; then
        echo "::notice::Codegen status check passed"
    fi
    EXIT_CODE=0
fi

# Exit with appropriate code
exit $EXIT_CODE
