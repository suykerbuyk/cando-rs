//! Power Management System Messages
//!
//! This module contains J1939 messages related to power management, energy storage,
//! and electrical systems. These messages are critical for EMP (Engineered Machined Products)
//! fan, pump, and power supply integration.
//!
//! # Message Categories
//!
//! ## High Voltage Energy Storage System (HVESS)
//! - Power control and management
//! - Cell balancing and battery management
//! - Voltage, current, and temperature monitoring
//! - Charging and discharging control
//!
//! ## DC-DC Converters (DCDC)
//! - Power supply regulation and conversion
//! - Voltage level management
//! - Power distribution control
//!
//! ## Motor/Generator Control (MG)
//! - Inverter control for fans and pumps
//! - Motor torque and speed management
//! - Power electronics control
//!
//! # EMP Integration Priority
//!
//! These messages have been prioritized based on the EMP Component CAN Communication
//! specification and are directly applicable to:
//! - Fan motor control and power management
//! - Pump motor control and power management
//! - Power supply monitoring and protection
//! - Temperature monitoring for over-temperature protection
//! - Voltage monitoring for over/under-voltage protection
//!
//! # Implementation Notes
//!
//! All power management messages use proper 29-bit CAN ID masking to ensure
//! compliance with CAN extended frame limits. The raw CAN IDs from the DBC
//! are masked using `CAN_EFF_MASK` to prevent issues with bits above the
//! 29-bit limit.


// Future: Generated implementations could be imported here

// Re-export encoder functions for external use
// Re-export implementation types (they're already available via the main j1939 module)
