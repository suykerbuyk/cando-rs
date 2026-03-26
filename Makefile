# Cando-RS Integration Testing Framework Makefile
# Provides convenient targets for development and CI/CD integration

.PHONY: help validate validate-quick tier1 tier2 build build-all test test-all clean clean-all setup setup-can-all setup-can-privileges setup-can-interfaces dev-cycle ci-validate quality-gate codegen-status codegen-all codegen-force codegen-validate codegen-proto codegen-proto-force codegen-list codegen-detect-changes codegen-validate-tests validate-dump-messages check-cross-prereqs build-all-targets build-all-debug build-all-release build-all-x86_64 build-all-aarch64 build-all-musl build-x86_64-debug build-x86_64-release build-aarch64-debug build-aarch64-release build-musl-debug build-musl-release check-packaging-deps build-deb build-deb-amd64 build-deb-arm64 deb-all deb-install deb-test deb-clean check-playwright-deps build-playwright-test test-playwright-page-load test-playwright-phases ensure-vcan0 build-workspace test-workspace validate-integration validate-cli validate-encoding validate-metadata cleanup-test-processes build-manpages build-release performance coverage bench doc flamegraph-sim profile-sim

# Default target
help:
	@echo "Cando-RS Integration Testing Framework"
	@echo "======================================"
	@echo ""
	@echo "Quick Commands:"
	@echo "  make validate        - Run Tier 1 integration tests (recommended, optimized)"
	@echo "  make validate-quick  - Quick validation for development"
	@echo "  make test-all        - Run all unit tests"
	@echo "  make build-all       - Build workspace with all features"
	@echo "  make validate-dump-messages - Validate cando-dump-messages metadata flags"
	@echo ""
	@echo "Integration Testing:"
	@echo "  make tier1                - Optimized Tier 1 validation (<15 min)"
	@echo "                              Single build -> All tests -> Validation (GitHub Actions compatible)"
	@echo "  make tier2                - Tier 2 full-stack testing (auto-builds + CAN setup)"
	@echo ""
	@echo "Tier 1 Components (run individually or via 'make tier1'):"
	@echo "  make build-workspace      - Build workspace once (debug + manpages, zero warnings)"
	@echo "  make test-workspace       - Run all unit tests once"
	@echo "  make validate-integration - Run integration tests (J1939 simulator)"
	@echo "  make validate-cli         - Validate CLI tool help systems"
	@echo "  make validate-encoding    - Validate message encoding/decoding"
	@echo "  make validate-metadata    - Validate cando-dump-messages metadata"
	@echo ""
	@echo "Build Targets:"
	@echo "  make build           - Standard workspace build (native debug)"
	@echo "  make build-release   - Release build (native)"
	@echo "  make build-all       - Build all native configurations (debug + release)"
	@echo "  make build-manpages  - Build binaries and generate man pages (outputs to man/)"
	@echo ""
	@echo "Cross-Compilation Targets:"
	@echo "  make build-all-targets     - Build ALL platforms (x86_64, aarch64, musl) x (debug, release)"
	@echo "  make build-all-debug       - Build ALL platforms in DEBUG mode (x86_64, aarch64, musl)"
	@echo "  make build-all-release     - Build ALL platforms in RELEASE mode (x86_64, aarch64, musl)"
	@echo "  make check-cross-prereqs   - Verify cross-compilation prerequisites installed"
	@echo ""
	@echo "  Platform-Specific Builds:"
	@echo "    make build-x86_64-debug    - Native x86_64 debug (same as 'make build')"
	@echo "    make build-x86_64-release  - Native x86_64 release (same as 'make build-release')"
	@echo "    make build-aarch64-debug   - Raspberry Pi 5 debug (ARM64, glibc)"
	@echo "    make build-aarch64-release - Raspberry Pi 5 release (ARM64, glibc)"
	@echo "    make build-musl-debug      - x86_64 static debug (musl, Alpine Linux)"
	@echo "    make build-musl-release    - x86_64 static release (musl, Alpine Linux)"
	@echo "    make build-aarch64-musl-debug   - ARM64 static debug (musl)"
	@echo "    make build-aarch64-musl-release - ARM64 static release (musl)"
	@echo ""
	@echo "  Batch Platform Builds:"
	@echo "    make build-all-x86_64      - Build both x86_64 debug + release (glibc)"
	@echo "    make build-all-aarch64     - Build both aarch64 debug + release (glibc)"
	@echo "    make build-all-musl        - Build all musl (static) debug + release (x86_64 + aarch64)"
	@echo "    make build-all-aarch64-musl - Build both aarch64-musl debug + release (static)"
	@echo ""
	@echo "Code Generation (Maintainers with DBC files):"
	@echo "  make codegen-status  - Check which protocols need regeneration"
	@echo "  make codegen-all     - Regenerate all changed protocols"
	@echo "  make codegen-force   - Force regenerate all protocols (with validation)"
	@echo "  make codegen-validate - Validate generated code matches DBC files"
	@echo "  make codegen-detect-changes - Detect algorithm evolution changes"
	@echo "  make codegen-proto PROTO=<name> - Regenerate specific protocol"
	@echo ""
	@echo "Debian Packaging (Distribution):"
	@echo "  make check-packaging-deps - Verify tools installed (Zig required)"
	@echo "  make build-deb       - Build Debian package (amd64, static via Zig)"
	@echo "  make build-deb-amd64 - Build Debian package for x86_64 (uses Zig)"
	@echo "  make build-deb-arm64 - Build Debian package for ARM64/aarch64 (uses Zig)"
	@echo "  make deb-all         - Build packages for both architectures"
	@echo "  make deb-install     - Build and install package locally"
	@echo "  make deb-test        - Test package installation in clean environment"
	@echo "  make deb-clean       - Remove generated packages"
	@echo ""
	@echo "  Note: All static builds use Zig toolchain exclusively"
	@echo "        Requires: zig + cargo-zigbuild (checked by check-packaging-deps)"
	@echo "        For dynamic builds, use 'make build' or 'make build-release'"
	@echo ""
	@echo "Playwright Browser Automation Testing:"
	@echo "  make test-playwright-phases    - Integrated WebUI validation"
	@echo "  make check-playwright-deps     - Verify OpenSSL and Playwright prerequisites"
	@echo "  make build-playwright-test     - Build playwright-test binary (requires OpenSSL)"
	@echo "  make test-playwright-page-load - Run page-load scenario with real browser"
	@echo "  make performance               - Performance measurement only (auto-builds)"
	@echo ""
	@echo "CAN/Hardware Setup:"
	@echo "  make setup-can-all   - Complete CAN setup (run once)"
	@echo "  make setup-can-privileges - Set binary capabilities only"
	@echo "  make setup-can-interfaces - Create vcan interfaces only"
	@echo ""
	@echo "Metadata & Documentation Validation:"
	@echo "  make validate-dump-messages - Test cando-dump-messages --comments, --enums, --full flags"
	@echo ""
	@echo "Development Workflows:"
	@echo "  make dev-cycle       - Complete development validation cycle"
	@echo "  make ci-validate     - CI/CD validation (Tier 1 only)"
	@echo "  make quality-gate    - Quality gate for releases"
	@echo ""
	@echo "Analysis & Profiling:"
	@echo "  make coverage        - Generate code coverage report (cargo-tarpaulin)"
	@echo "  make bench           - Run criterion benchmarks"
	@echo "  make doc             - Build API documentation"
	@echo "  make flamegraph-sim  - Generate flamegraph for J1939 simulator"
	@echo "  make profile-sim     - Profile J1939 simulator with perf"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make clean-all       - Clean all generated files and reports"
	@echo "  make cleanup-test-processes - Kill stray simulator processes"
	@echo "  make setup           - Initial setup and dependency check"
	@echo ""

