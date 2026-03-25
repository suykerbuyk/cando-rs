use serde::{Deserialize, Serialize};

/// Aftertreatment bank 1 & 2 state (Batches 6 and 11)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AftertreatmentState {
    // ============================================================================
    // Aftertreatment Bank 1 States (Batch 6)
    // ============================================================================

    // AT1S1 - Aftertreatment 1 Service 1 (DPF status)
    pub at1s1_dpf_soot_load_percent: u8,          // DPF soot load percent (0-250%)
    pub at1s1_dpf_ash_load_percent: u8,           // DPF ash load percent (0-250%)
    pub at1s1_dpf_time_since_last_regen: u32,     // Time since last active regen (s)
    pub at1s1_dpf_soot_load_regen_threshold: f64, // Soot load regen threshold (0-160.64%)

    // AT1S2 - Aftertreatment 1 Service 2
    pub at1s2_dpf_time_to_next_regen: u32,  // Time to next active regen (s)
    pub at1s2_scr_time_since_cleaning: u32, // SCR time since last cleaning (s)

    // AT1T1I1 - DEF Tank 1 Info 1
    pub at1t1i1_def_tank_volume: f64,              // DEF tank volume percent (0-100%)
    pub at1t1i1_def_tank_temp: f64,                // DEF tank temperature (-40 to 210 degC)
    pub at1t1i1_def_tank_level: f64,               // DEF tank level mm (0-6425.5)
    pub at1t1i1_def_tank_level_prelim_fmi: u8,     // Level preliminary FMI (0-31)
    pub at1t1i1_def_tank_low_level_indicator: u8,  // Low level indicator (0-7)
    pub at1t1i1_def_tank_temp_prelim_fmi: u8,      // Temperature preliminary FMI (0-31)
    pub at1t1i1_scr_operator_inducement_severity: u8, // Inducement severity (0-7)
    pub at1t1i1_def_tank_heater: f64,              // DEF tank heater percent (0-100%)
    pub at1t1i1_def_tank_heater_prelim_fmi: u8,    // Heater preliminary FMI (0-31)

    // AT1T1I2 - DEF Tank 1 Info 2
    pub at1t1i2_def_tank_volume_2: f64, // DEF tank volume 2 percent (0-100%)
    pub at1t1i2_def_tank_temp_2: f64,   // DEF tank temperature 2 (-40 to 210 degC)
    pub at1t1i2_def_tank_heater_2: f64, // DEF tank heater 2 percent (0-100%)

    // AT1TI - Aftertreatment 1 Trip Info (DPF trip statistics)
    pub at1ti_dpf_trip_fuel_used: f64,              // DPF trip fuel used (l)
    pub at1ti_dpf_trip_active_regen_time: u32,      // Trip active regen time (s)
    pub at1ti_dpf_trip_disabled_time: u32,          // Trip disabled time (s)
    pub at1ti_dpf_trip_num_active_regens: u32,      // Trip number of active regens
    pub at1ti_dpf_trip_passive_regen_time: u32,     // Trip passive regen time (s)
    pub at1ti_dpf_trip_num_passive_regens: u32,     // Trip number of passive regens
    pub at1ti_dpf_trip_num_regen_inhibit_requests: u32, // Trip inhibit requests
    pub at1ti_dpf_trip_num_regen_manual_requests: u32,  // Trip manual requests

    // AT1OG1 - Aftertreatment 1 Outlet Gas 1 (NOx/O2 at outlet)
    pub at1og1_outlet_nox: f64,                          // Outlet NOx ppm (-200 to 3012.75)
    pub at1og1_outlet_oxygen: f64,                       // Outlet percent oxygen (-12 to 21.03%)
    pub at1og1_outlet_gas_sensor_power_in_range: u8,     // Sensor power in range (0-3)
    pub at1og1_outlet_gas_sensor_at_temp: u8,            // Sensor at temperature (0-3)
    pub at1og1_outlet_nox_reading_stable: u8,            // NOx reading stable (0-3)
    pub at1og1_outlet_oxygen_reading_stable: u8,         // O2 reading stable (0-3)
    pub at1og1_outlet_gas_sensor_heater_prelim_fmi: u8,  // Heater prelim FMI (0-31)
    pub at1og1_outlet_gas_sensor_heater_control: u8,     // Heater control (0-3)
    pub at1og1_outlet_nox_sensor_prelim_fmi: u8,         // NOx sensor prelim FMI (0-31)
    pub at1og1_outlet_nox_sensor_self_diag: u8,          // NOx sensor self-diag (0-7)
    pub at1og1_outlet_oxygen_sensor_prelim_fmi: u8,      // O2 sensor prelim FMI (0-31)

    // AT1OG2 - Aftertreatment 1 Outlet Gas 2 (downstream temps)
    pub at1og2_exhaust_temp_3: f64,              // Exhaust temperature 3 (-273 to 1734.97 degC)
    pub at1og2_dpf_outlet_temp: f64,             // DPF outlet temperature (-273 to 1734.97 degC)
    pub at1og2_exhaust_temp_3_prelim_fmi: u8,    // Temp 3 prelim FMI (0-31)
    pub at1og2_dpf_outlet_temp_prelim_fmi: u8,   // DPF outlet temp prelim FMI (0-31)
    pub at1og2_exhaust_dew_point_detected: u8,   // Dew point detected (0-3)

    // AT1IG1 - Aftertreatment 1 Intake Gas 1 (NOx/O2 at intake)
    pub at1ig1_inlet_nox: f64,                          // Inlet NOx ppm (-200 to 3012.75)
    pub at1ig1_inlet_oxygen: f64,                       // Inlet percent oxygen (-12 to 21.03%)
    pub at1ig1_inlet_gas_sensor_power_in_range: u8,     // Sensor power in range (0-3)
    pub at1ig1_inlet_gas_sensor_at_temp: u8,            // Sensor at temperature (0-3)
    pub at1ig1_inlet_nox_reading_stable: u8,            // NOx reading stable (0-3)
    pub at1ig1_inlet_oxygen_reading_stable: u8,         // O2 reading stable (0-3)
    pub at1ig1_inlet_gas_sensor_heater_prelim_fmi: u8,  // Heater prelim FMI (0-31)
    pub at1ig1_inlet_gas_sensor_heater_control: u8,     // Heater control (0-3)
    pub at1ig1_inlet_nox_sensor_prelim_fmi: u8,         // NOx sensor prelim FMI (0-31)
    pub at1ig1_inlet_nox_sensor_self_diag: u8,          // NOx sensor self-diag (0-7)
    pub at1ig1_inlet_oxygen_sensor_prelim_fmi: u8,      // O2 sensor prelim FMI (0-31)

    // AT1IG2 - Aftertreatment 1 Intake Gas 2 (upstream temps)
    pub at1ig2_exhaust_temp_1: f64,              // Exhaust temperature 1 (-273 to 1734.97 degC)
    pub at1ig2_dpf_intake_temp: f64,             // DPF intake temperature (-273 to 1734.97 degC)
    pub at1ig2_exhaust_temp_1_prelim_fmi: u8,    // Temp 1 prelim FMI (0-31)
    pub at1ig2_dpf_intake_temp_prelim_fmi: u8,   // DPF intake temp prelim FMI (0-31)
    pub at1ig2_engine_exhaust_dew_point: u8,     // Dew point detected (0-3)

    // AT1HI1 - Aftertreatment 1 Historical Info 1 (lifetime stats)
    pub at1hi1_total_fuel_used: f64,                        // Total fuel used (l)
    pub at1hi1_total_regen_time: u32,                       // Total regeneration time (s)
    pub at1hi1_total_disabled_time: u32,                    // Total disabled time (s)
    pub at1hi1_total_num_active_regens: u32,                // Total number of active regens
    pub at1hi1_dpf_total_passive_regen_time: u32,           // DPF total passive regen time (s)
    pub at1hi1_dpf_total_num_passive_regens: u32,           // DPF total num passive regens
    pub at1hi1_dpf_total_num_regen_inhibit_requests: u32,   // Total inhibit requests
    pub at1hi1_dpf_total_num_regen_manual_requests: u32,    // Total manual requests
    pub at1hi1_dpf_avg_time_between_regens: u32,            // Avg time between regens (s)
    pub at1hi1_dpf_avg_distance_between_regens: f64,        // Avg distance between regens (km)
    pub at1hi1_dpf_num_active_regens: u32,                  // Number of active regens since filter install

    // AT1GP - Aftertreatment 1 Gas Pressure
    pub at1gp_dpf_intake_pressure: f64, // DPF intake pressure (0-6425.5 kPa)
    pub at1gp_dpf_outlet_pressure: f64, // DPF outlet pressure (0-6425.5 kPa)

    // AT1FC1 - Aftertreatment 1 Fuel Control 1
    pub at1fc1_fuel_pressure_1: f64,            // Fuel pressure 1 (0-6425.5 kPa)
    pub at1fc1_fuel_rate: f64,                  // Fuel rate (0-3212.75 l/h)
    pub at1fc1_fuel_pressure_1_control: f64,    // Fuel pressure 1 control (0-160.64%)
    pub at1fc1_fuel_drain_actuator: u8,         // Fuel drain actuator (0-3)
    pub at1fc1_ignition: u8,                    // Ignition (0-3)
    pub at1fc1_regen_status: u8,                // Regeneration status (0-3)
    pub at1fc1_fuel_enable_actuator: u8,        // Fuel enable actuator (0-3)
    pub at1fc1_fuel_injector_heater_control: f64, // Fuel injector heater control (0-100%)

    // AT1FC2 - Aftertreatment 1 Fuel Control 2
    pub at1fc2_fuel_pressure_2: f64,            // Fuel pressure 2 (0-6425.5 kPa)
    pub at1fc2_fuel_pump_relay_control: u8,     // Fuel pump relay control (0-3)
    pub at1fc2_fuel_flow_diverter_valve: u8,    // Fuel flow diverter valve (0-3)
    pub at1fc2_fuel_pressure_2_control: f64,    // Fuel pressure 2 control (0-160.64%)
    pub at1fc2_hc_doser_intake_fuel_temp: f64,  // HC doser intake fuel temperature (-40 to 210 degC)

    // AT1AC1 - Aftertreatment 1 Air Control 1
    pub at1ac1_supply_air_pressure: f64,        // Supply air pressure (0-6425.5 kPa)
    pub at1ac1_purge_air_pressure: f64,         // Purge air pressure (0-6425.5 kPa)
    pub at1ac1_air_pressure_control: f64,       // Air pressure control (0-160.64%)
    pub at1ac1_air_pressure_actuator_pos: f64,  // Air pressure actuator position (0-100%)
    pub at1ac1_air_system_relay: u8,            // Air system relay (0-3)
    pub at1ac1_atomization_air_actuator: u8,    // Atomization air actuator (0-3)
    pub at1ac1_purge_air_actuator: u8,          // Purge air actuator (0-3)
    pub at1ac1_air_enable_actuator: u8,         // Air enable actuator (0-3)

    // A1DOC1 - Aftertreatment 1 DOC 1 (Diesel Oxidation Catalyst)
    pub a1doc1_intake_temp: f64,              // DOC intake temperature (-273 to 1734.97 degC)
    pub a1doc1_outlet_temp: f64,              // DOC outlet temperature (-273 to 1734.97 degC)
    pub a1doc1_delta_pressure: f64,           // DOC differential pressure (0-6425.5 kPa)
    pub a1doc1_intake_temp_prelim_fmi: u8,    // Intake temp prelim FMI (0-31)
    pub a1doc1_outlet_temp_prelim_fmi: u8,    // Outlet temp prelim FMI (0-31)
    pub a1doc1_delta_pressure_prelim_fmi: u8, // Delta pressure prelim FMI (0-31)

    // A1DOC2 - Aftertreatment 1 DOC 2
    pub a1doc2_intake_pressure: f64,              // DOC intake pressure (0-6425.5 kPa)
    pub a1doc2_outlet_pressure: f64,              // DOC outlet pressure (0-6425.5 kPa)
    pub a1doc2_intake_to_dpf_outlet_delta: f64,   // DOC intake to DPF outlet delta pressure (0-6425.5 kPa)

    // A1SCRAI - Aftertreatment 1 SCR Ammonia Info
    pub a1scrai_outlet_nh3: f64,                          // Outlet NH3 ppm (-200 to 3012.75)
    pub a1scrai_outlet_nh3_prelim_fmi: u8,                // NH3 sensor prelim FMI (0-31)
    pub a1scrai_outlet_nh3_reading_stable: u8,            // NH3 reading stable (0-3)
    pub a1scrai_outlet_nh3_sensor_power_in_range: u8,     // Sensor power in range (0-3)
    pub a1scrai_outlet_nh3_sensor_at_temp: u8,            // Sensor at temperature (0-3)
    pub a1scrai_outlet_nh3_sensor_heater_prelim_fmi: u8,  // Heater prelim FMI (0-31)
    pub a1scrai_outlet_nh3_sensor_heater_control: u8,     // Heater control (0-3)

    // A1SCRSI1 - Aftertreatment 1 SCR Status Info 1
    pub a1scrsi1_def_avg_consumption: f64,              // DEF average consumption (0-3212.75 l/h)
    pub a1scrsi1_scr_commanded_def_consumption: f64,    // SCR commanded DEF (0-3212.75 l/h)
    pub a1scrsi1_scr_conversion_efficiency: f64,        // SCR conversion efficiency (0-100%)
    pub a1scrsi1_scr_inducement_travel_distance: u16,   // Inducement travel distance (0-64255 km)
    pub a1scrsi1_scr_sulfation_level: u8,               // SCR sulfation level (0-250%)

    // A1SCRSI2 - Aftertreatment 1 SCR Status Info 2
    pub a1scrsi2_total_def_used: f64, // Total DEF used (l)
    pub a1scrsi2_trip_def_used: f64,  // Trip DEF used (l)

    // DPF1S - DPF 1 Soot Status
    pub dpf1s_soot_mass: f64,             // DPF soot mass (0-1000 g)
    pub dpf1s_soot_density: f64,          // DPF soot density (0-20 g/L)
    pub dpf1s_mean_soot_signal: f64,      // DPF mean soot signal (0-160.64%)
    pub dpf1s_median_soot_signal: f64,    // DPF median soot signal (0-160.64%)
    pub dpf1s_soot_sensor_prelim_fmi: u8, // Soot sensor prelim FMI (0-31)
    pub dpf1s_soot_sensor_ecu_temp: f64,  // Soot sensor ECU internal temp (-40 to 210 degC)

    // DPF1S2 - DPF 1 Soot Status 2
    pub dpf1s2_soot_signal_std_dev: f64, // Soot signal standard deviation (0-160.64%)
    pub dpf1s2_soot_signal_max: f64,     // Soot signal maximum (0-160.64%)
    pub dpf1s2_soot_signal_min: f64,     // Soot signal minimum (0-160.64%)

    // DPFC1 - DPF Control 1
    pub dpfc1_dpf_lamp_command: u8,                     // DPF lamp command (0-7)
    pub dpfc1_dpf_active_regen_availability: u8,        // Active regen availability (0-3)
    pub dpfc1_dpf_passive_regen_status: u8,             // Passive regen status (0-3)
    pub dpfc1_dpf_active_regen_status: u8,              // Active regen status (0-3)
    pub dpfc1_dpf_status: u8,                           // DPF status (0-7)
    pub dpfc1_dpf_active_regen_inhibited: u8,           // Active regen inhibited (0-3)
    pub dpfc1_dpf_regen_inhibited_switch: u8,           // Inhibited due to switch (0-3)
    pub dpfc1_dpf_regen_inhibited_clutch: u8,           // Inhibited due to clutch (0-3)
    pub dpfc1_dpf_regen_inhibited_brake: u8,            // Inhibited due to brake (0-3)
    pub dpfc1_dpf_regen_inhibited_pto: u8,              // Inhibited due to PTO (0-3)
    pub dpfc1_dpf_regen_inhibited_accel: u8,            // Inhibited due to accelerator (0-3)
    pub dpfc1_dpf_regen_inhibited_neutral: u8,          // Inhibited due to out of neutral (0-3)
    pub dpfc1_dpf_regen_inhibited_speed: u8,            // Inhibited due to vehicle speed (0-3)
    pub dpfc1_dpf_regen_inhibited_parking: u8,          // Inhibited due to parking brake (0-3)
    pub dpfc1_dpf_regen_inhibited_low_temp: u8,         // Inhibited due to low exhaust temp (0-3)
    pub dpfc1_dpf_regen_inhibited_fault: u8,            // Inhibited due to system fault (0-3)
    pub dpfc1_dpf_regen_inhibited_timeout: u8,          // Inhibited due to timeout (0-3)
    pub dpfc1_dpf_regen_inhibited_temp_lockout: u8,     // Inhibited due to temp lockout (0-3)
    pub dpfc1_dpf_regen_inhibited_perm_lockout: u8,     // Inhibited due to perm lockout (0-3)
    pub dpfc1_dpf_regen_inhibited_engine_not_warm: u8,  // Inhibited due to engine not warm (0-3)
    pub dpfc1_dpf_regen_inhibited_speed_below: u8,      // Inhibited due to speed below allowed (0-3)
    pub dpfc1_dpf_auto_regen_config: u8,                // Auto active regen config (0-3)
    pub dpfc1_exhaust_high_temp_lamp: u8,               // Exhaust high temp lamp command (0-7)
    pub dpfc1_dpf_regen_forced_status: u8,              // Regen forced status (0-7)
    pub dpfc1_hc_doser_purging_enable: u8,              // HC doser purging enable (0-3)
    pub dpfc1_dpf_regen_inhibited_low_pressure: u8,     // Inhibited due to low exhaust pressure (0-3)
    pub dpfc1_dpf_conditions_not_met: u8,               // Conditions not met for active regen (0-3)
    pub dpfc1_dpf_regen_inhibited_thresher: u8,         // Inhibited due to thresher (0-3)

    // DPFC2 - DPF Control 2
    pub dpfc2_dpf_intake_temp_setpoint: f64, // DPF intake temp setpoint (-273 to 1734.97 degC)
    pub dpfc2_engine_unburned_fuel_pct: f64, // Engine unburned fuel % (0-160.64%)
    pub dpfc2_at1_fuel_mass_rate: f64,       // AT1 fuel mass rate (0-3212.75 g/min)
    pub dpfc2_at2_fuel_mass_rate: f64,       // AT2 fuel mass rate (0-3212.75 g/min)

    // ============================================================================
    // Aftertreatment Bank 2 + EGR States (Batch 11)
    // ============================================================================

    // AT2S1 - Aftertreatment 2 DPF Soot Status 1
    pub at2s1_dpf_soot_load_percent: u8,          // DPF soot load percent (0-250%)
    pub at2s1_dpf_ash_load_percent: u8,           // DPF ash load percent (0-250%)
    pub at2s1_dpf_time_since_last_regen: u32,     // Time since last active regen (s)
    pub at2s1_dpf_soot_load_regen_threshold: f64, // Soot load regen threshold (0-160.64%)

    // AT2S2 - Aftertreatment 2 DPF Status 2
    pub at2s2_dpf_time_to_next_regen: u32,    // Time to next active regen (s)
    pub at2s2_scr_time_since_last_clean: u32, // SCR time since last cleaning event (s)

    // AT2OG1 - Aftertreatment 2 Outlet Gas Sensor 1
    pub at2og1_outlet_nox: f64,                  // Outlet NOx (ppm, -200 to 3012.75)
    pub at2og1_outlet_percent_oxygen: f64,       // Outlet percent oxygen (-12 to 21.03%)
    pub at2og1_outlet_sensor_power_in_range: u8, // Sensor power in range (0-3)
    pub at2og1_outlet_sensor_at_temp: u8,        // Sensor at temperature (0-3)

    // AT2IG1 - Aftertreatment 2 Inlet Gas Sensor 1 (Engine Exhaust 2)
    pub at2ig1_inlet_nox: f64,                  // Engine exhaust 2 NOx 1 (ppm, -200 to 3012.75)
    pub at2ig1_inlet_percent_oxygen: f64,       // Engine exhaust 2 percent oxygen (-12 to 21.03%)
    pub at2ig1_inlet_sensor_power_in_range: u8, // Gas sensor 1 power in range (0-3)
    pub at2ig1_inlet_sensor_at_temp: u8,        // Gas sensor 1 at temperature (0-3)

    // AT2HI1 - Aftertreatment 2 Historical Info 1
    pub at2hi1_total_fuel_used: f64,    // Total fuel used (liters)
    pub at2hi1_total_regen_time: u32,   // Total regeneration time (s)
    pub at2hi1_total_disabled_time: u32, // Total disabled time (s)
    pub at2hi1_total_active_regens: u32, // Total number of active regens

    // AT2GP - Aftertreatment 2 Gas Pressures
    pub at2gp_dpf_intake_pressure: f64, // DPF intake pressure (kPa, 0-6425.5)
    pub at2gp_dpf_outlet_pressure: f64, // DPF outlet pressure (kPa, 0-6425.5)

    // AT2FC1 - Aftertreatment 2 Fuel Control 1
    pub at2fc1_fuel_pressure: f64,         // Fuel pressure 1 (kPa, 0-6425.5)
    pub at2fc1_fuel_rate: f64,             // Fuel rate (l/h, 0-3212.75)
    pub at2fc1_fuel_pressure_control: f64, // Fuel pressure 1 control (0-160.64%)
    pub at2fc1_regen_status: u8,           // Regeneration status (0-3)

    // AT2AC1 - Aftertreatment 2 Air Control 1
    pub at2ac1_supply_air_pressure: f64,          // Supply air pressure (kPa, 0-6425.5)
    pub at2ac1_purge_air_pressure: f64,           // Purge air pressure (kPa, 0-6425.5)
    pub at2ac1_air_pressure_control: f64,         // Air pressure control (0-160.64%)
    pub at2ac1_air_pressure_actuator_position: f64, // Air pressure actuator position (0-100%)

    // A2DOC1 - Aftertreatment 2 Diesel Oxidation Catalyst 1
    pub a2doc1_inlet_temp: f64,    // DOC inlet temperature (degC, -273 to 1734.97)
    pub a2doc1_outlet_temp: f64,   // DOC outlet temperature (degC, -273 to 1734.97)
    pub a2doc1_diff_pressure: f64, // DOC differential pressure (kPa, 0-32127.5)

    // A2SCRAI - Aftertreatment 2 SCR Ammonia Info
    pub a2scrai_outlet_nh3: f64, // Outlet NH3 (ppm, -200 to 3012.75)

    // A2SCRSI1 - Aftertreatment 2 SCR Status Info 1
    pub a2scrsi1_def_avg_consumption: f64,     // DEF average consumption (l/h, 0-3212.75)
    pub a2scrsi1_scr_commanded_consumption: f64, // SCR commanded consumption (l/h, 0-3212.75)
    pub a2scrsi1_scr_conversion_efficiency: f64, // SCR conversion efficiency (0-100%)

    // A1SCRDSI1 - Aftertreatment 1 SCR Dosing System Info 1 (Bank 1 Dosing)
    pub a1scrdsi1_dosing_rate: f64,         // Actual dosing quantity (g/h, 0-19276.5)
    pub a1scrdsi1_scr_system_1_state: u8,   // SCR system 1 state (0-15)
    pub a1scrdsi1_doser_1_abs_pressure: f64, // Doser 1 absolute pressure (kPa, 0-2000)

    // A1SCRDSI2 - Aftertreatment 1 SCR Dosing System Info 2
    pub a1scrdsi2_air_assist_pressure: f64, // SCR dosing air assist pressure (kPa, 0-2000)
    pub a1scrdsi2_air_assist_valve: f64,    // Air assist valve position (0-100%)
    pub a1scrdsi2_doser_1_temp: f64,        // DEF doser 1 temperature (degC, -40 to 210)

    // A1SCRDSI3 - Aftertreatment 1 SCR Dosing System Info 3
    pub a1scrdsi3_doser_1_pressure: f64,    // DEF doser 1 gage pressure (kPa, 0-1000)
    pub a1scrdsi3_doser_2_abs_pressure: f64, // DEF doser 2 absolute pressure (kPa, 0-2000)
    pub a1scrdsi3_doser_2_temp: f64,        // DEF doser 2 temperature (degC, -40 to 210)

    // A2SCRDSI1 - Aftertreatment 2 SCR Dosing System Info 1 (Bank 2 Dosing)
    pub a2scrdsi1_dosing_rate: f64,         // Actual dosing quantity (g/h, 0-19276.5)
    pub a2scrdsi1_scr_system_1_state: u8,   // SCR system 1 state (0-15)
    pub a2scrdsi1_doser_1_abs_pressure: f64, // Doser 1 absolute pressure (kPa, 0-2000)

    // A2SCRDSI2 - Aftertreatment 2 SCR Dosing System Info 2
    pub a2scrdsi2_air_assist_pressure: f64, // SCR dosing air assist pressure (kPa, 0-2000)
    pub a2scrdsi2_air_assist_valve: f64,    // Air assist valve position (0-100%)
    pub a2scrdsi2_doser_1_temp: f64,        // DEF doser 1 temperature (degC, -40 to 210)

    // A2SCRDSI3 - Aftertreatment 2 SCR Dosing System Info 3
    pub a2scrdsi3_doser_1_pressure: f64,    // DEF doser 1 gage pressure (kPa, 0-1000)
    pub a2scrdsi3_doser_2_abs_pressure: f64, // DEF doser 2 absolute pressure (kPa, 0-2000)
    pub a2scrdsi3_doser_2_temp: f64,        // DEF doser 2 temperature (degC, -40 to 210)

    // EEGR1A - Engine EGR 1 Actuator (Bank 1 EGR)
    pub eegr1a_actuator_1_desired_position: f64, // Actuator 1 desired position (0-100%)
    pub eegr1a_actuator_1_temp: f64,             // Actuator 1 temperature (degC, -40 to 210)
    pub eegr1a_actuator_2_desired_position: f64, // Actuator 2 desired position (0-100%)
    pub eegr1a_actuator_2_temp: f64,             // Actuator 2 temperature (degC, -40 to 210)

    // EEGR2A - Engine EGR 2 Actuator (Bank 2 EGR)
    pub eegr2a_actuator_1_desired_position: f64, // Actuator 1 desired position (0-100%)
    pub eegr2a_actuator_1_temp: f64,             // Actuator 1 temperature (degC, -40 to 210)
    pub eegr2a_actuator_2_desired_position: f64, // Actuator 2 desired position (0-100%)
    pub eegr2a_actuator_2_temp: f64,             // Actuator 2 temperature (degC, -40 to 210)

    // DPF2S - Diesel Particulate Filter 2 Soot
    pub dpf2s_soot_mass: f64,          // DPF2 soot mass (g, 0-1000)
    pub dpf2s_soot_density: f64,       // DPF2 soot density (g/L, 0-20)
    pub dpf2s_mean_soot_signal: f64,   // DPF2 mean soot signal (0-160.64%)
    pub dpf2s_median_soot_signal: f64, // DPF2 median soot signal (0-160.64%)
}

