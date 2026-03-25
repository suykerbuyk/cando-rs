#![warn(missing_docs)]
//! # Cando Meta-Package
//!
//! This is a meta-package that exists solely for Debian package generation.
//! It contains no actual code - its purpose is to provide a single Cargo
//! package that can be used with `cargo-deb` to bundle all cando-rs
//! binaries into a single `.deb` package.
//!
//! ## Usage
//!
//! To build the Debian package:
//!
//! ```bash
//! # For x86_64 (amd64)
//! cargo deb -p cando-meta --target=x86_64-unknown-linux-musl
//!
//! # For aarch64 (arm64)
//! cargo deb -p cando-meta --target=aarch64-unknown-linux-musl
//! ```

// This module intentionally left empty - no code needed for packaging
