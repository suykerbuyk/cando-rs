use serde::{Deserialize, Serialize};

/// EV Charging & HV Bus state (Batch 12)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvChargingState {
    // Charging state machine
    pub ev_charging_state: u8, // 0=Idle, 1=Communication, 2=PreCharge, 3=Charging, 4=Complete

    // EVDCS1 - EV DC Charging Status 1
    pub evdcs1_cabin_conditioning_flag: u8, // 0-3
    pub evdcs1_ress_conditioning_flag: u8,  // 0-3
    pub evdcs1_error_code: u8,              // 0-15

    // EVDCTGT - DC Charging Target
    pub evdctgt_target_voltage: f64, // 0-3212.75 V
    pub evdctgt_target_current: f64, // 0-4015.94 A
    pub evdctgt_counter: u8,         // 0-15

    // EVDCLIM1 - EV DC Charging Limits 1
    pub evdclim1_max_voltage: f64,         // 0-3212.75 V
    pub evdclim1_max_current: f64,         // 0-4015.94 A
    pub evdclim1_max_power: f64,           // 0-3212.75 kW
    pub evdclim1_energy_transfer_type: u8, // 0-15

    // EVDCLIM2 - EV DC Charging Limits 2
    pub evdclim2_bulk_soc: f64,        // 0-125 %
    pub evdclim2_full_soc: f64,        // 0-125 %
    pub evdclim2_energy_capacity: u16, // kWh
    pub evdclim2_energy_requested: u16, // kWh

    // EVDCCIP - EV DC Charging In-Progress
    pub evdccip_bulk_charging_complete: u8,      // 0-3
    pub evdccip_full_charging_complete: u8,      // 0-3
    pub evdccip_bulk_charge_time_remaining: f64, // seconds
    pub evdccip_full_charge_time_remaining: f64, // seconds
    pub evdccip_departure_time: f64,             // seconds

    // EVSE1CS1 - EV Supply Equipment 1 Contactor Status 1
    pub evse1cs1_contactor_input_voltage: f64, // 0-3212.75 V
    pub evse1cs1_charging_bus_voltage: f64,    // 0-3212.75 V
    pub evse1cs1_contactor_1_state: u8,        // 0-3

    // EVSE1CC1 - EV Supply Equipment 1 Contactor Command 1
    pub evse1cc1_contactor_1_command: u8, // 0-3

    // EVSEC1 - EVSE Control 1
    pub evsec1_connector_lock_request: u8, // 0-15
    pub evsec1_dc_stage_request: u8,       // 0-15
    pub evsec1_ev_ready: u8,               // 0-3
    pub evsec1_contactor_command: u8,      // 0-3

    // EVSEDCS1 - EVSE DC Charging Status 1
    pub evsedcs1_dc_charging_state: u8,      // 0-15
    pub evsedcs1_isolation_status: u8,       // 0-15
    pub evsedcs1_present_voltage: f64,       // 0-3212.75 V
    pub evsedcs1_present_current: f64,       // -1600 to 1612.75 A
    pub evsedcs1_voltage_limit_achieved: u8, // 0-3
    pub evsedcs1_current_limit_achieved: u8, // 0-3
    pub evsedcs1_power_limit_achieved: u8,   // 0-3
    pub evsedcs1_processing_state: u8,       // 0-3
    pub evsedcs1_status: u8,                 // 0-15
    pub evsedcs1_response_code: u8,          // 0-250

    // EVSES1 - EVSE Status 1
    pub evses1_connector_release_latch: u8,     // 0-3
    pub evses1_manual_override: u8,             // 0-3
    pub evses1_connector_lock_state: u8,        // 0-7
    pub evses1_connector_lock_permission: u8,   // 0-3
    pub evses1_inlet_contactor_state: u8,       // 0-3
    pub evses1_inlet_state: u8,                 // 0-15
    pub evses1_connection_type: u8,             // 0-15
    pub evses1_communications_physical_layer: u8, // 0-15

    // EVSES2 - EVSE Status 2
    pub evses2_temp_sensor_type: u8,             // 0-15
    pub evses2_connector_temperature_status: u8, // 0-3
    pub evses2_inlet_connector_temperature: f64, // -40 to 210 degC
    pub evses2_temp_sensor_resistance: u32,      // ohm

    // EVC - Engine Valve Control
    pub evc_valve_control_modules: [u8; 8], // 8 module preliminary FMIs (0-31)

    // EVEI - EV Energy Info
    pub evei_total_trip_energy_consumed: f64, // kWh
    pub evei_trip_drive_energy_economy: f64,  // kWh/km

    // EVOI1 - EV Operating Info 1
    pub evoi1_estimated_remaining_distance: f64, // 0-16063.75 km

    // HVBCS1 - HV Bus Contactor Status 1
    pub hvbcs1_positive_contactor_states: [u8; 8], // 8 interfaces, 0-3 each
    pub hvbcs1_negative_contactor_states: [u8; 8], // 8 interfaces, 0-3 each
    pub hvbcs1_embedded_integrity_support: u8,
    pub hvbcs1_counter: u8,

    // HVBCS2 - HV Bus Contactor Status 2
    pub hvbcs2_embedded_integrity_support: u8,
    pub hvbcs2_counter: u8,

    // HVBCS3 - HV Bus Contactor Status 3
    pub hvbcs3_embedded_integrity_support: u8,
    pub hvbcs3_counter: u8,

    // HVBCC1 - HV Bus Contactor Command 1
    pub hvbcc1_connect_commands: [u8; 24], // 24 interfaces, 0-3 each
    pub hvbcc1_embedded_integrity_support: u8,
    pub hvbcc1_counter: u8,

    // HVBCC2 - HV Bus Contactor Command 2
    pub hvbcc2_connect_commands: [u8; 8], // Interfaces 25-32, 0-3 each
    pub hvbcc2_embedded_integrity_support: u8,
    pub hvbcc2_counter: u8,

    // HVBI - HV Bus Info
    pub hvbi_dc_bus_availability: u8,          // 0-15
    pub hvbi_driveline_availability: u8,       // 0-15
    pub hvbi_auxiliaries_availability: u8,     // 0-15
    pub hvbi_epto_availability: u8,            // 0-15
    pub hvbi_on_board_charger_availability: u8, // 0-15
    pub hvbi_off_board_charger_availability: u8, // 0-15

    // EVVT - Engine VVT
    pub evvt_intake_commanded_offset: f64,  // -125 to 125 deg
    pub evvt_intake_offset_position: f64,   // -125 to 125 deg
    pub evvt_exhaust_commanded_offset: f64, // -125 to 125 deg
    pub evvt_exhaust_offset_position: f64,  // -125 to 125 deg
}

