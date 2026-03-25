#![warn(missing_docs)]
//! # Cando Core - Legacy Compatibility Layer
//!
//! This crate serves as a compatibility layer that re-exports the modern
//! `cando-messages` crate. All new code should directly use `cando-messages`.
//!
//! ## Migration Notice
//!
//! This crate previously contained runtime DBC parsing functionality which has
//! been fully replaced by compile-time code generation in `cando-messages`.
//!
//! **For new code:** Use `cando-messages` directly:
//! ```rust
//! use cando_messages::{DeviceId, j1939::*};
//! ```
//!
//! ## What This Crate Provides
//!
//! This crate now simply re-exports all of `cando-messages` for backward
//! compatibility with existing code that imports `cando_core`.

// Re-export everything from cando-messages
pub use cando_messages::*;

/// Field name conversion utilities for converting DBC field names to Rust snake_case
pub mod field_name_converter;
pub use field_name_converter::{Collision, detect_collisions, to_rust_field_name};

/// Device ID parsing utilities
pub mod device_id;
pub use device_id::parse_device_id;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexport_works() {
        // Verify that we can access cando-messages types through cando-core
        let device = DeviceId::new(0x42);
        assert_eq!(u8::from(device), 0x42);
    }
}
