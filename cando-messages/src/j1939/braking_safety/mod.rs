//! Braking and Safety J1939 Messages
//!
//! This module contains implementations for J1939 messages related to braking
//! and safety systems, including:
//!
//! - **AEBS Series**: Advanced Emergency Braking System messages
//! - **ABS Series**: Anti-lock Braking System messages (future)
//! - **EBS Series**: Electronic Braking System messages (future)
//! - **Safety Monitoring**: Vehicle safety system status messages (future)
//!
//! # Implemented Messages
//!
//! ## Advanced Emergency Braking System (AEBS)
//! - `AEBS2` - Advanced Emergency Braking System 2 (3 signals)
//!
//! # Signal Types Supported
//!
//! - **Type 1: Simple Integer** - Direct bit manipulation for status and counters
//! - **Type 2: Scaled Float** - Factor/offset scaling for pressure and performance values (future)
//!
//! # Usage Example
//!
//! ```rust
//! use cando_messages::{DeviceId, j1939::AEBS2};
//!
//! // Create advanced emergency braking status message
//! let message = AEBS2 {
//!     device_id: DeviceId::from(0x42),
//!     dv_atvt_dd_f_advd_eb_sst: 1,     // Driver activated/deactivated advanced EBS status
//!     aebs_2_message_counter: 10,   // Message counter for integrity
//!     aebs_2_message_checksum: 5,   // Message checksum for validation
//! };
//!
//! // Encode to CAN frame
//! let (can_id, data) = message.encode().unwrap();
//!
//! // Decode back
//! let decoded = AEBS2::decode(can_id, &data).unwrap();
//! ```

// Re-export all encoder functions for public use

// Note: Implementations are automatically available on the generated message types
// when this module is imported, so no explicit re-export needed for impl methods