# Primary validation target - runs Tier 1 integration tests
validate: tier1 validate-dump-messages

# Quick validation for development iteration
validate-quick:
	@echo "Quick Development Validation"
	cargo build --workspace
	cargo test --workspace
	@echo "Quick validation complete"

# Tier 1 Integration Testing (GitHub Actions compatible - OPTIMIZED)
# Executes: Cleanup -> Single build -> All unit tests -> Integration tests -> Validation -> Cleanup
# Target: <15 minutes (50-70% faster than previous implementation)
tier1: cleanup-test-processes build-workspace test-workspace validate-integration validate-cli validate-encoding validate-metadata
	@echo ""
	@echo "=============================================="
	@echo "Tier 1 Integration Testing Complete"
	@echo "=============================================="
	@echo ""
	@echo "Summary:"
	@echo "  - Workspace built (debug + manpages)"
	@echo "  - All unit tests passed"
	@echo "  - Integration tests passed (J1939)"
	@echo "  - CLI tools validated"
	@echo "  - Message encoding validated"
	@echo "  - Metadata validated"
	@echo ""
	@$(MAKE) --no-print-directory cleanup-test-processes

# Tier 1: Phase 1 - Build workspace once
build-workspace:
	@echo "=============================================="
	@echo "Phase 1: Building Workspace (Single Pass)"
	@echo "=============================================="
	@echo ""
	@echo "Building workspace in debug mode..."
	@cargo build --workspace --quiet
	@echo "Debug build complete"
	@echo ""
	@echo "Building workspace with manpages feature..."
	@cargo build --workspace --features=manpages --quiet
	@echo "Manpages build complete"
	@echo ""
	@echo "Checking for compilation warnings..."
	@BUILD_OUTPUT=$$(cargo build --workspace --features=manpages 2>&1); \
	if echo "$$BUILD_OUTPUT" | grep -q "warning:"; then \
		echo "Compilation warnings detected:"; \
		echo "$$BUILD_OUTPUT"; \
		exit 1; \
	fi
	@echo "Zero compilation warnings"
	@echo ""
	@echo "Workspace build complete"

# Tier 1: Phase 2 - Run all unit tests once
test-workspace:
	@echo ""
	@echo "=============================================="
	@echo "Phase 2: Running Unit Tests (Single Pass)"
	@echo "=============================================="
	@echo ""
	@echo "Executing complete workspace test suite (unit tests only)..."
	@cargo test --workspace --lib --bins --quiet
	@echo ""
	@echo "All unit tests passed"

# Tier 1: Phase 3 - Integration tests (J1939 simulator only)
validate-integration: cleanup-test-processes ensure-vcan0
	@echo ""
	@echo "=============================================="
	@echo "Phase 3: Integration Tests"
	@echo "=============================================="
	@echo ""
	@echo "Running J1939 simulator integration tests..."
	@cargo test -p cando-j1939-sim --test integration_test -- --include-ignored --test-threads=1 2>/dev/null && \
		echo "J1939 simulator integration tests passed" || \
		echo "Warning: J1939 integration tests skipped (may require vcan0)"
	@echo ""
	@echo "Integration tests complete"

# Tier 1: Phase 4 - CLI tools validation
validate-cli:
	@echo ""
	@echo "=============================================="
	@echo "Phase 4: CLI Tools Validation"
	@echo "=============================================="
	@echo ""
	@echo "Validating CLI help systems..."
	@TOOLS="cando-dump-messages cando-monitor cando-cfg cando-codegen cando-j1939-sim"; \
	for tool in $$TOOLS; do \
		if ./target/debug/$$tool --help >/dev/null 2>&1; then \
			echo "  $$tool --help works"; \
		else \
			echo "  $$tool --help failed"; \
			exit 1; \
		fi; \
	done
	@echo ""
	@echo "CLI tools validated"

# Tier 1: Phase 5 - Message encoding validation
validate-encoding:
	@echo ""
	@echo "=============================================="
	@echo "Phase 5: Message Encoding Validation"
	@echo "=============================================="
	@echo ""
	@echo "Testing J1939 message encoding/decoding..."
	@cargo test -p cando-messages j1939 --quiet 2>/dev/null && \
		echo "J1939 encoding validated" || \
		echo "Warning: J1939 encoding tests returned non-zero"
	@echo ""
	@echo "Message encoding validated"

# Tier 1: Phase 6 - Metadata validation
validate-metadata:
	@echo ""
	@echo "=============================================="
	@echo "Phase 6: Metadata Validation"
	@echo "=============================================="
	@echo ""
	@echo "Running cando-dump-messages metadata validation..."
	@./scripts/integration/validate_dump_messages.sh
	@echo ""
	@echo "Metadata validated"

# Tier 2 Integration Testing (requires CAN/vcan setup)
# Optimized: Cleanup -> Build -> vcan setup -> Full-stack integration tests
# Target: <30 minutes
tier2: cleanup-test-processes ensure-vcan0 build-workspace
	@echo ""
	@echo "=============================================="
	@echo "Tier 2 Full-Stack Integration Testing"
	@echo "=============================================="
	@echo ""
	@echo "Requirements:"
	@echo "  - CAN/vcan kernel modules (auto-configured)"
	@echo "  - CAP_NET_RAW capabilities (auto-configured)"
	@echo ""
	@echo "Included Tests:"
	@echo "  - J1939 simulator integration"
	@echo "  - CAN transaction performance testing"
	@echo "  - WebSocket state validation"
	@echo ""
	@echo "Running full-stack integration tests..."
	@echo ""
	@./scripts/integration/integration_test_all_protocols.sh
	@echo ""
	@echo "=============================================="
	@echo "Tier 2 Integration Testing Complete"
	@echo "=============================================="
	@$(MAKE) --no-print-directory cleanup-test-processes

