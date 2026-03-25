use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorState {
    // Wand angle state
    pub wand_angle: f64,
    pub target_wand_angle: f64,
    pub wand_quality: u8,

    // Linear displacement state
    pub linear_displacement: f64,
    pub target_displacement: f64,
    pub displacement_quality: u8,

    // ============================================================================
    // Batch 3: Engine Temps, Fluids & Sensors States
    // ============================================================================

    // ET1 Engine Temperature 1 States
    pub et1_coolant_temp: f64,           // Engine coolant temperature (-40 to 210 degC)
    pub et1_fuel_temp: f64,              // Engine fuel 1 temperature 1 (-40 to 210 degC)
    pub et1_oil_temp: f64,               // Engine oil temperature 1 (-273 to 1735 degC)
    pub et1_turbo_oil_temp: f64,         // Turbocharger 1 oil temperature (-273 to 1735 degC)
    pub et1_intercooler_temp: f64,       // Engine intercooler temperature (-40 to 210 degC)
    pub et1_charge_air_cooler_thermostat: f64, // Charge air cooler thermostat opening (0-100%)

    // ET2 Engine Temperature 2 States
    pub et2_oil_temp_2: f64,             // Engine oil temperature 2 (-273 to 1735 degC)
    pub et2_ecu_temp: f64,               // Engine ECU temperature (-40 to 210 degC)
    pub et2_egr_diff_pressure: f64,      // EGR 1 differential pressure (-250 to 252 kPa)
    pub et2_egr_temp: f64,               // EGR 1 temperature (-40 to 210 degC)

    // ET3 Engine Temperature 3 States
    pub et3_intake_manifold_temp_hr: f64, // Intake manifold 1 temp high resolution
    pub et3_coolant_temp_hr: f64,         // Engine coolant temp high resolution
    pub et3_intake_valve_oil_temp: f64,   // Intake valve actuation system oil temp
    pub et3_charge_air_cooler_outlet_temp: f64, // Charge air cooler 1 outlet temp

    // ET4 Engine Temperature 4 States
    pub et4_coolant_temp_2: f64,             // Engine coolant temperature 2
    pub et4_coolant_pump_outlet_temp: f64,   // Engine coolant pump outlet temperature
    pub et4_coolant_thermostat_opening: f64, // Engine coolant thermostat opening (0-100%)
    pub et4_exhaust_valve_oil_temp: f64,     // Exhaust valve actuation system oil temp
    pub et4_egr_mixer_intake_temp: f64,      // EGR 1 mixer intake temperature
    pub et4_coolant_temp_3: f64,             // Engine coolant temperature 3

    // ET5 Engine Temperature 5 States
    pub et5_egr2_temp: f64,              // EGR 2 temperature
    pub et5_egr2_mixer_intake_temp: f64, // EGR 2 mixer intake temperature
    pub et5_coolant_temp_2_hr: f64,      // Coolant temp 2 high resolution extended range

    // ET6 Engine Temperature 6 States
    pub et6_charge_air_cooler_intake_coolant_temp: f64,
    pub et6_charge_air_cooler_outlet_coolant_temp: f64,
    pub et6_intake_coolant_temp: f64,
    pub et6_intake_mixed_air_side_coolant_temp: f64,

    // LFE1 Liquid Fuel Economy 1 States
    pub lfe1_fuel_rate: f64,             // Engine fuel rate (0-3212.75 L/h)
    pub lfe1_instant_fuel_economy: f64,  // Instantaneous fuel economy (0-125 km/L)
    pub lfe1_average_fuel_economy: f64,  // Average fuel economy (0-125 km/L)
    pub lfe1_throttle_valve_1_pos: f64,  // Throttle valve 1 position (0-100%)
    pub lfe1_throttle_valve_2_pos: f64,  // Throttle valve 2 position (0-100%)

    // LFE2 Liquid Fuel Economy 2 States
    pub lfe2_fuel_rate_hr: f64,          // Fuel rate high resolution (0-3212.75 L/h)
    pub lfe2_diesel_fuel_demand_rate: f64, // Diesel fuel demand rate

    // IC1 Intake/Exhaust Conditions 1 States
    pub ic1_aftertreatment_intake_pressure: f64,
    pub ic1_intake_manifold_pressure: f64,   // Intake manifold 1 pressure (0-500 kPa)
    pub ic1_intake_manifold_temp: f64,       // Intake manifold 1 temperature (-40 to 210 degC)
    pub ic1_intake_air_pressure: f64,        // Intake air pressure (0-500 kPa)
    pub ic1_air_filter_diff_pressure: f64,   // Air filter 1 differential pressure (0-12.5 kPa)
    pub ic1_exhaust_temp: f64,               // Engine exhaust temperature (-273 to 1735 degC)
    pub ic1_coolant_filter_diff_pressure: f64, // Coolant filter differential pressure (0-125 kPa)

    // IC2 Intake/Exhaust Conditions 2 States
    pub ic2_air_filter_2_diff_pressure: f64,
    pub ic2_air_filter_3_diff_pressure: f64,
    pub ic2_air_filter_4_diff_pressure: f64,
    pub ic2_intake_manifold_2_pressure: f64,
    pub ic2_intake_manifold_1_abs_pressure: f64,
    pub ic2_intake_manifold_1_abs_pressure_hr: f64,
    pub ic2_intake_manifold_2_abs_pressure: f64,

    // IC3 Intake/Exhaust Conditions 3 States
    pub ic3_mixer_1_intake_pressure: f64,
    pub ic3_mixer_2_intake_pressure: f64,
    pub ic3_intake_manifold_2_abs_pressure_hr: f64,
    pub ic3_desired_intake_manifold_pressure_high_limit: f64,

    // AMB Ambient Conditions States
    pub amb_barometric_pressure: f64,    // Barometric pressure (0-125 kPa)
    pub amb_cab_interior_temp: f64,      // Cab interior temperature (-273 to 1735 degC)
    pub amb_ambient_temp: f64,           // Ambient air temperature (-273 to 1735 degC)
    pub amb_intake_air_temp: f64,        // Engine intake 1 air temperature (-40 to 210 degC)
    pub amb_road_surface_temp: f64,      // Road surface temperature (-273 to 1735 degC)

    // AMB2 Ambient Conditions 2 States
    pub amb2_solar_intensity: f64,
    pub amb2_solar_sensor_max: f64,
    pub amb2_specific_humidity: f64,
    pub amb2_calculated_ambient_temp: f64,
    pub amb2_barometric_abs_pressure_hr: f64,

    // AMB3 Ambient Conditions 3 States
    pub amb3_barometric_abs_pressure_2: f64,
    pub amb3_intake_2_air_temp: f64,
    pub amb3_power_derate_humidity_diff: f64,

    // AMB4 Ambient Conditions 4 States
    pub amb4_fuel_specific_humidity: f64,
    pub amb4_charge_air_specific_humidity: f64,
    pub amb4_fuel_relative_humidity: f64,
    pub amb4_charge_air_relative_humidity: f64,

    // FD2 Fan Drive 2 States
    pub fd2_estimated_fan_2_speed_pct: f64,  // Estimated percent fan 2 speed (0-100%)
    pub fd2_fan_2_drive_state: u8,           // Fan 2 drive state (0-15)
    pub fd2_fan_2_speed: f64,                // Fan 2 speed (0-32127.5 rpm)
    pub fd2_hydraulic_fan_2_pressure: f64,   // Hydraulic fan 2 motor pressure
    pub fd2_fan_2_bypass_command_status: f64,

    // DD2 Dash Display 2 States
    pub dd2_oil_filter_diff_pressure_ext: f64, // Oil filter diff pressure extended range
    pub dd2_fuel_2_tank_1_level: f64,          // Fuel 2 tank 1 level (0-100%)
    pub dd2_fuel_2_tank_2_level: f64,          // Fuel 2 tank 2 level
    pub dd2_fuel_2_tank_3_level: f64,
    pub dd2_fuel_2_tank_4_level: f64,
    pub dd2_display_remain_powered: u8,        // Display remain powered (0-3)
    pub dd2_oil_level_high_low: f64,

    // DD3 Dash Display 3 States
    pub dd3_predictive_speed_adj_indicator_state: u8,
    pub dd3_predictive_speed_adj_speed: f64,

    // HOURS Engine Hours States
    pub hours_engine_total_hours: f64,       // Total hours of operation (0-210554060.75 hr)
    pub hours_total_revolutions: f64,        // Total revolutions (0-4211081215 rev)

    // HOURS2 Engine Hours 2 States
    pub hours2_idle_management_active_total_time: f64,

    // IO Idle Operation States
    pub io_total_idle_fuel_used: f64,    // Total idle fuel used (0-3212.75 L)
    pub io_total_idle_hours: f64,        // Total idle hours (0-210554060.75 hr)

    // FL Fuel Leakage States
    pub fl_fuel_leakage_1: u8,           // Fuel leakage 1 (0-3)
    pub fl_fuel_leakage_2: u8,           // Fuel leakage 2 (0-3)
    pub fl_fluid_bund_level: f64,        // Fluid bund level (0-100%)

    // LFC1 Lifetime Fuel Consumption 1 States
    pub lfc1_trip_fuel: f64,             // Engine trip fuel (0-2105540607.5 L)
    pub lfc1_total_fuel_used: f64,       // Engine total fuel used (0-2105540607.5 L)
}

