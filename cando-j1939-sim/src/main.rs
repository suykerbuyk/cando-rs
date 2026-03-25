//! J1939 (SAE J1939 Vehicle Bus Standard) Device Simulator
//!
//! This simulator emulates J1939-compliant devices on a CAN bus, providing realistic
//! responses and state management for vehicle bus standard messages.
//!
//! Features:
//! - Listens for J1939 commands on configurable CAN interfaces
//! - Responds with CN (Crash Notification), WAND (Wand Angle), and LDISP (Linear Displacement)
//! - Stateful behavior with realistic state transitions
//! - Console UI for manual control and monitoring
//! - WebSocket interface for remote control and telemetry streaming

use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use socketcan::{EmbeddedFrame, Frame};

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::broadcast;

// Use common simulator library
use cando_messages::common::CAN_EFF_MASK;
use cando_simulator_common::{
    CanInterface, CommonSimulatorArgs, J1939DeviceIdValidator,
    ResolvedConfig, StateQueryable, parse_device_id,
    start_simulator_websocket,
};

use cando_messages::common::DeviceId;
use cando_messages::j1939::DM03;

// Import simulator core types from library
use cando_j1939_sim::SimulatorState;

#[cfg(test)]
use cando_j1939_sim::{CrashState, DiagnosticsState, MessageStatus, SensorState};
#[cfg(test)]
use cando_messages::j1939::*;
#[cfg(test)]
use cando_simulator_common::{create_can_frame, FrameType};

/// Command-line arguments for the J1939 simulator
#[derive(Parser, Debug)]
#[command(
    version,
    about = "J1939 (SAE J1939 Vehicle Bus Standard) Device Simulator"
)]
pub struct Args {
    /// Common simulator arguments (interface, websocket_port, debug, etc.)
    #[command(flatten)]
    common: CommonSimulatorArgs,

    /// Device ID (0x00-0xFF)
    #[arg(long, short = 'I', default_value = "0x8A")]
    device_id: String,

    /// Generate man page and exit (internal use)
    #[arg(long = "generate-manpage", hide = true)]
    generate_manpage: bool,

    #[arg(skip)]
    pub test_mode: bool,
}


/// WebSocket message types for remote control
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    /// Get current device state
    GetState,
    /// Set crash state
    SetCrash {
        detected: bool,
        crash_type: u64,
    },
    /// Set wand angle
    SetWandAngle {
        angle: f64,
    },
    /// Set linear displacement
    SetDisplacement {
        displacement: f64,
    },

    /// EMP Motor/Generator 1 Controls
    SetMG1Speed {
        speed_setpoint: f64,
    },
    SetMG1Torque {
        torque_setpoint: f64,
    },
    SetMG1TorqueLimits {
        max_torque: f64,
        min_torque: f64,
    },

    /// EMP Motor/Generator 2 Controls
    SetMG2Speed {
        speed_setpoint: f64,
    },
    SetMG2Torque {
        torque_setpoint: f64,
    },

    /// HVESS (High Voltage Energy Storage System) Controls
    SetHVESS {
        power_down: bool,
        cell_balancing: bool,
    },
    SetHVESSVoltage {
        voltage_level: f64,
    },
    SetHVESSD2 {
        fast_update_state_of_charge: f64,
        highest_cell_voltage: f64,
        lowest_cell_voltage: f64,
        cell_voltage_differential_status: u64,
    },
    SetHVESSD3 {
        highest_cell_temperature: f64,
        lowest_cell_temperature: f64,
        average_cell_temperature: f64,
        cell_temp_differential_status: u64,
    },
    SetHVESSFS1 {
        fan_speed_status: u64,
        fan_status_reason_code: u64,
        fan_command_status: u64,
        fan_speed: f64,
        fan_medium_temperature: f64,
        fan_power: f64,
        fan_service_indicator: u64,
        fan_operating_status: u64,
        fan_status1_instance: u64,
    },
    SetHVESSThermal {
        coolant_temp: f64,
        electronics_temp: f64,
    },

    /// DC-DC Converter Controls
    SetDCDC {
        operational_command: u64,
        low_voltage: f64,
        high_voltage: f64,
    },

    /// Engine Controls
    SetEngineRPM {
        rpm: f64,
    },
    SetEngineLoad {
        load_percent: f64,
    },
    SetEngineTemp {
        coolant_temp: f64,
        exhaust_temp: f64,
    },
    SetTransmission {
        gear: u64,
    },
    SetETC9 {
        current_preselection_gear: f64,
        input_shaft1_speed: f64,
        input_shaft2_speed: f64,
        selected_preselection_gear: f64,
    },

    /// ALTC (Alternator Control) Controls
    SetALTC {
        setpoint_voltage: f64,
        excitation_current_limit: f64,
        torque_ramp_time: f64,
        torque_ramp_max_speed: f64,
    },

    /// GC2 (Generator Control 2) Controls
    SetGC2 {
        engine_load_setpoint: f64,
        derate_inhibit: u64,
        governing_bias: f64,
    },

    /// DCACAI1S2 (DC/AC Accessory Inverter 1 Status 2) Controls
    SetDCACAI1S2 {
        desired_power: f64,
        desired_voltage: f64,
        desired_current: f64,
        desired_ground_voltage: f64,
    },

    /// DCDC1OS (DC/DC Converter 1 Operating Status) Controls
    SetDCDC1OS {
        hvil_status: u64,
        loadshed_request: u64,
        operational_status: u64,
        operating_status_counter: u64,
        operating_status_crc: u64,
        power_limit_high_side_current: u64,
        power_limit_low_side_current: u64,
        power_limit_high_side_voltage_min: u64,
        power_limit_high_side_voltage_max: u64,
        power_limit_low_side_voltage_min: u64,
        power_limit_low_side_voltage_max: u64,
        power_limit_converter_temperature: u64,
        power_limit_electronic_filter_temperature: u64,
        power_limit_power_electronics_temperature: u64,
        power_limit_sli_battery_terminal_voltage: u64,
        power_limit_sli_battery_terminal_current: u64,
        power_limit_sli_battery_terminal_temperature: u64,
        power_limit_undefined_reason: u64,
    },

    /// DCDC1SBS (DC/DC Converter 1 SLI Battery Status) Controls
    SetDCDC1SBS {
        terminal_current: f64,
        terminal_voltage: f64,
        terminal_temperature: f64,
    },

    /// DCDC1S2 (DC/DC Converter 1 Status 2) Controls
    SetDCDC1S2 {
        high_side_power: f64,
        low_side_power: f64,
        high_side_ground_voltage: f64,
    },

    /// DCACAI1V (DC/AC Accessory Inverter 1 Voltage) Controls
    SetDCACAI1V {
        ignition_voltage: f64,
        unswitched_voltage: f64,
    },

    /// GTRACE (Generator Trip Energy) Controls
    SetGTRACE {
        kwh_export: u64,
        kvarh_export: u64,
    },

    /// DCDC2SBS (DC/DC Converter 2 SLI Battery Status) Controls
    SetDCDC2SBS {
        terminal_voltage: f64,
        terminal_current: f64,
        terminal_temperature: f64,
    },

    /// DCDC2S2 (DC/DC Converter 2 Status 2) Controls
    SetDCDC2S2 {
        high_side_power: f64,
        low_side_power: f64,
        high_side_ground_voltage: f64,
    },

    /// HVESSTS1 (HVESS Thermal Management System Status 1) Controls - Phase 2 Pumps
    SetHVESSTS1 {
        system_input_power: f64,
        hv_input_power: f64,
        compressor_speed: f64,
        relative_humidity: f64,
        heater_status: u64,
        hvil_status: u64,
        system_mode: u64,
        coolant_level: u64,
        coolant_level_full: u64,
    },

    /// HVESSTC1 (HVESS Thermal Management System Temperature Control) Controls - Phase 2 Pumps
    SetHVESSTC1 {
        intake_coolant_temp_request: f64,
        outlet_coolant_temp_request: f64,
        coolant_flow_rate_request: f64,
        heater_enable_command: u64,
        coolant_pump_enable_code: u64,
        compressor_enable_code: u64,
    },

    /// HVESSTC2 (HVESS Thermal Management System Temperature Control 2) Controls - Phase 2 Pumps
    SetHVESSTC2 {
        pump_speed_command: f64,
        pump_speed_command_percent: f64,
        compressor_speed_command: f64,
        compressor_speed_command_percent: f64,
    },

    /// ETCC3 (Electronic Transmission Controller Clutch 3) Engine Thermal Control
    SetETCC3 {
        etc_bypass_actuator_1: u64,
        turbo_wastegate_actuator_1: u64,
        cylinder_head_bypass_actuator: u64,
        throttle_valve_1: u64,
        etc_bypass_pass_actuator_1: u64,
        etc_bypass_pass_actuator_2: u64,
        turbo_wastegate_actuator_2: u64,
    },

    /// AEBS (Advanced Emergency Braking System) Controls
    SetAEBS {
        enabled: bool,
        brake_demand: f64,
    },

    /// DM01 - Diagnostic Message Controls
    SetDM01 {
        protect_lamp: u64,
        amber_lamp: u64,
        red_stop_lamp: u64,
        mil: u64,
        active_dtc_spn: u64,
        active_dtc_fmi: u64,
        active_dtc_oc: u64,
    },

    /// DM02 - Previously Active Diagnostic Message Controls
    SetDM02 {
        protect_lamp: u64,
        amber_lamp: u64,
        red_stop_lamp: u64,
        mil: u64,
        previously_active_dtc_spn: u64,
        previously_active_dtc_fmi: u64,
        previously_active_dtc_oc: u64,
    },

    /// DM03 - Diagnostic Clear/Reset Command Controls
    SetDM03 {
        clear_operations_enabled: bool,
        auto_response_enabled: bool,
        command_generation_enabled: bool,
        target_device_id: u8,
        command_interval_seconds: u64,
        trigger_clear_command: bool,
    },

    /// Reset state to defaults
    Reset,
    /// Pause CAN message broadcasting for test isolation
    PauseBroadcast,
    /// Resume CAN message broadcasting
    ResumeBroadcast,
    /// State update
    StateUpdate {
        state: Box<SimulatorState>,
    },
    /// Response containing complete simulator state as JSON (state query framework)
    StateResponse {
        state_json: String,
    },
    /// Error response
    Error {
        message: String,
    },
    /// Wait for a specific CAN message to be received
    WaitForMessage {
        can_id: u32,
        timeout_ms: u64,
    },
    /// Response to WaitForMessage command
    MessageReceived {
        can_id: u32,
        timestamp_ms: u64,
        elapsed_ms: u64,
        found: bool,
    },
}

/// Main simulator structure
pub struct J1939Simulator {
    device_id: u32,
    can_interface: Option<CanInterface>,
    interface_name: String,
    state: Arc<Mutex<SimulatorState>>,
    websocket_tx: broadcast::Sender<String>,
    websocket_port: u16,
    debug: bool,
    no_console: bool,
}

impl J1939Simulator {
    /// Create a new J1939 simulator instance
    pub fn new(args: Args, config: ResolvedConfig) -> Result<Self> {
        // Initialize tracing from common args
        args.common.init_tracing();

        // Print startup banner with configuration details
        config.print_banner("J1939");

        let device_id = parse_j1939_device_id(&args.device_id)?;
        let can_interface = if !args.test_mode {
            Some(CanInterface::open(&config.interface)?)
        } else {
            None
        };

        let (websocket_tx, websocket_rx) = broadcast::channel(100);
        // Keep the receiver alive to prevent the broadcast channel from closing
        // WebSocket clients need at least one receiver to exist for the channel to work
        std::mem::forget(websocket_rx);

        let initial_state = SimulatorState {
            device_id,
            ..Default::default()
        };

        let simulator = Self {
            device_id,
            can_interface,
            interface_name: config.interface.clone(),
            state: Arc::new(Mutex::new(initial_state)),
            websocket_tx: websocket_tx.clone(),
            websocket_port: config.websocket_port,
            debug: config.debug,
            no_console: config.no_console,
        };

        // Start subsystems unless in test mode
        // Thread startup will be handled in run() method

        Ok(simulator)
    }

    /// Start physics simulation thread
    fn start_physics_simulation(&self) -> Result<()> {
        let state = Arc::clone(&self.state);
        let websocket_tx = self.websocket_tx.clone();

        thread::spawn(move || {
            let mut last_update = Instant::now();
            let update_interval = Duration::from_millis(50); // 20 Hz

            loop {
                thread::sleep(update_interval);

                let now = Instant::now();
                let delta_time = now.duration_since(last_update).as_secs_f64();
                last_update = now;

                // Update state
                let mut state_lock = state
                    .lock()
                    .expect("Failed to acquire state lock in physics update thread");
                state_lock.update_physics(delta_time);
                state_lock.uptime_seconds = now.elapsed().as_secs();

                // Broadcast state update via WebSocket
                let state_update = WebSocketMessage::StateUpdate {
                    state: Box::new(state_lock.clone()),
                };
                if let Ok(json) = serde_json::to_string(&state_update) {
                    let _ = websocket_tx.send(json);
                }
            }
        });

        Ok(())
    }