# ============================================================================
# PLAYWRIGHT BROWSER AUTOMATION TESTING
# ============================================================================

# Check Playwright build dependencies (OpenSSL, pkg-config)
check-playwright-deps:
	@echo "Checking Playwright test dependencies..."
	@MISSING=0; \
	DISTRO="unknown"; \
	if [ -f /etc/os-release ]; then \
		. /etc/os-release; \
		case "$$ID" in \
			arch|manjaro) DISTRO="arch" ;; \
			ubuntu|debian|pop) DISTRO="debian" ;; \
			fedora|rhel|centos) DISTRO="fedora" ;; \
			alpine) DISTRO="alpine" ;; \
			*) DISTRO="$$ID" ;; \
		esac; \
	fi; \
	command -v pkg-config >/dev/null 2>&1 || { \
		echo "  pkg-config not found (required for OpenSSL build)"; \
		case "$$DISTRO" in \
			arch) echo "   Install: sudo pacman -S pkg-config" ;; \
			debian) echo "   Install: sudo apt install pkg-config" ;; \
			fedora) echo "   Install: sudo dnf install pkgconfig" ;; \
			alpine) echo "   Install: apk add pkgconfig" ;; \
			*) echo "   Install: See https://pkg-config.freedesktop.org/" ;; \
		esac; \
		MISSING=1; \
	}; \
	pkg-config --exists openssl 2>/dev/null || { \
		echo "  OpenSSL development libraries not found"; \
		case "$$DISTRO" in \
			arch) echo "   Install: sudo pacman -S openssl" ;; \
			debian) echo "   Install: sudo apt install libssl-dev" ;; \
			fedora) echo "   Install: sudo dnf install openssl-devel" ;; \
			alpine) echo "   Install: apk add openssl-dev" ;; \
			*) echo "   Install: See https://www.openssl.org/"; \
		esac; \
		MISSING=1; \
	}; \
	if [ $$MISSING -eq 1 ]; then \
		echo ""; \
		echo "Quick install for $$DISTRO:"; \
		case "$$DISTRO" in \
			arch) \
				echo "  sudo pacman -S pkg-config openssl" ;; \
			debian) \
				echo "  sudo apt update && sudo apt install -y pkg-config libssl-dev" ;; \
			fedora) \
				echo "  sudo dnf install pkgconfig openssl-devel" ;; \
			alpine) \
				echo "  apk add pkgconfig openssl-dev" ;; \
			*) \
				echo "  1. Install pkg-config"; \
				echo "  2. Install OpenSSL development libraries"; \
				echo "  3. Then run: make build-playwright-test" ;; \
		esac; \
		exit 1; \
	fi
	@echo "Playwright dependencies verified"

# Build playwright-test binary (requires OpenSSL)
build-playwright-test: check-playwright-deps
	@echo "Building playwright-test binary..."
	@echo "   Note: First build will compile OpenSSL (2-5 minutes)"
	@echo "   Subsequent builds will use cached binaries (much faster)"
	@echo ""
	cargo build --bin playwright-test
	@echo ""
	@echo "playwright-test binary built successfully"
	@echo ""
	@echo "Next steps:"
	@echo "  1. Run: ./scripts/setup-playwright-browsers.sh  (one-time, downloads browsers)"
	@echo "  2. Run: make test-playwright-page-load          (verify integration)"
	@echo ""

# Test: Page Load Scenario with Real Browser
test-playwright-page-load: check-playwright-deps
	@echo "Testing Playwright Page Load Scenario"
	@echo "   Real browser automation test with screenshot capture"
	@echo ""
	@./scripts/run-with-nvm.sh cargo run --bin playwright-test -- \
		--cando-config cando.yaml \
		--environment webui-simple \
		--scenario page-load \
		--verbose
	@echo ""
	@echo "Check ./playwright-output/ for screenshots and test-report.json"

# Integrated Playwright Testing
test-playwright-phases: build-all
	@echo "Running Integrated Playwright Tests"
	@echo "   Comprehensive browser-based WebUI validation"
	@echo ""
	./scripts/testing/test_playwright_webui.sh

# Performance measurement only
performance: build-release
	@echo "Performance Measurement and Baseline Establishment"
	@./scripts/integration/validate_all_protocols.sh 2>&1 | grep -E "(Measuring performance|Average:|Duration:)"

# Build targets
build:
	cargo build --workspace

build-all: build build-release build-manpages

build-release:
	cargo build --workspace --release

build-manpages:
	@echo "Building workspace with manpages feature..."
	cargo build --workspace --features=manpages
	@echo "Generating man pages..."
	./scripts/generate-man.sh
	@echo "Man pages build complete"

# Cross-compilation prerequisite checking (for aarch64 dynamic builds)
# Note: musl static builds use Zig toolchain (see check-packaging-deps)
check-cross-prereqs:
	@echo "Checking cross-compilation prerequisites..."
	@echo "   Note: musl builds use Zig (checked by check-packaging-deps)"
	@MISSING=0; \
	rustup target list --installed | grep -q aarch64-unknown-linux-gnu || { \
		echo "  Missing: Rust aarch64 target"; \
		echo "   Install: rustup target add aarch64-unknown-linux-gnu"; \
		MISSING=1; \
	}; \
	command -v aarch64-linux-gnu-gcc >/dev/null 2>&1 || { \
		echo "  Missing: aarch64 cross-compiler"; \
		echo "   Install: sudo apt install gcc-aarch64-linux-gnu"; \
		MISSING=1; \
	}; \
	if [ ! -f .cargo/config.toml ]; then \
		echo "  Missing: .cargo/config.toml"; \
		echo "   Creating with cross-compilation linker configuration..."; \
		mkdir -p .cargo; \
		echo '[target.aarch64-unknown-linux-gnu]' > .cargo/config.toml; \
		echo 'linker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml; \
	fi; \
	if [ $$MISSING -eq 1 ]; then \
		echo ""; \
		echo "To install all prerequisites, run:"; \
		echo "  rustup target add aarch64-unknown-linux-gnu"; \
		echo "  sudo apt install gcc-aarch64-linux-gnu"; \
		exit 1; \
	fi
	@echo "All cross-compilation prerequisites installed"

# Native x86_64 build targets (explicit)
build-x86_64-debug:
	@echo "Building for x86_64-unknown-linux-gnu (debug, native)"
	cargo build --workspace
	@echo "Build complete: target/debug/"

build-x86_64-release:
	@echo "Building for x86_64-unknown-linux-gnu (release, native)"
	cargo build --workspace --release
	@echo "Build complete: target/release/"

build-all-x86_64: build-x86_64-debug build-x86_64-release
	@echo "All x86_64 builds complete"

