use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSupplyState {
    // ALTC
    pub altc_setpoint_voltage: f64,
    pub altc_excitation_current_limit: f64,
    pub altc_torque_ramp_time: f64,
    pub altc_torque_ramp_max_speed: f64,

    // GC2
    pub gc2_engine_load_setpoint: f64,
    pub gc2_derate_inhibit: u8,
    pub gc2_governing_bias: f64,

    // DCACAI1S2
    pub dcacai1s2_desired_power: f64,
    pub dcacai1s2_desired_voltage: f64,
    pub dcacai1s2_desired_current: f64,
    pub dcacai1s2_desired_ground_voltage: f64,

    // DCACAI1V
    pub dcacai1v_ignition_voltage: f64,
    pub dcacai1v_unswitched_voltage: f64,

    // GTRACE
    pub gtrace_kwh_export: u32,
    pub gtrace_kvarh_export: u32,

    // GC1 Generator Control 1
    pub gc1_requested_engine_control_mode: u8,
    pub gc1_not_in_auto_start_state: u8,
    pub gc1_not_ready_to_parallel_state: u8,
    pub gc1_alternator_efficiency: f64,
    pub gc1_governing_speed_command: u8,
    pub gc1_frequency_selection: u8,
    pub gc1_speed_governor_gain_adjust: f64,
    pub gc1_speed_governor_droop: f64,

    // GTRACE2 Generator Trip Energy 2
    pub gtrace2_kvarh_import: u32,

    // GAAC Generator Average AC
    pub gaac_avg_line_line_voltage: u16,
    pub gaac_avg_line_neutral_voltage: u16,
    pub gaac_avg_frequency: f64,
    pub gaac_avg_rms_current: u16,
}

impl Default for PowerSupplyState {
    fn default() -> Self {
        Self {
            altc_setpoint_voltage: 14.4,
            altc_excitation_current_limit: 10.0,
            altc_torque_ramp_time: 1.0,
            altc_torque_ramp_max_speed: 1500.0,
            gc2_engine_load_setpoint: 50.0,
            gc2_derate_inhibit: 0,
            gc2_governing_bias: 0.0,
            dcacai1s2_desired_power: 1.5,
            dcacai1s2_desired_voltage: 120.0,
            dcacai1s2_desired_current: 12.5,
            dcacai1s2_desired_ground_voltage: 0.0,
            dcacai1v_ignition_voltage: 12.8,
            dcacai1v_unswitched_voltage: 13.2,
            gtrace_kwh_export: 1500000,
            gtrace_kvarh_export: 750000,
            gc1_requested_engine_control_mode: 0,
            gc1_not_in_auto_start_state: 0,
            gc1_not_ready_to_parallel_state: 0,
            gc1_alternator_efficiency: 92.0,
            gc1_governing_speed_command: 0,
            gc1_frequency_selection: 0,
            gc1_speed_governor_gain_adjust: 50.0,
            gc1_speed_governor_droop: 5.0,
            gtrace2_kvarh_import: 500000,
            gaac_avg_line_line_voltage: 480,
            gaac_avg_line_neutral_voltage: 277,
            gaac_avg_frequency: 60.0,
            gaac_avg_rms_current: 125,
        }
    }
}
