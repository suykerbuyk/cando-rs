use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrakingState {
    pub aebs_enabled: bool,
    pub aebs_brake_demand: f64,
    pub aebs_target_deceleration: f64,
    pub aebs_status: u64,

    // EBC1 - Electronic Brake Controller 1
    pub ebc1_brake_pedal_position: f64,        // 0-100%
    pub ebc1_ebs_brake_switch: u8,             // 0-3
    pub ebc1_abs_active: u8,                   // 0-3
    pub ebc1_asr_engine_control_active: u8,    // 0-3
    pub ebc1_asr_brake_control_active: u8,     // 0-3
    pub ebc1_abs_off_road_switch: u8,          // 0-3
    pub ebc1_asr_off_road_switch: u8,          // 0-3
    pub ebc1_asr_hill_holder_switch: u8,       // 0-3
    pub ebc1_traction_control_override: u8,    // 0-3
    pub ebc1_accelerator_interlock: u8,        // 0-3
    pub ebc1_engine_derate_switch: u8,         // 0-3
    pub ebc1_aux_engine_shutdown_switch: u8,   // 0-3
    pub ebc1_remote_accel_enable_switch: u8,   // 0-3
    pub ebc1_engine_retarder_selection: f64,   // 0-100%
    pub ebc1_abs_fully_operational: u8,        // 0-3
    pub ebc1_ebs_red_warning: u8,              // 0-3
    pub ebc1_abs_ebs_amber_warning: u8,        // 0-3
    pub ebc1_atc_asr_information_signal: u8,   // 0-3
    pub ebc1_source_address_brake_control: u8, // 0-253
    pub ebc1_halt_brake_switch: u8,            // 0-3
    pub ebc1_trailer_abs_status: u8,           // 0-3
    pub ebc1_tractor_trailer_abs_warning: u8,  // 0-3

    // EBC2 - Electronic Brake Controller 2 (Wheel Speeds)
    pub ebc2_front_axle_speed: f64,            // 0-251 km/h
    pub ebc2_rel_speed_front_left: f64,        // -7.81 to 7.81 km/h
    pub ebc2_rel_speed_front_right: f64,       // -7.81 to 7.81 km/h
    pub ebc2_rel_speed_rear1_left: f64,        // -7.81 to 7.81 km/h
    pub ebc2_rel_speed_rear1_right: f64,       // -7.81 to 7.81 km/h
    pub ebc2_rel_speed_rear2_left: f64,        // -7.81 to 7.81 km/h
    pub ebc2_rel_speed_rear2_right: f64,       // -7.81 to 7.81 km/h

    // EBC3 - Electronic Brake Controller 3 (Brake Pressures)
    pub ebc3_pressure_front_left: f64,   // 0-1250 kPa
    pub ebc3_pressure_front_right: f64,  // 0-1250 kPa
    pub ebc3_pressure_rear1_left: f64,   // 0-1250 kPa
    pub ebc3_pressure_rear1_right: f64,  // 0-1250 kPa
    pub ebc3_pressure_rear2_left: f64,   // 0-1250 kPa
    pub ebc3_pressure_rear2_right: f64,  // 0-1250 kPa
    pub ebc3_pressure_rear3_left: f64,   // 0-1250 kPa
    pub ebc3_pressure_rear3_right: f64,  // 0-1250 kPa

    // EBC4 - Electronic Brake Controller 4 (Brake Lining - Axles 1-3)
    pub ebc4_lining_front_left: f64,  // 0-100%
    pub ebc4_lining_front_right: f64, // 0-100%
    pub ebc4_lining_rear1_left: f64,  // 0-100%
    pub ebc4_lining_rear1_right: f64, // 0-100%
    pub ebc4_lining_rear2_left: f64,  // 0-100%
    pub ebc4_lining_rear2_right: f64, // 0-100%
    pub ebc4_lining_rear3_left: f64,  // 0-100%
    pub ebc4_lining_rear3_right: f64, // 0-100%

    // EBC5 - Electronic Brake Controller 5 (Brake Status)
    pub ebc5_brake_temp_warning: u8,       // 0-3
    pub ebc5_halt_brake_mode: u8,          // 0-7
    pub ebc5_hill_holder_mode: u8,         // 0-7
    pub ebc5_foundation_brake_use: u8,     // 0-3
    pub ebc5_xbr_system_state: u8,         // 0-3
    pub ebc5_xbr_active_control_mode: u8,  // 0-15
    pub ebc5_xbr_acceleration_limit: f64,  // -10 to 10 m/s^2
    pub ebc5_parking_brake_actuator: u8,   // 0-3
    pub ebc5_emergency_braking_active: u8, // 0-3
    pub ebc5_railroad_mode: u8,            // 0-3
    pub ebc5_xbr_brake_hold_mode: u8,      // 0-15
    pub ebc5_driver_brake_demand: f64,     // -12.5 to 12.5 m/s^2
    pub ebc5_overall_brake_demand: f64,    // -12.5 to 12.5 m/s^2

    // EBC6 - Electronic Brake Controller 6 (Brake Lining - Axles 4-7)
    pub ebc6_lining_rear4_left: f64,  // 0-100%
    pub ebc6_lining_rear4_right: f64, // 0-100%
    pub ebc6_lining_rear5_left: f64,  // 0-100%
    pub ebc6_lining_rear5_right: f64, // 0-100%
    pub ebc6_lining_rear6_left: f64,  // 0-100%
    pub ebc6_lining_rear6_right: f64, // 0-100%
    pub ebc6_lining_rear7_left: f64,  // 0-100%
    pub ebc6_lining_rear7_right: f64, // 0-100%

    // EBC7 - Electronic Brake Controller 7 (Brake Lining - Axles 8-10)
    pub ebc7_lining_rear8_left: f64,   // 0-100%
    pub ebc7_lining_rear8_right: f64,  // 0-100%
    pub ebc7_lining_rear9_left: f64,   // 0-100%
    pub ebc7_lining_rear9_right: f64,  // 0-100%
    pub ebc7_lining_rear10_left: f64,  // 0-100%
    pub ebc7_lining_rear10_right: f64, // 0-100%

    // EBCC - Engine Brake Continuous Control
    pub ebcc_turbo1_outlet_pressure: f64,         // 0-500 kPa
    pub ebcc_turbo1_desired_outlet_pressure: f64, // 0-500 kPa
    pub ebcc_exhaust_brake_command: f64,          // 0-100%
    pub ebcc_turbo2_outlet_pressure: f64,         // 0-500 kPa
    pub ebcc_turbo2_desired_outlet_pressure: f64, // 0-500 kPa

    // XBR - External Brake Request
    pub xbr_acceleration_demand: f64,      // -10 to 10 m/s^2
    pub xbr_ebi_mode: u8,                  // 0-3
    pub xbr_priority: u8,                  // 0-3
    pub xbr_control_mode: u8,              // 0-3
    pub xbr_compensation_mode: u8,         // 0-3
    pub xbr_urgency: f64,                  // 0-100%
    pub xbr_brake_hold_request: u8,        // 0-3
    pub xbr_reason: u8,                    // 0-15
    pub xbr_message_counter: u8,           // 0-15
    pub xbr_message_checksum: u8,          // 0-255

    // AEBS1 - Advanced Emergency Braking System 1
    pub aebs1_forward_collision_status: u8,  // 0-7
    pub aebs1_collision_warning_level: u8,   // 0-3
    pub aebs1_relevant_object_detected: u8,  // 0-3
    pub aebs1_bound_offset: u8,              // 0-3
    pub aebs1_time_to_collision: f64,        // 0-25 seconds
    pub aebs1_road_departure_status: u8,     // 0-7

    // ACC1 - Adaptive Cruise Control 1
    pub acc1_speed_of_forward_vehicle: u8,    // 0-250 km/h
    pub acc1_distance_to_forward_vehicle: u8, // 0-250 m
    pub acc1_set_speed: u8,                   // 0-250 km/h
    pub acc1_mode: u8,                        // 0-7
    pub acc1_set_distance_mode: u8,           // 0-7
    pub acc1_road_curvature: f64,             // -250 to 250 1/km
    pub acc1_target_detected: u8,             // 0-3
    pub acc1_system_shutoff_warning: u8,      // 0-3
    pub acc1_distance_alert: u8,              // 0-3
    pub acc1_forward_collision_warning: u8,   // 0-3

    // ACC2 - Adaptive Cruise Control 2
    pub acc2_usage_demand: u8,         // 0-3
    pub acc2_distance_mode: u8,        // 0-7

    // ACCS - Acceleration Sensor
    pub accs_lateral_acceleration: f64,      // -320 to 320 m/s^2
    pub accs_longitudinal_acceleration: f64, // -320 to 320 m/s^2
    pub accs_vertical_acceleration: f64,     // -320 to 320 m/s^2
    pub accs_lateral_fom: u8,                // 0-3
    pub accs_longitudinal_fom: u8,           // 0-3
    pub accs_vertical_fom: u8,               // 0-3
    pub accs_support_report_rate: u8,        // 0-7

    // ACCVC - Aftercooler Coolant Valve Control
    pub accvc_aftercooler_thermostat_mode: u8,         // 0-3
    pub accvc_desired_aftercooler_temp: f64,           // -40 to 210 C
    pub accvc_desired_thermostat_opening: f64,         // 0-100%
    pub accvc_charge_air_bypass_valve1_cmd: f64,       // 0-100%
    pub accvc_charge_air_bypass_valve2_cmd: f64,       // 0-100%
    pub accvc_aftercooler_diverter_valve_cmd: f64,     // 0-100%

    // ERC1 - Electronic Retarder Controller 1
    pub erc1_retarder_torque_mode: u8,           // 0-15
    pub erc1_enable_brake_assist: u8,            // 0-3
    pub erc1_enable_shift_assist: u8,            // 0-3
    pub erc1_actual_retarder_torque: f64,        // -125 to 125%
    pub erc1_intended_retarder_torque: f64,      // -125 to 125%
    pub erc1_coolant_load_increase: u8,          // 0-3
    pub erc1_requesting_brake_light: u8,         // 0-3
    pub erc1_road_speed_limit_switch: u8,        // 0-3
    pub erc1_road_speed_exceeded: u8,            // 0-3
    pub erc1_source_address: u8,                 // 0-253
    pub erc1_drivers_demand_torque: f64,         // -125 to 125%
    pub erc1_selection_non_engine: f64,          // 0-100%
    pub erc1_max_available_torque: f64,          // -125 to 125%

    // ERC2 - Electronic Retarder Controller 2
    pub erc2_transmission_output_retarder: u8,   // 0-3
    pub erc2_road_speed_limit_enable: u8,        // 0-3
    pub erc2_road_speed_limit_active: u8,        // 0-3
    pub erc2_transmission_retarder_enable: u8,   // 0-3
    pub erc2_cruise_control_speed_offset: f64,   // -12.5 to 12.5 km/h
    pub erc2_road_speed_limit_set_speed: f64,    // 0-250 km/h
    pub erc2_road_speed_limit_readiness: u8,     // 0-3
    pub erc2_retarder_derate_status: u8,         // 0-3

    // RC - Retarder Configuration
    pub rc_retarder_type: u8,             // 0-15
    pub rc_retarder_location: u8,         // 0-15
    pub rc_control_method: u8,            // 0-15
    pub rc_speed_at_idle: f64,            // 0-8031.875 rpm
    pub rc_torque_at_idle: f64,           // -125 to 125%
    pub rc_max_speed: f64,                // 0-8031.875 rpm
    pub rc_torque_at_max_speed: f64,      // -125 to 125%
    pub rc_speed_at_point3: f64,          // 0-8031.875 rpm
    pub rc_torque_at_point3: f64,         // -125 to 125%
    pub rc_speed_at_point4: f64,          // 0-8031.875 rpm
    pub rc_torque_at_point4: f64,         // -125 to 125%
    pub rc_speed_at_peak_torque: f64,     // 0-8031.875 rpm
    pub rc_reference_torque: u16,         // 0-64255 Nm
    pub rc_torque_at_peak: f64,           // -125 to 125%

    // LMP - Mast Position
    pub lmp_mast_position: f64, // 0-100%
}