    /// Start CAN message sender thread
    fn start_can_sender(&self) -> Result<()> {
        if self.can_interface.is_none() {
            return Ok(());
        }

        let state = Arc::clone(&self.state);
        let interface_name = self.interface_name.clone();
        let debug = self.debug;

        thread::spawn(move || {
            let interface = match CanInterface::open(&interface_name) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("Failed to open CAN interface: {}", e);
                    return;
                }
            };

            loop {
                thread::sleep(Duration::from_millis(100)); // 10 Hz for crash, 20 Hz for others

                let state_lock = state
                    .lock()
                    .expect("Failed to acquire state lock for broadcast");

                // Skip broadcasting if paused (for test isolation)
                if state_lock.broadcast_paused {
                    drop(state_lock);
                    continue;
                }

                let frames = state_lock.generate_can_frames();
                drop(state_lock);

                for frame in frames {
                    if let Err(e) = interface.write_frame(&frame) {
                        if debug {
                            eprintln!("Failed to send CAN frame: {}", e);
                        }
                    } else if debug {
                        println!("📤 Sent CAN frame: {:?}", frame);
                    }
                }
            }
        });

        Ok(())
    }

    /// Start CAN receiver thread to process incoming control messages
    fn start_can_receiver(&self) -> Result<()> {
        if self.can_interface.is_none() {
            return Ok(());
        }

        let state = Arc::clone(&self.state);
        let interface_name = self.interface_name.clone();
        let debug = self.debug;

        thread::spawn(move || {
            let interface = match CanInterface::open(&interface_name) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("Failed to open CAN receiver interface: {}", e);
                    return;
                }
            };

            loop {
                match interface.read_frame() {
                    Ok(frame) => {
                        let can_id = frame.raw_id() & CAN_EFF_MASK; // Mask to 29-bit CAN ID for extended frames
                        let data = frame.data();

                        if debug {
                            println!(
                                "📥 Received CAN frame: ID=0x{:08X}, DLC={}, Data={:02X?}",
                                can_id,
                                data.len(),
                                data
                            );
                        }

                        // Process the message
                        if let Ok(mut state_lock) = state.try_lock()
                            && let Err(e) = state_lock.process_incoming_message(can_id, data)
                            && debug
                        {
                            eprintln!("Failed to process incoming message: {}", e);
                        }
                    }
                    Err(e) => {
                        // Handle WouldBlock by sleeping (non-blocking mode)
                        let error_msg = e.to_string();
                        if error_msg.contains("WouldBlock") || error_msg.contains("would block") {
                            thread::sleep(Duration::from_millis(10));
                        } else if debug {
                            eprintln!("CAN read error: {}", e);
                            thread::sleep(Duration::from_millis(100));
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Start console UI thread
    fn start_console_ui(&self) -> Result<()> {
        let state = Arc::clone(&self.state);

        thread::spawn(move || {
            println!("\n📟 J1939 Simulator Console");
            println!("Type 'help' for available commands\n");

            loop {
                print!("> ");
                io::stdout()
                    .flush()
                    .expect("Failed to flush stdout in console thread");

                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_err() {
                    break;
                }

                let input = input.trim();
                if input.is_empty() {
                    continue;
                }

                match input {
                    "help" => print_help(),
                    "status" => print_status(&state),
                    "reset" => {
                        let mut state_lock = state
                            .lock()
                            .expect("Failed to acquire state lock for console command");
                        *state_lock = SimulatorState::default();
                        println!("✅ State reset to defaults");
                    }
                    "quit" | "exit" => {
                        println!("Exiting...");
                        std::process::exit(0);
                    }
                    _ if input.starts_with("set ") => {
                        handle_set_command(&input[4..], &state);
                    }
                    _ => {
                        println!("Unknown command. Type 'help' for available commands.");
                    }
                }
            }
        });

        Ok(())
    }

    /// Run the simulator (blocking)
    pub fn run(&mut self) -> Result<()> {
        // Start WebSocket server with shared initialization
        let test_mode = self.can_interface.is_none();
        let no_websocket = false; // J1939 doesn't have no_websocket flag

        // Create command handler for WebSocket messages
        let state_for_handler = self.state.clone();
        let command_handler: Option<cando_simulator_common::CommandHandler> =
            Some(Arc::new(move |json_text: String| {
                // Deserialize incoming JSON to WebSocketMessage
                match serde_json::from_str::<WebSocketMessage>(&json_text) {
                    Ok(command) => {
                        // Handle command and get response
                        let response = handle_websocket_message(command, &state_for_handler);

                        // Serialize response back to JSON
                        match serde_json::to_string(&response) {
                            Ok(response_json) => Some(response_json),
                            Err(e) => {
                                eprintln!("Failed to serialize WebSocket response: {}", e);
                                None
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize WebSocket command: {}", e);
                        None
                    }
                }
            }));

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                start_simulator_websocket(
                    self.state.clone(),
                    self.websocket_tx.clone(),
                    self.websocket_port,
                    test_mode,
                    no_websocket,
                    "cando-j1939-sim",
                    command_handler,
                )
                .await
            })
        })?;

        // Start all background threads
        self.start_physics_simulation()?;
        self.start_can_sender()?;
        self.start_can_receiver()?;

        if !self.no_console {
            self.start_console_ui()?;
        }

        if self.no_console {
            println!(
                "\n✅ J1939 Simulator running on {} (Device ID: 0x{:02X}) - No console mode",
                self.interface_name, self.device_id
            );
            // Keep main thread alive
            loop {
                thread::sleep(Duration::from_secs(60));
            }
        } else {
            println!(
                "\n✅ J1939 Simulator running on {} (Device ID: 0x{:02X})",
                self.interface_name, self.device_id
            );
            println!("📡 Broadcasting J1939 status messages (18 message types)");
            println!("📥 Listening for incoming J1939 control commands");
            println!("🌐 WebSocket interface available for remote control");
            println!("Press Ctrl+C to stop\n");

            // Keep main thread alive
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

/// Handle WebSocket messages
#[allow(dead_code)]
fn handle_websocket_message(
    msg: WebSocketMessage,
    state: &Arc<Mutex<SimulatorState>>,
) -> WebSocketMessage {
    match msg {
        WebSocketMessage::GetState => {
            // Use StateQueryable trait for state query framework
            let state_lock = state
                .lock()
                .expect("Failed to acquire state lock for state query");
            match state_lock.get_state_json() {
                Ok(state_json) => WebSocketMessage::StateResponse { state_json },
                Err(e) => WebSocketMessage::Error {
                    message: format!("Failed to serialize state: {}", e),
                },
            }
        }
        WebSocketMessage::WaitForMessage {
            can_id,
            timeout_ms: _,
        } => {
            // Use MessageTracking trait for ACK framework
            let state_lock = state
                .lock()
                .expect("Failed to acquire state lock for message tracking");

            // Look for the message in recent_messages (most recent first)
            for msg in state_lock.recent_messages.iter().rev() {
                if msg.can_id == can_id && msg.processed {
                    return WebSocketMessage::MessageReceived {
                        can_id,
                        timestamp_ms: msg.timestamp_ms,
                        elapsed_ms: 0, // Immediate check, no elapsed time
                        found: true,
                    };
                }
            }

            // Message not found in history
            WebSocketMessage::MessageReceived {
                can_id,
                timestamp_ms: 0,
                elapsed_ms: 0,
                found: false,
            }
        }
        WebSocketMessage::SetCrash {
            detected,
            crash_type,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for set command");
            state_lock.crash.crash_detected = detected;
            state_lock.crash.crash_type = crash_type as u8;
            if detected {
                state_lock.crash.crash_counter += 1;
            }
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetWandAngle { angle } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for increment command");
            state_lock.sensors.target_wand_angle = angle.clamp(-250.0, 252.19);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetDisplacement { displacement } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for decrement command");
            state_lock.sensors.target_displacement = displacement.clamp(0.0, 6425.5);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // EMP Motor/Generator 1 Controls
        WebSocketMessage::SetMG1Speed { speed_setpoint } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for MG1 speed command");
            state_lock.motor.mg1_speed_setpoint = speed_setpoint.clamp(-125.0, 125.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetMG1Torque { torque_setpoint } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for MG1 torque command");
            state_lock.motor.mg1_torque_setpoint = torque_setpoint.clamp(-125.0, 125.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetMG1TorqueLimits {
            max_torque,
            min_torque,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for MG1 torque limits command");
            state_lock.motor.mg1_max_torque = max_torque.clamp(-125.0, 125.0);
            state_lock.motor.mg1_min_torque = min_torque.clamp(-125.0, 125.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // EMP Motor/Generator 2 Controls
        WebSocketMessage::SetMG2Speed { speed_setpoint } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for MG2 speed command");
            state_lock.motor.mg2_speed_setpoint = speed_setpoint.clamp(-125.0, 125.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetMG2Torque { torque_setpoint } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for MG2 torque command");
            state_lock.motor.mg2_torque_setpoint = torque_setpoint.clamp(-125.0, 125.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // HVESS Controls
        WebSocketMessage::SetHVESS {
            power_down,
            cell_balancing,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for HVESS command");
            state_lock.hvess.hvess_power_down_command = power_down;
            state_lock.hvess.hvess_cell_balancing_command = cell_balancing;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetHVESSVoltage { voltage_level } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for HVESS voltage command");
            state_lock.hvess.hvess_voltage_level = voltage_level.clamp(400.0, 1000.0);
            state_lock.hvess.hvess_bus_voltage = state_lock.hvess.hvess_voltage_level;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetHVESSD2 {
            fast_update_state_of_charge,
            highest_cell_voltage,
            lowest_cell_voltage,
            cell_voltage_differential_status,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for HVESSD2 command");
            state_lock.hvess.hvess_fast_update_state_of_charge =
                fast_update_state_of_charge.clamp(0.0, 100.4);
            state_lock.hvess.hvess_highest_cell_voltage = highest_cell_voltage.clamp(0.0, 64.255);
            state_lock.hvess.hvess_lowest_cell_voltage = lowest_cell_voltage.clamp(0.0, 64.255);
            state_lock.hvess.hvess_cell_voltage_differential_status =
                cell_voltage_differential_status.clamp(0, 15);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetHVESSD3 {
            highest_cell_temperature,
            lowest_cell_temperature,
            average_cell_temperature,
            cell_temp_differential_status,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for HVESSD3 command");
            state_lock.hvess.hvess_highest_cell_temperature =
                highest_cell_temperature.clamp(-273.0, 1734.97);
            state_lock.hvess.hvess_lowest_cell_temperature =
                lowest_cell_temperature.clamp(-273.0, 1734.97);
            state_lock.hvess.hvess_average_cell_temperature =
                average_cell_temperature.clamp(-273.0, 1734.97);
            state_lock.hvess.hvess_cell_temp_differential_status =
                cell_temp_differential_status.clamp(0, 3);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetHVESSFS1 {
            fan_speed_status,
            fan_status_reason_code,
            fan_command_status,
            fan_speed,
            fan_medium_temperature,
            fan_power,
            fan_service_indicator,
            fan_operating_status,
            fan_status1_instance,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for HVESSFS1 command");
            state_lock.hvess.hvess_fan_speed_status = fan_speed_status.clamp(0, 3);
            state_lock.hvess.hvess_fan_status_reason_code = fan_status_reason_code.clamp(0, 15);
            state_lock.hvess.hvess_fan_command_status = fan_command_status.clamp(0, 3);
            state_lock.hvess.hvess_fan_speed = fan_speed.clamp(0.0, 32127.5);
            state_lock.hvess.hvess_fan_medium_temperature = fan_medium_temperature.clamp(-273.0, 1734.97);
            state_lock.hvess.hvess_fan_power = fan_power.clamp(0.0, 32127.5);
            state_lock.hvess.hvess_fan_service_indicator = fan_service_indicator.clamp(0, 3);
            state_lock.hvess.hvess_fan_operating_status = fan_operating_status.clamp(0, 3);
            state_lock.hvess.hvess_fan_status1_instance = fan_status1_instance.clamp(0, 15);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetHVESSThermal {
            coolant_temp,
            electronics_temp,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for HVESS thermal command");
            state_lock.hvess.hvess_coolant_temp = coolant_temp.clamp(-40.0, 100.0);
            state_lock.hvess.hvess_electronics_temp = electronics_temp.clamp(-40.0, 150.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DC-DC Converter Controls
        WebSocketMessage::SetDCDC {
            operational_command,
            low_voltage,
            high_voltage,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DCDC command");
            state_lock.dcdc.dcdc_operational_command = operational_command as u8;
            state_lock.dcdc.dcdc_low_side_voltage_setpoint = low_voltage.clamp(12.0, 60.0);
            state_lock.dcdc.dcdc_high_side_voltage_setpoint = high_voltage.clamp(400.0, 1000.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // Engine Controls
        WebSocketMessage::SetEngineRPM { rpm } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for engine RPM command");
            state_lock.engine.engine_speed = rpm.clamp(600.0, 6000.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetEngineLoad { load_percent } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for engine load command");
            state_lock.engine.engine_load = load_percent.clamp(0.0, 100.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetEngineTemp {
            coolant_temp,
            exhaust_temp,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for engine temp command");
            state_lock.engine.engine_coolant_temp = coolant_temp.clamp(0.0, 120.0);
            state_lock.engine.engine_exhaust_temp = exhaust_temp.clamp(100.0, 1000.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetTransmission { gear } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for transmission command");
            state_lock.transmission.transmission_gear = gear.clamp(0, 16);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::SetETC9 {
            current_preselection_gear,
            input_shaft1_speed,
            input_shaft2_speed,
            selected_preselection_gear,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for ETC9 command");
            state_lock.transmission.etc9_current_preselection_gear =
                current_preselection_gear.clamp(-125.0, 125.0);
            state_lock.transmission.etc9_input_shaft1_speed = input_shaft1_speed.clamp(0.0, 8031.875);
            state_lock.transmission.etc9_input_shaft2_speed = input_shaft2_speed.clamp(0.0, 8031.875);
            state_lock.transmission.etc9_selected_preselection_gear =
                selected_preselection_gear.clamp(-125.0, 125.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // ALTC Alternator Control
        WebSocketMessage::SetALTC {
            setpoint_voltage,
            excitation_current_limit,
            torque_ramp_time,
            torque_ramp_max_speed,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for ALTC command");
            state_lock.power_supply.altc_setpoint_voltage = setpoint_voltage.clamp(0.0, 64.255);
            state_lock.power_supply.altc_excitation_current_limit = excitation_current_limit.clamp(0.0, 62.5);
            state_lock.power_supply.altc_torque_ramp_time = torque_ramp_time.clamp(0.1, 25.0);
            state_lock.power_supply.altc_torque_ramp_max_speed = torque_ramp_max_speed.clamp(0.0, 8000.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // GC2 Generator Control 2
        WebSocketMessage::SetGC2 {
            engine_load_setpoint,
            derate_inhibit,
            governing_bias,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for GC2 command");
            state_lock.power_supply.gc2_engine_load_setpoint = engine_load_setpoint.clamp(0.0, 32127.5);
            state_lock.power_supply.gc2_derate_inhibit = derate_inhibit.clamp(0, 3) as u8;
            state_lock.power_supply.gc2_governing_bias = governing_bias.clamp(-125.0, 125.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DCACAI1S2 DC/AC Accessory Inverter 1 Status 2
        WebSocketMessage::SetDCACAI1S2 {
            desired_power,
            desired_voltage,
            desired_current,
            desired_ground_voltage,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DCACAI1S2 command");
            state_lock.power_supply.dcacai1s2_desired_power = desired_power.clamp(0.0, 4015.9375);
            state_lock.power_supply.dcacai1s2_desired_voltage = desired_voltage.clamp(0.0, 3212.75);
            state_lock.power_supply.dcacai1s2_desired_current = desired_current.clamp(0.0, 4015.9375);
            state_lock.power_supply.dcacai1s2_desired_ground_voltage =
                desired_ground_voltage.clamp(0.0, 3212.75);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DCDC1OS DC/DC Converter 1 Operating Status
        WebSocketMessage::SetDCDC1OS {
            hvil_status,
            loadshed_request,
            operational_status,
            operating_status_counter,
            operating_status_crc,
            power_limit_high_side_current,
            power_limit_low_side_current,
            power_limit_high_side_voltage_min,
            power_limit_high_side_voltage_max,
            power_limit_low_side_voltage_min,
            power_limit_low_side_voltage_max,
            power_limit_converter_temperature,
            power_limit_electronic_filter_temperature,
            power_limit_power_electronics_temperature,
            power_limit_sli_battery_terminal_voltage,
            power_limit_sli_battery_terminal_current,
            power_limit_sli_battery_terminal_temperature,
            power_limit_undefined_reason,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DCDC1OS command");
            state_lock.dcdc.dcdc1os_hvil_status = hvil_status.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_loadshed_request = loadshed_request.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_operational_status = operational_status.clamp(0, 15) as u8;
            state_lock.dcdc.dcdc1os_operating_status_counter =
                operating_status_counter.clamp(0, 15) as u8;
            state_lock.dcdc.dcdc1os_operating_status_crc = operating_status_crc.clamp(0, 250) as u8;
            state_lock.dcdc.dcdc1os_power_limit_high_side_current =
                power_limit_high_side_current.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_low_side_current =
                power_limit_low_side_current.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_high_side_voltage_min =
                power_limit_high_side_voltage_min.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_high_side_voltage_max =
                power_limit_high_side_voltage_max.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_low_side_voltage_min =
                power_limit_low_side_voltage_min.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_low_side_voltage_max =
                power_limit_low_side_voltage_max.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_converter_temperature =
                power_limit_converter_temperature.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_electronic_filter_temperature =
                power_limit_electronic_filter_temperature.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_power_electronics_temperature =
                power_limit_power_electronics_temperature.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_sli_battery_terminal_voltage =
                power_limit_sli_battery_terminal_voltage.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_sli_battery_terminal_current =
                power_limit_sli_battery_terminal_current.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_sli_battery_terminal_temperature =
                power_limit_sli_battery_terminal_temperature.clamp(0, 3) as u8;
            state_lock.dcdc.dcdc1os_power_limit_undefined_reason =
                power_limit_undefined_reason.clamp(0, 3) as u8;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DCDC1SBS DC/DC Converter 1 SLI Battery Status
        WebSocketMessage::SetDCDC1SBS {
            terminal_current,
            terminal_voltage,
            terminal_temperature,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DCDC1SBS command");
            state_lock.dcdc.dcdc1sbs_terminal_current = terminal_current.clamp(-3212.7, 3212.8);
            state_lock.dcdc.dcdc1sbs_terminal_voltage = terminal_voltage.clamp(0.0, 642.55);
            state_lock.dcdc.dcdc1sbs_terminal_temperature = terminal_temperature.clamp(-40.0, 210.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DCDC1S2 DC/DC Converter 1 Status 2
        WebSocketMessage::SetDCDC1S2 {
            high_side_power,
            low_side_power,
            high_side_ground_voltage,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DCDC1S2 command");
            state_lock.dcdc.dcdc1s2_high_side_power = high_side_power.clamp(-1600.0, 1612.75);
            state_lock.dcdc.dcdc1s2_low_side_power = low_side_power.clamp(-1600.0, 1612.75);
            state_lock.dcdc.dcdc1s2_high_side_ground_voltage =
                high_side_ground_voltage.clamp(0.0, 3212.75);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DCACAI1V Controls
        WebSocketMessage::SetDCACAI1V {
            ignition_voltage,
            unswitched_voltage,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DCACAI1V command");
            state_lock.power_supply.dcacai1v_ignition_voltage = ignition_voltage.clamp(0.0, 3212.75);
            state_lock.power_supply.dcacai1v_unswitched_voltage = unswitched_voltage.clamp(0.0, 3212.75);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // GTRACE Controls
        WebSocketMessage::SetGTRACE {
            kwh_export,
            kvarh_export,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for GTRACE command");
            state_lock.power_supply.gtrace_kwh_export = kwh_export.clamp(0, 4211081215) as u32;
            state_lock.power_supply.gtrace_kvarh_export = kvarh_export.clamp(0, 4211081215) as u32;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DCDC2SBS Controls
        WebSocketMessage::SetDCDC2SBS {
            terminal_voltage,
            terminal_current,
            terminal_temperature,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DCDC2SBS command");
            state_lock.dcdc.dcdc2sbs_terminal_voltage = terminal_voltage.clamp(0.0, 642.55);
            state_lock.dcdc.dcdc2sbs_terminal_current = terminal_current.clamp(-3212.7, 3212.8);
            state_lock.dcdc.dcdc2sbs_terminal_temperature = terminal_temperature.clamp(-40.0, 210.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DCDC2S2 Controls
        WebSocketMessage::SetDCDC2S2 {
            high_side_power,
            low_side_power,
            high_side_ground_voltage,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DCDC2S2 command");
            state_lock.dcdc.dcdc2s2_high_side_power = high_side_power.clamp(-1600.0, 1612.75);
            state_lock.dcdc.dcdc2s2_low_side_power = low_side_power.clamp(-1600.0, 1612.75);
            state_lock.dcdc.dcdc2s2_high_side_ground_voltage =
                high_side_ground_voltage.clamp(0.0, 3212.75);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // HVESSTS1 Controls - Phase 2 Pumps Thermal Management System
        WebSocketMessage::SetHVESSTS1 {
            system_input_power,
            hv_input_power,
            compressor_speed,
            relative_humidity,
            heater_status,
            hvil_status,
            system_mode,
            coolant_level,
            coolant_level_full,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for HVESSTS1 command");
            state_lock.thermal.hvessts1_system_input_power = system_input_power.clamp(0.0, 32127.5);
            state_lock.thermal.hvessts1_hv_input_power = hv_input_power.clamp(0.0, 32127.5);
            state_lock.thermal.hvessts1_compressor_speed = compressor_speed.clamp(0.0, 8000.0);
            state_lock.thermal.hvessts1_relative_humidity = relative_humidity.clamp(0.0, 100.0);
            state_lock.thermal.hvessts1_heater_status = heater_status.clamp(0, 3) as u8;
            state_lock.thermal.hvessts1_hvil_status = hvil_status.clamp(0, 3) as u8;
            state_lock.thermal.hvessts1_system_mode = system_mode.clamp(0, 15) as u8;
            state_lock.thermal.hvessts1_coolant_level = coolant_level.clamp(0, 3) as u8;
            state_lock.thermal.hvessts1_coolant_level_full = coolant_level_full.clamp(0, 3) as u8;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // HVESSTC1 HVESS Thermal Management System Temperature Control - Phase 2 Pumps
        WebSocketMessage::SetHVESSTC1 {
            intake_coolant_temp_request,
            outlet_coolant_temp_request,
            coolant_flow_rate_request,
            heater_enable_command,
            coolant_pump_enable_code,
            compressor_enable_code,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for HVESSTC1 command");
            state_lock.thermal.hvesstc1_intake_coolant_temp_request =
                intake_coolant_temp_request.clamp(-40.0, 210.0);
            state_lock.thermal.hvesstc1_outlet_coolant_temp_request =
                outlet_coolant_temp_request.clamp(-40.0, 210.0);
            state_lock.thermal.hvesstc1_coolant_flow_rate_request =
                coolant_flow_rate_request.clamp(0.0, 32127.5);
            state_lock.thermal.hvesstc1_heater_enable_command = heater_enable_command.clamp(0, 3) as u8;
            state_lock.thermal.hvesstc1_coolant_pump_enable_code =
                coolant_pump_enable_code.clamp(0, 3) as u8;
            state_lock.thermal.hvesstc1_compressor_enable_code = compressor_enable_code.clamp(0, 3) as u8;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // HVESSTC2 HVESS Thermal Management System Temperature Control 2 - Phase 2 Pumps
        WebSocketMessage::SetHVESSTC2 {
            pump_speed_command,
            pump_speed_command_percent,
            compressor_speed_command,
            compressor_speed_command_percent,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for HVESSTC2 command");
            state_lock.thermal.hvesstc2_pump_speed_command = pump_speed_command.clamp(0.0, 32127.5);
            state_lock.thermal.hvesstc2_pump_speed_command_percent =
                pump_speed_command_percent.clamp(0.0, 100.0);
            state_lock.thermal.hvesstc2_compressor_speed_command =
                compressor_speed_command.clamp(0.0, 32127.5);
            state_lock.thermal.hvesstc2_compressor_speed_command_percent =
                compressor_speed_command_percent.clamp(0.0, 100.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // ETCC3 Engine Thermal Control
        WebSocketMessage::SetETCC3 {
            etc_bypass_actuator_1,
            turbo_wastegate_actuator_1,
            cylinder_head_bypass_actuator,
            throttle_valve_1,
            etc_bypass_pass_actuator_1,
            etc_bypass_pass_actuator_2,
            turbo_wastegate_actuator_2,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for ETCC3 command");
            state_lock.thermal.etcc3_etc_bypass_actuator_1 = etc_bypass_actuator_1.clamp(0, 3) as u8;
            state_lock.thermal.etcc3_turbo_wastegate_actuator_1 =
                turbo_wastegate_actuator_1.clamp(0, 3) as u8;
            state_lock.thermal.etcc3_cylinder_head_bypass_actuator =
                cylinder_head_bypass_actuator.clamp(0, 3) as u8;
            state_lock.thermal.etcc3_throttle_valve_1 = throttle_valve_1.clamp(0, 3) as u8;
            state_lock.thermal.etcc3_etc_bypass_pass_actuator_1 =
                etc_bypass_pass_actuator_1.clamp(0, 3) as u8;
            state_lock.thermal.etcc3_etc_bypass_pass_actuator_2 =
                etc_bypass_pass_actuator_2.clamp(0, 3) as u8;
            state_lock.thermal.etcc3_turbo_wastegate_actuator_2 =
                turbo_wastegate_actuator_2.clamp(0, 3) as u8;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DM01 Diagnostic Controls
        WebSocketMessage::SetDM01 {
            protect_lamp,
            amber_lamp,
            red_stop_lamp,
            mil,
            active_dtc_spn,
            active_dtc_fmi,
            active_dtc_oc,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DM01 command");
            state_lock.diagnostics.dm01_protect_lamp_status = protect_lamp.clamp(0, 3) as u8;
            state_lock.diagnostics.dm01_amber_warning_lamp_status = amber_lamp.clamp(0, 3) as u8;
            state_lock.diagnostics.dm01_red_stop_lamp_status = red_stop_lamp.clamp(0, 3) as u8;
            state_lock.diagnostics.dm01_malfunction_indicator_lamp_status = mil.clamp(0, 3) as u8;
            state_lock.diagnostics.dm01_active_dtc_spn = active_dtc_spn.clamp(0, 0xFFFF) as u16;
            state_lock.diagnostics.dm01_active_dtc_fmi = active_dtc_fmi.clamp(0, 0xFF) as u8;
            state_lock.diagnostics.dm01_active_dtc_occurrence_count = active_dtc_oc.clamp(0, 0xFF) as u8;
            state_lock.diagnostics.dm01_fault_injection_enabled = true;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DM02 Diagnostic Controls
        WebSocketMessage::SetDM02 {
            protect_lamp,
            amber_lamp,
            red_stop_lamp,
            mil,
            previously_active_dtc_spn,
            previously_active_dtc_fmi,
            previously_active_dtc_oc,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DM02 command");
            state_lock.diagnostics.dm02_protect_lamp_status = protect_lamp.clamp(0, 3) as u8;
            state_lock.diagnostics.dm02_amber_warning_lamp_status = amber_lamp.clamp(0, 3) as u8;
            state_lock.diagnostics.dm02_red_stop_lamp_status = red_stop_lamp.clamp(0, 3) as u8;
            state_lock.diagnostics.dm02_malfunction_indicator_lamp_status = mil.clamp(0, 3) as u8;
            state_lock.diagnostics.dm02_previously_active_dtc_spn =
                previously_active_dtc_spn.clamp(0, 0xFFFF) as u16;
            state_lock.diagnostics.dm02_previously_active_dtc_fmi =
                previously_active_dtc_fmi.clamp(0, 0xFF) as u8;
            state_lock.diagnostics.dm02_previously_active_dtc_occurrence_count =
                previously_active_dtc_oc.clamp(0, 0xFF) as u8;
            state_lock.diagnostics.dm02_fault_injection_enabled = true;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // DM03 Diagnostic Clear/Reset Controls
        WebSocketMessage::SetDM03 {
            clear_operations_enabled,
            auto_response_enabled,
            command_generation_enabled,
            target_device_id,
            command_interval_seconds,
            trigger_clear_command,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for DM03 command");

            state_lock.diagnostics.dm03_clear_operations_enabled = clear_operations_enabled;
            state_lock.diagnostics.dm03_auto_response_enabled = auto_response_enabled;
            state_lock.diagnostics.dm03_command_generation_enabled = command_generation_enabled;
            state_lock.diagnostics.dm03_target_device_id = target_device_id;
            state_lock.diagnostics.dm03_command_interval_seconds = command_interval_seconds;

            if trigger_clear_command && clear_operations_enabled {
                // Simulate DM03 clear operation
                state_lock.diagnostics.dm03_clear_commands_received += 1;
                state_lock.diagnostics.dm03_last_clear_timestamp = state_lock.uptime_seconds;

                // Clear active DTCs (DM01)
                state_lock.diagnostics.dm01_active_dtc_spn = 0xFFFF;
                state_lock.diagnostics.dm01_active_dtc_fmi = 0xFF;
                state_lock.diagnostics.dm01_active_dtc_occurrence_count = 0xFF;
                state_lock.diagnostics.dm01_active_dtc_conversion_method = 0xFF;

                // Reset lamp states
                state_lock.diagnostics.dm01_protect_lamp_status = 0;
                state_lock.diagnostics.dm01_amber_warning_lamp_status = 0;
                state_lock.diagnostics.dm01_red_stop_lamp_status = 0;
                state_lock.diagnostics.dm01_malfunction_indicator_lamp_status = 0;

                // Move to previously active if there was an active fault
                if state_lock.diagnostics.dm01_fault_injection_enabled {
                    state_lock.diagnostics.dm02_previously_active_dtc_spn = 7945;
                    state_lock.diagnostics.dm02_previously_active_dtc_fmi = 9;
                    state_lock.diagnostics.dm02_previously_active_dtc_occurrence_count = 5;
                    state_lock.diagnostics.dm02_previously_active_dtc_conversion_method = 0;
                }

                state_lock.diagnostics.dm01_fault_injection_enabled = false;
            }

            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        // AEBS Controls
        WebSocketMessage::SetAEBS {
            enabled,
            brake_demand,
        } => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for AEBS command");
            state_lock.braking.aebs_enabled = enabled;
            state_lock.braking.aebs_brake_demand = brake_demand.clamp(0.0, 100.0);
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        WebSocketMessage::Reset => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for reset command");
            let device_id = state_lock.device_id;
            *state_lock = SimulatorState::default();
            state_lock.device_id = device_id;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::PauseBroadcast => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for pause broadcast command");
            state_lock.broadcast_paused = true;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }
        WebSocketMessage::ResumeBroadcast => {
            let mut state_lock = state
                .lock()
                .expect("Failed to acquire state lock for resume broadcast command");
            state_lock.broadcast_paused = false;
            WebSocketMessage::StateUpdate {
                state: Box::new(state_lock.clone()),
            }
        }

        _ => WebSocketMessage::Error {
            message: "Unsupported message type".to_string(),
        },
    }
}

/// Parse device ID from string (hex or decimal)
fn parse_j1939_device_id(device_id_str: &str) -> Result<u32> {
    parse_device_id(device_id_str, &J1939DeviceIdValidator)
}

/// Print help text
fn print_help() {
    println!("Available commands:");
    println!("  set <field>=<value>  - Set a state field");
    println!("    Fields:");
    println!("      # Original J1939 Fields:");
    println!("      crash_detected=<true|false>");
    println!("      crash_type=<0-31>");
    println!("      wand_angle=<-250.0 to 252.19>");
    println!("      linear_displacement=<0.0 to 6425.5>");
    println!("      # EMP Motor/Generator 1 Controls:");
    println!("      mg1_speed_setpoint=<-125.0 to 125.0>");
    println!("      mg1_torque_setpoint=<-125.0 to 125.0>");
    println!("      mg1_max_torque=<-125.0 to 125.0>");
    println!("      mg1_min_torque=<-125.0 to 125.0>");
    println!("      # EMP Motor/Generator 2 Controls:");
    println!("      mg2_speed_setpoint=<-125.0 to 125.0>");
    println!("      mg2_torque_setpoint=<-125.0 to 125.0>");
    println!("      # HVESS (Battery) Controls:");
    println!("      hvess_power_down_command=<true|false>");
    println!("      hvess_cell_balancing_command=<true|false>");
    println!("      hvess_voltage_level=<400.0 to 1000.0>");
    println!("      hvess_coolant_temp=<-40.0 to 100.0>");
    println!("      hvess_electronics_temp=<-40.0 to 150.0>");
    println!("      # HVESSD2 (Cell Voltage & State of Charge) Controls:");
    println!("      hvess_fast_update_state_of_charge=<0.0 to 100.4> # %");
    println!("      hvess_highest_cell_voltage=<0.0 to 64.255>       # V");
    println!("      hvess_lowest_cell_voltage=<0.0 to 64.255>        # V");
    println!("      hvess_cell_voltage_differential_status=<0-15>");
    println!("      # HVESSD3 (Cell Temperature Monitoring) Controls:");
    println!("      hvess_highest_cell_temperature=<-273.0 to 1734.97> # °C");
    println!("      hvess_lowest_cell_temperature=<-273.0 to 1734.97>  # °C");
    println!("      hvess_average_cell_temperature=<-273.0 to 1734.97> # °C");
    println!("      hvess_cell_temp_differential_status=<0-3>");
    println!("      # HVESSFS1 (Fan Status & Monitoring) Controls:");
    println!("      hvess_fan_speed=<0.0 to 32127.5>                   # rpm");
    println!("      hvess_fan_power=<0.0 to 32127.5>                   # W");
    println!("      hvess_fan_medium_temperature=<-273.0 to 1734.97>   # °C");
    println!("      hvess_fan_speed_status=<0-3>");
    println!("      hvess_fan_status_reason_code=<0-15>");
    println!("      hvess_fan_command_status=<0-3>");
    println!("      hvess_fan_service_indicator=<0-3>");
    println!("      hvess_fan_operating_status=<0-3>");
    println!("      hvess_fan_status1_instance=<0-15>");
    println!("      # DC-DC Converter Controls:");
    println!("      dcdc_operational_command=<0-15>");
    println!("      dcdc_low_side_voltage_setpoint=<12.0 to 60.0>");
    println!("      dcdc_high_side_voltage_setpoint=<400.0 to 1000.0>");
    println!("      # Engine Controls:");
    println!("      engine_speed=<600.0 to 6000.0>  # RPM");
    println!("      engine_load=<0.0 to 100.0>      # Percent");
    println!("      engine_coolant_temp=<0.0 to 120.0> # °C");
    println!("      engine_exhaust_temp=<100.0 to 1000.0> # °C");
    println!("      transmission_gear=<0-16>");
    println!("      # ETC9 Dual Clutch Transmission Controls:");
    println!("      etc9_current_preselection_gear=<-125.0 to 125.0>");
    println!("      etc9_input_shaft1_speed=<0.0 to 8031.875> # rpm");
    println!("      etc9_input_shaft2_speed=<0.0 to 8031.875> # rpm");
    println!("      etc9_selected_preselection_gear=<-125.0 to 125.0>");
    println!("      # ALTC (Alternator Control) Controls:");
    println!("      altc_setpoint_voltage=<0.0 to 64.255> # V");
    println!("      altc_excitation_current_limit=<0.0 to 62.5> # A");
    println!("      altc_torque_ramp_time=<0.1 to 25.0> # s");
    println!("      altc_torque_ramp_max_speed=<0.0 to 8000.0> # rpm");
    println!("      # GC2 (Generator Control 2) Controls:");
    println!("      gc2_engine_load_setpoint=<0.0 to 32127.5> # kW");
    println!("      gc2_derate_inhibit=<0 to 3>");
    println!("      gc2_governing_bias=<-125.0 to 125.0> # %");
    println!("      # DCACAI1S2 (DC/AC Accessory Inverter 1 Status 2) Controls:");
    println!("      dcacai1s2_desired_power=<0.0 to 4015.9375> # kW");
    println!("      dcacai1s2_desired_voltage=<0.0 to 3212.75> # V");
    println!("      dcacai1s2_desired_current=<0.0 to 4015.9375> # A");
    println!("      dcacai1s2_desired_ground_voltage=<0.0 to 3212.75> # V");
    println!("      # DCDC1OS (DC/DC Converter 1 Operating Status) Controls:");
    println!("      dcdc1os_operational_status=<0 to 15>");
    println!("      dcdc1os_hvil_status=<0 to 3>");
    println!("      dcdc1os_loadshed_request=<0 to 3>");
    println!("      dcdc1os_operating_status_crc=<0 to 250>");
    println!("      # DCDC1SBS (DC/DC Converter 1 SLI Battery Status) Controls:");
    println!("      dcdc1sbs_terminal_current=<-3212.7 to 3212.8> # A");
    println!("      dcdc1sbs_terminal_voltage=<0.0 to 642.55> # V");
    println!("      dcdc1sbs_terminal_temperature=<-40.0 to 210.0> # °C");
    println!("      # DCDC1S2 (DC/DC Converter 1 Status 2) Controls:");
    println!("      dcdc1s2_high_side_power=<-1600.0 to 1612.75> # kW");
    println!("      dcdc1s2_low_side_power=<-1600.0 to 1612.75> # kW");
    println!("      dcdc1s2_high_side_ground_voltage=<0.0 to 3212.75> # V");
    println!("      # DCACAI1V (DC/AC Accessory Inverter 1 Voltage) Controls:");
    println!("      dcacai1v_ignition_voltage=<0.0 to 3212.75> # V");
    println!("      dcacai1v_unswitched_voltage=<0.0 to 3212.75> # V");
    println!("      # GTRACE (Generator Trip Energy) Controls:");
    println!("      gtrace_kwh_export=<0 to 4211081215> # kWh");
    println!("      gtrace_kvarh_export=<0 to 4211081215> # kVArh");
    println!("      # DCDC2SBS (DC/DC Converter 2 SLI Battery Status) Controls:");
    println!("      dcdc2sbs_terminal_voltage=<0.0 to 642.55> # V");
    println!("      dcdc2sbs_terminal_current=<-3212.7 to 3212.8> # A");
    println!("      dcdc2sbs_terminal_temperature=<-40.0 to 210.0> # °C");
    println!("      # DCDC2S2 (DC/DC Converter 2 Status 2) Controls:");
    println!("      dcdc2s2_high_side_power=<-1600.0 to 1612.75> # kW");
    println!("      dcdc2s2_low_side_power=<-1600.0 to 1612.75> # kW");
    println!("      dcdc2s2_high_side_ground_voltage=<0.0 to 3212.75> # V");
    println!(
        "      # HVESSTS1 (HVESS Thermal Management System Status 1) Controls - Phase 2 Pumps:"
    );
    println!("      hvessts1_system_input_power=<0.0 to 32127.5> # W");
    println!("      hvessts1_hv_input_power=<0.0 to 32127.5> # W");
    println!("      hvessts1_compressor_speed=<0.0 to 8000.0> # rpm");
    println!("      hvessts1_relative_humidity=<0.0 to 100.0> # %");
    println!("      hvessts1_heater_status=<0 to 3>");
    println!("      hvessts1_hvil_status=<0 to 3>");
    println!("      hvessts1_system_mode=<0 to 15>");
    println!("      hvessts1_coolant_level=<0 to 3>");
    println!("      hvessts1_coolant_level_full=<0 to 3>");
    println!(
        "      # HVESSTC1 (HVESS Thermal Management System Temperature Control) Controls - Phase 2 Pumps:"
    );
    println!("      hvesstc1_intake_coolant_temp_request=<-40.0 to 210.0> # °C");
    println!("      hvesstc1_outlet_coolant_temp_request=<-40.0 to 210.0> # °C");
    println!("      hvesstc1_coolant_flow_rate_request=<0.0 to 32127.5> # l/min");
    println!("      hvesstc1_heater_enable_command=<0 to 3>");
    println!("      hvesstc1_coolant_pump_enable_code=<0 to 3>");
    println!("      hvesstc1_compressor_enable_code=<0 to 3>");
    println!(
        "      # HVESSTC2 (HVESS Thermal Management System Temperature Control 2) Controls - Phase 2 Pumps:"
    );
    println!("      hvesstc2_pump_speed_command=<0.0 to 32127.5> # rpm");
    println!("      hvesstc2_pump_speed_command_percent=<0.0 to 100.0> # %");
    println!("      hvesstc2_compressor_speed_command=<0.0 to 32127.5> # rpm");
    println!("      hvesstc2_compressor_speed_command_percent=<0.0 to 100.0> # %");
    println!("      # ETCC3 (Engine Thermal Control) Controls:");
    println!("      etcc3_etc_bypass_actuator_1=<0-3> # ETC Bypass Actuator 1");
    println!("      etcc3_turbo_wastegate_actuator_1=<0-3> # Turbo Wastegate 1");
    println!("      etcc3_cylinder_head_bypass_actuator=<0-3> # Cylinder Head Bypass");
    println!("      etcc3_throttle_valve_1=<0-3> # Throttle Valve 1");
    println!("      etcc3_etc_bypass_pass_actuator_1=<0-3> # ETC Bypass Pass 1");
    println!("      etcc3_etc_bypass_pass_actuator_2=<0-3> # ETC Bypass Pass 2");
    println!("      etcc3_turbo_wastegate_actuator_2=<0-3> # Turbo Wastegate 2");
    println!("      # AEBS (Braking) Controls:");
    println!("      aebs_enabled=<true|false>");
    println!("      aebs_brake_demand=<0.0 to 100.0>");
    println!("      # DM03 - Diagnostic Clear/Reset Controls (J1939-73 Diagnostics):");
    println!("      dm03_clear_operations_enabled=<true|false>  # Enable/disable clear operations");
    println!("      dm03_auto_response_enabled=<true|false>     # Enable auto DM01/DM02 response");
    println!("      dm03_trigger_clear=<true>                   # Trigger DTC clear operation");
    println!("      # DM03 - Command Generation Controls (J1939-73 Diagnostics):");
    println!("      dm03_command_generation_enabled=<true|false> # Enable DM03 command generation");
    println!("      dm03_target_device_id=<0x00-0xFF>           # Target device for DM03 commands");
    println!(
        "      dm03_command_interval_seconds=<0-3600>      # Auto-generation interval (0=manual)"
    );
    println!("      dm03_send_command=<true>                    # Send DM03 command immediately");
    println!("  status               - Show current state");
    println!("  reset                - Reset to default state");
    println!("  help                 - Show this help");
    println!("  quit/exit            - Exit simulator");
}

/// Print current status
fn print_status(state: &Arc<Mutex<SimulatorState>>) {
    let state_lock = state
        .lock()
        .expect("Failed to acquire state lock for display");
    println!("\nCurrent State:");
    println!("  Device ID: 0x{:02X}", state_lock.device_id);
    println!("  Uptime: {} seconds", state_lock.uptime_seconds);

    // Original J1939 Status
    println!("\n📡 Original J1939 Messages:");
    println!("  Crash Detected: {}", state_lock.crash.crash_detected);
    println!("  Crash Type: {}", state_lock.crash.crash_type);
    println!("  Crash Counter: {}", state_lock.crash.crash_counter);
    println!("  Wand Angle: {:.2}°", state_lock.sensors.wand_angle);
    println!(
        "  Linear Displacement: {:.2} mm",
        state_lock.sensors.linear_displacement
    );

    // EMP Motor Status
    println!("\n⚡ EMP Motor/Generator 1 Status:");
    println!(
        "  Speed Setpoint: {:.1}% | Actual: {:.1}%",
        state_lock.motor.mg1_speed_setpoint, state_lock.motor.mg1_actual_speed
    );
    println!(
        "  Torque Setpoint: {:.1}% | Actual: {:.1}%",
        state_lock.motor.mg1_torque_setpoint, state_lock.motor.mg1_actual_torque
    );
    println!(
        "  Current: {:.1}A | Voltage: {:.1}V",
        state_lock.motor.mg1_current, state_lock.motor.mg1_voltage
    );
    println!(
        "  Torque Limits: {:.1}% to {:.1}%",
        state_lock.motor.mg1_min_torque, state_lock.motor.mg1_max_torque
    );

    println!("\n⚡ EMP Motor/Generator 2 Status:");
    println!(
        "  Speed Setpoint: {:.1}% | Actual: {:.1}%",
        state_lock.motor.mg2_speed_setpoint, state_lock.motor.mg2_actual_speed
    );
    println!(
        "  Torque Setpoint: {:.1}% | Actual: {:.1}%",
        state_lock.motor.mg2_torque_setpoint, state_lock.motor.mg2_actual_torque
    );
    println!(
        "  Current: {:.1}A | Voltage: {:.1}V",
        state_lock.motor.mg2_current, state_lock.motor.mg2_voltage
    );

    // HVESS Status
    println!("\n🔋 HVESS (Battery) Status:");
    println!(
        "  Power Down: {} | Cell Balancing: {}",
        state_lock.hvess.hvess_power_down_command, state_lock.hvess.hvess_cell_balancing_command
    );
    println!(
        "  Bus Voltage: {:.1}V | Ignition: {:.1}V",
        state_lock.hvess.hvess_bus_voltage, state_lock.hvess.hvess_ignition_voltage
    );
    println!(
        "  Discharge Power: {:.1}kW | Charge Power: {:.1}kW",
        state_lock.hvess.hvess_discharge_power, state_lock.hvess.hvess_charge_power
    );
    println!(
        "  Coolant Temp: {:.1}°C | Electronics Temp: {:.1}°C",
        state_lock.hvess.hvess_coolant_temp, state_lock.hvess.hvess_electronics_temp
    );

    // DC-DC Converter Status
    println!("\n🔌 DC-DC Converter Status:");
    println!(
        "  Operational Command: {}",
        state_lock.dcdc.dcdc_operational_command
    );
    println!(
        "  Low Side Setpoint: {:.1}V | High Side Setpoint: {:.1}V",
        state_lock.dcdc.dcdc_low_side_voltage_setpoint, state_lock.dcdc.dcdc_high_side_voltage_setpoint
    );

    // Engine Status
    println!("\n🚗 Engine Status:");
    println!(
        "  RPM: {:.0} | Load: {:.1}%",
        state_lock.engine.engine_speed, state_lock.engine.engine_load
    );
    println!(
        "  Torque: {:.1}% | Fuel Rate: {:.1} L/h",
        state_lock.engine.engine_torque, state_lock.engine.engine_fuel_rate
    );
    println!(
        "  Coolant: {:.1}°C | Exhaust: {:.1}°C",
        state_lock.engine.engine_coolant_temp, state_lock.engine.engine_exhaust_temp
    );
    println!(
        "  Oil Pressure: {:.1} kPa | Turbo Speed: {:.0} RPM",
        state_lock.engine.engine_oil_pressure, state_lock.engine.turbo_speed
    );
    println!("  Transmission Gear: {}", state_lock.transmission.transmission_gear);

    // ETC9 Dual Clutch Transmission Status
    println!(
        "  ETC9 Current Pre-selection Gear: {}",
        state_lock.transmission.etc9_current_preselection_gear
    );
    println!(
        "  ETC9 Input Shaft 1 Speed: {:.1} rpm",
        state_lock.transmission.etc9_input_shaft1_speed
    );
    println!(
        "  ETC9 Input Shaft 2 Speed: {:.1} rpm",
        state_lock.transmission.etc9_input_shaft2_speed
    );
    println!(
        "  ETC9 Selected Pre-selection Gear: {}",
        state_lock.transmission.etc9_selected_preselection_gear
    );

    // ALTC Alternator Control Status
    println!("\n⚡ ALTC (Alternator Control) Status:");
    println!(
        "  Setpoint Voltage: {:.3}V | Current Limit: {:.1}A",
        state_lock.power_supply.altc_setpoint_voltage, state_lock.power_supply.altc_excitation_current_limit
    );
    println!(
        "  Torque Ramp Time: {:.1}s | Max Speed: {:.0} rpm",
        state_lock.power_supply.altc_torque_ramp_time, state_lock.power_supply.altc_torque_ramp_max_speed
    );

    // GC2 Generator Control 2 Status
    println!("\n⚡ GC2 (Generator Control 2) Status:");
    println!(
        "  Load Setpoint: {:.1}kW | Derate Inhibit: {}",
        state_lock.power_supply.gc2_engine_load_setpoint, state_lock.power_supply.gc2_derate_inhibit
    );
    println!("  Governing Bias: {:.3}%", state_lock.power_supply.gc2_governing_bias);

    // DCACAI1S2 DC/AC Accessory Inverter 1 Status 2 Status
    println!("\n⚡ DCACAI1S2 (DC/AC Accessory Inverter 1 Status 2) Status:");
    println!(
        "  Power: {:.1}kW | Voltage: {:.1}V",
        state_lock.power_supply.dcacai1s2_desired_power, state_lock.power_supply.dcacai1s2_desired_voltage
    );
    println!(
        "  Current: {:.1}A | Ground Voltage: {:.1}V",
        state_lock.power_supply.dcacai1s2_desired_current, state_lock.power_supply.dcacai1s2_desired_ground_voltage
    );

    // DCDC1OS DC/DC Converter 1 Operating Status Status
    println!("\n⚡ DCDC1OS (DC/DC Converter 1 Operating Status) Status:");
    println!(
        "  Operational Status: {} | HVIL Status: {}",
        state_lock.dcdc.dcdc1os_operational_status, state_lock.dcdc.dcdc1os_hvil_status
    );
    println!(
        "  Load Shed Request: {} | Status Counter: {} | CRC: {}",
        state_lock.dcdc.dcdc1os_loadshed_request,
        state_lock.dcdc.dcdc1os_operating_status_counter,
        state_lock.dcdc.dcdc1os_operating_status_crc
    );

    // DCDC1SBS DC/DC Converter 1 SLI Battery Status
    println!("\n🔋 DCDC1SBS (DC/DC Converter 1 SLI Battery Status) Status:");
    println!(
        "  Terminal Current: {:.1}A | Terminal Voltage: {:.1}V",
        state_lock.dcdc.dcdc1sbs_terminal_current, state_lock.dcdc.dcdc1sbs_terminal_voltage
    );
    println!(
        "  Terminal Temperature: {:.1}°C",
        state_lock.dcdc.dcdc1sbs_terminal_temperature
    );

    // DCDC1S2 DC/DC Converter 1 Status 2
    println!("\n⚡ DCDC1S2 (DC/DC Converter 1 Status 2) Status:");
    println!(
        "  High-Side Power: {:.1}kW | Low-Side Power: {:.1}kW",
        state_lock.dcdc.dcdc1s2_high_side_power, state_lock.dcdc.dcdc1s2_low_side_power
    );
    println!(
        "  High-Side Ground Voltage: {:.1}V",
        state_lock.dcdc.dcdc1s2_high_side_ground_voltage
    );

    // DCACAI1V DC/AC Accessory Inverter 1 Voltage
    println!("\n🔌 DCACAI1V (DC/AC Accessory Inverter 1 Voltage) Status:");
    println!(
        "  Ignition Voltage: {:.1}V | Unswitched Voltage: {:.1}V",
        state_lock.power_supply.dcacai1v_ignition_voltage, state_lock.power_supply.dcacai1v_unswitched_voltage
    );

    // GTRACE Generator Trip Energy
    println!("\n⚡ GTRACE (Generator Trip Energy) Status:");
    println!(
        "  kWh Export: {} kWh | kVArh Export: {} kVArh",
        state_lock.power_supply.gtrace_kwh_export, state_lock.power_supply.gtrace_kvarh_export
    );

    // DCDC2SBS DC/DC Converter 2 SLI Battery Status
    println!("\n🔋 DCDC2SBS (DC/DC Converter 2 SLI Battery Status) Status:");
    println!(
        "  Voltage: {:.2}V | Current: {:.1}A | Temperature: {:.1}°C",
        state_lock.dcdc.dcdc2sbs_terminal_voltage,
        state_lock.dcdc.dcdc2sbs_terminal_current,
        state_lock.dcdc.dcdc2sbs_terminal_temperature
    );

    // DCDC2S2 DC/DC Converter 2 Status 2
    println!("\n⚡ DCDC2S2 (DC/DC Converter 2 Status 2) Status:");
    println!(
        "  High-Side Power: {:.1}kW | Low-Side Power: {:.1}kW",
        state_lock.dcdc.dcdc2s2_high_side_power, state_lock.dcdc.dcdc2s2_low_side_power
    );
    println!(
        "  High-Side Ground Voltage: {:.1}V",
        state_lock.dcdc.dcdc2s2_high_side_ground_voltage
    );

    // AEBS Status
    println!("\n🛑 AEBS (Braking) Status:");
    println!(
        "  Enabled: {} | Brake Demand: {:.1}%",
        state_lock.braking.aebs_enabled, state_lock.braking.aebs_brake_demand
    );
    println!(
        "  Target Deceleration: {:.1} m/s² | Status: {}",
        state_lock.braking.aebs_target_deceleration, state_lock.braking.aebs_status
    );

    // DM03 Diagnostic Clear/Reset Status
    println!("\n🔧 DM03 (Diagnostic Clear/Reset) Status:");
    println!(
        "  Clear Operations Enabled: {} | Auto Response: {}",
        state_lock.diagnostics.dm03_clear_operations_enabled, state_lock.diagnostics.dm03_auto_response_enabled
    );
    println!(
        "  Commands Received: {} | Last Clear: {} seconds ago",
        state_lock.diagnostics.dm03_clear_commands_received,
        if state_lock.diagnostics.dm03_last_clear_timestamp > 0 {
            state_lock.uptime_seconds - state_lock.diagnostics.dm03_last_clear_timestamp
        } else {
            0
        }
    );
    println!(
        "  Command Generation: {} | Target Device: 0x{:02X}",
        if state_lock.diagnostics.dm03_command_generation_enabled {
            "Enabled"
        } else {
            "Disabled"
        },
        state_lock.diagnostics.dm03_target_device_id
    );
    println!(
        "  Commands Sent: {} | Interval: {}s",
        state_lock.diagnostics.dm03_commands_sent,
        if state_lock.diagnostics.dm03_command_interval_seconds > 0 {
            format!("{}", state_lock.diagnostics.dm03_command_interval_seconds)
        } else {
            "Manual".to_string()
        }
    );
    println!();
}

/// Handle set command
fn handle_set_command(args: &str, state: &Arc<Mutex<SimulatorState>>) {
    if let Some((field, value)) = args.split_once('=') {
        let field = field.trim();
        let value = value.trim();

        let mut state_lock = state
            .lock()
            .expect("Failed to acquire state lock for WebSocket message");

        match field {
            "crash_detected" => {
                if let Ok(detected) = value.parse::<bool>() {
                    state_lock.crash.crash_detected = detected;
                    if detected {
                        state_lock.crash.crash_counter += 1;
                    }
                    println!("✅ Set crash_detected = {}", detected);
                } else {
                    println!("❌ Invalid boolean value");
                }
            }
            "crash_type" => {
                if let Ok(ct) = value.parse::<u64>() {
                    if ct <= 31 {
                        state_lock.crash.crash_type = ct as u8;
                        println!("✅ Set crash_type = {}", ct);
                    } else {
                        println!("❌ Crash type must be 0-31");
                    }
                } else {
                    println!("❌ Invalid number");
                }
            }
            "wand_angle" => {
                if let Ok(angle) = value.parse::<f64>() {
                    state_lock.sensors.target_wand_angle = angle.clamp(-250.0, 252.19);
                    println!("✅ Set wand_angle = {:.2}", state_lock.sensors.target_wand_angle);
                } else {
                    println!("❌ Invalid number");
                }
            }
            "linear_displacement" => {
                if let Ok(disp) = value.parse::<f64>() {
                    state_lock.sensors.target_displacement = disp.clamp(0.0, 6425.5);
                    println!(
                        "✅ Set linear_displacement = {:.2}",
                        state_lock.sensors.target_displacement
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            // EMP Motor/Generator 1 Controls
            "mg1_speed_setpoint" => {
                if let Ok(speed) = value.parse::<f64>() {
                    state_lock.motor.mg1_speed_setpoint = speed.clamp(-125.0, 125.0);
                    println!(
                        "✅ Set mg1_speed_setpoint = {:.1}%",
                        state_lock.motor.mg1_speed_setpoint
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "mg1_torque_setpoint" => {
                if let Ok(torque) = value.parse::<f64>() {
                    state_lock.motor.mg1_torque_setpoint = torque.clamp(-125.0, 125.0);
                    println!(
                        "✅ Set mg1_torque_setpoint = {:.1}%",
                        state_lock.motor.mg1_torque_setpoint
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "mg1_max_torque" => {
                if let Ok(torque) = value.parse::<f64>() {
                    state_lock.motor.mg1_max_torque = torque.clamp(-125.0, 125.0);
                    println!("✅ Set mg1_max_torque = {:.1}%", state_lock.motor.mg1_max_torque);
                } else {
                    println!("❌ Invalid number");
                }
            }
            "mg1_min_torque" => {
                if let Ok(torque) = value.parse::<f64>() {
                    state_lock.motor.mg1_min_torque = torque.clamp(-125.0, 125.0);
                    println!("✅ Set mg1_min_torque = {:.1}%", state_lock.motor.mg1_min_torque);
                } else {
                    println!("❌ Invalid number");
                }
            }

            // EMP Motor/Generator 2 Controls
            "mg2_speed_setpoint" => {
                if let Ok(speed) = value.parse::<f64>() {
                    state_lock.motor.mg2_speed_setpoint = speed.clamp(-125.0, 125.0);
                    println!(
                        "✅ Set mg2_speed_setpoint = {:.1}%",
                        state_lock.motor.mg2_speed_setpoint
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "mg2_torque_setpoint" => {
                if let Ok(torque) = value.parse::<f64>() {
                    state_lock.motor.mg2_torque_setpoint = torque.clamp(-125.0, 125.0);
                    println!(
                        "✅ Set mg2_torque_setpoint = {:.1}%",
                        state_lock.motor.mg2_torque_setpoint
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }

            // HVESS Controls
            "hvess_power_down_command" => {
                if let Ok(power_down) = value.parse::<bool>() {
                    state_lock.hvess.hvess_power_down_command = power_down;
                    println!("✅ Set hvess_power_down_command = {}", power_down);
                } else {
                    println!("❌ Invalid boolean value");
                }
            }
            "hvess_cell_balancing_command" => {
                if let Ok(balancing) = value.parse::<bool>() {
                    state_lock.hvess.hvess_cell_balancing_command = balancing;
                    println!("✅ Set hvess_cell_balancing_command = {}", balancing);
                } else {
                    println!("❌ Invalid boolean value");
                }
            }
            "hvess_voltage_level" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    state_lock.hvess.hvess_voltage_level = voltage.clamp(400.0, 1000.0);
                    state_lock.hvess.hvess_bus_voltage = state_lock.hvess.hvess_voltage_level;
                    println!(
                        "✅ Set hvess_voltage_level = {:.1}V",
                        state_lock.hvess.hvess_voltage_level
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_coolant_temp" => {
                if let Ok(temp) = value.parse::<f64>() {
                    state_lock.hvess.hvess_coolant_temp = temp.clamp(-40.0, 100.0);
                    println!(
                        "✅ Set hvess_coolant_temp = {:.1}°C",
                        state_lock.hvess.hvess_coolant_temp
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_electronics_temp" => {
                if let Ok(temp) = value.parse::<f64>() {
                    state_lock.hvess.hvess_electronics_temp = temp.clamp(-40.0, 150.0);
                    println!(
                        "✅ Set hvess_electronics_temp = {:.1}°C",
                        state_lock.hvess.hvess_electronics_temp
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }

            // HVESSD2 Cell Voltage & State of Charge Controls
            "hvess_fast_update_state_of_charge" => {
                if let Ok(charge) = value.parse::<f64>() {
                    state_lock.hvess.hvess_fast_update_state_of_charge = charge.clamp(0.0, 100.4);
                    println!(
                        "✅ Set hvess_fast_update_state_of_charge = {:.1}%",
                        state_lock.hvess.hvess_fast_update_state_of_charge
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_highest_cell_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    state_lock.hvess.hvess_highest_cell_voltage = voltage.clamp(0.0, 64.255);
                    println!(
                        "✅ Set hvess_highest_cell_voltage = {:.3}V",
                        state_lock.hvess.hvess_highest_cell_voltage
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_lowest_cell_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    state_lock.hvess.hvess_lowest_cell_voltage = voltage.clamp(0.0, 64.255);
                    println!(
                        "✅ Set hvess_lowest_cell_voltage = {:.3}V",
                        state_lock.hvess.hvess_lowest_cell_voltage
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_cell_voltage_differential_status" => {
                if let Ok(status) = value.parse::<u64>() {
                    state_lock.hvess.hvess_cell_voltage_differential_status = status.clamp(0, 15);
                    println!(
                        "✅ Set hvess_cell_voltage_differential_status = {}",
                        state_lock.hvess.hvess_cell_voltage_differential_status
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }

            // HVESSD3 Cell Temperature Monitoring Controls
            "hvess_highest_cell_temperature" => {
                if let Ok(temp) = value.parse::<f64>() {
                    state_lock.hvess.hvess_highest_cell_temperature = temp.clamp(-273.0, 1734.97);
                    println!(
                        "✅ Set hvess_highest_cell_temperature = {:.1}°C",
                        state_lock.hvess.hvess_highest_cell_temperature
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_lowest_cell_temperature" => {
                if let Ok(temp) = value.parse::<f64>() {
                    state_lock.hvess.hvess_lowest_cell_temperature = temp.clamp(-273.0, 1734.97);
                    println!(
                        "✅ Set hvess_lowest_cell_temperature = {:.1}°C",
                        state_lock.hvess.hvess_lowest_cell_temperature
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_average_cell_temperature" => {
                if let Ok(temp) = value.parse::<f64>() {
                    state_lock.hvess.hvess_average_cell_temperature = temp.clamp(-273.0, 1734.97);
                    println!(
                        "✅ Set hvess_average_cell_temperature = {:.1}°C",
                        state_lock.hvess.hvess_average_cell_temperature
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_cell_temp_differential_status" => {
                if let Ok(status) = value.parse::<u64>() {
                    state_lock.hvess.hvess_cell_temp_differential_status = status.clamp(0, 3);
                    println!(
                        "✅ Set hvess_cell_temp_differential_status = {}",
                        state_lock.hvess.hvess_cell_temp_differential_status
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }

            // HVESSFS1 Fan Status & Monitoring Controls
            "hvess_fan_speed" => {
                if let Ok(speed) = value.parse::<f64>() {
                    state_lock.hvess.hvess_fan_speed = speed.clamp(0.0, 32127.5);
                    println!(
                        "✅ Set hvess_fan_speed = {:.1} rpm",
                        state_lock.hvess.hvess_fan_speed
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_fan_power" => {
                if let Ok(power) = value.parse::<f64>() {
                    state_lock.hvess.hvess_fan_power = power.clamp(0.0, 32127.5);
                    println!(
                        "✅ Set hvess_fan_power = {:.1}W",
                        state_lock.hvess.hvess_fan_power
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_fan_medium_temperature" => {
                if let Ok(temp) = value.parse::<f64>() {
                    state_lock.hvess.hvess_fan_medium_temperature = temp.clamp(-273.0, 1734.97);
                    println!(
                        "✅ Set hvess_fan_medium_temperature = {:.1}°C",
                        state_lock.hvess.hvess_fan_medium_temperature
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_fan_speed_status" => {
                if let Ok(status) = value.parse::<u64>() {
                    state_lock.hvess.hvess_fan_speed_status = status.clamp(0, 3);
                    println!(
                        "✅ Set hvess_fan_speed_status = {}",
                        state_lock.hvess.hvess_fan_speed_status
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_fan_status_reason_code" => {
                if let Ok(code) = value.parse::<u64>() {
                    state_lock.hvess.hvess_fan_status_reason_code = code.clamp(0, 15);
                    println!(
                        "✅ Set hvess_fan_status_reason_code = {}",
                        state_lock.hvess.hvess_fan_status_reason_code
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_fan_command_status" => {
                if let Ok(status) = value.parse::<u64>() {
                    state_lock.hvess.hvess_fan_command_status = status.clamp(0, 3);
                    println!(
                        "✅ Set hvess_fan_command_status = {}",
                        state_lock.hvess.hvess_fan_command_status
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_fan_service_indicator" => {
                if let Ok(indicator) = value.parse::<u64>() {
                    state_lock.hvess.hvess_fan_service_indicator = indicator.clamp(0, 3);
                    println!(
                        "✅ Set hvess_fan_service_indicator = {}",
                        state_lock.hvess.hvess_fan_service_indicator
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_fan_operating_status" => {
                if let Ok(status) = value.parse::<u64>() {
                    state_lock.hvess.hvess_fan_operating_status = status.clamp(0, 3);
                    println!(
                        "✅ Set hvess_fan_operating_status = {}",
                        state_lock.hvess.hvess_fan_operating_status
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "hvess_fan_status1_instance" => {
                if let Ok(instance) = value.parse::<u64>() {
                    state_lock.hvess.hvess_fan_status1_instance = instance.clamp(0, 15);
                    println!(
                        "✅ Set hvess_fan_status1_instance = {}",
                        state_lock.hvess.hvess_fan_status1_instance
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }

            // DC-DC Converter Controls
            "dcdc_operational_command" => {
                if let Ok(cmd) = value.parse::<u64>() {
                    if cmd <= 15 {
                        state_lock.dcdc.dcdc_operational_command = cmd as u8;
                        println!("✅ Set dcdc_operational_command = {}", cmd);
                    } else {
                        println!("❌ Operational command must be 0-15");
                    }
                } else {
                    println!("❌ Invalid number");
                }
            }
            "dcdc_low_side_voltage_setpoint" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    state_lock.dcdc.dcdc_low_side_voltage_setpoint = voltage.clamp(12.0, 60.0);
                    println!(
                        "✅ Set dcdc_low_side_voltage_setpoint = {:.1}V",
                        state_lock.dcdc.dcdc_low_side_voltage_setpoint
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "dcdc_high_side_voltage_setpoint" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    state_lock.dcdc.dcdc_high_side_voltage_setpoint = voltage.clamp(400.0, 1000.0);
                    println!(
                        "✅ Set dcdc_high_side_voltage_setpoint = {:.1}V",
                        state_lock.dcdc.dcdc_high_side_voltage_setpoint
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }

            // Engine Controls
            "engine_speed" => {
                if let Ok(rpm) = value.parse::<f64>() {
                    state_lock.engine.engine_speed = rpm.clamp(600.0, 6000.0);
                    println!("✅ Set engine_speed = {:.0} RPM", state_lock.engine.engine_speed);
                } else {
                    println!("❌ Invalid number");
                }
            }
            "engine_load" => {
                if let Ok(load) = value.parse::<f64>() {
                    state_lock.engine.engine_load = load.clamp(0.0, 100.0);
                    println!("✅ Set engine_load = {:.1}%", state_lock.engine.engine_load);
                } else {
                    println!("❌ Invalid number");
                }
            }
            "engine_coolant_temp" => {
                if let Ok(temp) = value.parse::<f64>() {
                    state_lock.engine.engine_coolant_temp = temp.clamp(0.0, 120.0);
                    println!(
                        "✅ Set engine_coolant_temp = {:.1}°C",
                        state_lock.engine.engine_coolant_temp
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "etc9_current_preselection_gear" => {
                if let Ok(gear) = value.parse::<f64>() {
                    if (-125.0..=125.0).contains(&gear) {
                        state_lock.transmission.etc9_current_preselection_gear = gear;
                        println!("✅ Set etc9_current_preselection_gear = {}", gear);
                    } else {
                        println!(
                            "❌ Error: etc9_current_preselection_gear must be between -125.0 and 125.0"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for etc9_current_preselection_gear");
                }
            }
            "etc9_input_shaft1_speed" => {
                if let Ok(speed) = value.parse::<f64>() {
                    if (0.0..=8031.875).contains(&speed) {
                        state_lock.transmission.etc9_input_shaft1_speed = speed;
                        println!("✅ Set etc9_input_shaft1_speed = {} rpm", speed);
                    } else {
                        println!(
                            "❌ Error: etc9_input_shaft1_speed must be between 0.0 and 8031.875 rpm"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for etc9_input_shaft1_speed");
                }
            }
            "etc9_input_shaft2_speed" => {
                if let Ok(speed) = value.parse::<f64>() {
                    if (0.0..=8031.875).contains(&speed) {
                        state_lock.transmission.etc9_input_shaft2_speed = speed;
                        println!("✅ Set etc9_input_shaft2_speed = {} rpm", speed);
                    } else {
                        println!(
                            "❌ Error: etc9_input_shaft2_speed must be between 0.0 and 8031.875 rpm"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for etc9_input_shaft2_speed");
                }
            }
            "etc9_selected_preselection_gear" => {
                if let Ok(gear) = value.parse::<f64>() {
                    if (-125.0..=125.0).contains(&gear) {
                        state_lock.transmission.etc9_selected_preselection_gear = gear;
                        println!("✅ Set etc9_selected_preselection_gear = {}", gear);
                    } else {
                        println!(
                            "❌ Error: etc9_selected_preselection_gear must be between -125.0 and 125.0"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for etc9_selected_preselection_gear");
                }
            }
            "engine_exhaust_temp" => {
                if let Ok(temp) = value.parse::<f64>() {
                    state_lock.engine.engine_exhaust_temp = temp.clamp(100.0, 1000.0);
                    println!(
                        "✅ Set engine_exhaust_temp = {:.1}°C",
                        state_lock.engine.engine_exhaust_temp
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }
            "transmission_gear" => {
                if let Ok(gear) = value.parse::<u64>() {
                    if gear <= 16 {
                        state_lock.transmission.transmission_gear = gear;
                        println!("✅ Set transmission_gear = {}", gear);
                    } else {
                        println!("❌ Gear must be 0-16");
                    }
                } else {
                    println!("❌ Invalid number");
                }
            }

            // ALTC Alternator Control
            "altc_setpoint_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    if (0.0..=64.255).contains(&voltage) {
                        state_lock.power_supply.altc_setpoint_voltage = voltage;
                        println!("✅ Set altc_setpoint_voltage = {:.3}V", voltage);
                    } else {
                        println!("❌ Error: altc_setpoint_voltage must be between 0.0 and 64.255V");
                    }
                } else {
                    println!("❌ Error: Invalid float value for altc_setpoint_voltage");
                }
            }
            "altc_excitation_current_limit" => {
                if let Ok(current) = value.parse::<f64>() {
                    if (0.0..=62.5).contains(&current) {
                        state_lock.power_supply.altc_excitation_current_limit = current;
                        println!("✅ Set altc_excitation_current_limit = {:.1}A", current);
                    } else {
                        println!(
                            "❌ Error: altc_excitation_current_limit must be between 0.0 and 62.5A"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for altc_excitation_current_limit");
                }
            }
            "altc_torque_ramp_time" => {
                if let Ok(time) = value.parse::<f64>() {
                    if (0.1..=25.0).contains(&time) {
                        state_lock.power_supply.altc_torque_ramp_time = time;
                        println!("✅ Set altc_torque_ramp_time = {:.1}s", time);
                    } else {
                        println!("❌ Error: altc_torque_ramp_time must be between 0.1 and 25.0s");
                    }
                } else {
                    println!("❌ Error: Invalid float value for altc_torque_ramp_time");
                }
            }
            "altc_torque_ramp_max_speed" => {
                if let Ok(speed) = value.parse::<f64>() {
                    if (0.0..=8000.0).contains(&speed) {
                        state_lock.power_supply.altc_torque_ramp_max_speed = speed;
                        println!("✅ Set altc_torque_ramp_max_speed = {:.0} rpm", speed);
                    } else {
                        println!(
                            "❌ Error: altc_torque_ramp_max_speed must be between 0.0 and 8000.0 rpm"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for altc_torque_ramp_max_speed");
                }
            }

            // GC2 Generator Control 2
            "gc2_engine_load_setpoint" => {
                if let Ok(setpoint) = value.parse::<f64>() {
                    if (0.0..=32127.5).contains(&setpoint) {
                        state_lock.power_supply.gc2_engine_load_setpoint = setpoint;
                        println!("✅ Set gc2_engine_load_setpoint = {:.1}kW", setpoint);
                    } else {
                        println!(
                            "❌ Error: gc2_engine_load_setpoint must be between 0.0 and 32127.5kW"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for gc2_engine_load_setpoint");
                }
            }
            "gc2_derate_inhibit" => {
                if let Ok(inhibit) = value.parse::<u64>() {
                    if inhibit <= 3 {
                        state_lock.power_supply.gc2_derate_inhibit = inhibit as u8;
                        println!("✅ Set gc2_derate_inhibit = {}", inhibit);
                    } else {
                        println!("❌ Error: gc2_derate_inhibit must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for gc2_derate_inhibit");
                }
            }
            "gc2_governing_bias" => {
                if let Ok(bias) = value.parse::<f64>() {
                    if (-125.0..=125.0).contains(&bias) {
                        state_lock.power_supply.gc2_governing_bias = bias;
                        println!("✅ Set gc2_governing_bias = {:.3}%", bias);
                    } else {
                        println!("❌ Error: gc2_governing_bias must be between -125.0 and 125.0%");
                    }
                } else {
                    println!("❌ Error: Invalid float value for gc2_governing_bias");
                }
            }

            // DCACAI1S2 DC/AC Accessory Inverter 1 Status 2
            "dcacai1s2_desired_power" => {
                if let Ok(power) = value.parse::<f64>() {
                    if (0.0..=4015.9375).contains(&power) {
                        state_lock.power_supply.dcacai1s2_desired_power = power;
                        println!("✅ Set dcacai1s2_desired_power = {:.1}kW", power);
                    } else {
                        println!(
                            "❌ Error: dcacai1s2_desired_power must be between 0.0 and 4015.9375kW"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcacai1s2_desired_power");
                }
            }
            "dcacai1s2_desired_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    if (0.0..=3212.75).contains(&voltage) {
                        state_lock.power_supply.dcacai1s2_desired_voltage = voltage;
                        println!("✅ Set dcacai1s2_desired_voltage = {:.1}V", voltage);
                    } else {
                        println!(
                            "❌ Error: dcacai1s2_desired_voltage must be between 0.0 and 3212.75V"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcacai1s2_desired_voltage");
                }
            }
            "dcacai1s2_desired_current" => {
                if let Ok(current) = value.parse::<f64>() {
                    if (0.0..=4015.9375).contains(&current) {
                        state_lock.power_supply.dcacai1s2_desired_current = current;
                        println!("✅ Set dcacai1s2_desired_current = {:.1}A", current);
                    } else {
                        println!(
                            "❌ Error: dcacai1s2_desired_current must be between 0.0 and 4015.9375A"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcacai1s2_desired_current");
                }
            }
            "dcacai1s2_desired_ground_voltage" => {
                if let Ok(ground_voltage) = value.parse::<f64>() {
                    if (0.0..=3212.75).contains(&ground_voltage) {
                        state_lock.power_supply.dcacai1s2_desired_ground_voltage = ground_voltage;
                        println!(
                            "✅ Set dcacai1s2_desired_ground_voltage = {:.1}V",
                            ground_voltage
                        );
                    } else {
                        println!(
                            "❌ Error: dcacai1s2_desired_ground_voltage must be between 0.0 and 3212.75V"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcacai1s2_desired_ground_voltage");
                }
            }

            // AEBS Controls
            "aebs_enabled" => {
                if let Ok(enabled) = value.parse::<bool>() {
                    state_lock.braking.aebs_enabled = enabled;
                    println!("✅ Set aebs_enabled = {}", enabled);
                } else {
                    println!("❌ Invalid boolean value");
                }
            }
            "aebs_brake_demand" => {
                if let Ok(demand) = value.parse::<f64>() {
                    state_lock.braking.aebs_brake_demand = demand.clamp(0.0, 100.0);
                    println!(
                        "✅ Set aebs_brake_demand = {:.1}%",
                        state_lock.braking.aebs_brake_demand
                    );
                } else {
                    println!("❌ Invalid number");
                }
            }

            // DCDC1SBS Controls
            "dcdc1sbs_terminal_current" => {
                if let Ok(current) = value.parse::<f64>() {
                    if (-3212.7..=3212.8).contains(&current) {
                        state_lock.dcdc.dcdc1sbs_terminal_current = current;
                        println!("✅ Set dcdc1sbs_terminal_current = {:.1}A", current);
                    } else {
                        println!(
                            "❌ Error: dcdc1sbs_terminal_current must be between -3212.7 and 3212.8A"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc1sbs_terminal_current");
                }
            }
            "dcdc1sbs_terminal_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    if (0.0..=642.55).contains(&voltage) {
                        state_lock.dcdc.dcdc1sbs_terminal_voltage = voltage;
                        println!("✅ Set dcdc1sbs_terminal_voltage = {:.1}V", voltage);
                    } else {
                        println!(
                            "❌ Error: dcdc1sbs_terminal_voltage must be between 0.0 and 642.55V"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc1sbs_terminal_voltage");
                }
            }
            "dcdc1sbs_terminal_temperature" => {
                if let Ok(temperature) = value.parse::<f64>() {
                    if (-40.0..=210.0).contains(&temperature) {
                        state_lock.dcdc.dcdc1sbs_terminal_temperature = temperature;
                        println!(
                            "✅ Set dcdc1sbs_terminal_temperature = {:.1}°C",
                            temperature
                        );
                    } else {
                        println!(
                            "❌ Error: dcdc1sbs_terminal_temperature must be between -40.0 and 210.0°C"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc1sbs_terminal_temperature");
                }
            }

            // DCDC1S2 Controls
            "dcdc1s2_high_side_power" => {
                if let Ok(power) = value.parse::<f64>() {
                    if (-1600.0..=1612.75).contains(&power) {
                        state_lock.dcdc.dcdc1s2_high_side_power = power;
                        println!("✅ Set dcdc1s2_high_side_power = {:.1}kW", power);
                    } else {
                        println!(
                            "❌ Error: dcdc1s2_high_side_power must be between -1600.0 and 1612.75kW"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc1s2_high_side_power");
                }
            }
            "dcdc1s2_low_side_power" => {
                if let Ok(power) = value.parse::<f64>() {
                    if (-1600.0..=1612.75).contains(&power) {
                        state_lock.dcdc.dcdc1s2_low_side_power = power;
                        println!("✅ Set dcdc1s2_low_side_power = {:.1}kW", power);
                    } else {
                        println!(
                            "❌ Error: dcdc1s2_low_side_power must be between -1600.0 and 1612.75kW"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc1s2_low_side_power");
                }
            }
            "dcdc1s2_high_side_ground_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    if (0.0..=3212.75).contains(&voltage) {
                        state_lock.dcdc.dcdc1s2_high_side_ground_voltage = voltage;
                        println!("✅ Set dcdc1s2_high_side_ground_voltage = {:.1}V", voltage);
                    } else {
                        println!(
                            "❌ Error: dcdc1s2_high_side_ground_voltage must be between 0.0 and 3212.75V"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc1s2_high_side_ground_voltage");
                }
            }

            // DCACAI1V Controls
            "dcacai1v_ignition_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    if (0.0..=3212.75).contains(&voltage) {
                        state_lock.power_supply.dcacai1v_ignition_voltage = voltage;
                        println!("✅ Set dcacai1v_ignition_voltage = {:.1}V", voltage);
                    } else {
                        println!(
                            "❌ Error: dcacai1v_ignition_voltage must be between 0.0 and 3212.75V"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcacai1v_ignition_voltage");
                }
            }
            "dcacai1v_unswitched_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    if (0.0..=3212.75).contains(&voltage) {
                        state_lock.power_supply.dcacai1v_unswitched_voltage = voltage;
                        println!("✅ Set dcacai1v_unswitched_voltage = {:.1}V", voltage);
                    } else {
                        println!(
                            "❌ Error: dcacai1v_unswitched_voltage must be between 0.0 and 3212.75V"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcacai1v_unswitched_voltage");
                }
            }

            // GTRACE Controls
            "gtrace_kwh_export" => {
                if let Ok(kwh) = value.parse::<u64>() {
                    if kwh <= 4211081215 {
                        state_lock.power_supply.gtrace_kwh_export = kwh as u32;
                        println!("✅ Set gtrace_kwh_export = {} kWh", kwh);
                    } else {
                        println!(
                            "❌ Error: gtrace_kwh_export must be between 0 and 4211081215 kWh"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid integer value for gtrace_kwh_export");
                }
            }
            "gtrace_kvarh_export" => {
                if let Ok(kvarh) = value.parse::<u64>() {
                    if kvarh <= 4211081215 {
                        state_lock.power_supply.gtrace_kvarh_export = kvarh as u32;
                        println!("✅ Set gtrace_kvarh_export = {} kVArh", kvarh);
                    } else {
                        println!(
                            "❌ Error: gtrace_kvarh_export must be between 0 and 4211081215 kVArh"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid integer value for gtrace_kvarh_export");
                }
            }

            // DCDC2SBS Controls
            "dcdc2sbs_terminal_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    if (0.0..=642.55).contains(&voltage) {
                        state_lock.dcdc.dcdc2sbs_terminal_voltage = voltage;
                        println!("✅ Set dcdc2sbs_terminal_voltage = {:.2}V", voltage);
                    } else {
                        println!(
                            "❌ Error: dcdc2sbs_terminal_voltage must be between 0.0 and 642.55V"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc2sbs_terminal_voltage");
                }
            }
            "dcdc2sbs_terminal_current" => {
                if let Ok(current) = value.parse::<f64>() {
                    if (-3212.7..=3212.8).contains(&current) {
                        state_lock.dcdc.dcdc2sbs_terminal_current = current;
                        println!("✅ Set dcdc2sbs_terminal_current = {:.1}A", current);
                    } else {
                        println!(
                            "❌ Error: dcdc2sbs_terminal_current must be between -3212.7 and 3212.8A"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc2sbs_terminal_current");
                }
            }
            "dcdc2sbs_terminal_temperature" => {
                if let Ok(temp) = value.parse::<f64>() {
                    if (-40.0..=210.0).contains(&temp) {
                        state_lock.dcdc.dcdc2sbs_terminal_temperature = temp;
                        println!("✅ Set dcdc2sbs_terminal_temperature = {:.1}°C", temp);
                    } else {
                        println!(
                            "❌ Error: dcdc2sbs_terminal_temperature must be between -40.0 and 210.0°C"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc2sbs_terminal_temperature");
                }
            }

            // DCDC2S2 Controls
            "dcdc2s2_high_side_power" => {
                if let Ok(power) = value.parse::<f64>() {
                    if (-1600.0..=1612.75).contains(&power) {
                        state_lock.dcdc.dcdc2s2_high_side_power = power;
                        println!("✅ Set dcdc2s2_high_side_power = {:.1}kW", power);
                    } else {
                        println!(
                            "❌ Error: dcdc2s2_high_side_power must be between -1600.0 and 1612.75kW"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc2s2_high_side_power");
                }
            }
            "dcdc2s2_low_side_power" => {
                if let Ok(power) = value.parse::<f64>() {
                    if (-1600.0..=1612.75).contains(&power) {
                        state_lock.dcdc.dcdc2s2_low_side_power = power;
                        println!("✅ Set dcdc2s2_low_side_power = {:.1}kW", power);
                    } else {
                        println!(
                            "❌ Error: dcdc2s2_low_side_power must be between -1600.0 and 1612.75kW"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc2s2_low_side_power");
                }
            }
            "dcdc2s2_high_side_ground_voltage" => {
                if let Ok(voltage) = value.parse::<f64>() {
                    if (0.0..=3212.75).contains(&voltage) {
                        state_lock.dcdc.dcdc2s2_high_side_ground_voltage = voltage;
                        println!("✅ Set dcdc2s2_high_side_ground_voltage = {:.1}V", voltage);
                    } else {
                        println!(
                            "❌ Error: dcdc2s2_high_side_ground_voltage must be between 0.0 and 3212.75V"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for dcdc2s2_high_side_ground_voltage");
                }
            }

            // HVESSTS1 Controls - Phase 2 Pumps Thermal Management System
            "hvessts1_system_input_power" => {
                if let Ok(power) = value.parse::<f64>() {
                    if (0.0..=32127.5).contains(&power) {
                        state_lock.thermal.hvessts1_system_input_power = power;
                        println!("✅ Set hvessts1_system_input_power = {:.1}W", power);
                    } else {
                        println!(
                            "❌ Error: hvessts1_system_input_power must be between 0.0 and 32127.5W"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for hvessts1_system_input_power");
                }
            }
            "hvessts1_hv_input_power" => {
                if let Ok(power) = value.parse::<f64>() {
                    if (0.0..=32127.5).contains(&power) {
                        state_lock.thermal.hvessts1_hv_input_power = power;
                        println!("✅ Set hvessts1_hv_input_power = {:.1}W", power);
                    } else {
                        println!(
                            "❌ Error: hvessts1_hv_input_power must be between 0.0 and 32127.5W"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for hvessts1_hv_input_power");
                }
            }
            "hvessts1_compressor_speed" => {
                if let Ok(speed) = value.parse::<f64>() {
                    if (0.0..=8000.0).contains(&speed) {
                        state_lock.thermal.hvessts1_compressor_speed = speed;
                        println!("✅ Set hvessts1_compressor_speed = {:.0} rpm", speed);
                    } else {
                        println!(
                            "❌ Error: hvessts1_compressor_speed must be between 0.0 and 8000.0 rpm"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for hvessts1_compressor_speed");
                }
            }
            "hvessts1_relative_humidity" => {
                if let Ok(humidity) = value.parse::<f64>() {
                    if (0.0..=100.0).contains(&humidity) {
                        state_lock.thermal.hvessts1_relative_humidity = humidity;
                        println!("✅ Set hvessts1_relative_humidity = {:.1}%", humidity);
                    } else {
                        println!(
                            "❌ Error: hvessts1_relative_humidity must be between 0.0 and 100.0%"
                        );
                    }
                } else {
                    println!("❌ Error: Invalid float value for hvessts1_relative_humidity");
                }
            }
            "hvessts1_heater_status" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.thermal.hvessts1_heater_status = status as u8;
                        println!("✅ Set hvessts1_heater_status = {}", status);
                    } else {
                        println!("❌ Error: hvessts1_heater_status must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for hvessts1_heater_status");
                }
            }
            "hvessts1_hvil_status" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.thermal.hvessts1_hvil_status = status as u8;
                        println!("✅ Set hvessts1_hvil_status = {}", status);
                    } else {
                        println!("❌ Error: hvessts1_hvil_status must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for hvessts1_hvil_status");
                }
            }
            "hvessts1_system_mode" => {
                if let Ok(mode) = value.parse::<u64>() {
                    if mode <= 15 {
                        state_lock.thermal.hvessts1_system_mode = mode as u8;
                        println!("✅ Set hvessts1_system_mode = {}", mode);
                    } else {
                        println!("❌ Error: hvessts1_system_mode must be between 0 and 15");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for hvessts1_system_mode");
                }
            }
            "hvessts1_coolant_level" => {
                if let Ok(level) = value.parse::<u64>() {
                    if level <= 3 {
                        state_lock.thermal.hvessts1_coolant_level = level as u8;
                        println!("✅ Set hvessts1_coolant_level = {}", level);
                    } else {
                        println!("❌ Error: hvessts1_coolant_level must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for hvessts1_coolant_level");
                }
            }
            "hvessts1_coolant_level_full" => {
                if let Ok(level) = value.parse::<u64>() {
                    if level <= 3 {
                        state_lock.thermal.hvessts1_coolant_level_full = level as u8;
                        println!("✅ Set hvessts1_coolant_level_full = {}", level);
                    } else {
                        println!("❌ Error: hvessts1_coolant_level_full must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for hvessts1_coolant_level_full");
                }
            }

            // DM01 - Active Diagnostic Trouble Codes (J1939-73 Diagnostics)
            "dm01_protect_lamp" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.diagnostics.dm01_protect_lamp_status = status as u8;
                        println!("✅ Set dm01_protect_lamp_status = {}", status);
                    } else {
                        println!("❌ Error: dm01_protect_lamp must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm01_protect_lamp");
                }
            }
            "dm01_amber_lamp" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.diagnostics.dm01_amber_warning_lamp_status = status as u8;
                        println!("✅ Set dm01_amber_warning_lamp_status = {}", status);
                    } else {
                        println!("❌ Error: dm01_amber_lamp must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm01_amber_lamp");
                }
            }
            "dm01_red_stop_lamp" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.diagnostics.dm01_red_stop_lamp_status = status as u8;
                        println!("✅ Set dm01_red_stop_lamp_status = {}", status);
                    } else {
                        println!("❌ Error: dm01_red_stop_lamp must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm01_red_stop_lamp");
                }
            }
            "dm01_mil" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.diagnostics.dm01_malfunction_indicator_lamp_status = status as u8;
                        println!("✅ Set dm01_malfunction_indicator_lamp_status = {}", status);
                    } else {
                        println!("❌ Error: dm01_mil must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm01_mil");
                }
            }
            "dm01_fault_spn" => {
                if let Ok(spn) = value.parse::<u64>() {
                    if spn <= 0xFFFF {
                        state_lock.diagnostics.dm01_active_dtc_spn = spn as u16;
                        state_lock.diagnostics.dm01_fault_injection_enabled = true;
                        println!(
                            "✅ Set dm01_active_dtc_spn = {} (fault injection enabled)",
                            spn
                        );
                    } else {
                        println!("❌ Error: dm01_fault_spn must be between 0 and 65535");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm01_fault_spn");
                }
            }
            "dm01_fault_fmi" => {
                if let Ok(fmi) = value.parse::<u64>() {
                    if fmi <= 0xFF {
                        state_lock.diagnostics.dm01_active_dtc_fmi = fmi as u8;
                        state_lock.diagnostics.dm01_fault_injection_enabled = true;
                        println!(
                            "✅ Set dm01_active_dtc_fmi = {} (fault injection enabled)",
                            fmi
                        );
                    } else {
                        println!("❌ Error: dm01_fault_fmi must be between 0 and 255");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm01_fault_fmi");
                }
            }
            "dm01_fault_oc" => {
                if let Ok(oc) = value.parse::<u64>() {
                    if oc <= 0xFF {
                        state_lock.diagnostics.dm01_active_dtc_occurrence_count = oc as u8;
                        state_lock.diagnostics.dm01_fault_injection_enabled = true;
                        println!(
                            "✅ Set dm01_active_dtc_occurrence_count = {} (fault injection enabled)",
                            oc
                        );
                    } else {
                        println!("❌ Error: dm01_fault_oc must be between 0 and 255");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm01_fault_oc");
                }
            }

            // DM02 - Previously Active Diagnostic Trouble Codes (J1939-73 Diagnostics)
            "dm02_protect_lamp" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.diagnostics.dm02_protect_lamp_status = status as u8;
                        println!("✅ Set dm02_protect_lamp_status = {}", status);
                    } else {
                        println!("❌ Error: dm02_protect_lamp must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm02_protect_lamp");
                }
            }
            "dm02_amber_lamp" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.diagnostics.dm02_amber_warning_lamp_status = status as u8;
                        println!("✅ Set dm02_amber_warning_lamp_status = {}", status);
                    } else {
                        println!("❌ Error: dm02_amber_lamp must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm02_amber_lamp");
                }
            }
            "dm02_red_stop_lamp" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.diagnostics.dm02_red_stop_lamp_status = status as u8;
                        println!("✅ Set dm02_red_stop_lamp_status = {}", status);
                    } else {
                        println!("❌ Error: dm02_red_stop_lamp must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm02_red_stop_lamp");
                }
            }
            "dm02_mil" => {
                if let Ok(status) = value.parse::<u64>() {
                    if status <= 3 {
                        state_lock.diagnostics.dm02_malfunction_indicator_lamp_status = status as u8;
                        println!("✅ Set dm02_malfunction_indicator_lamp_status = {}", status);
                    } else {
                        println!("❌ Error: dm02_mil must be between 0 and 3");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm02_mil");
                }
            }
            "dm02_fault_spn" => {
                if let Ok(spn) = value.parse::<u64>() {
                    if spn <= 0xFFFF {
                        state_lock.diagnostics.dm02_previously_active_dtc_spn = spn as u16;
                        state_lock.diagnostics.dm02_fault_injection_enabled = true;
                        println!(
                            "✅ Set dm02_previously_active_dtc_spn = {} (fault injection enabled)",
                            spn
                        );
                    } else {
                        println!("❌ Error: dm02_fault_spn must be between 0 and 65535");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm02_fault_spn");
                }
            }
            "dm02_fault_fmi" => {
                if let Ok(fmi) = value.parse::<u64>() {
                    if fmi <= 0xFF {
                        state_lock.diagnostics.dm02_previously_active_dtc_fmi = fmi as u8;
                        state_lock.diagnostics.dm02_fault_injection_enabled = true;
                        println!(
                            "✅ Set dm02_previously_active_dtc_fmi = {} (fault injection enabled)",
                            fmi
                        );
                    } else {
                        println!("❌ Error: dm02_fault_fmi must be between 0 and 255");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm02_fault_fmi");
                }
            }

            // DM03 - Diagnostic Clear/Reset Command (J1939-73 Diagnostics)
            "dm03_clear_operations_enabled" => {
                if let Ok(enabled) = value.parse::<bool>() {
                    state_lock.diagnostics.dm03_clear_operations_enabled = enabled;
                    println!("✅ Set dm03_clear_operations_enabled = {}", enabled);
                } else {
                    println!("❌ Error: Invalid boolean value for dm03_clear_operations_enabled");
                }
            }
            "dm03_auto_response_enabled" => {
                if let Ok(enabled) = value.parse::<bool>() {
                    state_lock.diagnostics.dm03_auto_response_enabled = enabled;
                    println!("✅ Set dm03_auto_response_enabled = {}", enabled);
                } else {
                    println!("❌ Error: Invalid boolean value for dm03_auto_response_enabled");
                }
            }
            "dm03_command_generation_enabled" => {
                if let Ok(enabled) = value.parse::<bool>() {
                    state_lock.diagnostics.dm03_command_generation_enabled = enabled;
                    println!("✅ Set dm03_command_generation_enabled = {}", enabled);
                } else {
                    println!("❌ Error: Invalid boolean value for dm03_command_generation_enabled");
                }
            }
            "dm03_target_device_id" => {
                if let Ok(device_id) = parse_j1939_device_id(value) {
                    state_lock.diagnostics.dm03_target_device_id = device_id as u8;
                    println!("✅ Set dm03_target_device_id = 0x{:02X}", device_id as u8);
                } else {
                    println!("❌ Error: Invalid device ID for dm03_target_device_id");
                }
            }
            "dm03_command_interval_seconds" => {
                if let Ok(interval) = value.parse::<u64>() {
                    if interval <= 3600 {
                        state_lock.diagnostics.dm03_command_interval_seconds = interval;
                        if interval > 0 {
                            println!(
                                "✅ Set dm03_command_interval_seconds = {} (auto-generation enabled)",
                                interval
                            );
                        } else {
                            println!("✅ Set dm03_command_interval_seconds = 0 (manual only)");
                        }
                    } else {
                        println!("❌ Error: dm03_command_interval_seconds must be 0-3600");
                    }
                } else {
                    println!("❌ Error: Invalid number for dm03_command_interval_seconds");
                }
            }
            "dm03_send_command" => {
                if let Ok(send) = value.parse::<bool>() {
                    if send {
                        // Generate and send DM03 command immediately
                        let dm03_msg = DM03 {
                            device_id: DeviceId::from(state_lock.diagnostics.dm03_target_device_id),
                        };

                        if let Ok((can_id, _data)) = dm03_msg.encode() {
                            state_lock.diagnostics.dm03_commands_sent += 1;
                            state_lock.diagnostics.dm03_last_send_timestamp = state_lock.uptime_seconds;
                            println!(
                                "✅ DM03 command sent to device 0x{:02X} (CAN ID: 0x{:08X})",
                                state_lock.diagnostics.dm03_target_device_id, can_id
                            );
                        } else {
                            println!("❌ Error: Failed to encode DM03 command");
                        }
                    }
                } else {
                    println!("❌ Error: Invalid boolean value for dm03_send_command");
                }
            }
            "dm03_trigger_clear" => {
                if let Ok(trigger) = value.parse::<bool>() {
                    if trigger && state_lock.diagnostics.dm03_clear_operations_enabled {
                        // Simulate DM03 clear operation
                        state_lock.diagnostics.dm03_clear_commands_received += 1;
                        state_lock.diagnostics.dm03_last_clear_timestamp = state_lock.uptime_seconds;

                        // Clear active DTCs (DM01)
                        state_lock.diagnostics.dm01_active_dtc_spn = 0xFFFF;
                        state_lock.diagnostics.dm01_active_dtc_fmi = 0xFF;
                        state_lock.diagnostics.dm01_active_dtc_occurrence_count = 0xFF;
                        state_lock.diagnostics.dm01_active_dtc_conversion_method = 0xFF;

                        // Reset lamp states
                        state_lock.diagnostics.dm01_protect_lamp_status = 0;
                        state_lock.diagnostics.dm01_amber_warning_lamp_status = 0;
                        state_lock.diagnostics.dm01_red_stop_lamp_status = 0;
                        state_lock.diagnostics.dm01_malfunction_indicator_lamp_status = 0;

                        // Move to previously active if there was an active fault
                        if state_lock.diagnostics.dm01_fault_injection_enabled {
                            state_lock.diagnostics.dm02_previously_active_dtc_spn = 7945;
                            state_lock.diagnostics.dm02_previously_active_dtc_fmi = 9;
                            state_lock.diagnostics.dm02_previously_active_dtc_occurrence_count = 5;
                            state_lock.diagnostics.dm02_previously_active_dtc_conversion_method = 0;
                        }

                        state_lock.diagnostics.dm01_fault_injection_enabled = false;
                        println!("✅ DM03 clear operation triggered - DTCs cleared");
                    } else if trigger && !state_lock.diagnostics.dm03_clear_operations_enabled {
                        println!(
                            "❌ Error: dm03_clear_operations_enabled must be true to trigger clear"
                        );
                    } else {
                        println!("✅ Set dm03_trigger_clear = {}", trigger);
                    }
                } else {
                    println!("❌ Error: Invalid boolean value for dm03_trigger_clear");
                }
            }
            "dm02_fault_oc" => {
                if let Ok(oc) = value.parse::<u64>() {
                    if oc <= 0xFF {
                        state_lock.diagnostics.dm02_previously_active_dtc_occurrence_count = oc as u8;
                        state_lock.diagnostics.dm02_fault_injection_enabled = true;
                        println!(
                            "✅ Set dm02_previously_active_dtc_occurrence_count = {} (fault injection enabled)",
                            oc
                        );
                    } else {
                        println!("❌ Error: dm02_fault_oc must be between 0 and 255");
                    }
                } else {
                    println!("❌ Error: Invalid integer value for dm02_fault_oc");
                }
            }

            _ => {
                println!(
                    "❌ Unknown field '{}'. Type 'help' for available fields.",
                    field
                );
            }
        }
    } else {
        println!("❌ Invalid format. Use: set field=value");
    }
}

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.contains(&"--generate-manpage".to_string()) {
        #[cfg(feature = "manpages")]
        {
            use clap::CommandFactory;
            use clap_mangen::Man;
            let man = Man::new(Args::command());
            man.render(&mut std::io::stdout())
                .expect("Failed to render man page to stdout");
            return;
        }
        #[cfg(not(feature = "manpages"))]
        {
            eprintln!(
                "clap_mangen feature not enabled. Build with --features manpages to generate man pages"
            );
            std::process::exit(1);
        }
    }

    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    // Resolve configuration with precedence
    let config = args.common.resolve_config()?;

    // Create Tokio runtime for WebSocket
    let rt = tokio::runtime::Runtime::new()?;
    let _guard = rt.enter();

    let mut simulator = J1939Simulator::new(args, config)?;
    simulator.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_device_id_hex() {
        assert_eq!(parse_j1939_device_id("0x82").unwrap(), 0x82);
        assert_eq!(parse_j1939_device_id("0xFF").unwrap(), 0xFF);
    }

    #[test]
    fn test_parse_device_id_decimal() {
        assert_eq!(parse_j1939_device_id("138").unwrap(), 138);
        assert_eq!(parse_j1939_device_id("255").unwrap(), 255);
    }

    #[test]
    fn test_parse_device_id_invalid() {
        assert!(parse_j1939_device_id("0x100").is_err());
        assert!(parse_j1939_device_id("256").is_err());
    }

    #[test]
    fn test_simulator_state_physics() {
        let mut state = SimulatorState {
            sensors: SensorState { target_wand_angle: 45.0, ..Default::default() },
            ..Default::default()
        };

        // Set target wand angle
        state.update_physics(0.1); // 100ms
        assert!(state.sensors.wand_angle > 0.0);
        assert!(state.sensors.wand_angle < 45.0);

        // Set target displacement
        state.sensors.target_displacement = 100.0;
        state.update_physics(0.1);
        assert!(state.sensors.linear_displacement > 0.0);
        assert!(state.sensors.linear_displacement < 100.0);
    }

    #[test]
    fn test_crash_checksum() {
        let mut state = SimulatorState {
            crash: CrashState {
                crash_detected: true,
                crash_type: 5,
                crash_counter: 3,
                ..Default::default()
            },
            ..Default::default()
        };
        state.update_physics(0.01);

        // Checksum should be (5 + 3) % 16 = 8
        assert_eq!(state.crash.crash_checksum, 8);
    }

    #[test]
    fn test_generate_can_frames() {
        let mut state = SimulatorState {
            device_id: 0x82,
            crash: CrashState {
                crash_detected: true,
                crash_type: 1,
                crash_counter: 1,
                ..Default::default()
            },
            sensors: SensorState {
                wand_angle: 45.0,
                linear_displacement: 100.0,
                ..Default::default()
            },
            ..Default::default()
        };
        state.update_physics(0.01); // Update checksum

        let frames = state.generate_can_frames();

        // Total frame count (with crash notification + all batch messages integrated)
        assert_eq!(frames.len(), 306);
    }

    #[test]
    fn test_wand_angle_clamping_upper_bound() {
        let mut state = SimulatorState {
            sensors: SensorState { target_wand_angle: 300.0, ..Default::default() }, // Beyond max
            ..Default::default()
        };
        state.update_physics(10.0); // Long time to reach target

        // Should be clamped to max (252.19)
        assert_eq!(state.sensors.wand_angle, 252.19);
    }

    #[test]
    fn test_wand_angle_clamping_lower_bound() {
        let mut state = SimulatorState {
            sensors: SensorState { target_wand_angle: -300.0, ..Default::default() }, // Below min
            ..Default::default()
        };
        state.update_physics(10.0); // Long time to reach target

        // Should be clamped to min (-250.0)
        assert_eq!(state.sensors.wand_angle, -250.0);
    }

    #[test]
    fn test_displacement_clamping_upper_bound() {
        let mut state = SimulatorState {
            sensors: SensorState { target_displacement: 10000.0, ..Default::default() }, // Beyond max
            ..Default::default()
        };
        state.update_physics(50.0); // Long time to reach target

        // Should be clamped to max (6425.5)
        assert_eq!(state.sensors.linear_displacement, 6425.5);
    }

    #[test]
    fn test_displacement_clamping_lower_bound() {
        let mut state = SimulatorState {
            sensors: SensorState {
                linear_displacement: 100.0,
                target_displacement: -100.0, // Below zero
                ..Default::default()
            },
            ..Default::default()
        };
        state.update_physics(1.0);

        // Should be clamped to min (0.0)
        assert_eq!(state.sensors.linear_displacement, 0.0);
    }

    #[test]
    fn test_physics_convergence() {
        let mut state = SimulatorState {
            sensors: SensorState { target_wand_angle: 90.0, ..Default::default() },
            ..Default::default()
        };

        // Simulate for long enough to reach target
        for _ in 0..100 {
            state.update_physics(0.1);
        }

        // Should have converged to target
        assert!((state.sensors.wand_angle - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_gradual_wand_angle_transition() {
        let mut state = SimulatorState {
            sensors: SensorState { target_wand_angle: 100.0, ..Default::default() },
            ..Default::default()
        };

        state.update_physics(0.1); // 100ms, max change = 5 degrees
        let first_update = state.sensors.wand_angle;

        assert!(first_update > 0.0);
        assert!(first_update <= 5.0); // Should not jump immediately

        state.update_physics(0.1);
        let second_update = state.sensors.wand_angle;

        assert!(second_update > first_update); // Should continue moving
        assert!(second_update < 100.0); // Should not reach target yet
    }

    #[test]
    fn test_gradual_displacement_transition() {
        let mut state = SimulatorState {
            sensors: SensorState { target_displacement: 1000.0, ..Default::default() },
            ..Default::default()
        };

        state.update_physics(0.1); // 100ms, max change = 50mm
        let first_update = state.sensors.linear_displacement;

        assert!(first_update > 0.0);
        assert!(first_update <= 50.0); // Should not jump immediately

        state.update_physics(0.1);
        let second_update = state.sensors.linear_displacement;

        assert!(second_update > first_update); // Should continue moving
        assert!(second_update < 1000.0); // Should not reach target yet
    }

    #[test]
    fn test_crash_checksum_updates() {
        let mut state = SimulatorState {
            crash: CrashState {
                crash_detected: true,
                crash_type: 7,
                crash_counter: 8,
                ..Default::default()
            },
            ..Default::default()
        };

        state.update_physics(0.01);
        assert_eq!(state.crash.crash_checksum, (7 + 8)); // = 15

        // Change counter
        state.crash.crash_counter = 10;
        state.update_physics(0.01);
        assert_eq!(state.crash.crash_checksum, (7 + 10) % 16); // = 1
    }

    #[test]
    fn test_checksum_wraps_at_16() {
        let mut state = SimulatorState {
            crash: CrashState {
                crash_detected: true,
                crash_type: 15,
                crash_counter: 15,
                ..Default::default()
            },
            ..Default::default()
        };

        state.update_physics(0.01);
        assert_eq!(state.crash.crash_checksum, (15 + 15) % 16); // = 14

        state.crash.crash_type = 20;
        state.crash.crash_counter = 30;
        state.update_physics(0.01);
        assert_eq!(state.crash.crash_checksum, (20 + 30) % 16); // = 2
    }

    #[test]
    fn test_no_checksum_update_when_crash_not_detected() {
        let mut state = SimulatorState {
            crash: CrashState {
                crash_detected: false,
                crash_type: 5,
                crash_counter: 3,
                crash_checksum: 99, // Intentionally wrong,
                ..Default::default()
            },
            ..Default::default()
        };
        state.crash.crash_checksum = 99; // Set to unusual value

        state.update_physics(0.01);

        // Checksum should not be updated
        assert_eq!(state.crash.crash_checksum, 99);
    }

    #[test]
    fn test_generate_frames_without_crash() {
        let state = SimulatorState {
            device_id: 0x82,
            crash: CrashState { crash_detected: false, ..Default::default() },
            sensors: SensorState {
                wand_angle: 30.0,
                linear_displacement: 50.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let frames = state.generate_can_frames();

        // Total frame count (no crash notification + all batch messages integrated)
        assert_eq!(frames.len(), 305);
    }

    #[test]
    fn test_default_state_values() {
        let state = SimulatorState::default();

        assert_eq!(state.device_id, 0x8A);
        assert!(!state.crash.crash_detected);
        assert_eq!(state.crash.crash_type, 0);
        assert_eq!(state.crash.crash_counter, 0);
        assert_eq!(state.crash.crash_checksum, 0);
        assert_eq!(state.sensors.wand_angle, 0.0);
        assert_eq!(state.sensors.target_wand_angle, 0.0);
        assert_eq!(state.sensors.wand_quality, 3);
        assert_eq!(state.sensors.linear_displacement, 0.0);
        assert_eq!(state.sensors.target_displacement, 0.0);
        assert_eq!(state.sensors.displacement_quality, 3);
        assert_eq!(state.uptime_seconds, 0);
    }

    #[test]
    fn test_device_id_variants() {
        for (id, expected_count) in &[(0x82, 306), (0x8B, 306), (0x8C, 306), (0x8D, 306)] {
            let state = SimulatorState {
                device_id: *id,
                crash: CrashState {
                    crash_detected: true,
                    crash_type: 1,
                    ..Default::default()
                },
                ..Default::default()
                    };

            let frames = state.generate_can_frames();
            assert_eq!(frames.len(), *expected_count);
        }
    }

    #[test]
    fn test_websocket_message_get_state() {
        let state = Arc::new(Mutex::new(SimulatorState::default()));
        let msg = WebSocketMessage::GetState;

        let response = handle_websocket_message(msg, &state);

        match response {
            WebSocketMessage::StateResponse { state_json } => {
                let parsed_state: SimulatorState =
                    serde_json::from_str(&state_json).expect("Failed to parse state JSON");
                assert_eq!(parsed_state.device_id, 0x8A);
                assert!(!parsed_state.crash.crash_detected);
            }
            _ => panic!("Expected StateResponse with JSON"),
        }
    }

    #[test]
    fn test_websocket_message_set_crash() {
        let state = Arc::new(Mutex::new(SimulatorState::default()));
        let msg = WebSocketMessage::SetCrash {
            detected: true,
            crash_type: 5,
        };

        let response = handle_websocket_message(msg, &state);

        match response {
            WebSocketMessage::StateUpdate { state: s } => {
                assert!(s.crash.crash_detected);
                assert_eq!(s.crash.crash_type, 5);
                assert_eq!(s.crash.crash_counter, 1); // Should increment
            }
            _ => panic!("Expected StateUpdate response"),
        }
    }

    #[test]
    fn test_websocket_message_set_wand_angle() {
        let state = Arc::new(Mutex::new(SimulatorState::default()));
        let msg = WebSocketMessage::SetWandAngle { angle: 45.5 };

        let response = handle_websocket_message(msg, &state);

        match response {
            WebSocketMessage::StateUpdate { state: s } => {
                assert_eq!(s.sensors.target_wand_angle, 45.5);
            }
            _ => panic!("Expected StateUpdate response"),
        }
    }

    #[test]
    fn test_websocket_message_set_displacement() {
        let state = Arc::new(Mutex::new(SimulatorState::default()));
        let msg = WebSocketMessage::SetDisplacement {
            displacement: 123.4,
        };

        let response = handle_websocket_message(msg, &state);

        match response {
            WebSocketMessage::StateUpdate { state: s } => {
                assert_eq!(s.sensors.target_displacement, 123.4);
            }
            _ => panic!("Expected StateUpdate response"),
        }
    }

    #[test]
    fn test_websocket_message_reset() {
        let initial_state = SimulatorState {
            device_id: 0x8B,
            crash: CrashState { crash_detected: true, ..Default::default() },
            sensors: SensorState { wand_angle: 100.0, ..Default::default() },
            ..Default::default()
        };

        let state = Arc::new(Mutex::new(initial_state));
        let msg = WebSocketMessage::Reset;

        let response = handle_websocket_message(msg, &state);

        match response {
            WebSocketMessage::StateUpdate { state: s } => {
                assert_eq!(s.device_id, 0x8B); // Device ID preserved
                assert!(!s.crash.crash_detected); // Reset to default
                assert_eq!(s.sensors.wand_angle, 0.0); // Reset to default
            }
            _ => panic!("Expected StateUpdate response"),
        }
    }

    #[test]
    fn test_websocket_wand_angle_clamping() {
        let state = Arc::new(Mutex::new(SimulatorState::default()));

        // Test upper bound clamping
        let msg = WebSocketMessage::SetWandAngle { angle: 300.0 };
        let response = handle_websocket_message(msg, &state);

        match response {
            WebSocketMessage::StateUpdate { state: s } => {
                assert_eq!(s.sensors.target_wand_angle, 252.19);
            }
            _ => panic!("Expected StateUpdate response"),
        }

        // Test lower bound clamping
        let msg = WebSocketMessage::SetWandAngle { angle: -300.0 };
        let response = handle_websocket_message(msg, &state);

        match response {
            WebSocketMessage::StateUpdate { state: s } => {
                assert_eq!(s.sensors.target_wand_angle, -250.0);
            }
            _ => panic!("Expected StateUpdate response"),
        }
    }

    #[test]
    fn test_websocket_displacement_clamping() {
        let state = Arc::new(Mutex::new(SimulatorState::default()));

        // Test upper bound clamping
        let msg = WebSocketMessage::SetDisplacement {
            displacement: 10000.0,
        };
        let response = handle_websocket_message(msg, &state);

        match response {
            WebSocketMessage::StateUpdate { state: s } => {
                assert_eq!(s.sensors.target_displacement, 6425.5);
            }
            _ => panic!("Expected StateUpdate response"),
        }

        // Test lower bound clamping
        let msg = WebSocketMessage::SetDisplacement {
            displacement: -100.0,
        };
        let response = handle_websocket_message(msg, &state);

        match response {
            WebSocketMessage::StateUpdate { state: s } => {
                assert_eq!(s.sensors.target_displacement, 0.0);
            }
            _ => panic!("Expected StateUpdate response"),
        }
    }

    #[test]
    fn test_create_can_frame_with_valid_id() {
        let can_id = 0x80F02B82;
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

        let result = create_can_frame(can_id, &data, FrameType::Extended);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_can_frame_masks_to_29_bits() {
        // ID that's larger than 29 bits
        let can_id = 0xFFFFFFFF;
        let data = [0u8; 8];

        let result = create_can_frame(can_id, &data, FrameType::Extended);
        // Should succeed because it's masked to 29 bits
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_physics_updates_converge() {
        let mut state = SimulatorState {
            sensors: SensorState {
                target_wand_angle: 50.0,
                target_displacement: 500.0,
                ..Default::default()
            },
            ..Default::default()
        };

        // Run physics for 2 seconds
        for _ in 0..200 {
            state.update_physics(0.01);
        }

        // Both should have converged
        assert!((state.sensors.wand_angle - 50.0).abs() < 0.1);
        assert!((state.sensors.linear_displacement - 500.0).abs() < 1.0);
    }

    #[test]
    fn test_state_quality_defaults() {
        let state = SimulatorState::default();

        // Quality should default to 3 (excellent)
        assert_eq!(state.sensors.wand_quality, 3);
        assert_eq!(state.sensors.displacement_quality, 3);
    }

    #[test]
    fn test_crash_counter_increment_on_detection() {
        let state = Arc::new(Mutex::new(SimulatorState::default()));

        // First crash
        let msg = WebSocketMessage::SetCrash {
            detected: true,
            crash_type: 1,
        };
        handle_websocket_message(msg, &state);

        let counter1 = state.lock().unwrap().crash.crash_counter;
        assert_eq!(counter1, 1);

        // Second crash
        let msg = WebSocketMessage::SetCrash {
            detected: true,
            crash_type: 2,
        };
        handle_websocket_message(msg, &state);

        let counter2 = state.lock().unwrap().crash.crash_counter;
        assert_eq!(counter2, 2);
    }

    #[test]
    fn test_no_crash_counter_increment_when_clearing() {
        let initial_state = SimulatorState {
            crash: CrashState {
                crash_counter: 5,
                crash_detected: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let state = Arc::new(Mutex::new(initial_state));

        // Clear crash
        let msg = WebSocketMessage::SetCrash {
            detected: false,
            crash_type: 0,
        };
        handle_websocket_message(msg, &state);

        let counter = state.lock().unwrap().crash.crash_counter;
        assert_eq!(counter, 5); // Should not increment
    }

    #[test]
    fn test_parse_device_id_boundary_values() {
        assert_eq!(parse_j1939_device_id("0x00").unwrap(), 0);
        assert_eq!(parse_j1939_device_id("0xFF").unwrap(), 255);
        assert_eq!(parse_j1939_device_id("0").unwrap(), 0);
        assert_eq!(parse_j1939_device_id("255").unwrap(), 255);
    }

    #[test]
    fn test_physics_small_delta_time() {
        let mut state = SimulatorState {
            sensors: SensorState { target_wand_angle: 100.0, ..Default::default() },
            ..Default::default()
        };

        // Very small time step
        state.update_physics(0.001); // 1ms

        // Should move but not much
        assert!(state.sensors.wand_angle > 0.0);
        assert!(state.sensors.wand_angle < 0.1);
    }

    #[test]
    fn test_physics_large_delta_time() {
        let mut state = SimulatorState {
            sensors: SensorState { target_wand_angle: 45.0, ..Default::default() },
            ..Default::default()
        };

        // Large time step (1 second)
        state.update_physics(1.0);

        // Should reach or be very close to target
        assert!((state.sensors.wand_angle - 45.0).abs() < 0.01);
    }

    // ========================================================================
    // Command Processing Tests (Phase 2: Critical Testing Gap Resolution)
    // ========================================================================
    // These tests verify that incoming CAN messages are properly recognized
    // and processed by the simulator, addressing the critical 0% test coverage
    // gap identified in doc/testing/simulator-command-handling-analysis.md

    #[test]
    fn test_mg1ic_command_processing() {
        let mut state = SimulatorState::default();
        // MG1IC: Test with raw CAN data (matches simulator base ID 0x0C26FE00)
        let can_id = 0x0C26FE82;
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
    }

    #[test]
    fn test_mg2ic_command_processing() {
        let mut state = SimulatorState::default();
        // MG2IC: Test with raw CAN data (matches simulator base ID 0x0C27FE00)
        let can_id = 0x0C27FE82;
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
    }

    #[test]
    fn test_hvessc1_command_processing() {
        let mut state = SimulatorState::default();
        // Use 29-bit CAN ID that matches real CAN traffic (after CAN_EFF_MASK)
        // HVESSC1::BASE_CAN_ID = 0x0C1BFE00, add device 0x82 = 0x0C1BFE82
        let can_id = 0x0C1BFE82;
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
    }

    #[test]
    fn test_dcdc1c_command_processing() {
        let mut state = SimulatorState::default();
        // DCDC1C: Use 29-bit CAN ID that matches real CAN traffic (after CAN_EFF_MASK)
        // DCDC1C::BASE_CAN_ID = 0x0CF11200, add device 0x82 = 0x0CF11282
        let can_id = 0x0CF11282;
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
    }

    #[test]
    fn test_etc9_command_processing() {
        let mut state = SimulatorState::default();
        let msg = ETC9 {
            device_id: DeviceId::from(0x00),
            dl_clth_trnsmssn_crrnt_pr_sltn_gr: 3.0,
            dl_clth_trnsmssn_inpt_shft_1_spd: 1500.0,
            dl_clth_trnsmssn_inpt_shft_2_spd: 1450.0,
            dl_clth_trnsmssn_sltd_pr_sltn_gr: 4.0,
        };
        let (can_id, data) = msg.encode().unwrap();

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
        assert_eq!(state.transmission.etc9_current_preselection_gear, 3.0);
        assert_eq!(state.transmission.etc9_input_shaft1_speed, 1500.0);
    }

    // Tests for simpler well-known messages

    // ========================================================================
    // Tests for the 5 Recently Added J1939 Messages (EEC12, ETC5, EEC22, EEC21, ETCC2)
    // ========================================================================
    // These messages were added recently and have full encode_real() support

    #[test]
    fn test_eec12_command_processing() {
        let mut state = SimulatorState::default();
        // Use raw CAN ID that matches simulator expectation (base 0x18FCCC00)
        let can_id = 0x18FCCC82; // EEC12 base + device 0x82
        let data = [0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x00, 0x00]; // Valid EEC12 data

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
        // State should be updated with values from message
    }

    #[test]
    fn test_altc_command_processing() {
        let mut state = SimulatorState::default();
        // Use 29-bit CAN ID that matches real CAN traffic (after CAN_EFF_MASK)
        // ALTC::BASE_CAN_ID = 0x0C1EFE00, add device 0x82 = 0x0C1EFE82
        let can_id = 0x0C1EFE82;
        let data = [0x00, 0x00, 0x00, 0x00, 0x00]; // Valid ALTC data

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
    }

    #[test]
    fn test_etc5_command_processing() {
        let mut state = SimulatorState::default();
        // Use raw CAN ID that matches simulator expectation
        // ETC5::BASE_CAN_ID = 0x1CFEC300, add device 0x82 = 0x1CFEC382
        let can_id = 0x1CFEC382;
        let data = [0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Valid ETC5 data

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
        // State should be updated with values from message
    }

    #[test]
    fn test_eec22_command_processing() {
        let mut state = SimulatorState::default();
        // Use raw CAN ID that matches simulator expectation (base 0x18FB9400)
        let can_id = 0x18FB9482; // EEC22 base + device 0x82
        let data = [0x00, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00]; // Valid EEC22 data

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
        // State should be updated with values from message
    }

    #[test]
    fn test_eec21_command_processing() {
        let mut state = SimulatorState::default();
        // Use raw CAN ID that matches simulator expectation (base 0x18FBD600)
        let can_id = 0x18FBD682; // EEC21 base + device 0x82
        let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Valid EEC21 data

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
        // State should be updated with values from message
    }

    #[test]
    fn test_etcc2_command_processing() {
        let mut state = SimulatorState::default();
        // Use raw CAN ID that matches simulator expectation (base 0x18FC3F00)
        let can_id = 0x18FC3F82; // ETCC2 base + device 0x82
        let data = [0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Valid ETCC2 data

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
        // State should be updated with values from message
    }

    #[test]
    fn test_etcc3_command_processing() {
        let mut state = SimulatorState::default();
        let msg = ETCC3 {
            device_id: DeviceId::from(0x00),
            engn_clndr_hd_bpss_attr_1_mtr_crrnt_dsl: 1,
            engn_thrttl_vlv_1_mtr_crrnt_dsl: 2,
            engn_trhrgr_wstgt_attr_1_mtr_crrnt_dsl: 1,
            engn_trhrgr_wstgt_attr_2_mtr_crrnt_dsl: 1,
            et_cpss_bpss_att_1_mt_ct_ds: 2,
            et_cpss_bpss_att_2_mt_ct_ds: 2,
            et_cpss_bw_att_1_mt_ct_ds: 1,
        };
        let (can_id, data) = msg.encode().unwrap();

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
        assert_eq!(state.thermal.etcc3_throttle_valve_1, 2);
        assert_eq!(state.thermal.etcc3_etc_bypass_actuator_1, 1);
    }

    // ========================================================================
    // Tests for J1939-73 Diagnostic Messages
    // ========================================================================

    #[test]
    fn test_dm03_command_processing() {
        let mut state = SimulatorState {
            diagnostics: DiagnosticsState {
                dm03_clear_operations_enabled: true,
                dm01_active_dtc_spn: 1234,
                ..Default::default()
            },
            ..Default::default()
        };

        let msg = DM03 {
            device_id: DeviceId::from(0x00),
        };
        let (can_id, data) = msg.encode().unwrap();

        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);
        // Verify DTCs were cleared
        assert_eq!(state.diagnostics.dm01_active_dtc_spn, 0xFFFF);
    }

    // ========================================================================
    // Core Recognition Logic Tests
    // ========================================================================
    // These tests verify the MessageStatus enum works correctly

    #[test]
    fn test_unrecognized_message() {
        let mut state = SimulatorState::default();

        // Send a CAN message with an unknown base ID (29-bit CAN ID masked)
        let unknown_can_id = 0x12345678;
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

        let status = state
            .process_incoming_message(unknown_can_id, &data)
            .unwrap();
        assert_eq!(status, MessageStatus::Unrecognized);
    }

    #[test]
    fn test_recognized_message_with_valid_data() {
        let mut state = SimulatorState::default();

        // Test MG1IC with valid data
        let mg1ic_can_id = 0x0C26FE82; // MG1IC base + device ID 0x82
        let valid_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let result = state.process_incoming_message(mg1ic_can_id, &valid_data);
        assert!(result.is_ok());

        let status = result.unwrap();
        // Should be recognized (might decode successfully or fail, but not unrecognized)
        assert!(status == MessageStatus::Recognized || status == MessageStatus::DecodeFailed);
    }

    #[test]
    fn test_etcc1_command_processing() {
        let mut state = SimulatorState::default();

        // Create ETCC1 message
        let msg = ETCC1 {
            device_id: DeviceId::from(0x00),
            engn_trhrgr_wstgt_attr_1_cmmnd: 75.5,
            engn_trhrgr_wstgt_attr_2_cmmnd: 80.25,
            e_exst_b_1_pss_rt_ct_cd: 60.0,
            et_cpss_bw_att_1_cd: 45.5,
        };

        let (can_id, data) = msg.encode().unwrap();

        // Process the message
        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);

        // Verify state was updated
        assert!((state.transmission.etcc1_engn_trhrgr_wstgt_attr_1_cmmnd - 75.5).abs() < 0.1);
        assert!((state.transmission.etcc1_engn_trhrgr_wstgt_attr_2_cmmnd - 80.25).abs() < 0.1);
        assert!((state.transmission.etcc1_e_exst_b_1_pss_rt_ct_cd - 60.0).abs() < 0.1);
        assert!((state.transmission.etcc1_et_cpss_bw_att_1_cd - 45.5).abs() < 0.1);
    }

    #[test]
    fn test_eec17_command_processing() {
        let mut state = SimulatorState::default();

        // Create EEC17 message
        let msg = EEC17 {
            device_id: DeviceId::from(0x00),
            pems_engine_fuel_mass_flow_rate: 250.5,
            vehicle_fuel_rate: 240.0,
            engine_exhaust_flow_rate: 3500.0,
            cylinder_fuel_rate: 75.25,
        };

        let (can_id, data) = msg.encode().unwrap();

        // Process the message
        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);

        // Verify state was updated
        assert!((state.engine.eec17_pems_engine_fuel_mass_flow_rate - 250.5).abs() < 0.5);
        assert!((state.engine.eec17_vehicle_fuel_rate - 240.0).abs() < 0.5);
        assert!((state.engine.eec17_engine_exhaust_flow_rate - 3500.0).abs() < 1.0);
        assert!((state.engine.eec17_cylinder_fuel_rate - 75.25).abs() < 0.5);
    }

    #[test]
    fn test_etc6_command_processing() {
        let mut state = SimulatorState::default();

        // Create ETC6 message
        let msg = ETC6 {
            device_id: DeviceId::from(0x00),
            recommended_gear: 4.0,
            lowest_possible_gear: 1.0,
            highest_possible_gear: 8.0,
            clutch_life_remaining: 72.5,
        };

        let (can_id, data) = msg.encode().unwrap();

        // Process the message
        let status = state.process_incoming_message(can_id, &data).unwrap();
        assert_eq!(status, MessageStatus::Recognized);

        // Verify state was updated
        assert!((state.transmission.etc6_recommended_gear - 4.0).abs() < 0.5);
        assert!((state.transmission.etc6_lowest_possible_gear - 1.0).abs() < 0.5);
        assert!((state.transmission.etc6_highest_possible_gear - 8.0).abs() < 0.5);
        assert!((state.transmission.etc6_clutch_life_remaining - 72.5).abs() < 0.5);
    }
}

#[test]
fn test_etc2_command_processing() {
    let mut state = SimulatorState::default();

    // Create ETC2 message
    let msg = ETC2 {
        device_id: DeviceId::from(0x00),
        transmission_selected_gear: 5.0,
        transmission_actual_gear_ratio: 2.85,
        transmission_current_gear: 4.0,
        transmission_requested_range: 5,
        transmission_current_range: 5,
    };

    let (can_id, data) = msg.encode().unwrap();

    // Process the message
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);

    // Verify state was updated
    assert!((state.transmission.etc2_transmission_selected_gear - 5.0).abs() < 0.5);
    assert!((state.transmission.etc2_transmission_actual_gear_ratio - 2.85).abs() < 0.01);
    assert!((state.transmission.etc2_transmission_current_gear - 4.0).abs() < 0.5);
    assert_eq!(state.transmission.etc2_transmission_requested_range, 5);
    assert_eq!(state.transmission.etc2_transmission_current_range, 5);
}

#[test]
fn test_eec8_command_processing() {
    let mut state = SimulatorState::default();

    // Create EEC8 message
    let msg = EEC8 {
        device_id: DeviceId::from(0x00),
        engn_exhst_gs_rrltn_1_vlv_2_cntrl: 55.5,
        engn_exhst_gs_rrltn_1_clr_intk_tmprtr: 95.0,
        e_exst_gs_rt_1_c_it_ast_pss: 175.5,
        engn_exhst_gs_rrltn_1_clr_effn: 80.0,
        e_exst_gs_rt_at_it_ct_tpt: 105.0,
    };

    let (can_id, data) = msg.encode().unwrap();

    // Process the message
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);

    // Verify state was updated
    assert!((state.engine.eec8_engn_exhst_gs_rrltn_1_vlv_2_cntrl - 55.5).abs() < 0.1);
    assert!((state.engine.eec8_engn_exhst_gs_rrltn_1_clr_intk_tmprtr - 95.0).abs() < 0.5);
    assert!((state.engine.eec8_e_exst_gs_rt_1_c_it_ast_pss - 175.5).abs() < 1.0);
    assert!((state.engine.eec8_engn_exhst_gs_rrltn_1_clr_effn - 80.0).abs() < 0.5);
    assert!((state.engine.eec8_e_exst_gs_rt_at_it_ct_tpt - 105.0).abs() < 0.5);
}

#[test]
fn test_eec15_command_processing() {
    let mut state = SimulatorState::default();

    // Create EEC15 message
    let msg = EEC15 {
        device_id: DeviceId::from(0x00),
        accelerator_pedal_1_channel_2: 45.5,
        accelerator_pedal_1_channel_3: 46.0,
        accelerator_pedal_2_channel_2: 44.5,
        accelerator_pedal_2_channel_3: 45.0,
        engn_exhst_gs_rstrtn_vlv_cntrl: 35.5,
    };

    let (can_id, data) = msg.encode().unwrap();

    // Process the message
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);

    // Verify state was updated
    assert!((state.engine.eec15_accelerator_pedal_1_channel_2 - 45.5).abs() < 0.5);
    assert!((state.engine.eec15_accelerator_pedal_1_channel_3 - 46.0).abs() < 0.5);
    assert!((state.engine.eec15_accelerator_pedal_2_channel_2 - 44.5).abs() < 0.5);
    assert!((state.engine.eec15_accelerator_pedal_2_channel_3 - 45.0).abs() < 0.5);
    assert!((state.engine.eec15_engn_exhst_gs_rstrtn_vlv_cntrl - 35.5).abs() < 0.1);
}