impl Default for SensorState {
    fn default() -> Self {
        Self {
            wand_angle: 0.0,
            target_wand_angle: 0.0,
            wand_quality: 3,
            linear_displacement: 0.0,
            target_displacement: 0.0,
            displacement_quality: 3,

            // ============================================================================
            // Batch 3: Engine Temps, Fluids & Sensors Defaults
            // ============================================================================

            // ET1 Engine Temperature 1 defaults
            et1_coolant_temp: 85.0,           // Normal operating temperature
            et1_fuel_temp: 45.0,              // Normal fuel temperature
            et1_oil_temp: 95.0,               // Normal oil temperature
            et1_turbo_oil_temp: 110.0,        // Turbo oil runs hotter
            et1_intercooler_temp: 50.0,       // After charge air cooling
            et1_charge_air_cooler_thermostat: 60.0, // 60% open

            // ET2 Engine Temperature 2 defaults
            et2_oil_temp_2: 92.0,             // Secondary oil temp sensor
            et2_ecu_temp: 55.0,               // ECU internal temperature
            et2_egr_diff_pressure: 5.0,       // Low differential at idle
            et2_egr_temp: 180.0,              // EGR gas temperature

            // ET3 Engine Temperature 3 defaults
            et3_intake_manifold_temp_hr: 45.0,
            et3_coolant_temp_hr: 85.0,
            et3_intake_valve_oil_temp: 80.0,
            et3_charge_air_cooler_outlet_temp: 42.0,

            // ET4 Engine Temperature 4 defaults
            et4_coolant_temp_2: 84.0,
            et4_coolant_pump_outlet_temp: 82.0,
            et4_coolant_thermostat_opening: 65.0,
            et4_exhaust_valve_oil_temp: 90.0,
            et4_egr_mixer_intake_temp: 120.0,
            et4_coolant_temp_3: 83.0,

            // ET5 Engine Temperature 5 defaults
            et5_egr2_temp: 175.0,
            et5_egr2_mixer_intake_temp: 115.0,
            et5_coolant_temp_2_hr: 84.0,

            // ET6 Engine Temperature 6 defaults
            et6_charge_air_cooler_intake_coolant_temp: 35.0,
            et6_charge_air_cooler_outlet_coolant_temp: 45.0,
            et6_intake_coolant_temp: 38.0,
            et6_intake_mixed_air_side_coolant_temp: 40.0,

            // LFE1 Liquid Fuel Economy 1 defaults
            lfe1_fuel_rate: 3.5,              // Idle fuel rate (L/h)
            lfe1_instant_fuel_economy: 8.5,   // km/L
            lfe1_average_fuel_economy: 7.2,   // km/L
            lfe1_throttle_valve_1_pos: 15.0,  // Idle throttle position
            lfe1_throttle_valve_2_pos: 0.0,   // Secondary throttle closed

            // LFE2 Liquid Fuel Economy 2 defaults
            lfe2_fuel_rate_hr: 3.5,
            lfe2_diesel_fuel_demand_rate: 3.2,

            // IC1 Intake/Exhaust Conditions 1 defaults
            ic1_aftertreatment_intake_pressure: 102.0,
            ic1_intake_manifold_pressure: 101.3, // Atmospheric at idle
            ic1_intake_manifold_temp: 45.0,
            ic1_intake_air_pressure: 100.0,
            ic1_air_filter_diff_pressure: 1.5,  // Clean filter
            ic1_exhaust_temp: 250.0,
            ic1_coolant_filter_diff_pressure: 8.0,

            // IC2 Intake/Exhaust Conditions 2 defaults
            ic2_air_filter_2_diff_pressure: 1.2,
            ic2_air_filter_3_diff_pressure: 1.0,
            ic2_air_filter_4_diff_pressure: 0.8,
            ic2_intake_manifold_2_pressure: 100.0,
            ic2_intake_manifold_1_abs_pressure: 101.3,
            ic2_intake_manifold_1_abs_pressure_hr: 101.3,
            ic2_intake_manifold_2_abs_pressure: 100.0,

            // IC3 Intake/Exhaust Conditions 3 defaults
            ic3_mixer_1_intake_pressure: 100.0,
            ic3_mixer_2_intake_pressure: 99.0,
            ic3_intake_manifold_2_abs_pressure_hr: 100.0,
            ic3_desired_intake_manifold_pressure_high_limit: 200.0,

            // AMB Ambient Conditions defaults
            amb_barometric_pressure: 101.3,   // Standard atmospheric pressure
            amb_cab_interior_temp: 22.0,      // Comfortable cab temp
            amb_ambient_temp: 25.0,           // 25C outdoor temp
            amb_intake_air_temp: 30.0,        // Slightly above ambient (under hood)
            amb_road_surface_temp: 30.0,      // Road surface temp

            // AMB2 Ambient Conditions 2 defaults
            amb2_solar_intensity: 50.0,
            amb2_solar_sensor_max: 100.0,
            amb2_specific_humidity: 8.0,
            amb2_calculated_ambient_temp: 25.0,
            amb2_barometric_abs_pressure_hr: 101.325,

            // AMB3 Ambient Conditions 3 defaults
            amb3_barometric_abs_pressure_2: 101.3,
            amb3_intake_2_air_temp: 32.0,
            amb3_power_derate_humidity_diff: 0.0,

            // AMB4 Ambient Conditions 4 defaults
            amb4_fuel_specific_humidity: 5.0,
            amb4_charge_air_specific_humidity: 8.0,
            amb4_fuel_relative_humidity: 30.0,
            amb4_charge_air_relative_humidity: 45.0,

            // FD2 Fan Drive 2 defaults
            fd2_estimated_fan_2_speed_pct: 25.0, // Low speed at idle
            fd2_fan_2_drive_state: 1,             // Normal operation
            fd2_fan_2_speed: 1200.0,              // Low RPM
            fd2_hydraulic_fan_2_pressure: 500.0,
            fd2_fan_2_bypass_command_status: 0.0,

            // DD2 Dash Display 2 defaults
            dd2_oil_filter_diff_pressure_ext: 15.0,
            dd2_fuel_2_tank_1_level: 75.0,    // 75% fuel
            dd2_fuel_2_tank_2_level: 0.0,
            dd2_fuel_2_tank_3_level: 0.0,
            dd2_fuel_2_tank_4_level: 0.0,
            dd2_display_remain_powered: 0,
            dd2_oil_level_high_low: 0.0,

            // DD3 Dash Display 3 defaults
            dd3_predictive_speed_adj_indicator_state: 0,
            dd3_predictive_speed_adj_speed: 0.0,

            // HOURS Engine Hours defaults
            hours_engine_total_hours: 12500.5,    // Hours of operation
            hours_total_revolutions: 850000000.0, // Total revolutions

            // HOURS2 Engine Hours 2 defaults
            hours2_idle_management_active_total_time: 3200.0,

            // IO Idle Operation defaults
            io_total_idle_fuel_used: 1250.0,  // Liters over engine life
            io_total_idle_hours: 4500.0,      // Idle hours over engine life

            // FL Fuel Leakage defaults
            fl_fuel_leakage_1: 0,             // No leakage
            fl_fuel_leakage_2: 0,             // No leakage
            fl_fluid_bund_level: 0.0,         // Empty bund (no leakage)

            // LFC1 Lifetime Fuel Consumption 1 defaults
            lfc1_trip_fuel: 125.5,            // Current trip fuel usage (L)
            lfc1_total_fuel_used: 95000.0,    // Total lifetime fuel (L)
        }
    }
}