impl Default for BrakingState {
    fn default() -> Self {
        Self {
            aebs_enabled: true,
            aebs_brake_demand: 0.0,
            aebs_target_deceleration: 0.0,
            aebs_status: 0,

            // EBC1 defaults
            ebc1_brake_pedal_position: 0.0,
            ebc1_ebs_brake_switch: 0,
            ebc1_abs_active: 0,
            ebc1_asr_engine_control_active: 0,
            ebc1_asr_brake_control_active: 0,
            ebc1_abs_off_road_switch: 0,
            ebc1_asr_off_road_switch: 0,
            ebc1_asr_hill_holder_switch: 0,
            ebc1_traction_control_override: 0,
            ebc1_accelerator_interlock: 0,
            ebc1_engine_derate_switch: 0,
            ebc1_aux_engine_shutdown_switch: 0,
            ebc1_remote_accel_enable_switch: 0,
            ebc1_engine_retarder_selection: 0.0,
            ebc1_abs_fully_operational: 1, // ABS operational by default
            ebc1_ebs_red_warning: 0,
            ebc1_abs_ebs_amber_warning: 0,
            ebc1_atc_asr_information_signal: 0,
            ebc1_source_address_brake_control: 0,
            ebc1_halt_brake_switch: 0,
            ebc1_trailer_abs_status: 0,
            ebc1_tractor_trailer_abs_warning: 0,

            // EBC2 defaults - wheel speeds
            ebc2_front_axle_speed: 0.0,
            ebc2_rel_speed_front_left: 0.0,
            ebc2_rel_speed_front_right: 0.0,
            ebc2_rel_speed_rear1_left: 0.0,
            ebc2_rel_speed_rear1_right: 0.0,
            ebc2_rel_speed_rear2_left: 0.0,
            ebc2_rel_speed_rear2_right: 0.0,

            // EBC3 defaults - brake pressures
            ebc3_pressure_front_left: 0.0,
            ebc3_pressure_front_right: 0.0,
            ebc3_pressure_rear1_left: 0.0,
            ebc3_pressure_rear1_right: 0.0,
            ebc3_pressure_rear2_left: 0.0,
            ebc3_pressure_rear2_right: 0.0,
            ebc3_pressure_rear3_left: 0.0,
            ebc3_pressure_rear3_right: 0.0,

            // EBC4 defaults - brake lining (healthy brakes at 80%)
            ebc4_lining_front_left: 80.0,
            ebc4_lining_front_right: 80.0,
            ebc4_lining_rear1_left: 80.0,
            ebc4_lining_rear1_right: 80.0,
            ebc4_lining_rear2_left: 80.0,
            ebc4_lining_rear2_right: 80.0,
            ebc4_lining_rear3_left: 80.0,
            ebc4_lining_rear3_right: 80.0,

            // EBC5 defaults
            ebc5_brake_temp_warning: 0,
            ebc5_halt_brake_mode: 0,
            ebc5_hill_holder_mode: 0,
            ebc5_foundation_brake_use: 0,
            ebc5_xbr_system_state: 0,         // All external demands accepted
            ebc5_xbr_active_control_mode: 0,  // No brake demand being executed
            ebc5_xbr_acceleration_limit: 0.0,
            ebc5_parking_brake_actuator: 0,
            ebc5_emergency_braking_active: 0,
            ebc5_railroad_mode: 0,
            ebc5_xbr_brake_hold_mode: 0,
            ebc5_driver_brake_demand: 0.0,
            ebc5_overall_brake_demand: 0.0,

            // EBC6 defaults - brake lining rear axles 4-7
            ebc6_lining_rear4_left: 80.0,
            ebc6_lining_rear4_right: 80.0,
            ebc6_lining_rear5_left: 80.0,
            ebc6_lining_rear5_right: 80.0,
            ebc6_lining_rear6_left: 80.0,
            ebc6_lining_rear6_right: 80.0,
            ebc6_lining_rear7_left: 80.0,
            ebc6_lining_rear7_right: 80.0,

            // EBC7 defaults - brake lining rear axles 8-10
            ebc7_lining_rear8_left: 80.0,
            ebc7_lining_rear8_right: 80.0,
            ebc7_lining_rear9_left: 80.0,
            ebc7_lining_rear9_right: 80.0,
            ebc7_lining_rear10_left: 80.0,
            ebc7_lining_rear10_right: 80.0,

            // EBCC defaults - engine brake continuous control
            ebcc_turbo1_outlet_pressure: 50.0,
            ebcc_turbo1_desired_outlet_pressure: 50.0,
            ebcc_exhaust_brake_command: 0.0,
            ebcc_turbo2_outlet_pressure: 50.0,
            ebcc_turbo2_desired_outlet_pressure: 50.0,

            // XBR defaults
            xbr_acceleration_demand: 0.0,
            xbr_ebi_mode: 0,
            xbr_priority: 3, // Lowest priority by default
            xbr_control_mode: 0,
            xbr_compensation_mode: 0,
            xbr_urgency: 0.0,
            xbr_brake_hold_request: 0,
            xbr_reason: 0,
            xbr_message_counter: 0,
            xbr_message_checksum: 0,

            // AEBS1 defaults
            aebs1_forward_collision_status: 0,
            aebs1_collision_warning_level: 0,
            aebs1_relevant_object_detected: 0,
            aebs1_bound_offset: 0,
            aebs1_time_to_collision: 25.0, // Max safe distance
            aebs1_road_departure_status: 0,

            // ACC1 defaults
            acc1_speed_of_forward_vehicle: 0,
            acc1_distance_to_forward_vehicle: 0,
            acc1_set_speed: 0,
            acc1_mode: 0,
            acc1_set_distance_mode: 0,
            acc1_road_curvature: 0.0,
            acc1_target_detected: 0,
            acc1_system_shutoff_warning: 0,
            acc1_distance_alert: 0,
            acc1_forward_collision_warning: 0,

            // ACC2 defaults
            acc2_usage_demand: 0,
            acc2_distance_mode: 0,

            // ACCS defaults
            accs_lateral_acceleration: 0.0,
            accs_longitudinal_acceleration: 0.0,
            accs_vertical_acceleration: 0.0,
            accs_lateral_fom: 0,
            accs_longitudinal_fom: 0,
            accs_vertical_fom: 0,
            accs_support_report_rate: 0,

            // ACCVC defaults
            accvc_aftercooler_thermostat_mode: 0,
            accvc_desired_aftercooler_temp: 25.0,
            accvc_desired_thermostat_opening: 50.0,
            accvc_charge_air_bypass_valve1_cmd: 0.0,
            accvc_charge_air_bypass_valve2_cmd: 0.0,
            accvc_aftercooler_diverter_valve_cmd: 0.0,

            // ERC1 defaults
            erc1_retarder_torque_mode: 0,
            erc1_enable_brake_assist: 0,
            erc1_enable_shift_assist: 0,
            erc1_actual_retarder_torque: 0.0,
            erc1_intended_retarder_torque: 0.0,
            erc1_coolant_load_increase: 0,
            erc1_requesting_brake_light: 0,
            erc1_road_speed_limit_switch: 0,
            erc1_road_speed_exceeded: 0,
            erc1_source_address: 0,
            erc1_drivers_demand_torque: 0.0,
            erc1_selection_non_engine: 0.0,
            erc1_max_available_torque: 0.0,

            // ERC2 defaults
            erc2_transmission_output_retarder: 0,
            erc2_road_speed_limit_enable: 0,
            erc2_road_speed_limit_active: 0,
            erc2_transmission_retarder_enable: 0,
            erc2_cruise_control_speed_offset: 0.0,
            erc2_road_speed_limit_set_speed: 0.0,
            erc2_road_speed_limit_readiness: 0,
            erc2_retarder_derate_status: 0,

            // RC defaults
            rc_retarder_type: 0,
            rc_retarder_location: 0,
            rc_control_method: 0,
            rc_speed_at_idle: 600.0,
            rc_torque_at_idle: 0.0,
            rc_max_speed: 2500.0,
            rc_torque_at_max_speed: 100.0,
            rc_speed_at_point3: 1000.0,
            rc_torque_at_point3: 25.0,
            rc_speed_at_point4: 1500.0,
            rc_torque_at_point4: 50.0,
            rc_speed_at_peak_torque: 2000.0,
            rc_reference_torque: 1000,
            rc_torque_at_peak: 100.0,

            // LMP defaults
            lmp_mast_position: 0.0,
        }
    }
}
