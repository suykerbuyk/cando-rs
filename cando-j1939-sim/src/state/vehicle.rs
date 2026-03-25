use serde::{Deserialize, Serialize};

/// Vehicle speed, distance, wheels, electrical, and time state (Batch 4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleState {
    // CCVS1 - Cruise Control / Vehicle Speed 1
    pub ccvs1_vehicle_speed: f64,            // Wheel-based vehicle speed (0-251 km/h)
    pub ccvs1_cruise_control_active: u8,     // Cruise control active (0-3)
    pub ccvs1_parking_brake: u8,             // Parking brake switch (0-3)
    pub ccvs1_brake_switch: u8,              // Brake switch (0-3)
    pub ccvs1_clutch_switch: u8,             // Clutch switch (0-3)
    pub ccvs1_cruise_control_set_speed: u8,  // Cruise control set speed (0-250 km/h)
    pub ccvs1_cruise_control_states: u8,     // Cruise control states (0-7)
    pub ccvs1_cruise_control_enable: u8,     // Cruise control enable switch (0-3)
    pub ccvs1_pto_governor_state: u8,        // PTO governor state (0-31)

    // CCVS2 - Cruise Control / Vehicle Speed 2
    pub ccvs2_cruise_disable_command: u8,    // Cruise control disable command (0-3)
    pub ccvs2_idle_speed_request: f64,       // Idle speed request (0-8031.88 rpm)

    // CCVS3 - Cruise Control / Vehicle Speed 3
    pub ccvs3_adaptive_cc_readiness: u8,     // Adaptive cruise control readiness (0-3)
    pub ccvs3_cc_system_command_state: u8,   // Cruise control system command state (0-7)
    pub ccvs3_cruise_control_speed: f64,     // Cruise control speed (0-251 km/h)

    // CCVS4 - Cruise Control / Vehicle Speed 4
    pub ccvs4_applied_speed_limit: f64,      // Applied vehicle speed limit (0-251 km/h)

    // CCVS5 - Cruise Control / Vehicle Speed 5
    pub ccvs5_directional_speed: f64,        // Directional vehicle speed (-250 to 251.99 km/h)

    // CCVS6 - Cruise Control / Vehicle Speed 6
    pub ccvs6_roadway_speed_limit_mode: u8,  // Current roadway speed limit mode (0-15)
    pub ccvs6_roadway_speed_limit: f64,      // Current roadway vehicle speed limit (0-251 km/h)

    // VD - Vehicle Distance
    pub vd_trip_distance: f64,               // Trip distance (km)
    pub vd_total_distance: f64,              // Total vehicle distance (km)

    // VDS - Vehicle Direction/Speed
    pub vds_compass_bearing: f64,            // Compass bearing (0-501.99 deg)
    pub vds_nav_speed: f64,                  // Navigation-based vehicle speed (0-251 km/h)
    pub vds_pitch: f64,                      // Pitch (-200 to 301.99 deg)
    pub vds_altitude: f64,                   // Altitude (-2500 to 5531.88 m)

    // VDS2 - Vehicle Direction/Speed 2
    pub vds2_vehicle_roll: f64,              // Vehicle roll (-90 to 90 deg)

    // HRW - High Resolution Wheel Speed
    pub hrw_front_left_speed: f64,           // Front axle left wheel speed (0-251 km/h)
    pub hrw_front_right_speed: f64,          // Front axle right wheel speed (0-251 km/h)
    pub hrw_rear_left_speed: f64,            // Rear axle left wheel speed (0-251 km/h)
    pub hrw_rear_right_speed: f64,           // Rear axle right wheel speed (0-251 km/h)

    // VW - Vehicle Weight
    pub vw_axle_location: u8,                // Axle location (0-255)
    pub vw_axle_weight: f64,                 // Axle weight (kg)
    pub vw_trailer_weight: f64,              // Trailer weight (kg)
    pub vw_cargo_weight: f64,                // Cargo weight (kg)

    // TIRE1 - Tire Condition 1
    pub tire1_location: u8,                  // Tire location (0-255)
    pub tire1_pressure: f64,                 // Tire pressure (kPa)
    pub tire1_temperature: f64,              // Tire temperature (deg C)
    pub tire1_status: u8,                    // Tire status (0-3)
    pub tire1_leakage_rate: f64,             // Tire air leakage rate (l/s)

    // TIRE2 - Tire Condition 2
    pub tire2_location: u8,                  // Tire location (0-255)
    pub tire2_pressure_extended: u16,        // Tire pressure extended range (kPa)
    pub tire2_required_pressure: u16,        // Required tire pressure (kPa)

    // SSI - Slope Sensor Information
    pub ssi_pitch_angle: f64,                // Pitch angle (deg)
    pub ssi_roll_angle: f64,                 // Roll angle (deg)
    pub ssi_pitch_rate: f64,                 // Pitch rate (deg/s)

    // VEP1 - Vehicle Electrical Power 1
    pub vep1_battery_current: f64,           // SLI battery 1 net current (A)
    pub vep1_alternator_current: u8,         // Alternator current (A)
    pub vep1_charging_voltage: f64,          // Charging system potential voltage (V)
    pub vep1_battery_potential: f64,         // Battery potential / power input 1 (V)
    pub vep1_key_switch_voltage: f64,        // Key switch battery potential (V)

    // VEP2 - Vehicle Electrical Power 2
    pub vep2_battery_potential_2: f64,       // Battery potential / power input 2 (V)
    pub vep2_ecu_supply_voltage_1: f64,      // ECU power output supply voltage 1 (V)
    pub vep2_ecu_supply_voltage_2: f64,      // ECU power output supply voltage 2 (V)
    pub vep2_ecu_supply_voltage_3: f64,      // ECU power output supply voltage 3 (V)

    // VEP3 - Vehicle Electrical Power 3
    pub vep3_alternator_current_hr: f64,     // Alternator current high range resolution (A)
    pub vep3_battery_current_hr: f64,        // SLI battery 1 net current high range (A)
    pub vep3_battery_2_current: f64,         // SLI battery 2 net current (A)
    pub vep3_key_switch_state: u8,           // ECU key switch state (0-3)

    // AS1 - Alternator Speed 1
    pub as1_alternator_speed: f64,           // Alternator speed (rpm)
    pub as1_alternator_1_status: u8,         // Alternator 1 status (0-3)

    // AS2 - Alternator Speed 2
    pub as2_setpoint_voltage_feedback: f64,  // Alternator setpoint voltage feedback (V)
    pub as2_output_voltage: f64,             // Alternator output voltage (V)
    pub as2_regulator_temperature: f64,      // Alternator voltage regulator temperature (deg C)
    pub as2_excitation_current: f64,         // Alternator excitation current (A)
    pub as2_excitation_duty_cycle: f64,      // Alternator excitation duty cycle (%)

    // EP - Electronic Process (ECU Power)
    pub ep_keep_alive_consumption: u16,      // Keep alive battery consumption (W)
    pub ep_data_memory_usage: f64,           // Data memory usage (%)

    // TD - Time/Date
    pub td_seconds: f64,                     // Seconds (0-59.75)
    pub td_minutes: u8,                      // Minutes (0-59)
    pub td_hours: u8,                        // Hours (0-23)
    pub td_day: f64,                         // Day (0.25-31.75)
    pub td_month: u8,                        // Month (1-12)
    pub td_year: f64,                        // Year (1985-2235)
    pub td_local_minute_offset: f64,         // Local minute offset
    pub td_local_hour_offset: f64,           // Local hour offset

    // OEL - Operator External Light Controls
    pub oel_work_light: u8,                  // Work light switch (0-3)
    pub oel_main_light: u8,                  // Main light switch (0-3)
    pub oel_turn_signal: u8,                 // Turn signal switch (0-3)
    pub oel_hazard_light: u8,                // Hazard light switch (0-3)
    pub oel_high_low_beam: u8,               // High/low beam switch (0-3)

    // SHUTDN - Shutdown
    pub shutdn_idle_shutdown: u8,            // Engine idle shutdown has shutdown engine (0-3)
    pub shutdn_wait_to_start: u8,            // Engine wait to start lamp (0-3)

    // BSA - Brake Stroke Alert
    pub bsa_axle1_left: u8,                  // Tractor brake stroke axle 1 left (0-3)
    pub bsa_axle1_right: u8,                 // Tractor brake stroke axle 1 right (0-3)
    pub bsa_axle2_left: u8,                  // Tractor brake stroke axle 2 left (0-3)
    pub bsa_axle2_right: u8,                 // Tractor brake stroke axle 2 right (0-3)

    // GFI1 - Gaseous Fuel Information 1
    pub gfi1_total_fuel_used: f64,           // Total engine PTO governor fuel used gaseous (kg)
    pub gfi1_trip_average_fuel_rate: f64,    // Trip average fuel rate gaseous (kg/h)
    pub gfi1_fuel_specific_gravity: f64,     // Engine fuel specific gravity
}

