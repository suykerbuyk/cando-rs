//! Engine Control J1939 Messages
//!
//! This module contains implementations for J1939 messages related to engine
//! control systems, including:
//!
//! - **EEC Series**: Electronic Engine Controller messages (EEC12, EEC19, EEC21, EEC22, EEC23)
//! - **ETCC Series**: Engine Turbocharger Control messages (ETCC2, ETCC4)
//! - **ETC Series**: Electronic Transmission Controller messages (ETC4, ETC5)
//!
//! # Implemented Messages
//!
//! ## Engine Controllers (EEC)
//! - `EEC12` - Engine Exhaust Sensor Power Supply (6 signals)
//! - `EEC19` - Engine exhaust flow and energy data (2 signals)
//! - `EEC21` - Engine exhaust manifold pressure (2 signals)
//! - `EEC22` - Engine exhaust gas and crank attempts (2 signals)
//!
//! ## Turbocharger Control (ETCC)
//! - `ETCC2` - Engine turbocharger solenoid control (2 signals)
//! - `ETCC3` - Engine thermal control with motor current disable (7 signals)
//!
//! ## Transmission Control (ETC)
//! - `ETC4` - Transmission synchronizer control (2 signals)
//! - `ETC5` - Transmission control status (6 signals)
//!
//! # Signal Types Supported
//!
//! - **Type 1: Simple Integer** - Direct bit manipulation, no scaling
//! - **Type 2: Scaled Float** - Factor/offset scaling with various factors (0.001, 0.1, 0.4, 5.0)
//!
//! # Usage Example
//!
//! ```rust
//! use cando_messages::{DeviceId, j1939::EEC21};
//!
//! // Create engine pressure message
//! let message = EEC21 {
//!     device_id: DeviceId::from(0x42),
//!     engn_exhst_mnfld_aslt_prssr_1: 250.5, // 250.5 kPa
//!     engn_exhst_mnfld_aslt_prssr_2: 180.2, // 180.2 kPa
//! };
//!
//! // Encode to CAN frame
//! let (can_id, data) = message.encode().unwrap();
//!
//! // Decode back
//! let decoded = EEC21::decode(can_id, &data).unwrap();
//! ```

// Re-export all encoder functions for public use

// Note: Implementations are automatically available on the generated message types
// when this module is imported, so no explicit re-export needed for impl methods
