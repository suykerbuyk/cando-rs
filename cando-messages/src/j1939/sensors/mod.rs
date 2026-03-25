//! Sensor J1939 Messages
//!
//! This module contains implementations for J1939 messages related to sensor
//! systems, including position sensors, angle sensors, and displacement
//! measurement systems.
//!
//! # Implemented Messages
//!
//! ## Position and Angle Sensors
//! - `WAND` - Wand Angle sensor with factor and offset scaling (2 signals)
//! - `LDISP` - Linear Displacement sensor with factor scaling (2 signals)
//!
//! # Signal Types Supported
//!
//! - **Type 2: Scaled Float** - Factor/offset scaling for precise measurements
//!   - WAND: factor=0.002, offset=-64.0 (angular measurements in degrees)
//!   - LDISP: factor=0.1, offset=0.0 (linear measurements in millimeters)
//!
//! # Usage Example
//!
//! ```rust
//! use cando_messages::{DeviceId, j1939::WAND};
//!
//! // Create wand angle sensor message
//! let message = WAND {
//!     device_id: DeviceId::from(0x42),
//!     wand_angle: 45.0, // 45 degrees
//!     wand_sensor_figure_of_merit: 3,
//! };
//!
//! // Encode to CAN frame
//! let (can_id, data) = message.encode().unwrap();
//!
//! // Decode back
//! let decoded = WAND::decode(can_id, &data).unwrap();
//! ```
//!
//! ## Linear Displacement Sensor
//!
//! ```rust
//! use cando_messages::{DeviceId, j1939::LDISP};
//!
//! // Create linear displacement sensor message
//! let message = LDISP {
//!     device_id: DeviceId::from(0x42),
//!     measured_linear_displacement: 1250.5, // 1250.5 mm
//!     lnr_dsplmnt_snsr_snsr_fgr_of_mrt: 2,
//! };
//!
//! // Encode to CAN frame
//! let (can_id, data) = message.encode().unwrap();
//! ```

// Re-export all encoder functions for public use

// Note: Implementations are automatically available on the generated message types
// when this module is imported, so no explicit re-export needed for impl methods
