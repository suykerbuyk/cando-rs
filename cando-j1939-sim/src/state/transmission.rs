use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmissionState {
    pub transmission_gear: u64,

    // ETC9
    pub etc9_current_preselection_gear: f64,
    pub etc9_input_shaft1_speed: f64,
    pub etc9_input_shaft2_speed: f64,
    pub etc9_selected_preselection_gear: f64,

    // ETC5
    pub etc5_trnsmssn_hgh_rng_sns_swth: u8,
    pub etc5_transmission_low_range_sense_switch: u8,
    pub etc5_transmission_splitter_position: u8,
    pub etc5_trnsmssn_rvrs_drtn_swth: u8,
    pub etc5_transmission_neutral_switch: u8,
    pub etc5_trnsmssn_frwrd_drtn_swth: u8,

    // ETC6
    pub etc6_recommended_gear: f64,
    pub etc6_lowest_possible_gear: f64,
    pub etc6_highest_possible_gear: f64,
    pub etc6_clutch_life_remaining: f64,

    // ETC2
    pub etc2_transmission_selected_gear: f64,
    pub etc2_transmission_actual_gear_ratio: f64,
    pub etc2_transmission_current_gear: f64,
    pub etc2_transmission_requested_range: u16,
    pub etc2_transmission_current_range: u16,

    // ETCC1
    pub etcc1_engn_trhrgr_wstgt_attr_1_cmmnd: f64,
    pub etcc1_engn_trhrgr_wstgt_attr_2_cmmnd: f64,
    pub etcc1_e_exst_b_1_pss_rt_ct_cd: f64,
    pub etcc1_et_cpss_bw_att_1_cd: f64,

    // ETCC2
    pub etcc2_engn_stgd_trhrgr_slnd_stts: f64,
    pub etcc2_nmr_of_engn_trhrgrs_cmmndd: u8,

    // ETC3 Electronic Transmission Controller 3 States (Shift Finger & Actuators)
    pub etc3_shift_finger_gear_position: f64,    // Shift finger gear position (0-100%)
    pub etc3_shift_finger_rail_position: f64,    // Shift finger rail position (0-100%)
    pub etc3_shift_finger_neutral_indicator: u8, // Shift finger neutral indicator (0-3)
    pub etc3_shift_finger_engagement_indicator: u8, // Shift finger engagement indicator (0-3)
    pub etc3_shift_finger_center_rail_indicator: u8, // Shift finger center rail indicator (0-3)
    pub etc3_shift_finger_rail_actuator_1: u8,   // Shift finger rail actuator 1 (0-3)
    pub etc3_shift_finger_gear_actuator_1: u8,   // Shift finger gear actuator 1 (0-3)
    pub etc3_shift_finger_rail_actuator_2: u8,   // Shift finger rail actuator 2 (0-3)
    pub etc3_shift_finger_gear_actuator_2: u8,   // Shift finger gear actuator 2 (0-3)
    pub etc3_range_high_actuator: u8,            // Range high actuator (0-3)
    pub etc3_range_low_actuator: u8,             // Range low actuator (0-3)
    pub etc3_splitter_direct_actuator: u8,       // Splitter direct actuator (0-3)
    pub etc3_splitter_indirect_actuator: u8,     // Splitter indirect actuator (0-3)
    pub etc3_clutch_actuator: u8,                // Clutch actuator (0-3)
    pub etc3_torque_converter_lockup_clutch_actuator: u8, // Torque converter lockup clutch actuator (0-3)
    pub etc3_defuel_actuator: u8,                // Defuel actuator (0-3)
    pub etc3_inertia_brake_actuator: u8,         // Inertia brake actuator (0-3)

    // ETC4 Electronic Transmission Controller 4 States (Synchronizer)
    pub etc4_synchronizer_clutch_value: f64,     // Synchronizer clutch value (0-100%)
    pub etc4_synchronizer_brake_value: f64,      // Synchronizer brake value (0-100%)

    // ETC7 Electronic Transmission Controller 7 States (Display & Indicators)
    pub etc7_current_range_display_blank_state: u8, // Current range display blank state (0-3)
    pub etc7_service_indicator: u8,              // Transmission service indicator (0-3)
    pub etc7_requested_range_display_blank_state: u8, // Requested range display blank state (0-3)
    pub etc7_requested_range_display_flash_state: u8, // Requested range display flash state (0-3)
    pub etc7_ready_for_brake_release: u8,        // Ready for brake release (0-3)
    pub etc7_active_shift_console_indicator: u8, // Active shift console indicator (0-3)
    pub etc7_engine_crank_enable: u8,            // Engine crank enable (0-3)
    pub etc7_shift_inhibit_indicator: u8,        // Shift inhibit indicator (0-3)
    pub etc7_mode_1_indicator: u8,               // Mode 1 indicator (0-3)
    pub etc7_mode_2_indicator: u8,               // Mode 2 indicator (0-3)
    pub etc7_mode_3_indicator: u8,               // Mode 3 indicator (0-3)
    pub etc7_mode_4_indicator: u8,               // Mode 4 indicator (0-3)
    pub etc7_mode_5_indicator: u8,               // Mode 5 indicator (0-3)
    pub etc7_mode_6_indicator: u8,               // Mode 6 indicator (0-3)
    pub etc7_mode_7_indicator: u8,               // Mode 7 indicator (0-3)
    pub etc7_mode_8_indicator: u8,               // Mode 8 indicator (0-3)
    pub etc7_requested_gear_feedback: f64,       // Requested gear feedback (-125 to 125)
    pub etc7_reverse_gear_shift_inhibit_status: u8, // Reverse gear shift inhibit status (0-3)
    pub etc7_warning_indicator: u8,              // Warning indicator (0-3)
    pub etc7_mode_9_indicator: u8,               // Mode 9 indicator (0-3)
    pub etc7_mode_10_indicator: u8,              // Mode 10 indicator (0-3)
    pub etc7_air_supply_pressure_indicator: u8,  // Air supply pressure indicator (0-7)
    pub etc7_auto_neutral_manual_return_state: u8, // Auto-neutral manual return state (0-7)
    pub etc7_manual_mode_indicator: u8,          // Manual mode indicator (0-3)
    pub etc7_load_reduction_indicator: u8,       // Load reduction indicator (0-3)
    pub etc7_pre_defined_range_limit_indicator: u8, // Pre-defined range limit indicator (0-3)
    pub etc7_coast_mode_indicator: u8,           // Coast mode indicator (0-3)
    pub etc7_output_shaft_brake_indicator: u8,   // Output shaft brake indicator (0-3)

    // ETC8 Electronic Transmission Controller 8 States (Torque Converter)
    pub etc8_torque_converter_ratio: f64,        // Torque converter ratio (0-64.25)
    pub etc8_clutch_converter_input_speed: f64,  // Clutch/converter input speed (0-8031.88 rpm)
    pub etc8_shift_inhibit_reason: u8,           // Shift inhibit reason (0-250)
    pub etc8_torque_converter_lockup_inhibit_reason: u8, // TC lockup inhibit reason (0-250)
    pub etc8_torque_converter_lockup_inhibit_indicator: u8, // TC lockup inhibit indicator (0-3)
    pub etc8_explicit_manual_mode_indicator: u8, // Explicit manual mode indicator (0-3)

    // ETC10 Electronic Transmission Controller 10 States (Clutch & Actuator Positions)
    pub etc10_clutch_1_actuator_position: f64,   // Clutch 1 actuator position (0-100%)
    pub etc10_clutch_2_actuator_position: f64,   // Clutch 2 actuator position (0-100%)
    pub etc10_hydraulic_pump_actuator_1_position: f64, // Hydraulic pump actuator 1 position (0-100%)
    pub etc10_shift_actuator_1_position: f64,    // Shift actuator 1 position (0-100%)
    pub etc10_shift_actuator_2_position: f64,    // Shift actuator 2 position (0-100%)
    pub etc10_clutch_1_cooling_actuator_status: u8, // Clutch 1 cooling actuator status (0-3)
    pub etc10_clutch_2_cooling_actuator_status: u8, // Clutch 2 cooling actuator status (0-3)
    pub etc10_shift_rail_1_actuator_status: u8,  // Shift rail 1 actuator status (0-3)
    pub etc10_shift_rail_2_actuator_status: u8,  // Shift rail 2 actuator status (0-3)
    pub etc10_shift_rail_3_actuator_status: u8,  // Shift rail 3 actuator status (0-3)
    pub etc10_shift_rail_4_actuator_status: u8,  // Shift rail 4 actuator status (0-3)
    pub etc10_shift_rail_5_actuator_status: u8,  // Shift rail 5 actuator status (0-3)
    pub etc10_shift_rail_6_actuator_status: u8,  // Shift rail 6 actuator status (0-3)
    pub etc10_hydraulic_pump_actuator_2_percent: f64, // Hydraulic pump actuator 2 percent (0-100%)

    // ETC11 Electronic Transmission Controller 11 States (Shift Rail Positions)
    pub etc11_shift_rail_1_position: f64,        // Shift rail 1 position (0-100%)
    pub etc11_shift_rail_2_position: f64,        // Shift rail 2 position (0-100%)
    pub etc11_shift_rail_3_position: f64,        // Shift rail 3 position (0-100%)
    pub etc11_shift_rail_4_position: f64,        // Shift rail 4 position (0-100%)
    pub etc11_shift_rail_5_position: f64,        // Shift rail 5 position (0-100%)
    pub etc11_shift_rail_6_position: f64,        // Shift rail 6 position (0-100%)

    // ETC12 Electronic Transmission Controller 12 States (Hydrostatic Loop)
    pub etc12_hydrostatic_loop_1_pressure: u16,  // Hydrostatic loop 1 pressure (0-64255 kPa)
    pub etc12_hydrostatic_loop_2_pressure: u16,  // Hydrostatic loop 2 pressure (0-64255 kPa)
    pub etc12_directional_output_shaft_speed: f64, // Directional output shaft speed (-32127 to 32128 rpm)
    pub etc12_intermediate_shaft_speed: f64,     // Intermediate shaft speed (0-8031.88 rpm)

    // ETC13 Electronic Transmission Controller 13 States (Max Speeds & Mode Indicators)
    pub etc13_max_forward_output_shaft_speed: f64, // Max forward output shaft speed (0-8031.88 rpm)
    pub etc13_max_reverse_output_shaft_speed: f64, // Max reverse output shaft speed (0-8031.88 rpm)
    pub etc13_source_address_requested_gear: u8, // Source address of active/pending requested gear (0-253)
    pub etc13_mode_11_indicator: u8,             // Mode 11 indicator (0-3)
    pub etc13_mode_12_indicator: u8,             // Mode 12 indicator (0-3)
    pub etc13_mode_13_indicator: u8,             // Mode 13 indicator (0-3)
    pub etc13_mode_14_indicator: u8,             // Mode 14 indicator (0-3)
    pub etc13_mode_15_indicator: u8,             // Mode 15 indicator (0-3)
    pub etc13_mode_16_indicator: u8,             // Mode 16 indicator (0-3)
    pub etc13_mode_17_indicator: u8,             // Mode 17 indicator (0-3)
    pub etc13_mode_18_indicator: u8,             // Mode 18 indicator (0-3)
    pub etc13_mode_19_indicator: u8,             // Mode 19 indicator (0-3)
    pub etc13_mode_20_indicator: u8,             // Mode 20 indicator (0-3)

    // ETC14 Electronic Transmission Controller 14 States (Clutch Temp & Capability)
    pub etc14_clutch_1_temperature: f64,         // Clutch 1 temperature (-273 to 1734.97 degC)
    pub etc14_clutch_1_overheat_indicator: u8,   // Clutch 1 overheat indicator (0-3)
    pub etc14_launch_capability: u8,             // Launch capability (0-7)
    pub etc14_gear_shift_capability: u8,         // Gear shift capability (0-7)
    pub etc14_damage_threshold_status: u8,       // Damage threshold status (0-7)

    // ETC15 Electronic Transmission Controller 15 States (CRC, Counter, Auto-Neutral)
    pub etc15_crc: u8,                           // ETC15 CRC (0-250)
    pub etc15_counter: u8,                       // ETC15 counter (0-15)
    pub etc15_auto_neutral_auto_return_request_feedback: u8, // Auto-neutral auto-return request feedback (0-7)
    pub etc15_launch_process_status: u8,         // Launch process status (0-3)
    pub etc15_auto_neutral_auto_return_function_state: u8, // Auto-neutral auto-return function state (0-15)

    // ETCC4 Engine Turbocharger Control 4 States
    pub etcc4_wastegate_actuator_3_command: f64, // Wastegate actuator 3 command (0-160.64%)
    pub etcc4_wastegate_actuator_4_command: f64, // Wastegate actuator 4 command (0-160.64%)

    // ETCBI Engine Turbocharger Compressor Bypass Information States
    pub etcbi_compressor_bypass_actuator_2_position: f64, // Bypass actuator 2 position (0-100%)
    pub etcbi_compressor_bypass_actuator_2_desired_position: f64, // Bypass actuator 2 desired position (0-100%)
    pub etcbi_compressor_bypass_actuator_2_preliminary_fmi: u8, // Bypass actuator 2 preliminary FMI (0-31)
    pub etcbi_compressor_bypass_actuator_2_temp_status: u8, // Bypass actuator 2 temp status (0-7)
    pub etcbi_compressor_bypass_actuator_1_operation_status: u8, // Bypass actuator 1 operation status (0-15)
    pub etcbi_compressor_bypass_actuator_2_operation_status: u8, // Bypass actuator 2 operation status (0-15)
    pub etcbi_compressor_bypass_actuator_1_temperature: f64, // Bypass actuator 1 temperature (-40 to 210 degC)
    pub etcbi_compressor_bypass_actuator_2_temperature: f64, // Bypass actuator 2 temperature (-40 to 210 degC)

    // TC1 Transmission Control 1 States (Gear Shift Commands)
    pub tc1_gear_shift_inhibit_request: u8,      // Gear shift inhibit request (0-3)
    pub tc1_torque_converter_lockup_request: u8, // Torque converter lockup request (0-3)
    pub tc1_disengage_driveline_request: u8,     // Disengage driveline request (0-3)
    pub tc1_reverse_gear_shift_inhibit_request: u8, // Reverse gear shift inhibit request (0-3)
    pub tc1_requested_percent_clutch_slip: f64,  // Requested percent clutch slip (0-100%)
    pub tc1_transmission_requested_gear: f64,    // Transmission requested gear (-64 to 64)
    pub tc1_disengage_differential_lock_front_axle_1: u8, // Disengage diff lock front axle 1 (0-3)
    pub tc1_disengage_differential_lock_front_axle_2: u8, // Disengage diff lock front axle 2 (0-3)
    pub tc1_disengage_differential_lock_rear_axle_1: u8, // Disengage diff lock rear axle 1 (0-3)
    pub tc1_disengage_differential_lock_rear_axle_2: u8, // Disengage diff lock rear axle 2 (0-3)
    pub tc1_disengage_differential_lock_central: u8, // Disengage diff lock central (0-3)
    pub tc1_disengage_differential_lock_central_front: u8, // Disengage diff lock central front (0-3)
    pub tc1_disengage_differential_lock_central_rear: u8, // Disengage diff lock central rear (0-3)
    pub tc1_load_reduction_inhibit_request: u8,  // Load reduction inhibit request (0-3)
    pub tc1_mode_1: u8,                          // Transmission mode 1 (0-3)
    pub tc1_mode_2: u8,                          // Transmission mode 2 (0-3)
    pub tc1_mode_3: u8,                          // Transmission mode 3 (0-3)
    pub tc1_mode_4: u8,                          // Transmission mode 4 (0-3)
    pub tc1_auto_neutral_manual_return_request: u8, // Auto-neutral manual return request (0-3)
    pub tc1_requested_launch_gear: u8,           // Requested launch gear (0-15)
    pub tc1_shift_selector_display_mode_switch: u8, // Shift selector display mode switch (0-3)
    pub tc1_mode_5: u8,                          // Transmission mode 5 (0-3)
    pub tc1_mode_6: u8,                          // Transmission mode 6 (0-3)
    pub tc1_mode_7: u8,                          // Transmission mode 7 (0-3)
    pub tc1_mode_8: u8,                          // Transmission mode 8 (0-3)

    // TC2 Transmission Control 2 States (Extended Mode Commands)
    pub tc2_mode_9: u8,                          // Transmission mode 9 (0-3)
    pub tc2_mode_10: u8,                         // Transmission mode 10 (0-3)
    pub tc2_pre_defined_max_gear_activation_request: u8, // Pre-defined max gear activation request (0-3)
    pub tc2_output_shaft_brake_request: u8,      // Output shaft brake request (0-3)
    pub tc2_requested_reverse_launch_gear: u8,   // Requested reverse launch gear (0-15)
    pub tc2_selected_max_gear_limit_activation_request: u8, // Selected max gear limit activation request (0-3)
    pub tc2_mode_11: u8,                         // Transmission mode 11 (0-3)
    pub tc2_mode_12: u8,                         // Transmission mode 12 (0-3)
    pub tc2_mode_13: u8,                         // Transmission mode 13 (0-3)
    pub tc2_mode_14: u8,                         // Transmission mode 14 (0-3)
    pub tc2_mode_15: u8,                         // Transmission mode 15 (0-3)
    pub tc2_mode_16: u8,                         // Transmission mode 16 (0-3)
    pub tc2_mode_17: u8,                         // Transmission mode 17 (0-3)
    pub tc2_mode_18: u8,                         // Transmission mode 18 (0-3)
    pub tc2_mode_19: u8,                         // Transmission mode 19 (0-3)
    pub tc2_mode_20: u8,                         // Transmission mode 20 (0-3)
    pub tc2_disengage_differential_lock_rear_axle_3: u8, // Disengage diff lock rear axle 3 (0-3)
    pub tc2_coast_mode_disable_request: u8,      // Coast mode disable request (0-3)
    pub tc2_explicit_manual_mode_request: u8,    // Explicit manual mode request (0-3)
}