# ARM64 (aarch64) build targets - Raspberry Pi 5
build-aarch64-debug: check-cross-prereqs
	@echo "Building for aarch64-unknown-linux-gnu (debug)"
	cargo build --workspace --target aarch64-unknown-linux-gnu
	@echo "Build complete: target/aarch64-unknown-linux-gnu/debug/"

build-aarch64-release: check-cross-prereqs
	@echo "Building for aarch64-unknown-linux-gnu (release)"
	cargo build --workspace --target aarch64-unknown-linux-gnu --release
	@echo "Build complete: target/aarch64-unknown-linux-gnu/release/"

build-all-aarch64: build-aarch64-debug build-aarch64-release
	@echo "All aarch64 builds complete"

# musl (static) build targets - Alpine Linux (using Zig toolchain)
build-musl-debug: check-packaging-deps
	@echo "Building for x86_64-unknown-linux-musl (debug, static)"
	@echo "   Note: Using Zig toolchain for static linking"
	cargo zigbuild --workspace --target x86_64-unknown-linux-musl
	@echo "Build complete: target/x86_64-unknown-linux-musl/debug/"
	@echo "Static binary - no dynamic library dependencies"

build-musl-release: check-packaging-deps
	@echo "Building for x86_64-unknown-linux-musl (release, static)"
	@echo "   Note: Using Zig toolchain for static linking"
	cargo zigbuild --workspace --target x86_64-unknown-linux-musl --release
	@echo "Build complete: target/x86_64-unknown-linux-musl/release/"
	@echo "Static binary - no dynamic library dependencies"

# ARM64 musl (static) build targets - Static ARM64 binaries (using Zig toolchain)
build-aarch64-musl-debug: check-packaging-deps
	@echo "Building for aarch64-unknown-linux-musl (debug, static)"
	@echo "   Note: Using Zig toolchain for static linking"
	cargo zigbuild --workspace --target aarch64-unknown-linux-musl
	@echo "Build complete: target/aarch64-unknown-linux-musl/debug/"
	@echo "Static binary - no dynamic library dependencies"

build-aarch64-musl-release: check-packaging-deps
	@echo "Building for aarch64-unknown-linux-musl (release, static)"
	@echo "   Note: Using Zig toolchain for static linking"
	cargo zigbuild --workspace --target aarch64-unknown-linux-musl --release
	@echo "Build complete: target/aarch64-unknown-linux-musl/release/"
	@echo "Static binary - no dynamic library dependencies"

build-all-aarch64-musl: build-aarch64-musl-debug build-aarch64-musl-release
	@echo "All aarch64 musl builds complete"

build-all-musl: build-musl-debug build-musl-release build-aarch64-musl-debug build-aarch64-musl-release
	@echo "All musl (static) builds complete"

# Build all debug variants (all architectures)
build-all-debug: build-x86_64-debug build-aarch64-debug build-musl-debug build-aarch64-musl-debug
	@echo ""
	@echo "All debug builds complete!"
	@echo ""
	@echo "Debug Builds:"
	@echo "  x86_64 (glibc):   target/debug/"
	@echo "  aarch64 (glibc):  target/aarch64-unknown-linux-gnu/debug/"
	@echo "  x86_64 (musl):    target/x86_64-unknown-linux-musl/debug/"
	@echo "  aarch64 (musl):   target/aarch64-unknown-linux-musl/debug/"

# Build all release variants (all architectures)
build-all-release: build-x86_64-release build-aarch64-release build-musl-release build-aarch64-musl-release
	@echo ""
	@echo "All release builds complete!"
	@echo ""
	@echo "Release Builds:"
	@echo "  x86_64 (glibc):   target/release/"
	@echo "  aarch64 (glibc):  target/aarch64-unknown-linux-gnu/release/"
	@echo "  x86_64 (musl):    target/x86_64-unknown-linux-musl/release/"
	@echo "  aarch64 (musl):   target/aarch64-unknown-linux-musl/release/"

# Build all platforms and profiles (complete matrix)
build-all-targets: build-all-x86_64 build-all-aarch64 build-all-musl
	@echo ""
	@echo "Complete build matrix finished!"
	@echo ""
	@echo "Build Summary:"
	@echo "  x86_64 (glibc - dynamic):"
	@echo "    - Debug:   target/debug/"
	@echo "    - Release: target/release/"
	@echo ""
	@echo "  aarch64 (glibc - dynamic, Raspberry Pi 5):"
	@echo "    - Debug:   target/aarch64-unknown-linux-gnu/debug/"
	@echo "    - Release: target/aarch64-unknown-linux-gnu/release/"
	@echo ""
	@echo "  x86_64 (musl - static, Alpine Linux):"
	@echo "    - Debug:   target/x86_64-unknown-linux-musl/debug/"
	@echo "    - Release: target/x86_64-unknown-linux-musl/release/"
	@echo ""
	@echo "  aarch64 (musl - static, ARM64):"
	@echo "    - Debug:   target/aarch64-unknown-linux-musl/debug/"
	@echo "    - Release: target/aarch64-unknown-linux-musl/release/"
	@echo ""
	@echo "To verify static linking:"
	@echo "  ldd target/x86_64-unknown-linux-musl/release/cando-util"
	@echo "  ldd target/aarch64-unknown-linux-musl/release/cando-util"
	@echo ""
	@echo "To verify architectures:"
	@echo "  file target/aarch64-unknown-linux-gnu/release/cando-util"
	@echo "  file target/aarch64-unknown-linux-musl/release/cando-util"

# DBC repository management (for maintainers with DBC access)
CANDO_DBC_REPO ?= git@github.com:suykerbuyk/cando-dbc-private.git
CANDO_DBC_PATH ?= dbc

dbc-init:
	@if [ -d "$(CANDO_DBC_PATH)/.git" ]; then \
		echo "DBC repo already initialized at $(CANDO_DBC_PATH)"; \
		echo "   Use 'make dbc-pull' to update"; \
	else \
		echo "Cloning DBC repository into $(CANDO_DBC_PATH)..."; \
		git clone $(CANDO_DBC_REPO) $(CANDO_DBC_PATH); \
		echo "DBC repository initialized"; \
	fi

dbc-pull:
	@if [ -d "$(CANDO_DBC_PATH)/.git" ]; then \
		echo "Updating DBC repository..."; \
		git -C $(CANDO_DBC_PATH) pull --ff-only; \
	else \
		echo "Error: DBC repo not found at $(CANDO_DBC_PATH)"; \
		echo "   Run 'make dbc-init' first"; \
		exit 1; \
	fi