impl Default for AftertreatmentState {
    fn default() -> Self {
        Self {
            // AT1S1 - DPF Service 1 defaults
            at1s1_dpf_soot_load_percent: 25,          // 25% soot load (below regen threshold)
            at1s1_dpf_ash_load_percent: 10,           // 10% ash load
            at1s1_dpf_time_since_last_regen: 7200,    // 2 hours since last regen
            at1s1_dpf_soot_load_regen_threshold: 80.0, // 80% threshold

            // AT1S2 - Service 2 defaults
            at1s2_dpf_time_to_next_regen: 14400,  // 4 hours to next regen
            at1s2_scr_time_since_cleaning: 86400, // 24 hours since SCR cleaning

            // AT1T1I1 - DEF Tank 1 Info 1 defaults
            at1t1i1_def_tank_volume: 75.0,                   // 75% full
            at1t1i1_def_tank_temp: 25.0,                     // 25 degC (room temp)
            at1t1i1_def_tank_level: 450.0,                   // 450mm level
            at1t1i1_def_tank_level_prelim_fmi: 31,           // Not available
            at1t1i1_def_tank_low_level_indicator: 0,         // Off (adequate DEF)
            at1t1i1_def_tank_temp_prelim_fmi: 31,            // Not available
            at1t1i1_scr_operator_inducement_severity: 0,     // No inducement
            at1t1i1_def_tank_heater: 0.0,                    // Heater off
            at1t1i1_def_tank_heater_prelim_fmi: 31,          // Not available

            // AT1T1I2 - DEF Tank 1 Info 2 defaults
            at1t1i2_def_tank_volume_2: 75.0, // Same as primary
            at1t1i2_def_tank_temp_2: 25.0,   // Same as primary
            at1t1i2_def_tank_heater_2: 0.0,  // Heater off

            // AT1TI - Trip Info defaults
            at1ti_dpf_trip_fuel_used: 5.0,              // 5 liters used this trip
            at1ti_dpf_trip_active_regen_time: 600,      // 10 minutes of active regen
            at1ti_dpf_trip_disabled_time: 0,            // No disabled time
            at1ti_dpf_trip_num_active_regens: 2,        // 2 active regens
            at1ti_dpf_trip_passive_regen_time: 1800,    // 30 minutes passive regen
            at1ti_dpf_trip_num_passive_regens: 5,       // 5 passive regens
            at1ti_dpf_trip_num_regen_inhibit_requests: 0, // No inhibit requests
            at1ti_dpf_trip_num_regen_manual_requests: 0,  // No manual requests

            // AT1OG1 - Outlet Gas 1 defaults
            at1og1_outlet_nox: 20.0,                         // 20 ppm NOx at outlet (low = good SCR)
            at1og1_outlet_oxygen: 10.0,                      // 10% oxygen
            at1og1_outlet_gas_sensor_power_in_range: 1,      // Power in range
            at1og1_outlet_gas_sensor_at_temp: 1,             // At temperature
            at1og1_outlet_nox_reading_stable: 1,             // Stable
            at1og1_outlet_oxygen_reading_stable: 1,          // Stable
            at1og1_outlet_gas_sensor_heater_prelim_fmi: 31,  // Not available
            at1og1_outlet_gas_sensor_heater_control: 3,      // Automatic
            at1og1_outlet_nox_sensor_prelim_fmi: 31,         // Not available
            at1og1_outlet_nox_sensor_self_diag: 0,           // Diagnosis not active
            at1og1_outlet_oxygen_sensor_prelim_fmi: 31,      // Not available

            // AT1OG2 - Outlet Gas 2 defaults
            at1og2_exhaust_temp_3: 250.0,            // 250 degC downstream
            at1og2_dpf_outlet_temp: 260.0,           // 260 degC DPF outlet
            at1og2_exhaust_temp_3_prelim_fmi: 31,    // Not available
            at1og2_dpf_outlet_temp_prelim_fmi: 31,   // Not available
            at1og2_exhaust_dew_point_detected: 0,    // No dew point

            // AT1IG1 - Intake Gas 1 defaults
            at1ig1_inlet_nox: 400.0,                         // 400 ppm NOx at inlet (typical engine out)
            at1ig1_inlet_oxygen: 8.0,                        // 8% oxygen
            at1ig1_inlet_gas_sensor_power_in_range: 1,       // Power in range
            at1ig1_inlet_gas_sensor_at_temp: 1,              // At temperature
            at1ig1_inlet_nox_reading_stable: 1,              // Stable
            at1ig1_inlet_oxygen_reading_stable: 1,           // Stable
            at1ig1_inlet_gas_sensor_heater_prelim_fmi: 31,   // Not available
            at1ig1_inlet_gas_sensor_heater_control: 3,       // Automatic
            at1ig1_inlet_nox_sensor_prelim_fmi: 31,          // Not available
            at1ig1_inlet_nox_sensor_self_diag: 0,            // Diagnosis not active
            at1ig1_inlet_oxygen_sensor_prelim_fmi: 31,       // Not available

            // AT1IG2 - Intake Gas 2 defaults
            at1ig2_exhaust_temp_1: 300.0,            // 300 degC upstream
            at1ig2_dpf_intake_temp: 310.0,           // 310 degC DPF intake
            at1ig2_exhaust_temp_1_prelim_fmi: 31,    // Not available
            at1ig2_dpf_intake_temp_prelim_fmi: 31,   // Not available
            at1ig2_engine_exhaust_dew_point: 0,      // No dew point

            // AT1HI1 - Historical Info 1 defaults
            at1hi1_total_fuel_used: 500.0,                       // 500 liters lifetime
            at1hi1_total_regen_time: 36000,                      // 10 hours total regen
            at1hi1_total_disabled_time: 1800,                    // 30 minutes disabled
            at1hi1_total_num_active_regens: 200,                 // 200 active regens
            at1hi1_dpf_total_passive_regen_time: 72000,          // 20 hours passive regen
            at1hi1_dpf_total_num_passive_regens: 500,            // 500 passive regens
            at1hi1_dpf_total_num_regen_inhibit_requests: 10,
            at1hi1_dpf_total_num_regen_manual_requests: 5,
            at1hi1_dpf_avg_time_between_regens: 43200,           // 12 hours between regens
            at1hi1_dpf_avg_distance_between_regens: 400.0,       // 400 km between regens
            at1hi1_dpf_num_active_regens: 150,                   // 150 since filter install

            // AT1GP - Gas Pressure defaults
            at1gp_dpf_intake_pressure: 105.0, // 105 kPa (slightly above atmospheric)
            at1gp_dpf_outlet_pressure: 101.3, // 101.3 kPa (atmospheric)

            // AT1FC1 - Fuel Control 1 defaults
            at1fc1_fuel_pressure_1: 400.0,          // 400 kPa
            at1fc1_fuel_rate: 0.0,                  // Not dosing
            at1fc1_fuel_pressure_1_control: 0.0,    // Closed
            at1fc1_fuel_drain_actuator: 0,          // Not active
            at1fc1_ignition: 0,                     // Not active
            at1fc1_regen_status: 0,                 // Not active
            at1fc1_fuel_enable_actuator: 0,         // Not active
            at1fc1_fuel_injector_heater_control: 0.0, // Off

            // AT1FC2 - Fuel Control 2 defaults
            at1fc2_fuel_pressure_2: 400.0,           // 400 kPa
            at1fc2_fuel_pump_relay_control: 0,       // Off
            at1fc2_fuel_flow_diverter_valve: 0,      // Off
            at1fc2_fuel_pressure_2_control: 0.0,     // Closed
            at1fc2_hc_doser_intake_fuel_temp: 60.0,  // 60 degC

            // AT1AC1 - Air Control 1 defaults
            at1ac1_supply_air_pressure: 800.0,       // 800 kPa supply air
            at1ac1_purge_air_pressure: 600.0,        // 600 kPa purge air
            at1ac1_air_pressure_control: 50.0,       // 50% control
            at1ac1_air_pressure_actuator_pos: 50.0,  // 50% position
            at1ac1_air_system_relay: 1,              // Active
            at1ac1_atomization_air_actuator: 0,      // Not active
            at1ac1_purge_air_actuator: 0,            // Not active
            at1ac1_air_enable_actuator: 1,           // Active

            // A1DOC1 - DOC 1 defaults
            a1doc1_intake_temp: 280.0,              // 280 degC DOC intake
            a1doc1_outlet_temp: 300.0,              // 300 degC DOC outlet (exothermic)
            a1doc1_delta_pressure: 3.5,             // 3.5 kPa differential
            a1doc1_intake_temp_prelim_fmi: 31,      // Not available
            a1doc1_outlet_temp_prelim_fmi: 31,      // Not available
            a1doc1_delta_pressure_prelim_fmi: 31,   // Not available

            // A1DOC2 - DOC 2 defaults
            a1doc2_intake_pressure: 105.0,              // 105 kPa
            a1doc2_outlet_pressure: 101.5,              // 101.5 kPa
            a1doc2_intake_to_dpf_outlet_delta: 3.5,     // 3.5 kPa

            // A1SCRAI - SCR Ammonia Info defaults
            a1scrai_outlet_nh3: 5.0,                         // 5 ppm NH3 (low slip)
            a1scrai_outlet_nh3_prelim_fmi: 31,               // Not available
            a1scrai_outlet_nh3_reading_stable: 1,            // Stable
            a1scrai_outlet_nh3_sensor_power_in_range: 1,     // In range
            a1scrai_outlet_nh3_sensor_at_temp: 1,            // At temperature
            a1scrai_outlet_nh3_sensor_heater_prelim_fmi: 31, // Not available
            a1scrai_outlet_nh3_sensor_heater_control: 3,     // Automatic

            // A1SCRSI1 - SCR Status Info 1 defaults
            a1scrsi1_def_avg_consumption: 2.5,           // 2.5 l/h DEF consumption
            a1scrsi1_scr_commanded_def_consumption: 2.8, // 2.8 l/h commanded
            a1scrsi1_scr_conversion_efficiency: 97.0,    // 97% efficiency (excellent)
            a1scrsi1_scr_inducement_travel_distance: 0,  // No inducement active
            a1scrsi1_scr_sulfation_level: 5,             // 5% sulfation

            // A1SCRSI2 - SCR Status Info 2 defaults
            a1scrsi2_total_def_used: 2000.0, // 2000 liters lifetime
            a1scrsi2_trip_def_used: 10.0,    // 10 liters this trip

            // DPF1S - DPF 1 Soot Status defaults
            dpf1s_soot_mass: 15.0,            // 15 grams soot
            dpf1s_soot_density: 1.5,          // 1.5 g/L
            dpf1s_mean_soot_signal: 25.0,     // 25% signal level
            dpf1s_median_soot_signal: 24.0,   // 24% median
            dpf1s_soot_sensor_prelim_fmi: 31, // Not available
            dpf1s_soot_sensor_ecu_temp: 65.0, // 65 degC ECU temp

            // DPF1S2 - DPF 1 Soot Status 2 defaults
            dpf1s2_soot_signal_std_dev: 2.0, // 2% std deviation
            dpf1s2_soot_signal_max: 30.0,    // 30% maximum
            dpf1s2_soot_signal_min: 20.0,    // 20% minimum

            // DPFC1 - DPF Control 1 defaults
            dpfc1_dpf_lamp_command: 0,
            dpfc1_dpf_active_regen_availability: 1, // Ready for initiation
            dpfc1_dpf_passive_regen_status: 0,
            dpfc1_dpf_active_regen_status: 0,
            dpfc1_dpf_status: 0,                    // Regen not needed
            dpfc1_dpf_active_regen_inhibited: 0,
            dpfc1_dpf_regen_inhibited_switch: 0,
            dpfc1_dpf_regen_inhibited_clutch: 0,
            dpfc1_dpf_regen_inhibited_brake: 0,
            dpfc1_dpf_regen_inhibited_pto: 0,
            dpfc1_dpf_regen_inhibited_accel: 0,
            dpfc1_dpf_regen_inhibited_neutral: 0,
            dpfc1_dpf_regen_inhibited_speed: 0,
            dpfc1_dpf_regen_inhibited_parking: 0,
            dpfc1_dpf_regen_inhibited_low_temp: 0,
            dpfc1_dpf_regen_inhibited_fault: 0,
            dpfc1_dpf_regen_inhibited_timeout: 0,
            dpfc1_dpf_regen_inhibited_temp_lockout: 0,
            dpfc1_dpf_regen_inhibited_perm_lockout: 0,
            dpfc1_dpf_regen_inhibited_engine_not_warm: 0,
            dpfc1_dpf_regen_inhibited_speed_below: 0,
            dpfc1_dpf_auto_regen_config: 1, // Auto regen enabled
            dpfc1_exhaust_high_temp_lamp: 0,
            dpfc1_dpf_regen_forced_status: 0,
            dpfc1_hc_doser_purging_enable: 0,
            dpfc1_dpf_regen_inhibited_low_pressure: 0,
            dpfc1_dpf_conditions_not_met: 0,
            dpfc1_dpf_regen_inhibited_thresher: 0,

            // DPFC2 - DPF Control 2 defaults
            dpfc2_dpf_intake_temp_setpoint: 550.0, // 550 degC setpoint for regen
            dpfc2_engine_unburned_fuel_pct: 2.0,   // 2% unburned fuel
            dpfc2_at1_fuel_mass_rate: 0.0,         // Not dosing
            dpfc2_at2_fuel_mass_rate: 0.0,         // Not dosing

            // ============================================================================
            // Aftertreatment Bank 2 + EGR defaults (Batch 11)
            // ============================================================================

            // AT2S1 - DPF Soot Status 1 defaults
            at2s1_dpf_soot_load_percent: 25,       // 25% soot load (moderate)
            at2s1_dpf_ash_load_percent: 10,        // 10% ash load (low)
            at2s1_dpf_time_since_last_regen: 3600, // 1 hour since last regen
            at2s1_dpf_soot_load_regen_threshold: 80.0, // 80% threshold for regen

            // AT2S2 - DPF Status 2 defaults
            at2s2_dpf_time_to_next_regen: 7200,    // 2 hours to next regen
            at2s2_scr_time_since_last_clean: 86400, // 24 hours since last SCR clean

            // AT2OG1 - Outlet Gas Sensor 1 defaults
            at2og1_outlet_nox: 20.0,               // 20 ppm outlet NOx (low after SCR)
            at2og1_outlet_percent_oxygen: 8.0,     // 8% outlet oxygen
            at2og1_outlet_sensor_power_in_range: 1, // In range
            at2og1_outlet_sensor_at_temp: 1,       // At temperature

            // AT2IG1 - Inlet Gas Sensor 1 defaults
            at2ig1_inlet_nox: 200.0,               // 200 ppm inlet NOx (before SCR)
            at2ig1_inlet_percent_oxygen: 10.0,     // 10% inlet oxygen
            at2ig1_inlet_sensor_power_in_range: 1, // In range
            at2ig1_inlet_sensor_at_temp: 1,        // At temperature

            // AT2HI1 - Historical Info 1 defaults
            at2hi1_total_fuel_used: 1500.0,        // 1500 liters total fuel used
            at2hi1_total_regen_time: 36000,        // 10 hours total regen time
            at2hi1_total_disabled_time: 1800,      // 30 min total disabled time
            at2hi1_total_active_regens: 500,       // 500 active regens lifetime

            // AT2GP - Gas Pressures defaults
            at2gp_dpf_intake_pressure: 120.0,      // 120 kPa intake pressure
            at2gp_dpf_outlet_pressure: 105.0,      // 105 kPa outlet pressure

            // AT2FC1 - Fuel Control 1 defaults
            at2fc1_fuel_pressure: 350.0,           // 350 kPa fuel pressure
            at2fc1_fuel_rate: 5.0,                 // 5 l/h fuel rate
            at2fc1_fuel_pressure_control: 45.0,    // 45% pressure control
            at2fc1_regen_status: 0,                // Not regenerating

            // AT2AC1 - Air Control 1 defaults
            at2ac1_supply_air_pressure: 550.0,     // 550 kPa supply air
            at2ac1_purge_air_pressure: 200.0,      // 200 kPa purge air
            at2ac1_air_pressure_control: 30.0,     // 30% air pressure control
            at2ac1_air_pressure_actuator_position: 25.0, // 25% actuator position

            // A2DOC1 - Diesel Oxidation Catalyst 1 defaults
            a2doc1_inlet_temp: 250.0,              // 250 degC inlet temp
            a2doc1_outlet_temp: 280.0,             // 280 degC outlet temp (exothermic)
            a2doc1_diff_pressure: 3.5,             // 3.5 kPa differential pressure

            // A2SCRAI - SCR Ammonia Info defaults
            a2scrai_outlet_nh3: 5.0,               // 5 ppm outlet NH3 (ammonia slip)

            // A2SCRSI1 - SCR Status Info 1 defaults
            a2scrsi1_def_avg_consumption: 2.5,     // 2.5 l/h DEF consumption
            a2scrsi1_scr_commanded_consumption: 2.8, // 2.8 l/h commanded consumption
            a2scrsi1_scr_conversion_efficiency: 95.0, // 95% conversion efficiency

            // A1SCRDSI1 - Bank 1 SCR Dosing System Info 1 defaults
            a1scrdsi1_dosing_rate: 500.0,          // 500 g/h dosing rate
            a1scrdsi1_scr_system_1_state: 2,       // Normal dosing operation
            a1scrdsi1_doser_1_abs_pressure: 800.0, // 800 kPa doser pressure

            // A1SCRDSI2 - Bank 1 SCR Dosing System Info 2 defaults
            a1scrdsi2_air_assist_pressure: 600.0,  // 600 kPa air assist pressure
            a1scrdsi2_air_assist_valve: 50.0,      // 50% air assist valve
            a1scrdsi2_doser_1_temp: 65.0,          // 65 degC doser temperature

            // A1SCRDSI3 - Bank 1 SCR Dosing System Info 3 defaults
            a1scrdsi3_doser_1_pressure: 400.0,     // 400 kPa doser 1 gage pressure
            a1scrdsi3_doser_2_abs_pressure: 750.0, // 750 kPa doser 2 abs pressure
            a1scrdsi3_doser_2_temp: 60.0,          // 60 degC doser 2 temperature

            // A2SCRDSI1 - Bank 2 SCR Dosing System Info 1 defaults
            a2scrdsi1_dosing_rate: 480.0,          // 480 g/h dosing rate
            a2scrdsi1_scr_system_1_state: 2,       // Normal dosing operation
            a2scrdsi1_doser_1_abs_pressure: 780.0, // 780 kPa doser pressure

            // A2SCRDSI2 - Bank 2 SCR Dosing System Info 2 defaults
            a2scrdsi2_air_assist_pressure: 580.0,  // 580 kPa air assist pressure
            a2scrdsi2_air_assist_valve: 48.0,      // 48% air assist valve
            a2scrdsi2_doser_1_temp: 63.0,          // 63 degC doser temperature

            // A2SCRDSI3 - Bank 2 SCR Dosing System Info 3 defaults
            a2scrdsi3_doser_1_pressure: 380.0,     // 380 kPa doser 1 gage pressure
            a2scrdsi3_doser_2_abs_pressure: 720.0, // 720 kPa doser 2 abs pressure
            a2scrdsi3_doser_2_temp: 58.0,          // 58 degC doser 2 temperature

            // EEGR1A - EGR 1 Actuator defaults
            eegr1a_actuator_1_desired_position: 30.0, // 30% EGR1 actuator 1 position
            eegr1a_actuator_1_temp: 75.0,          // 75 degC actuator 1 temp
            eegr1a_actuator_2_desired_position: 25.0, // 25% EGR1 actuator 2 position
            eegr1a_actuator_2_temp: 72.0,          // 72 degC actuator 2 temp

            // EEGR2A - EGR 2 Actuator defaults
            eegr2a_actuator_1_desired_position: 28.0, // 28% EGR2 actuator 1 position
            eegr2a_actuator_1_temp: 73.0,          // 73 degC actuator 1 temp
            eegr2a_actuator_2_desired_position: 22.0, // 22% EGR2 actuator 2 position
            eegr2a_actuator_2_temp: 70.0,          // 70 degC actuator 2 temp

            // DPF2S - DPF 2 Soot defaults
            dpf2s_soot_mass: 15.0,                 // 15g soot mass
            dpf2s_soot_density: 2.5,               // 2.5 g/L soot density
            dpf2s_mean_soot_signal: 30.0,          // 30% mean soot signal
            dpf2s_median_soot_signal: 28.0,        // 28% median soot signal
        }
    }
}