impl Default for TransmissionState {
    fn default() -> Self {
        Self {
            transmission_gear: 0,
            etc9_current_preselection_gear: 0.0,
            etc9_input_shaft1_speed: 0.0,
            etc9_input_shaft2_speed: 0.0,
            etc9_selected_preselection_gear: 0.0,
            etc5_trnsmssn_hgh_rng_sns_swth: 0,
            etc5_transmission_low_range_sense_switch: 0,
            etc5_transmission_splitter_position: 0,
            etc5_trnsmssn_rvrs_drtn_swth: 0,
            etc5_transmission_neutral_switch: 1,
            etc5_trnsmssn_frwrd_drtn_swth: 0,
            etc6_recommended_gear: 3.0,
            etc6_lowest_possible_gear: 1.0,
            etc6_highest_possible_gear: 6.0,
            etc6_clutch_life_remaining: 85.0,
            etc2_transmission_selected_gear: 4.0,
            etc2_transmission_actual_gear_ratio: 3.2,
            etc2_transmission_current_gear: 3.0,
            etc2_transmission_requested_range: 5,
            etc2_transmission_current_range: 5,
            etcc1_engn_trhrgr_wstgt_attr_1_cmmnd: 50.0,
            etcc1_engn_trhrgr_wstgt_attr_2_cmmnd: 55.0,
            etcc1_e_exst_b_1_pss_rt_ct_cd: 30.0,
            etcc1_et_cpss_bw_att_1_cd: 0.0,
            etcc2_engn_stgd_trhrgr_slnd_stts: 0.0,
            etcc2_nmr_of_engn_trhrgrs_cmmndd: 1,

            // ETC3 Electronic Transmission Controller 3 defaults (Shift Finger & Actuators)
            etc3_shift_finger_gear_position: 50.0, // 50% - centered position
            etc3_shift_finger_rail_position: 50.0, // 50% - centered position
            etc3_shift_finger_neutral_indicator: 1, // On - in neutral
            etc3_shift_finger_engagement_indicator: 0, // Off - not engaged
            etc3_shift_finger_center_rail_indicator: 1, // On - centered
            etc3_shift_finger_rail_actuator_1: 0, // Off
            etc3_shift_finger_gear_actuator_1: 0, // Off
            etc3_shift_finger_rail_actuator_2: 0, // Off
            etc3_shift_finger_gear_actuator_2: 0, // Off
            etc3_range_high_actuator: 0,          // Off
            etc3_range_low_actuator: 0,           // Off
            etc3_splitter_direct_actuator: 0,     // Off
            etc3_splitter_indirect_actuator: 0,   // Off
            etc3_clutch_actuator: 0,              // Off
            etc3_torque_converter_lockup_clutch_actuator: 0, // Off
            etc3_defuel_actuator: 0,              // Off
            etc3_inertia_brake_actuator: 0,       // Off

            // ETC4 Electronic Transmission Controller 4 defaults (Synchronizer)
            etc4_synchronizer_clutch_value: 0.0,  // 0% - not engaged
            etc4_synchronizer_brake_value: 0.0,   // 0% - not braking

            // ETC7 Electronic Transmission Controller 7 defaults (Display & Indicators)
            etc7_current_range_display_blank_state: 0, // Not blanked
            etc7_service_indicator: 0,             // Off - no service needed
            etc7_requested_range_display_blank_state: 0, // Not blanked
            etc7_requested_range_display_flash_state: 0, // Not flashing
            etc7_ready_for_brake_release: 1,       // Ready
            etc7_active_shift_console_indicator: 0, // Primary console active
            etc7_engine_crank_enable: 1,           // Crank enabled
            etc7_shift_inhibit_indicator: 0,       // Not inhibited
            etc7_mode_1_indicator: 0,              // Off
            etc7_mode_2_indicator: 0,              // Off
            etc7_mode_3_indicator: 0,              // Off
            etc7_mode_4_indicator: 0,              // Off
            etc7_mode_5_indicator: 0,              // Off
            etc7_mode_6_indicator: 0,              // Off
            etc7_mode_7_indicator: 0,              // Off
            etc7_mode_8_indicator: 0,              // Off
            etc7_requested_gear_feedback: 0.0,     // Neutral
            etc7_reverse_gear_shift_inhibit_status: 0, // Not inhibited
            etc7_warning_indicator: 0,             // Off
            etc7_mode_9_indicator: 0,              // Off
            etc7_mode_10_indicator: 0,             // Off
            etc7_air_supply_pressure_indicator: 0, // Normal pressure
            etc7_auto_neutral_manual_return_state: 0, // No request
            etc7_manual_mode_indicator: 0,         // Off
            etc7_load_reduction_indicator: 0,      // Off
            etc7_pre_defined_range_limit_indicator: 0, // Off
            etc7_coast_mode_indicator: 0,          // Off
            etc7_output_shaft_brake_indicator: 0,  // Off

            // ETC8 Electronic Transmission Controller 8 defaults (Torque Converter)
            etc8_torque_converter_ratio: 1.0,      // 1.0 = lockup (direct drive)
            etc8_clutch_converter_input_speed: 800.0, // Idle RPM
            etc8_shift_inhibit_reason: 0,          // No inhibit
            etc8_torque_converter_lockup_inhibit_reason: 0, // No inhibit
            etc8_torque_converter_lockup_inhibit_indicator: 0, // Off
            etc8_explicit_manual_mode_indicator: 0, // Off

            // ETC10 Electronic Transmission Controller 10 defaults (Clutch & Actuator Positions)
            etc10_clutch_1_actuator_position: 0.0, // 0% - fully released
            etc10_clutch_2_actuator_position: 0.0, // 0% - fully released
            etc10_hydraulic_pump_actuator_1_position: 50.0, // 50% - normal operating pressure
            etc10_shift_actuator_1_position: 0.0,  // 0% - neutral
            etc10_shift_actuator_2_position: 0.0,  // 0% - neutral
            etc10_clutch_1_cooling_actuator_status: 1, // On - cooling active
            etc10_clutch_2_cooling_actuator_status: 0, // Off
            etc10_shift_rail_1_actuator_status: 0, // Off
            etc10_shift_rail_2_actuator_status: 0, // Off
            etc10_shift_rail_3_actuator_status: 0, // Off
            etc10_shift_rail_4_actuator_status: 0, // Off
            etc10_shift_rail_5_actuator_status: 0, // Off
            etc10_shift_rail_6_actuator_status: 0, // Off
            etc10_hydraulic_pump_actuator_2_percent: 0.0, // 0%

            // ETC11 Electronic Transmission Controller 11 defaults (Shift Rail Positions)
            etc11_shift_rail_1_position: 50.0,     // 50% - centered
            etc11_shift_rail_2_position: 50.0,     // 50% - centered
            etc11_shift_rail_3_position: 50.0,     // 50% - centered
            etc11_shift_rail_4_position: 50.0,     // 50% - centered
            etc11_shift_rail_5_position: 50.0,     // 50% - centered
            etc11_shift_rail_6_position: 50.0,     // 50% - centered

            // ETC12 Electronic Transmission Controller 12 defaults (Hydrostatic Loop)
            etc12_hydrostatic_loop_1_pressure: 5000, // 5000 kPa normal operating pressure
            etc12_hydrostatic_loop_2_pressure: 5000, // 5000 kPa normal operating pressure
            etc12_directional_output_shaft_speed: 0.0, // 0 rpm - stationary
            etc12_intermediate_shaft_speed: 0.0,   // 0 rpm - stationary

            // ETC13 Electronic Transmission Controller 13 defaults (Max Speeds & Mode Indicators)
            etc13_max_forward_output_shaft_speed: 4000.0, // 4000 rpm max forward
            etc13_max_reverse_output_shaft_speed: 2000.0, // 2000 rpm max reverse
            etc13_source_address_requested_gear: 0xFF, // Not available
            etc13_mode_11_indicator: 0,            // Off
            etc13_mode_12_indicator: 0,            // Off
            etc13_mode_13_indicator: 0,            // Off
            etc13_mode_14_indicator: 0,            // Off
            etc13_mode_15_indicator: 0,            // Off
            etc13_mode_16_indicator: 0,            // Off
            etc13_mode_17_indicator: 0,            // Off
            etc13_mode_18_indicator: 0,            // Off
            etc13_mode_19_indicator: 0,            // Off
            etc13_mode_20_indicator: 0,            // Off

            // ETC14 Electronic Transmission Controller 14 defaults (Clutch Temp & Capability)
            etc14_clutch_1_temperature: 85.0,      // 85°C - normal operating temperature
            etc14_clutch_1_overheat_indicator: 0,  // Off - not overheating
            etc14_launch_capability: 0,            // Normal launch capability
            etc14_gear_shift_capability: 0,        // Normal gear shift capability
            etc14_damage_threshold_status: 0,      // No damage

            // ETC15 Electronic Transmission Controller 15 defaults (CRC, Counter, Auto-Neutral)
            etc15_crc: 0,                          // Initial CRC
            etc15_counter: 0,                      // Initial counter
            etc15_auto_neutral_auto_return_request_feedback: 0, // No request
            etc15_launch_process_status: 0,        // Not launching
            etc15_auto_neutral_auto_return_function_state: 0, // Inactive

            // ETCC4 Engine Turbocharger Control 4 defaults
            etcc4_wastegate_actuator_3_command: 50.0, // 50% - moderate position
            etcc4_wastegate_actuator_4_command: 50.0, // 50% - moderate position

            // ETCBI Engine Turbocharger Compressor Bypass Information defaults
            etcbi_compressor_bypass_actuator_2_position: 0.0, // 0% - fully closed
            etcbi_compressor_bypass_actuator_2_desired_position: 0.0, // 0% - fully closed
            etcbi_compressor_bypass_actuator_2_preliminary_fmi: 31, // Not available
            etcbi_compressor_bypass_actuator_2_temp_status: 0, // Normal
            etcbi_compressor_bypass_actuator_1_operation_status: 0, // Normal
            etcbi_compressor_bypass_actuator_2_operation_status: 0, // Normal
            etcbi_compressor_bypass_actuator_1_temperature: 65.0, // 65°C - normal operating temp
            etcbi_compressor_bypass_actuator_2_temperature: 65.0, // 65°C - normal operating temp

            // TC1 Transmission Control 1 defaults (Gear Shift Commands)
            tc1_gear_shift_inhibit_request: 0,     // Gear shifts allowed
            tc1_torque_converter_lockup_request: 3, // Take no action
            tc1_disengage_driveline_request: 0,    // Allow driveline engagement
            tc1_reverse_gear_shift_inhibit_request: 0, // Not inhibited
            tc1_requested_percent_clutch_slip: 0.0, // No slip requested
            tc1_transmission_requested_gear: 0.0,  // Neutral
            tc1_disengage_differential_lock_front_axle_1: 3, // Take no action
            tc1_disengage_differential_lock_front_axle_2: 3, // Take no action
            tc1_disengage_differential_lock_rear_axle_1: 3, // Take no action
            tc1_disengage_differential_lock_rear_axle_2: 3, // Take no action
            tc1_disengage_differential_lock_central: 3, // Take no action
            tc1_disengage_differential_lock_central_front: 3, // Take no action
            tc1_disengage_differential_lock_central_rear: 3, // Take no action
            tc1_load_reduction_inhibit_request: 0, // Not inhibited
            tc1_mode_1: 0,                         // Disabled
            tc1_mode_2: 0,                         // Disabled
            tc1_mode_3: 0,                         // Disabled
            tc1_mode_4: 0,                         // Disabled
            tc1_auto_neutral_manual_return_request: 3, // Take no action
            tc1_requested_launch_gear: 1,          // 1st gear launch
            tc1_shift_selector_display_mode_switch: 0, // Default display
            tc1_mode_5: 0,                         // Disabled
            tc1_mode_6: 0,                         // Disabled
            tc1_mode_7: 0,                         // Disabled
            tc1_mode_8: 0,                         // Disabled

            // TC2 Transmission Control 2 defaults (Extended Mode Commands)
            tc2_mode_9: 0,                         // Disabled
            tc2_mode_10: 0,                        // Disabled
            tc2_pre_defined_max_gear_activation_request: 0, // Disabled
            tc2_output_shaft_brake_request: 0,     // No brake
            tc2_requested_reverse_launch_gear: 1,  // 1st reverse gear
            tc2_selected_max_gear_limit_activation_request: 0, // Disabled
            tc2_mode_11: 0,                        // Disabled
            tc2_mode_12: 0,                        // Disabled
            tc2_mode_13: 0,                        // Disabled
            tc2_mode_14: 0,                        // Disabled
            tc2_mode_15: 0,                        // Disabled
            tc2_mode_16: 0,                        // Disabled
            tc2_mode_17: 0,                        // Disabled
            tc2_mode_18: 0,                        // Disabled
            tc2_mode_19: 0,                        // Disabled
            tc2_mode_20: 0,                        // Disabled
            tc2_disengage_differential_lock_rear_axle_3: 3, // Take no action
            tc2_coast_mode_disable_request: 0,     // Not disabled
            tc2_explicit_manual_mode_request: 0,   // Not requested
        }
    }
}
