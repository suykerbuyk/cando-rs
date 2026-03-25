use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalState {
    // HVESSTS1
    pub hvessts1_system_input_power: f64,
    pub hvessts1_hv_input_power: f64,
    pub hvessts1_compressor_speed: f64,
    pub hvessts1_relative_humidity: f64,
    pub hvessts1_heater_status: u8,
    pub hvessts1_hvil_status: u8,
    pub hvessts1_system_mode: u8,
    pub hvessts1_coolant_level: u8,
    pub hvessts1_coolant_level_full: u8,

    // HVESSTC1
    pub hvesstc1_intake_coolant_temp_request: f64,
    pub hvesstc1_outlet_coolant_temp_request: f64,
    pub hvesstc1_coolant_flow_rate_request: f64,
    pub hvesstc1_heater_enable_command: u8,
    pub hvesstc1_coolant_pump_enable_code: u8,
    pub hvesstc1_compressor_enable_code: u8,

    // HVESSTC2
    pub hvesstc2_pump_speed_command: f64,
    pub hvesstc2_pump_speed_command_percent: f64,
    pub hvesstc2_compressor_speed_command: f64,
    pub hvesstc2_compressor_speed_command_percent: f64,

    // ETCC3
    pub etcc3_etc_bypass_actuator_1: u8,
    pub etcc3_turbo_wastegate_actuator_1: u8,
    pub etcc3_cylinder_head_bypass_actuator: u8,
    pub etcc3_throttle_valve_1: u8,
    pub etcc3_etc_bypass_pass_actuator_1: u8,
    pub etcc3_etc_bypass_pass_actuator_2: u8,
    pub etcc3_turbo_wastegate_actuator_2: u8,
}

impl Default for ThermalState {
    fn default() -> Self {
        Self {
            hvessts1_system_input_power: 1500.0,
            hvessts1_hv_input_power: 2000.0,
            hvessts1_compressor_speed: 2400.0,
            hvessts1_relative_humidity: 65.0,
            hvessts1_heater_status: 1,
            hvessts1_hvil_status: 0,
            hvessts1_system_mode: 3,
            hvessts1_coolant_level: 2,
            hvessts1_coolant_level_full: 0,
            hvesstc1_intake_coolant_temp_request: 20.0,
            hvesstc1_outlet_coolant_temp_request: 25.0,
            hvesstc1_coolant_flow_rate_request: 100.0,
            hvesstc1_heater_enable_command: 1,
            hvesstc1_coolant_pump_enable_code: 2,
            hvesstc1_compressor_enable_code: 1,
            hvesstc2_pump_speed_command: 2400.0,
            hvesstc2_pump_speed_command_percent: 75.0,
            hvesstc2_compressor_speed_command: 3600.0,
            hvesstc2_compressor_speed_command_percent: 90.0,
            etcc3_etc_bypass_actuator_1: 0,
            etcc3_turbo_wastegate_actuator_1: 0,
            etcc3_cylinder_head_bypass_actuator: 1,
            etcc3_throttle_valve_1: 0,
            etcc3_etc_bypass_pass_actuator_1: 0,
            etcc3_etc_bypass_pass_actuator_2: 3,
            etcc3_turbo_wastegate_actuator_2: 0,
        }
    }
}
