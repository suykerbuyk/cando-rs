#!/bin/bash
# Generate shell completions for all Cando-RS binaries
#
# This script generates bash, zsh, and fish completions for all binaries
# that support the --generate-completion flag.
#
# Usage:
#   ./scripts/packaging/generate-completions.sh [TARGET]
#
# Arguments:
#   TARGET - Optional Rust target triple (default: x86_64-unknown-linux-musl)
#
# Output:
#   target/completions/bash/
#   target/completions/zsh/
#   target/completions/fish/
#
# Requirements:
#   - Binaries must be built first
#   - Each binary should support clap-based completion generation

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
TARGET="${1:-x86_64-unknown-linux-musl}"
BUILD_DIR="target/${TARGET}/release"
OUTPUT_DIR="target/completions"

# If using native build (no target specified), adjust path
if [ "$TARGET" = "native" ]; then
    BUILD_DIR="target/release"
fi

echo -e "${BLUE}Cando-RS Shell Completion Generator${NC}"
echo -e "${BLUE}====================================${NC}"
echo ""
echo "Target: $TARGET"
echo "Build dir: $BUILD_DIR"
echo "Output dir: $OUTPUT_DIR"
echo ""

# Create output directories
mkdir -p "$OUTPUT_DIR"/{bash,zsh,fish}

# List of all binaries to generate completions for
# Format: "binary-name:package-name"
BINARIES=(
    # CLI Tools
    "cando-util:cando-util"
    "cando-dump-messages:cando-dump-messages"
    "cando-monitor:cando-monitor"
    "cando-dump:cando-dump"
    "cando-send:cando-send"
    "cando-log-analyzer:cando-log-analyzer"
    "cando-ws-query:cando-ws-query"

    # Simulators
    "cando-j1939-sim:cando-j1939-sim"

    # Configuration & Code Generation
    "cando-cfg:cando-cfg"
    "cando-codegen:cando-codegen"
)

# Statistics
total_binaries=${#BINARIES[@]}
generated=0
skipped=0
failed=0

# Function to check if binary exists
check_binary() {
    local binary_path="$1"
    if [ ! -f "$binary_path" ]; then
        echo -e "${RED}Binary not found: $binary_path${NC}"
        echo -e "  ${YELLOW}Run 'cargo build --workspace --release --target=$TARGET' first${NC}"
        return 1
    fi

    if [ ! -x "$binary_path" ]; then
        echo -e "${RED}Binary not executable: $binary_path${NC}"
        return 1
    fi

    return 0
}

# Function to generate completion for a single binary
generate_completion() {
    local binary_name="$1"
    local binary_path="$BUILD_DIR/$binary_name"

    echo -e "${BLUE}Processing:${NC} $binary_name"

    # Check if binary exists
    if ! check_binary "$binary_path"; then
        ((failed++))
        return 1
    fi

    # Try to generate completions using clap_complete
    # Method 1: Check if binary has --generate-completion flag (preferred)
    if "$binary_path" --help 2>&1 | grep -q "generate-completion"; then
        echo "  Using built-in --generate-completion"

        # Generate bash completion
        if "$binary_path" --generate-completion bash > "$OUTPUT_DIR/bash/$binary_name" 2>/dev/null; then
            echo -e "  ${GREEN}bash${NC}"
        else
            echo -e "  ${YELLOW}bash (failed)${NC}"
        fi

        # Generate zsh completion
        if "$binary_path" --generate-completion zsh > "$OUTPUT_DIR/zsh/_$binary_name" 2>/dev/null; then
            echo -e "  ${GREEN}zsh${NC}"
        else
            echo -e "  ${YELLOW}zsh (failed)${NC}"
        fi

        # Generate fish completion
        if "$binary_path" --generate-completion fish > "$OUTPUT_DIR/fish/$binary_name.fish" 2>/dev/null; then
            echo -e "  ${GREEN}fish${NC}"
        else
            echo -e "  ${YELLOW}fish (failed)${NC}"
        fi

        ((generated++))

    # Method 2: Check if binary uses clap (fallback - create placeholder)
    elif "$binary_path" --help &>/dev/null; then
        echo -e "  ${YELLOW}Binary doesn't support --generate-completion${NC}"
        echo "  Creating placeholder completions"

        # Create basic bash completion (command name only)
        cat > "$OUTPUT_DIR/bash/$binary_name" <<EOF
# bash completion for $binary_name
_${binary_name//-/_}() {
    local cur=\${COMP_WORDS[COMP_CWORD]}
    COMPREPLY=( \$(compgen -W "--help --version" -- "\$cur") )
}
complete -F _${binary_name//-/_} $binary_name
EOF
        echo -e "  ${GREEN}bash (basic)${NC}"

        # Create basic zsh completion
        cat > "$OUTPUT_DIR/zsh/_$binary_name" <<EOF
#compdef $binary_name
_$binary_name() {
    _arguments \\
        '--help[Show help information]' \\
        '--version[Show version information]'
}
_$binary_name "\$@"
EOF
        echo -e "  ${GREEN}zsh (basic)${NC}"

        # Create basic fish completion
        cat > "$OUTPUT_DIR/fish/$binary_name.fish" <<EOF
# fish completion for $binary_name
complete -c $binary_name -l help -d 'Show help information'
complete -c $binary_name -l version -d 'Show version information'
EOF
        echo -e "  ${GREEN}fish (basic)${NC}"

        ((skipped++))

    else
        echo -e "  ${RED}Binary doesn't respond to --help${NC}"
        ((failed++))
        return 1
    fi

    echo ""
}

# Main generation loop
echo -e "${BLUE}Generating completions for $total_binaries binaries...${NC}"
echo ""

for entry in "${BINARIES[@]}"; do
    binary_name="${entry%%:*}"
    generate_completion "$binary_name" || true
done

# Summary
echo -e "${BLUE}====================================${NC}"
echo -e "${BLUE}Summary:${NC}"
echo ""
echo -e "  Total binaries: $total_binaries"
echo -e "  ${GREEN}Generated (full):${NC} $generated"
echo -e "  ${YELLOW}Generated (basic):${NC} $skipped"
echo -e "  ${RED}Failed:${NC} $failed"
echo ""

if [ $failed -gt 0 ]; then
    echo -e "${YELLOW}Some completions failed to generate${NC}"
    echo -e "  This is usually because binaries haven't been built yet."
    echo -e "  Run: ${BLUE}cargo build --workspace --release --target=$TARGET${NC}"
    echo ""
    exit 1
fi

echo -e "${GREEN}Completion generation complete!${NC}"
echo ""
echo "Output locations:"
echo "  Bash: $OUTPUT_DIR/bash/"
echo "  Zsh:  $OUTPUT_DIR/zsh/"
echo "  Fish: $OUTPUT_DIR/fish/"
echo ""
echo "To test completions:"
echo "  # Bash"
echo "  source $OUTPUT_DIR/bash/cando-util"
echo ""
echo "  # Zsh (add to fpath)"
echo "  fpath=($PWD/$OUTPUT_DIR/zsh \$fpath)"
echo "  autoload -Uz compinit && compinit"
echo ""
echo "  # Fish"
echo "  set -gx fish_complete_path $PWD/$OUTPUT_DIR/fish \$fish_complete_path"
echo ""