impl Default for VehicleState {
    fn default() -> Self {
        Self {
            // CCVS1
            ccvs1_vehicle_speed: 0.0,
            ccvs1_cruise_control_active: 0,     // Off
            ccvs1_parking_brake: 1,             // Parking brake set
            ccvs1_brake_switch: 0,              // Brake released
            ccvs1_clutch_switch: 0,             // Clutch released
            ccvs1_cruise_control_set_speed: 0,
            ccvs1_cruise_control_states: 0,     // Off/Disabled
            ccvs1_cruise_control_enable: 0,     // Disabled
            ccvs1_pto_governor_state: 0,

            // CCVS2
            ccvs2_cruise_disable_command: 0,
            ccvs2_idle_speed_request: 0.0,

            // CCVS3
            ccvs3_adaptive_cc_readiness: 0,
            ccvs3_cc_system_command_state: 0,
            ccvs3_cruise_control_speed: 0.0,

            // CCVS4
            ccvs4_applied_speed_limit: 120.0,    // 120 km/h typical limit

            // CCVS5
            ccvs5_directional_speed: 0.0,

            // CCVS6
            ccvs6_roadway_speed_limit_mode: 0,   // Disabled
            ccvs6_roadway_speed_limit: 0.0,

            // VD - Vehicle Distance
            vd_trip_distance: 0.0,
            vd_total_distance: 12500.0,          // 12,500 km on the odometer

            // VDS - Vehicle Direction/Speed
            vds_compass_bearing: 0.0,
            vds_nav_speed: 0.0,
            vds_pitch: 0.0,
            vds_altitude: 100.0,                 // 100m above sea level

            // VDS2
            vds2_vehicle_roll: 0.0,

            // HRW - High Resolution Wheel Speed
            hrw_front_left_speed: 0.0,
            hrw_front_right_speed: 0.0,
            hrw_rear_left_speed: 0.0,
            hrw_rear_right_speed: 0.0,

            // VW - Vehicle Weight
            vw_axle_location: 1,                  // Front axle
            vw_axle_weight: 5000.0,               // 5000 kg
            vw_trailer_weight: 0.0,
            vw_cargo_weight: 2000.0,              // 2000 kg cargo

            // TIRE1
            tire1_location: 1,
            tire1_pressure: 800.0,                // 800 kPa (~116 psi)
            tire1_temperature: 25.0,              // Ambient temp
            tire1_status: 0,                      // OK
            tire1_leakage_rate: 0.0,

            // TIRE2
            tire2_location: 1,
            tire2_pressure_extended: 800,
            tire2_required_pressure: 827,         // Target pressure

            // SSI - Slope Sensor
            ssi_pitch_angle: 0.0,
            ssi_roll_angle: 0.0,
            ssi_pitch_rate: 0.0,

            // VEP1
            vep1_battery_current: 0.0,
            vep1_alternator_current: 45,          // 45A charging
            vep1_charging_voltage: 14.2,          // Normal charging
            vep1_battery_potential: 12.8,         // Healthy battery
            vep1_key_switch_voltage: 12.6,

            // VEP2
            vep2_battery_potential_2: 12.7,
            vep2_ecu_supply_voltage_1: 12.0,
            vep2_ecu_supply_voltage_2: 5.0,
            vep2_ecu_supply_voltage_3: 3.3,

            // VEP3
            vep3_alternator_current_hr: 45.0,
            vep3_battery_current_hr: 0.0,
            vep3_battery_2_current: 0.0,
            vep3_key_switch_state: 1,             // On

            // AS1
            as1_alternator_speed: 0.0,
            as1_alternator_1_status: 1,           // Active

            // AS2
            as2_setpoint_voltage_feedback: 14.4,
            as2_output_voltage: 14.2,
            as2_regulator_temperature: 65.0,
            as2_excitation_current: 3.5,
            as2_excitation_duty_cycle: 50.0,

            // EP
            ep_keep_alive_consumption: 5,         // 5W standby
            ep_data_memory_usage: 35.0,           // 35% used

            // TD - Time/Date (will be overwritten by real clock)
            td_seconds: 0.0,
            td_minutes: 0,
            td_hours: 0,
            td_day: 1.0,
            td_month: 1,
            td_year: 2026.0,
            td_local_minute_offset: 0.0,
            td_local_hour_offset: 0.0,

            // OEL
            oel_work_light: 0,
            oel_main_light: 0,
            oel_turn_signal: 0,
            oel_hazard_light: 0,
            oel_high_low_beam: 0,

            // SHUTDN
            shutdn_idle_shutdown: 0,
            shutdn_wait_to_start: 0,

            // BSA
            bsa_axle1_left: 0,
            bsa_axle1_right: 0,
            bsa_axle2_left: 0,
            bsa_axle2_right: 0,

            // GFI1
            gfi1_total_fuel_used: 0.0,
            gfi1_trip_average_fuel_rate: 0.0,
            gfi1_fuel_specific_gravity: 0.72,     // Typical diesel
        }
    }
}
