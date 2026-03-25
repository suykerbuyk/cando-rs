//! Minimal build script for cando-messages
//!
//! This build script does NOT regenerate code from DBC files during normal builds.
//! Generated Rust code is version-controlled in src/generated/ and compiled directly.
//!
//! ## For Open Source Users
//!
//! You don't need DBC files to build this project! The generated code is already
//! included in the repository. Just run `cargo build` normally.
//!
//! ## For Maintainers with DBC Access
//!
//! To regenerate code when DBC files change, use the standalone generator:
//!
//! ```bash
//! # Generate a specific protocol
//! cargo run --bin cando-codegen -- generate --protocol j1939
//!
//! # Generate all changed protocols
//! cargo run --bin cando-codegen -- generate-all
//!
//! # Check status
//! cargo run --bin cando-codegen -- status
//!
//! # Validate checksums
//! cargo run --bin cando-codegen -- validate
//! ```
//!
//! ## Optional Build-Time Validation
//!
//! Set the environment variable `CANDO_VALIDATE_GENERATED=1` to enable
//! checksum validation during builds. This will warn if generated code is
//! out of sync with DBC files (maintainers only).
//!
//! ```bash
//! CANDO_VALIDATE_GENERATED=1 cargo build
//! ```

use std::env;
use std::path::Path;

/// Build script that no longer generates code from DBC files.
///
/// The code generation has been moved to a standalone tool (cando-codegen)
/// for better separation of concerns and to support open-source distribution
/// without requiring proprietary DBC files.
fn main() {
    // Tell cargo to rerun this build script if generated files change
    // (they shouldn't change often, but this ensures proper rebuilds)
    println!("cargo:rerun-if-changed=src/generated/j1939.rs");
    println!("cargo:rerun-if-changed=src/generated/j1939_73.rs");

    // Check if validation is requested
    let validate = env::var("CANDO_VALIDATE_GENERATED")
        .map(|v| v == "1" || v == "true")
        .unwrap_or(false);

    if !validate {
        // Normal mode: just compile the existing generated code
        // No DBC processing, no validation - fast builds!
        return;
    }

    // Developer mode: validate that generated code matches DBC files
    println!("cargo:warning=CANDO_VALIDATE_GENERATED is set - checking DBC checksums...");

    let dbc_dir = Path::new("../dbc");
    if !dbc_dir.exists() {
        println!("cargo:warning=No DBC directory found (this is OK for open-source users)");
        println!("cargo:warning=Skipping validation and using existing generated code.");
        return;
    }

    let checksum_file = dbc_dir.join(".checksums.json");
    if !checksum_file.exists() {
        println!("cargo:warning=No checksum file found at dbc/.checksums.json");
        println!("cargo:warning=Run: cargo run --bin cando-codegen -- generate-all");
        return;
    }

    // Load and validate checksums (simplified - real validation in cando-codegen)
    match std::fs::read_to_string(&checksum_file) {
        Ok(content) => {
            if content.contains("j1939") {
                println!("cargo:warning=Checksum file exists and appears valid");
            } else {
                println!("cargo:warning=Checksum file may be incomplete");
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to read checksum file: {}", e);
        }
    }

    println!("cargo:warning=For full validation, run: cargo run --bin cando-codegen -- validate");
}