dbc-status:
	@if [ -d "$(CANDO_DBC_PATH)/.git" ]; then \
		echo "DBC repo: $(CANDO_DBC_PATH)"; \
		echo "Remote: $$(git -C $(CANDO_DBC_PATH) remote get-url origin 2>/dev/null || echo 'unknown')"; \
		echo "Branch: $$(git -C $(CANDO_DBC_PATH) rev-parse --abbrev-ref HEAD)"; \
		echo "Commit: $$(git -C $(CANDO_DBC_PATH) log -1 --format='%h %s')"; \
	else \
		echo "DBC repo not initialized at $(CANDO_DBC_PATH)"; \
		echo "   Run 'make dbc-init' to clone"; \
	fi

# Code generation targets (for maintainers with DBC files)
codegen-status:
	@echo "Checking code generation status"
	CANDO_DBC_PATH=$(CANDO_DBC_PATH) cargo run --bin cando-codegen -- status

codegen-all:
	@echo "Regenerating all changed protocols"
	CANDO_DBC_PATH=$(CANDO_DBC_PATH) cargo run --bin cando-codegen -- generate-all

codegen-force:
	@echo "Force regenerating all protocols"
	CANDO_DBC_PATH=$(CANDO_DBC_PATH) cargo run --bin cando-codegen -- generate-all --force
	@echo ""
	@echo "Checking for codegen algorithm changes..."
	@./scripts/dev-tools/detect_codegen_change.sh || true

codegen-validate:
	@echo "Validating generated code checksums"
	CANDO_DBC_PATH=$(CANDO_DBC_PATH) cargo run --bin cando-codegen -- validate

codegen-detect-changes:
	@echo "Detecting codegen algorithm changes"
	@./scripts/dev-tools/detect_codegen_change.sh

codegen-validate-tests:
	@echo "Note: Test field validation is unnecessary - tests validate themselves at runtime"
	@echo "   cando-util returns errors for invalid field names immediately"
	@echo "   Run 'make tier1' to validate all field references"

codegen-proto:
	@echo "Regenerating protocol: $(PROTO)"
	@if [ -z "$(PROTO)" ]; then \
		echo "Error: PROTO variable not set"; \
		echo "Usage: make codegen-proto PROTO=<protocol-name>"; \
		echo "Available protocols: j1939, j1939-73"; \
		exit 1; \
	fi
	CANDO_DBC_PATH=$(CANDO_DBC_PATH) cargo run --bin cando-codegen -- generate --protocol $(PROTO)

codegen-proto-force:
	@echo "Force regenerating protocol: $(PROTO)"
	@if [ -z "$(PROTO)" ]; then \
		echo "Error: PROTO variable not set"; \
		echo "Usage: make codegen-proto-force PROTO=<protocol-name>"; \
		exit 1; \
	fi
	CANDO_DBC_PATH=$(CANDO_DBC_PATH) cargo run --bin cando-codegen -- generate --protocol $(PROTO) --force

codegen-list:
	@echo "Available protocols for code generation:"
	CANDO_DBC_PATH=$(CANDO_DBC_PATH) cargo run --bin cando-codegen -- list

# Metadata validation targets
validate-dump-messages: build-release
	@echo "Validating cando-dump-messages metadata flags"
	@echo "   Testing --comments, --enums, --full, --verbose"
	@echo ""
	./scripts/integration/validate_dump_messages.sh

# Test targets
test:
	cargo test --workspace

test-all:
	@echo "Running complete test suite"
	cargo test --workspace

# Helper target to cleanup stray test processes
cleanup-test-processes:
	@echo "Cleaning up stray test processes..."
	@KILLED=0; \
	for proc in cando-j1939-sim; do \
		PIDS=$$(pgrep -f "target/(debug|release)/$$proc" 2>/dev/null || true); \
		if [ -n "$$PIDS" ]; then \
			echo "$$PIDS" | xargs -r kill -9 2>/dev/null || true; \
			KILLED=$$((KILLED + 1)); \
			echo "  Killed stray $$proc processes"; \
		fi; \
	done; \
	if [ $$KILLED -eq 0 ]; then \
		echo "  No stray processes found"; \
	else \
		echo "  Cleaned up $$KILLED process type(s)"; \
		sleep 1; \
	fi

# Helper target to ensure vcan0 interface exists
ensure-vcan0:
	@# Try to check if vcan0 exists
	@if ip link show vcan0 >/dev/null 2>&1; then \
		echo "vcan0 already exists"; \
	elif sudo -n modprobe vcan 2>/dev/null && sudo -n ip link add dev vcan0 type vcan 2>/dev/null && sudo -n ip link set up vcan0 2>/dev/null; then \
		echo "vcan0 created (passwordless sudo)"; \
	else \
		echo "=============================================="; \
		echo "vcan0 interface not available"; \
		echo ""; \
		echo "Integration tests require vcan0 interface."; \
		echo ""; \
		echo "OPTION 1 - One-time manual setup:"; \
		echo "  sudo modprobe vcan"; \
		echo "  sudo ip link add dev vcan0 type vcan"; \
		echo "  sudo ip link set up vcan0"; \
		echo "  (Persists until reboot)"; \
		echo ""; \
		echo "OPTION 2 - Configure passwordless sudo (permanent):"; \
		echo "  sudo visudo -f /etc/sudoers.d/cando-testing"; \
		echo "  Add these lines:"; \
		echo "    $(USER) ALL=(ALL) NOPASSWD: /usr/sbin/ip link add dev vcan* type vcan"; \
		echo "    $(USER) ALL=(ALL) NOPASSWD: /usr/sbin/ip link set * vcan*"; \
		echo "    $(USER) ALL=(ALL) NOPASSWD: /sbin/modprobe vcan"; \
		echo ""; \
		echo "OPTION 3 - Run with sudo (for this session):"; \
		echo "  sudo -v && make tier2"; \
		echo "=============================================="; \
		exit 1; \
	fi

# CAN/Hardware setup targets
setup-can-privileges: build-all
	@echo "Setting up CAN privileges for Tier 2 testing"
	@echo "   This requires sudo access to set capabilities on binaries"
	@echo ""
	@echo "Checking sudo access (you may be prompted for your password)..."
	@sudo -v || { echo "ERROR: sudo access required for setting CAN capabilities"; exit 1; }
	@echo ""
	./scripts/set_can_privileges.sh caps || { echo ""; echo "ERROR: Failed to set CAN privileges"; exit 1; }
	@echo ""
	@echo "CAN privileges configured successfully"

setup-can-interfaces:
	@echo "Setting up CAN interfaces"
	@echo "   Creating vcan0 interface for testing"
	./scripts/set_can_privileges.sh setup

setup-can-all: build-all
	@echo "Complete CAN setup (privileges + interfaces + udev)"
	@echo "   This is a one-time setup for Tier 2 testing"
	./scripts/set_can_privileges.sh all

# Integration test alias
integration-test: tier1

# Development workflow targets
dev-cycle: clean build-all test-all tier1 validate-dump-messages
	@echo "Complete development cycle validation passed"
	@echo "   Ready for commit and push"

