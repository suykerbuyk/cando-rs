//! J1939 Simulator Library
//!
//! Core library for the J1939 device simulator, providing:
//! - [`SimulatorState`] - device state and parameters
//! - [`MessageStatus`] - message processing result type
//! - Message processing via [`SimulatorState::process_incoming_message`]
//! - CAN frame generation via [`SimulatorState::generate_can_frames`]
//! - Physics simulation via [`SimulatorState::update_physics`]

pub mod broadcasts;
pub mod handlers;
pub mod physics;
pub mod state;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Instant;

use cando_simulator_common::{
    MessageTracking, ReceivedMessage, Result as CommonResult, SimulatorState as SimulatorStateTrait,
    StateQueryable,
};

pub use state::*;

/// Message processing status returned by `process_incoming_message()`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    /// Message was recognized and processed successfully
    Recognized,
    /// Message ID was not recognized (unknown message type)
    Unrecognized,
    /// Message ID was recognized but decode failed
    DecodeFailed,
    /// Message was ignored (self-reception - message from this simulator)
    Ignored,
}

/// Current device state and parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatorState {
    // Device identification
    pub device_id: u32,

    // Message tracking for test verification
    #[serde(skip, default)]
    pub recent_messages: VecDeque<ReceivedMessage>,
    #[serde(skip, default = "Instant::now")]
    pub simulator_start_time: Instant,

    // Test isolation control
    pub broadcast_paused: bool,

    // Subsystem state (flatten preserves JSON serialization compatibility)
    #[serde(flatten)]
    pub crash: CrashState,
    #[serde(flatten)]
    pub sensors: SensorState,
    #[serde(flatten)]
    pub motor: MotorState,
    #[serde(flatten)]
    pub hvess: HvessState,
    #[serde(flatten)]
    pub dcdc: DcdcState,
    #[serde(flatten)]
    pub engine: EngineState,
    #[serde(flatten)]
    pub transmission: TransmissionState,
    #[serde(flatten)]
    pub braking: BrakingState,
    #[serde(flatten)]
    pub diagnostics: DiagnosticsState,
    #[serde(flatten)]
    pub power_supply: PowerSupplyState,
    #[serde(flatten)]
    pub thermal: ThermalState,
    #[serde(flatten)]
    pub vehicle: VehicleState,
    #[serde(flatten)]
    pub aftertreatment: AftertreatmentState,
    #[serde(flatten)]
    pub ev_charging: EvChargingState,

    // Timing
    #[serde(skip)]
    pub last_update_time: Option<DateTime<Utc>>,
    pub uptime_seconds: u64,
}

// Implement SimulatorState trait for WebSocket broadcasting
impl SimulatorStateTrait for SimulatorState {}

// Implement StateQueryable trait for state query framework
impl StateQueryable for SimulatorState {
    fn get_state_json(&self) -> CommonResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

// Implement MessageTracking trait for ACK framework
impl MessageTracking for SimulatorState {
    fn get_recent_messages(&self) -> &VecDeque<ReceivedMessage> {
        &self.recent_messages
    }

    fn get_recent_messages_mut(&mut self) -> &mut VecDeque<ReceivedMessage> {
        &mut self.recent_messages
    }

    fn get_simulator_start_time(&self) -> Instant {
        self.simulator_start_time
    }
}

impl Default for SimulatorState {
    fn default() -> Self {
        Self {
            device_id: 0x8A,
            recent_messages: VecDeque::new(),
            simulator_start_time: Instant::now(),
            broadcast_paused: false,
            crash: CrashState::default(),
            sensors: SensorState::default(),
            motor: MotorState::default(),
            hvess: HvessState::default(),
            dcdc: DcdcState::default(),
            engine: EngineState::default(),
            transmission: TransmissionState::default(),
            braking: BrakingState::default(),
            diagnostics: DiagnosticsState::default(),
            power_supply: PowerSupplyState::default(),
            thermal: ThermalState::default(),
            vehicle: VehicleState::default(),
            aftertreatment: AftertreatmentState::default(),
            ev_charging: EvChargingState::default(),
            last_update_time: None,
            uptime_seconds: 0,
        }
    }
}
