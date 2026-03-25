#!/bin/bash

#############################################################################
# NVM Environment Setup Wrapper
#
# Purpose: Set up NVM environment and run a command
# Usage: ./scripts/run-with-nvm.sh <command> [args...]
#
# This wrapper ensures Node.js from NVM is available in the PATH
# before running Playwright and other Node-dependent tools.
#############################################################################

set -euo pipefail

# Colors for output
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Set up NVM environment
export NVM_DIR="${HOME}/.local/apps/nvm"

# Load NVM if it exists
if [[ -s "$NVM_DIR/nvm.sh" ]]; then
    # Source nvm.sh but suppress its output
    . "$NVM_DIR/nvm.sh" >/dev/null 2>&1 || true
else
    echo -e "${BLUE}NVM not found at $NVM_DIR${NC}"
    echo "   Proceeding without NVM (system Node.js will be used if available)"
fi

# Verify Node.js is available
if ! command -v node >/dev/null 2>&1; then
    echo "Node.js not found in PATH"
    echo "   Please install Node.js via NVM:"
    echo "   nvm install --lts"
    exit 1
fi

# Run the provided command with all its arguments
exec "$@"