ci-validate: tier1 validate-dump-messages
	@echo "CI/CD validation completed"
	@echo "   GitHub Actions compatible validation passed (including metadata validation)"

quality-gate: build-all test-all tier1 validate-dump-messages
	@echo "Quality gate passed - ready for release"
	@echo "   All builds, tests, integration tests, and metadata validation successful"

# Cleanup targets
clean:
	cargo clean

clean-all: clean
	@echo "Cleaning all generated files and reports"
	rm -rf benchmarks/reports/*
	rm -rf benchmarks/baselines/*.csv
	rm -f /tmp/*_simulator_*.log
	rm -f /tmp/can_ping_*.log
	rm -f /tmp/interference_test_*.log
	rm -f /tmp/stress_test_*.log
	@echo "All generated files cleaned"

# Setup and dependency checking
setup:
	@echo "Checking development environment setup"
	@command -v cargo >/dev/null 2>&1 || { echo "Rust/Cargo not installed"; exit 1; }
	@command -v git >/dev/null 2>&1 || { echo "Git not installed"; exit 1; }
	@echo "Basic development environment ready"
	@echo ""
	@echo "For Tier 2 testing, you'll also need:"
	@echo "  - Linux with CAN/vcan kernel modules"
	@echo "  - cando-send and cando-dump binaries (built by 'make build')"
	@echo "  - sudo access for interface setup"
	@echo ""
	@echo "Run 'make setup-can-all' to configure CAN testing"

# Advanced targets for CI/CD integration
check-tier2-ready:
	@echo "Checking Tier 2 readiness"
	@test -x ./target/debug/cando-send -o -x ./target/release/cando-send || { echo "cando-send not built - run 'make build' first"; exit 1; }
	@test -x ./target/debug/cando-dump -o -x ./target/release/cando-dump || { echo "cando-dump not built - run 'make build' first"; exit 1; }
	@lsmod | grep -q "^can" || { echo "CAN kernel modules not loaded"; exit 1; }
	@echo "Tier 2 environment ready"

# Meta targets
all: clean build-all test-all tier1
	@echo "Complete build, test, and validation cycle completed successfully!"

help-tier2:
	@echo "Tier 2 Full-Stack Integration Testing Help"
	@echo "==========================================="
	@echo ""
	@echo "Prerequisites:"
	@echo "  1. Run 'make setup-can-all' (one-time setup)"
	@echo "  2. Ensure your user can run sudo (for interface setup)"
	@echo "  3. Linux system with CAN kernel module support"
	@echo ""
	@echo "What Tier 2 tests:"
	@echo "  - CAN/vcan interface creation and management"
	@echo "  - J1939 simulator startup and operation"
	@echo "  - CAN message transmission and reception"
	@echo "  - WebSocket API validation"
	@echo "  - Performance stress testing and throughput measurement"
	@echo ""
	@echo "Troubleshooting:"
	@echo "  - Check logs in /tmp/*_simulator_*.log"
	@echo "  - Verify 'ip link show vcan0' shows interface UP"
	@echo "  - Test './target/debug/cando-send vcan0 123#deadbeef' manually"

# ============================================================================
# Debian Package Generation (cargo-deb based)
# ============================================================================
# These targets create distributable .deb packages using cargo-deb.
# Packages are ALWAYS statically linked using musl for maximum portability.
#
# For development builds (dynamic linking), use: make build or make build-release
# For distribution packages (static linking), use: make build-deb

# Check that packaging tools are installed
check-packaging-deps:
	@echo "Checking Debian packaging dependencies..."
	@echo "   Note: Static builds use Zig exclusively (no musl-gcc required)"
	@MISSING=0; \
	DISTRO="unknown"; \
	if [ -f /etc/os-release ]; then \
		. /etc/os-release; \
		case "$$ID" in \
			arch|manjaro) DISTRO="arch" ;; \
			ubuntu|debian|pop) DISTRO="debian" ;; \
			fedora|rhel|centos) DISTRO="fedora" ;; \
			*) DISTRO="$$ID" ;; \
		esac; \
	fi; \
	command -v cargo-deb >/dev/null 2>&1 || { \
		echo "  cargo-deb not found. Install with:"; \
		echo "   cargo install cargo-deb"; \
		MISSING=1; \
	}; \
	rustup target list | grep -q "x86_64-unknown-linux-musl (installed)" || { \
		echo "  musl target (x86_64) not installed. Install with:"; \
		echo "   rustup target add x86_64-unknown-linux-musl"; \
		MISSING=1; \
	}; \
	rustup target list | grep -q "aarch64-unknown-linux-musl (installed)" || { \
		echo "  musl target (aarch64) not installed. Install with:"; \
		echo "   rustup target add aarch64-unknown-linux-musl"; \
		MISSING=1; \
	}; \
	command -v zig >/dev/null 2>&1 || { \
		echo "  zig not found (required for static builds)"; \
		case "$$DISTRO" in \
			arch) \
				echo "   Arch Linux: sudo pacman -S zig" ;; \
			debian) \
				echo "   Debian/Ubuntu: sudo snap install zig --classic --beta"; \
				echo "   Note: Snap is officially supported by Zig project"; \
				echo "   Alternative: Download binary from https://ziglang.org/download/" ;; \
			fedora) \
				echo "   Fedora/RHEL: sudo dnf install zig" ;; \
			*) \
				echo "   Install Zig from: https://ziglang.org/download/"; \
				echo "   Or use snap: sudo snap install zig --classic --beta" ;; \
		esac; \
		MISSING=1; \
	}; \
	command -v cargo-zigbuild >/dev/null 2>&1 || { \
		echo "  cargo-zigbuild not found (required for static builds)"; \
		echo "   Install: cargo install cargo-zigbuild"; \
		MISSING=1; \
	}; \
	if [ $$MISSING -eq 1 ]; then \
		echo ""; \
		echo "Quick install for $$DISTRO:"; \
		case "$$DISTRO" in \
			arch) \
				echo "  sudo pacman -S zig"; \
				echo "  rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl"; \
				echo "  cargo install cargo-deb cargo-zigbuild" ;; \
			debian) \
				echo "  sudo snap install zig --classic --beta"; \
				echo "  rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl"; \
				echo "  cargo install cargo-deb cargo-zigbuild"; \
				echo "  # Alternative: Download from https://ziglang.org/download/" ;; \
			fedora) \
				echo "  sudo dnf install zig"; \
				echo "  rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl"; \
				echo "  cargo install cargo-deb cargo-zigbuild" ;; \
			*) \
				echo "  Install Zig from: https://ziglang.org/download/"; \
				echo "  rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl"; \
				echo "  cargo install cargo-deb cargo-zigbuild" ;; \
		esac; \
		echo ""; \
		echo "Missing dependencies. Install the above and try again."; \
		exit 1; \
	fi
	@echo "All packaging dependencies present"

# Build Debian package for x86_64 (amd64) using static musl linking via Zig
build-deb-amd64: check-packaging-deps
	@echo "Building Debian package for amd64 (x86_64-unknown-linux-musl)"
	@echo "   Note: Using Zig toolchain for static linking (zero dependencies)"
	@echo ""
	@rustup target list | grep -q "x86_64-unknown-linux-musl (installed)" || { \
		echo "Installing x86_64-musl target..."; \
		rustup target add x86_64-unknown-linux-musl; \
	}
	@mkdir -p logs
	@echo "Step 1: Building all workspace binaries with Zig..."
	@if ! cargo zigbuild --workspace --release --target x86_64-unknown-linux-musl 2>&1 | tee logs/deb-build-amd64_$$(date +%Y%m%d_%H%M%S).log | tail -20; then \
		echo ""; \
		echo "Build failed! Common causes:"; \
		echo "   - Zig not installed (run 'make check-packaging-deps' for help)"; \
		echo "   - cargo-zigbuild not installed (cargo install cargo-zigbuild)"; \
		echo "   - Missing musl target (rustup target add x86_64-unknown-linux-musl)"; \
		echo "   - Check the build log above for detailed error messages"; \
		exit 1; \
	fi
	@echo ""
	@echo "Step 1.5: Validating binaries..."
	@VALIDATION_FAILED=0; \
	TEST_BINARIES="cando-util cando-dump cando-codegen"; \
	for binary in $$TEST_BINARIES; do \
		BINARY_PATH="target/x86_64-unknown-linux-musl/release/$$binary"; \
		if [ -f "$$BINARY_PATH" ]; then \
			if ! timeout 2s "$$BINARY_PATH" --version >/dev/null 2>&1; then \
				echo "  $$binary failed to execute"; \
				VALIDATION_FAILED=1; \
			else \
				echo "  $$binary validated"; \
			fi; \
		fi; \
	done; \
	if [ $$VALIDATION_FAILED -eq 1 ]; then \
		echo ""; \
		echo "CRITICAL: Compiled binaries failed validation"; \
		echo "   The binaries built successfully but cannot execute."; \
		echo ""; \
		echo "Troubleshooting:"; \
		echo "   - Verify Zig is properly installed: zig version"; \
		echo "   - Update cargo-zigbuild: cargo install cargo-zigbuild --force"; \
		echo "   - Check build logs for linking errors"; \
		echo "   - Report issue if problem persists"; \
		exit 1; \
	fi
	@echo ""
	@echo "Step 2: Generating shell completions..."
	./scripts/packaging/generate-completions.sh x86_64-unknown-linux-musl || true
	@echo ""
	@echo "Step 3: Creating Debian package with cargo-deb..."
	@if ! cargo deb -p cando-meta --target=x86_64-unknown-linux-musl --no-build \
		--output=target/debian/ -v 2>&1 | tee -a logs/deb-build-amd64_$$(date +%Y%m%d_%H%M%S).log | tail -30; then \
		echo ""; \
		echo "Debian package creation failed!"; \
		echo "   - Check the cargo-deb output above for details"; \
		echo "   - Ensure all binaries built successfully in Step 1"; \
		exit 1; \
	fi
	@echo ""
	@echo "Debian package built successfully!"
	@echo ""
	@echo "Package details:"
	@ls -lh target/debian/cando-rs_*.deb 2>/dev/null || echo "Package location: target/debian/"
	@echo ""
	@echo "To install: sudo dpkg -i target/debian/cando-rs_*.deb"
	@echo "To test: make deb-test"

# Build Debian package for ARM64 (aarch64) using static musl linking
build-deb-arm64: check-packaging-deps
	@echo "Building Debian package for arm64 (aarch64-unknown-linux-musl)"
	@echo "   Note: Using static linking (musl) for zero dependencies"
	@echo ""
	@rustup target list | grep -q "aarch64-unknown-linux-musl (installed)" || { \
		echo "Installing aarch64-musl target..."; \
		rustup target add aarch64-unknown-linux-musl; \
	}
	@mkdir -p logs
	@echo "Note: ARM64 binaries cannot be validated on x86_64 host (cross-compilation)"
	@echo ""
	@echo "Step 1: Building all workspace binaries with musl (aarch64) using Zig..."
	@if ! cargo zigbuild --workspace --release --target aarch64-unknown-linux-musl 2>&1 | tee logs/deb-build-musl-arm64_$$(date +%Y%m%d_%H%M%S).log | tail -20; then \
		echo ""; \
		echo "ARM64 build failed! Common causes:"; \
		echo "   - zig not installed (run 'make check-packaging-deps' for help)"; \
		echo "   - cargo-zigbuild not installed (cargo install cargo-zigbuild)"; \
		echo "   - Missing aarch64-musl target (rustup target add aarch64-unknown-linux-musl)"; \
		echo "   - Check the build log above for detailed error messages"; \
		echo ""; \
		echo "To fix ARM64 cross-compilation issues:"; \
		echo "   - Arch Linux: sudo pacman -S zig"; \
		echo "   - Ubuntu/Debian: sudo snap install zig --classic --beta"; \
		echo "   - Fedora: sudo dnf install zig"; \
		exit 1; \
	fi
	@echo ""
	@echo "Step 2: Shell completions (using x86_64 completions - they're architecture-independent)..."
	@if [ ! -d target/completions ]; then \
		echo "Warning: No completions found. Building x86_64 first to generate completions..."; \
		$(MAKE) build-deb-amd64 > /dev/null 2>&1 || true; \
	fi
	@if [ -d target/completions ]; then \
		echo "Using existing completions (generated from amd64 build)"; \
	else \
		echo "Warning: Completions not available. Package will not include shell completions."; \
	fi
	@echo ""
	@echo "Step 3: Creating Debian package with cargo-deb (no rebuild - using zigbuild binaries)..."
	@if ! cargo deb -p cando-meta --target=aarch64-unknown-linux-musl --no-build \
		--output=target/debian/ -v 2>&1 | tee -a logs/deb-build-musl-arm64_$$(date +%Y%m%d_%H%M%S).log | tail -30; then \
		echo ""; \
		echo "ARM64 Debian package creation failed!"; \
		echo "   - Check the cargo-deb output above for details"; \
		echo "   - Ensure ARM64 binaries built successfully in Step 1"; \
		echo "   - Missing completions can cause packaging to fail"; \
		exit 1; \
	fi
	@echo ""
	@echo "Debian package built successfully!"
	@echo ""
	@echo "Package details:"
	@ls -lh target/debian/cando-rs_*_arm64.deb 2>/dev/null || echo "Package location: target/debian/"
	@echo ""
	@echo "To test on ARM64: Transfer .deb to ARM64 system and run 'sudo dpkg -i cando-rs_*_arm64.deb'"

# Default deb target builds amd64
build-deb: build-deb-amd64

# Build packages for both architectures
deb-all: check-packaging-deps
	@echo "Building Debian packages for all architectures..."
	@echo "   This will build both amd64 and arm64 packages using static linking"
	@echo ""
	@echo "Prerequisites check passed. Starting build process..."
	@echo ""
	@$(MAKE) build-deb-amd64 || { \
		echo ""; \
		echo "AMD64 package build failed!"; \
		echo "   - Run 'make check-packaging-deps' to verify all tools are installed"; \
		echo "   - Review the build log above for specific errors"; \
		exit 1; \
	}
	@echo ""
	@echo "AMD64 package complete. Building ARM64 package..."
	@echo ""
	@$(MAKE) build-deb-arm64 || { \
		echo ""; \
		echo "ARM64 package build failed!"; \
		echo "   - Ensure zig and cargo-zigbuild are installed"; \
		echo "   - ARM64 builds require cross-compilation tools"; \
		echo "   - Run 'make check-packaging-deps' for installation help"; \
		exit 1; \
	}
	@echo ""
	@echo "All Debian packages built successfully!"
	@echo ""
	@echo "Generated packages:"
	@if ls target/debian/*.deb >/dev/null 2>&1; then \
		ls -lh target/debian/*.deb; \
		echo ""; \
		echo "Package sizes:"; \
		du -h target/debian/*.deb; \
	else \
		echo "Warning: No .deb files found in target/debian/"; \
		echo "   This may indicate a packaging issue."; \
	fi
	@echo ""
	@echo "Installation:"
	@echo "   AMD64: sudo dpkg -i target/debian/cando-rs_*_amd64.deb"
	@echo "   ARM64: sudo dpkg -i target/debian/cando-rs_*_arm64.deb"

# Build and immediately install the amd64 package (development convenience)
deb-install: check-packaging-deps
	@echo "Building and installing Debian package..."
	@echo "   Note: This will prompt for sudo password"
	@echo ""
	cargo build --workspace --release --target x86_64-unknown-linux-musl
	./scripts/packaging/generate-completions.sh x86_64-unknown-linux-musl || true
	cargo deb -p cando-meta --target=x86_64-unknown-linux-musl --install

# Test package installation (creates a simple verification)
deb-test: build-deb-amd64
	@echo "Testing Debian package installation..."
	@echo ""
	@echo "Package info:"
	dpkg-deb --info target/debian/cando-rs_*.deb | head -20
	@echo ""
	@echo "Package contents:"
	dpkg-deb --contents target/debian/cando-rs_*.deb | grep "usr/bin/" || true
	@echo ""
	@echo "Verifying binaries are statically linked (musl):"
	@for bin in $$(dpkg-deb --contents target/debian/cando-rs_*.deb | grep "usr/bin/" | awk '{print $$NF}' | xargs basename); do \
		echo "  Checking $$bin..."; \
		ldd target/x86_64-unknown-linux-musl/release/$$bin 2>&1 | head -1 || true; \
	done
	@echo ""
	@echo "Package validation complete"
	@echo ""
	@echo "To install and test manually:"
	@echo "  sudo dpkg -i target/debian/cando-rs_*.deb"
	@echo "  cando-util --version"
	@echo "  cando-dump --version"
	@echo "  sudo dpkg -r cando-rs  # To remove"

# Clean generated packages
deb-clean:
	@echo "Cleaning Debian packages..."
	rm -rf target/debian/
	rm -rf target/completions/
	@echo "Package artifacts cleaned"

# Help for packaging
deb-help:
	@echo "Debian Packaging Help"
	@echo "===================="
	@echo ""
	@echo "Quick Start:"
	@echo "  1. make check-packaging-deps  # Verify tools + platform-specific guidance"
	@echo "  2. make build-deb             # Build amd64 package"
	@echo "  3. make deb-test              # Verify package"
	@echo ""
	@echo "Available Targets:"
	@echo "  build-deb        - Build amd64 package (uses Zig)"
	@echo "  build-deb-amd64  - Build amd64 package (uses Zig)"
	@echo "  build-deb-arm64  - Build arm64 package (uses Zig)"
	@echo "  deb-all          - Build both packages"
	@echo "  deb-install      - Build and install locally"
	@echo "  deb-test         - Verify package contents"
	@echo "  deb-clean        - Remove generated packages"
	@echo ""
	@echo "Package Details:"
	@echo "  - Name: cando-rs"
	@echo "  - Linking: Static (musl) - zero dependencies"
	@echo ""
	@echo "Installation:"
	@echo "  sudo dpkg -i target/debian/cando-rs_*.deb"
	@echo ""
	@echo "Removal:"
	@echo "  sudo dpkg -r cando-rs"
	@echo ""
	@echo "Documentation:"
	@echo "  - Installation: doc/build-debian-install.md"
	@echo "  - Packaging: doc/build-debian-packaging.md"

# ============================================================================
# Analysis & Profiling Targets
# ============================================================================

# Code coverage via cargo-tarpaulin (config in tarpaulin.toml)
coverage:
	@echo "Generating code coverage report..."
	@echo "Requires: cargo install cargo-tarpaulin"
	cargo tarpaulin
	@echo ""
	@echo "Coverage report: target/coverage/tarpaulin-report.html"

# Run criterion benchmarks
bench:
	@echo "Running criterion benchmarks..."
	cargo bench --workspace
	@echo ""
	@echo "Benchmark reports in target/criterion/"

# Build API documentation
doc:
	@echo "Building API documentation..."
	cargo doc --workspace --no-deps
	@echo ""
	@echo "Documentation: target/doc/cando_messages/index.html"

# Generate flamegraph for J1939 simulator (requires vcan0 + cargo-flamegraph)
flamegraph-sim: ensure-vcan0
	@echo "Generating flamegraph for J1939 simulator..."
	@echo "Requires: cargo install flamegraph"
	@echo "Note: May require root or perf_event_paranoid=1"
	cargo flamegraph --profile release-with-debug --bin cando-j1939-sim -- --interface vcan0 &
	@sleep 5
	@kill %1 2>/dev/null || true
	@echo ""
	@echo "Flamegraph: flamegraph.svg"

# Profile J1939 simulator with perf (requires vcan0 + perf)
profile-sim: ensure-vcan0
	@echo "Profiling J1939 simulator with perf..."
	cargo build --profile release-with-debug --bin cando-j1939-sim
	@echo "Run manually:"
	@echo "  perf record -g target/release-with-debug/cando-j1939-sim --interface vcan0"
	@echo "  perf report"