impl Default for EvChargingState {
    fn default() -> Self {
        Self {
            ev_charging_state: 0, // Idle

            // EVDCS1 - EV DC Charging Status 1
            evdcs1_cabin_conditioning_flag: 0, // Not active
            evdcs1_ress_conditioning_flag: 0,  // Not active
            evdcs1_error_code: 0,              // No error

            // EVDCTGT - DC Charging Target
            evdctgt_target_voltage: 400.0, // 400V typical target
            evdctgt_target_current: 0.0,   // No current initially
            evdctgt_counter: 0,

            // EVDCLIM1 - EV DC Charging Limits 1
            evdclim1_max_voltage: 920.0,      // 920V max (typical for CCS2)
            evdclim1_max_current: 500.0,      // 500A max
            evdclim1_max_power: 350.0,        // 350kW max
            evdclim1_energy_transfer_type: 0, // DC-Extended

            // EVDCLIM2 - EV DC Charging Limits 2
            evdclim2_bulk_soc: 80.0,       // 80% bulk SOC target
            evdclim2_full_soc: 100.0,      // 100% full SOC
            evdclim2_energy_capacity: 100, // 100 kWh
            evdclim2_energy_requested: 50, // 50 kWh requested

            // EVDCCIP - EV DC Charging In-Progress
            evdccip_bulk_charging_complete: 0,       // Not complete
            evdccip_full_charging_complete: 0,       // Not complete
            evdccip_bulk_charge_time_remaining: 0.0, // No estimate
            evdccip_full_charge_time_remaining: 0.0, // No estimate
            evdccip_departure_time: 0.0,

            // EVSE1CS1 - EV Supply Equipment 1 Contactor Status 1
            evse1cs1_contactor_input_voltage: 0.0,
            evse1cs1_charging_bus_voltage: 0.0,
            evse1cs1_contactor_1_state: 0, // Open

            // EVSE1CC1 - EV Supply Equipment 1 Contactor Command 1
            evse1cc1_contactor_1_command: 0, // Open

            // EVSEC1 - EVSE Control 1
            evsec1_connector_lock_request: 0,
            evsec1_dc_stage_request: 0,
            evsec1_ev_ready: 0,
            evsec1_contactor_command: 0,

            // EVSEDCS1 - EVSE DC Charging Status 1
            evsedcs1_dc_charging_state: 0, // Idle
            evsedcs1_isolation_status: 0,
            evsedcs1_present_voltage: 0.0,
            evsedcs1_present_current: 0.0,
            evsedcs1_voltage_limit_achieved: 0,
            evsedcs1_current_limit_achieved: 0,
            evsedcs1_power_limit_achieved: 0,
            evsedcs1_processing_state: 0,
            evsedcs1_status: 0, // Not ready
            evsedcs1_response_code: 0,

            // EVSES1 - EVSE Status 1
            evses1_connector_release_latch: 0,
            evses1_manual_override: 0,
            evses1_connector_lock_state: 0,    // Unlocked
            evses1_connector_lock_permission: 0,
            evses1_inlet_contactor_state: 0,   // Open
            evses1_inlet_state: 0,
            evses1_connection_type: 0,
            evses1_communications_physical_layer: 0,

            // EVSES2 - EVSE Status 2
            evses2_temp_sensor_type: 1,              // RTD PT1000
            evses2_connector_temperature_status: 0,   // Normal
            evses2_inlet_connector_temperature: 25.0, // Room temp
            evses2_temp_sensor_resistance: 1000,      // 1000 ohm at 25C

            // EVC - Engine Valve Control
            evc_valve_control_modules: [31; 8], // 31 = not available

            // EVEI - EV Energy Info
            evei_total_trip_energy_consumed: 0.0,
            evei_trip_drive_energy_economy: 0.0,

            // EVOI1 - EV Operating Info 1
            evoi1_estimated_remaining_distance: 300.0, // 300 km range

            // HVBCS1 - HV Bus Contactor Status 1
            hvbcs1_positive_contactor_states: [0; 8], // All open
            hvbcs1_negative_contactor_states: [0; 8], // All open
            hvbcs1_embedded_integrity_support: 0,
            hvbcs1_counter: 0,

            // HVBCS2 - HV Bus Contactor Status 2
            hvbcs2_embedded_integrity_support: 0,
            hvbcs2_counter: 0,

            // HVBCS3 - HV Bus Contactor Status 3
            hvbcs3_embedded_integrity_support: 0,
            hvbcs3_counter: 0,

            // HVBCC1 - HV Bus Contactor Command 1
            hvbcc1_connect_commands: [0; 24], // All open
            hvbcc1_embedded_integrity_support: 0,
            hvbcc1_counter: 0,

            // HVBCC2 - HV Bus Contactor Command 2
            hvbcc2_connect_commands: [0; 8], // All open
            hvbcc2_embedded_integrity_support: 0,
            hvbcc2_counter: 0,

            // HVBI - HV Bus Info
            hvbi_dc_bus_availability: 0,            // Not connected
            hvbi_driveline_availability: 0,         // Not connected
            hvbi_auxiliaries_availability: 0,       // Not connected
            hvbi_epto_availability: 0,              // Not connected
            hvbi_on_board_charger_availability: 0,  // Not connected
            hvbi_off_board_charger_availability: 0, // Not connected

            // EVVT - Engine VVT
            evvt_intake_commanded_offset: 0.0,
            evvt_intake_offset_position: 0.0,
            evvt_exhaust_commanded_offset: 0.0,
            evvt_exhaust_offset_position: 0.0,
        }
    }
}
