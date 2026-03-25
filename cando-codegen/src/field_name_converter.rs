//! Field name conversion - Re-exported from cando-core
//!
//! This module is maintained for backward compatibility.
//! The actual implementation has been moved to `cando-core::field_name_converter`
//! to make it available to the entire project.
//!
//! See `cando_core::field_name_converter` for the main documentation.

#[allow(unused_imports)]
pub use cando_core::field_name_converter::{detect_collisions, to_rust_field_name, Collision};
