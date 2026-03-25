#!/bin/bash
# Man page generator for Cando-RS workspace
# This script automatically discovers all binaries and generates man pages

#set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="${SCRIPT_DIR}/../man"
TEMP_RUST_FILE=$(mktemp)

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Help function
show_help() {
    cat <<EOF
Usage: $0 [OPTIONS]

Generate man pages for all Cando-RS binaries.

OPTIONS:
    --output-dir DIR    Output directory for man pages [default: ./man]
    --list-only         List discovered binaries without generating man pages
    --clean             Remove existing man pages before generating
    --help              Show this help message

EXAMPLES:
    $0                          # Generate man pages in ./man
    $0 --output-dir /tmp/man   # Generate in custom directory
    $0 --list-only             # Just list discovered binaries
    $0 --clean                 # Clean and regenerate

REQUIREMENTS:
    - Rust and Cargo installed
    - clap_mangen dependency available with 'manpages' feature
EOF
}

# Parse command line arguments
CLEAN_FIRST=false
LIST_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
    --output-dir)
        OUTPUT_DIR="$2"
        shift 2
        ;;
    --list-only)
        LIST_ONLY=true
        shift
        ;;
    --clean)
        CLEAN_FIRST=true
        shift
        ;;
    --help | -h)
        show_help
        exit 0
        ;;
    *)
        log_error "Unknown option: $1"
        show_help
        exit 1
        ;;
    esac
done

# Check if we're in the project directory
if [[ ! -f "$SCRIPT_DIR/../Cargo.toml" ]]; then
    log_error "Not in cando-rs workspace directory"
    log_error "Please run this script from the project root"
    exit 1
fi

cd "$SCRIPT_DIR/.."

# List of binaries
BINARIES=("cando-cfg" "cando-codegen" "cando-dump" "cando-dump-messages" "cando-j1939-sim" "cando-log-analyzer" "cando-monitor" "cando-send" "cando-util" "cando-ws-query")

log_info "Found ${#BINARIES[@]} binaries: ${BINARIES[*]}"

if [[ "$LIST_ONLY" = true ]]; then
    echo "Discovered binaries:"
    for binary in "${BINARIES[@]}"; do
        echo "  $binary"
    done
    exit 0
fi

# Clean existing man pages if requested
if [[ "$CLEAN_FIRST" = true ]] && [[ -d "$OUTPUT_DIR" ]]; then
    log_info "Cleaning existing man pages in $OUTPUT_DIR"
    rm -rf "$OUTPUT_DIR"
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"
log_info "Generating man pages in: $OUTPUT_DIR"

# Generate man pages for each binary
GENERATED_COUNT=0
for binary in "${BINARIES[@]}"; do
    log_info "Generating man page for: $binary"
    filename="${binary//-/_}.1"
    if cargo run --bin "$binary" --features manpages -- --generate-manpage >"$OUTPUT_DIR/$filename" 2>/dev/null; then
        log_success "Generated: $filename"
        ((GENERATED_COUNT++))
    else
        log_warning "Failed to generate man page for: $binary"
    fi
done

# Summary
if [[ $GENERATED_COUNT -gt 0 ]]; then
    echo
    log_success "Successfully generated $GENERATED_COUNT man pages in $OUTPUT_DIR"
    echo
    log_info "To install the man pages:"
    echo "  ./install-man.sh --user     # Install for current user"
    echo "  sudo ./install-man.sh       # Install system-wide"
    echo
    log_info "To test a man page:"
    echo "  man -l $OUTPUT_DIR/cando_util.1"
else
    log_error "No man pages were generated successfully"
    exit 1
fi
