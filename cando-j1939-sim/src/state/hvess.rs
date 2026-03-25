use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HvessState {
    // HVESSC1 control
    pub hvess_power_down_command: bool,
    pub hvess_cell_balancing_command: bool,

    // HVESSD1 power monitoring
    pub hvess_discharge_power: f64,
    pub hvess_charge_power: f64,
    pub hvess_voltage_level: f64,
    pub hvess_current_level: f64,

    // HVESSD6 voltage/temperature
    pub hvess_bus_voltage: f64,
    pub hvess_ignition_voltage: f64,
    pub hvess_coolant_temp: f64,
    pub hvess_electronics_temp: f64,

    // HVESSD2 cell voltage & SOC
    pub hvess_fast_update_state_of_charge: f64,
    pub hvess_highest_cell_voltage: f64,
    pub hvess_lowest_cell_voltage: f64,
    pub hvess_cell_voltage_differential_status: u64,

    // HVESSD3 cell temperature
    pub hvess_highest_cell_temperature: f64,
    pub hvess_lowest_cell_temperature: f64,
    pub hvess_average_cell_temperature: f64,
    pub hvess_cell_temp_differential_status: u64,

    // HVESSFS1 fan status
    pub hvess_fan_speed_status: u64,
    pub hvess_fan_status_reason_code: u64,
    pub hvess_fan_command_status: u64,
    pub hvess_fan_speed: f64,
    pub hvess_fan_medium_temperature: f64,
    pub hvess_fan_power: f64,
    pub hvess_fan_service_indicator: u64,
    pub hvess_fan_operating_status: u64,
    pub hvess_fan_status1_instance: u64,

    // HVESSD4 Capacity & Cell Balancing States
    pub hvessd4_discharge_capacity: f64,    // HVESSD4 discharge capacity (0-642.55 Ah)
    pub hvessd4_charge_capacity: f64,       // HVESSD4 charge capacity (0-642.55 Ah)
    pub hvessd4_cell_balancing_count: u16,  // HVESSD4 cell balancing count (0-64255)

    // HVESSD5 Current Limits & Cell SOC States
    pub hvessd5_max_discharge_current_limit: f64, // HVESSD5 max instantaneous discharge current limit (-1600 to 0 A)
    pub hvessd5_max_charge_current_limit: f64,    // HVESSD5 max instantaneous charge current limit (0 to 1612.75 A)
    pub hvessd5_min_cell_soc: f64,                // HVESSD5 minimum cell state of charge (0-100%)
    pub hvessd5_max_cell_soc: f64,                // HVESSD5 maximum cell state of charge (0-100%)

    // HVESSD7 Energy Capacity & Charge Voltage States
    pub hvessd7_discharge_energy_capacity: f64,     // HVESSD7 discharge energy capacity (0-16449.53 kWh)
    pub hvessd7_charge_energy_capacity: f64,        // HVESSD7 charge energy capacity (0-16449.53 kWh)
    pub hvessd7_max_charge_voltage_limit: f64,      // HVESSD7 maximum charge voltage limit (0-3212.75 V)

    // HVESSD8 Cell Voltage Location States
    pub hvessd8_highest_cell_voltage_module: u8,  // HVESSD8 highest cell voltage module number
    pub hvessd8_highest_cell_voltage_cell: u8,    // HVESSD8 highest cell voltage cell number
    pub hvessd8_lowest_cell_voltage_module: u8,   // HVESSD8 lowest cell voltage module number
    pub hvessd8_lowest_cell_voltage_cell: u8,     // HVESSD8 lowest cell voltage cell number
    pub hvessd8_average_cell_voltage: f64,        // HVESSD8 average cell voltage (0-64.25 V)

    // HVESSD9 Cell Temperature Location States
    pub hvessd9_highest_cell_temp_module: u8,     // HVESSD9 highest cell temperature module number
    pub hvessd9_highest_cell_temp_cell: u8,       // HVESSD9 highest cell temperature cell number
    pub hvessd9_lowest_cell_temp_module: u8,      // HVESSD9 lowest cell temperature module number
    pub hvessd9_lowest_cell_temp_cell: u8,        // HVESSD9 lowest cell temperature cell number
    pub hvessd9_thermal_event_detected: u8,       // HVESSD9 thermal event detected (0-3)
    pub hvessd9_counter: u8,                      // HVESSD9 message counter (0-15)

    // HVESSD10 Cell SOC Location & Isolation States
    pub hvessd10_highest_cell_soc_module: u8,     // HVESSD10 highest cell SOC module number
    pub hvessd10_highest_cell_soc_cell: u8,       // HVESSD10 highest cell SOC cell number
    pub hvessd10_lowest_cell_soc_module: u8,      // HVESSD10 lowest cell SOC module number
    pub hvessd10_lowest_cell_soc_cell: u8,        // HVESSD10 lowest cell SOC cell number
    pub hvessd10_active_isolation_test: f64,      // HVESSD10 active isolation test results (kOhm)
    pub hvessd10_passive_isolation_test: f64,     // HVESSD10 passive isolation test results (kOhm)

    // HVESSD11 Voltage & Energy States
    pub hvessd11_bus_voltage_neg_to_chassis: f64, // HVESSD11 bus voltage negative to chassis ground (V)
    pub hvessd11_voltage_neg_to_chassis: f64,     // HVESSD11 voltage level negative to chassis ground (V)
    pub hvessd11_actual_charge_rate: f64,         // HVESSD11 actual charge rate
    pub hvessd11_total_stored_energy: f64,        // HVESSD11 total stored energy user level
    pub hvessd11_power_module_electronics_temp: f64, // HVESSD11 power module electronics temperature

    // HVESSD12 Coolant & Time Remaining States
    pub hvessd12_intake_coolant_pressure: f64,    // HVESSD12 intake coolant pressure
    pub hvessd12_estimated_discharge_time: f64,   // HVESSD12 estimated discharge time remaining
    pub hvessd12_estimated_charge_time: f64,      // HVESSD12 estimated charge time remaining
    pub hvessd12_hv_exposure_indicator: u8,       // HVESSD12 high voltage exposure indicator (0-3)
    pub hvessd12_power_hold_relay_status: u8,     // HVESSD12 power hold relay status (0-3)
    pub hvessd12_positive_precharge_relay: u8,    // HVESSD12 HV bus positive pre-charge relay status (0-3)
    pub hvessd12_negative_precharge_relay: u8,    // HVESSD12 HV bus negative pre-charge relay status (0-3)

    // HVESSD13 Extended Range Power & Voltage States
    pub hvessd13_discharge_power_extended: f64,   // HVESSD13 available discharge power extended range
    pub hvessd13_charge_power_extended: f64,      // HVESSD13 available charge power extended range
    pub hvessd13_voltage_extended: f64,           // HVESSD13 voltage level extended range
    pub hvessd13_current_extended: f64,           // HVESSD13 current extended range

    // HVESSD14 Extended Range Current & Voltage States
    pub hvessd14_max_discharge_current_extended: f64, // HVESSD14 max instantaneous discharge current limit extended
    pub hvessd14_max_charge_current_extended: f64,    // HVESSD14 max instantaneous charge current limit extended
    pub hvessd14_bus_voltage_extended: f64,           // HVESSD14 bus voltage extended range
    pub hvessd14_min_discharge_voltage_limit: f64,    // HVESSD14 minimum discharge voltage limit

    // HVESSD15 Nominal Current Limits
    pub hvessd15_nominal_discharge_current_limit: f64, // HVESSD15 nominal discharge current limit
    pub hvessd15_nominal_charge_current_limit: f64,    // HVESSD15 nominal charge current limit

    // HVESSIS1 Internal Segment 1 States (voltage/current pairs 1-2)
    pub hvessis1_internal_voltage_1: f64,         // HVESSIS1 internal voltage level 1 (V)
    pub hvessis1_internal_current_1: f64,         // HVESSIS1 internal current 1 (A)
    pub hvessis1_internal_voltage_2: f64,         // HVESSIS1 internal voltage level 2 (V)
    pub hvessis1_internal_current_2: f64,         // HVESSIS1 internal current 2 (A)

    // HVESSIS2 Internal Segment 2 States (voltage/current pairs 3-4)
    pub hvessis2_internal_voltage_3: f64,
    pub hvessis2_internal_current_3: f64,
    pub hvessis2_internal_voltage_4: f64,
    pub hvessis2_internal_current_4: f64,

    // HVESSIS3 Internal Segment 3 States (voltage/current pairs 5-6)
    pub hvessis3_internal_voltage_5: f64,
    pub hvessis3_internal_current_5: f64,
    pub hvessis3_internal_voltage_6: f64,
    pub hvessis3_internal_current_6: f64,

    // HVESSIS4 Internal Segment 4 States (voltage/current pairs 7-8)
    pub hvessis4_internal_voltage_7: f64,
    pub hvessis4_internal_current_7: f64,
    pub hvessis4_internal_voltage_8: f64,
    pub hvessis4_internal_current_8: f64,

    // HVESSIS5 Internal Segment 5 States (contactor/relay/heater for segments 1-2)
    pub hvessis5_positive_contactor_1_state: u8,
    pub hvessis5_negative_contactor_1_state: u8,
    pub hvessis5_precharge_relay_1_state: u8,
    pub hvessis5_inline_heater_1_status: u8,
    pub hvessis5_bus_voltage_1: f64,
    pub hvessis5_positive_contactor_2_state: u8,
    pub hvessis5_negative_contactor_2_state: u8,
    pub hvessis5_precharge_relay_2_state: u8,
    pub hvessis5_inline_heater_2_status: u8,
    pub hvessis5_bus_voltage_2: f64,

    // HVESSIS6 Internal Segment 6 States (contactor/relay/heater for segments 3-4)
    pub hvessis6_positive_contactor_3_state: u8,
    pub hvessis6_negative_contactor_3_state: u8,
    pub hvessis6_precharge_relay_3_state: u8,
    pub hvessis6_inline_heater_3_status: u8,
    pub hvessis6_bus_voltage_3: f64,
    pub hvessis6_positive_contactor_4_state: u8,
    pub hvessis6_negative_contactor_4_state: u8,
    pub hvessis6_precharge_relay_4_state: u8,
    pub hvessis6_inline_heater_4_status: u8,
    pub hvessis6_bus_voltage_4: f64,

    // HVESSIS7 Internal Segment 7 States
    pub hvessis7_number_of_internal_circuits: u8, // Number of internal circuits read (0-250)

    // HVESSMS1 Module Status 1 (modules 1-32)
    pub hvessms1_module_status: [u8; 32],

    // HVESSMS2 Module Status 2 (modules 33-64)
    pub hvessms2_module_status: [u8; 32],

    // HVESSMS3 Module Status 3 (modules 65-96)
    pub hvessms3_module_status: [u8; 32],

    // HVESSS1 System Status 1 States
    pub hvesss1_positive_contactor_state: u8,
    pub hvesss1_negative_contactor_state: u8,
    pub hvesss1_disconnect_forewarning: u8,
    pub hvesss1_precharge_relay_state: u8,
    pub hvesss1_center_of_pack_contactor: u8,
    pub hvesss1_active_isolation_test_status: u8,
    pub hvesss1_passive_isolation_test_status: u8,
    pub hvesss1_hvil_status: u8,
    pub hvesss1_inertia_switch_status: u8,
    pub hvesss1_soc_status: u8,
    pub hvesss1_cell_balance_status: u8,
    pub hvesss1_cell_balancing_active: u8,
    pub hvesss1_internal_charger_status: u8,
    pub hvesss1_counter: u8,
    pub hvesss1_bus_connection_status: u8,
    pub hvesss1_operational_status: u8,
    pub hvesss1_num_packs_ready: u8,
    pub hvesss1_crc: u8,

    // HVESSS2 System Status 2 States (power limit derating reasons)
    pub hvesss2_discharge_limit_soc: u8,
    pub hvesss2_discharge_limit_temp: u8,
    pub hvesss2_discharge_limit_diag: u8,
    pub hvesss2_discharge_limit_voltage: u8,
    pub hvesss2_discharge_limit_current: u8,
    pub hvesss2_discharge_limit_undefined: u8,
    pub hvesss2_discharge_limit_electronics_temp: u8,
    pub hvesss2_charge_limit_soc: u8,
    pub hvesss2_charge_limit_temp: u8,
    pub hvesss2_charge_limit_diag: u8,
    pub hvesss2_charge_limit_voltage: u8,
    pub hvesss2_charge_limit_current: u8,
    pub hvesss2_charge_limit_undefined: u8,
    pub hvesss2_charge_limit_electronics_temp: u8,

    // HVESSFS2 Fan Status 2 States
    pub hvessfs2_fan_voltage: f64,                // HVESSFS2 fan voltage (V)
    pub hvessfs2_fan_current: f64,                // HVESSFS2 fan current (A)
    pub hvessfs2_fan_hvil_status: u8,             // HVESSFS2 fan HVIL status (0-3)
    pub hvessfs2_fan_status_2_instance: u8,       // HVESSFS2 instance (0-15)
    pub hvessfs2_fan_percent_speed_status: u8,    // HVESSFS2 percent speed status (0-3)
    pub hvessfs2_fan_percent_speed: f64,          // HVESSFS2 fan percent speed (0-100%)

    // HVESSFC Fan Command States
    pub hvessfc_fan_enable_command: u8,           // HVESSFC fan enable command (0-3)
    pub hvessfc_fan_power_hold: u8,               // HVESSFC fan power hold (0-3)
    pub hvessfc_fan_speed_command: f64,           // HVESSFC fan speed command (rpm)
    pub hvessfc_fan_percent_speed_command: f64,   // HVESSFC fan percent speed command (%)

    // HVESSCFG Configuration States
    pub hvesscfg_nominal_voltage: f64,            // HVESSCFG nominal voltage (V)
    pub hvesscfg_min_operating_voltage: f64,      // HVESSCFG recommended minimum operating voltage
    pub hvesscfg_max_operating_voltage: f64,      // HVESSCFG recommended maximum operating voltage
    pub hvesscfg_min_soc: f64,                    // HVESSCFG recommended minimum SOC
    pub hvesscfg_max_soc: f64,                    // HVESSCFG recommended maximum SOC
    pub hvesscfg_max_operating_temp: f64,         // HVESSCFG recommended maximum operating temp
    pub hvesscfg_min_operating_temp: f64,         // HVESSCFG recommended minimum operating temp
    pub hvesscfg_cell_max_voltage: f64,           // HVESSCFG cell maximum voltage limit (V)
    pub hvesscfg_cell_min_voltage: f64,           // HVESSCFG cell minimum voltage limit (V)
    pub hvesscfg_num_packs: u8,                   // HVESSCFG number of HVESP packs configured
    pub hvesscfg_nominal_capacity: f64,           // HVESSCFG nominal rated capacity (Ah)

    // HVESSCP1C Coolant Pump 1 Command States
    pub hvesscp1c_enable_command: u8,             // HVESSCP1C pump 1 enable command (0-3)
    pub hvesscp1c_power_hold: u8,                 // HVESSCP1C pump 1 power hold (0-3)
    pub hvesscp1c_speed_command: f64,             // HVESSCP1C pump 1 speed command (rpm)
    pub hvesscp1c_percent_speed_command: f64,     // HVESSCP1C pump 1 percent speed command (%)

    // HVESSCP1S1 Coolant Pump 1 Status 1 States
    pub hvesscp1s1_motor_speed_status: u8,        // HVESSCP1S1 motor speed status (0-3)
    pub hvesscp1s1_controller_status_reason: u8,  // HVESSCP1S1 controller status reason (0-15)
    pub hvesscp1s1_controller_command_status: u8, // HVESSCP1S1 controller command status (0-3)
    pub hvesscp1s1_motor_speed: f64,              // HVESSCP1S1 motor speed (rpm)
    pub hvesscp1s1_control_temperature: f64,      // HVESSCP1S1 control temperature (C)
    pub hvesscp1s1_power: f64,                    // HVESSCP1S1 power (W)
    pub hvesscp1s1_service_indicator: u8,         // HVESSCP1S1 service indicator (0-3)
    pub hvesscp1s1_operating_status: u8,          // HVESSCP1S1 operating status (0-3)

    // HVESSCP1S2 Coolant Pump 1 Status 2 States
    pub hvesscp1s2_voltage: f64,                  // HVESSCP1S2 pump 1 voltage (V)
    pub hvesscp1s2_current: f64,                  // HVESSCP1S2 pump 1 current (A)
    pub hvesscp1s2_hvil_status: u8,               // HVESSCP1S2 pump 1 HVIL status (0-3)
    pub hvesscp1s2_percent_speed_status: u8,      // HVESSCP1S2 percent speed status (0-3)
    pub hvesscp1s2_percent_speed: f64,            // HVESSCP1S2 percent speed (%)

    // HVESSCP2C Coolant Pump 2 Command States
    pub hvesscp2c_enable_command: u8,
    pub hvesscp2c_power_hold: u8,
    pub hvesscp2c_speed_command: f64,
    pub hvesscp2c_percent_speed_command: f64,

    // HVESSCP2S1 Coolant Pump 2 Status 1 States
    pub hvesscp2s1_motor_speed_status: u8,
    pub hvesscp2s1_controller_status_reason: u8,
    pub hvesscp2s1_controller_command_status: u8,
    pub hvesscp2s1_motor_speed: f64,
    pub hvesscp2s1_control_temperature: f64,
    pub hvesscp2s1_power: f64,
    pub hvesscp2s1_service_indicator: u8,
    pub hvesscp2s1_operating_status: u8,

    // HVESSCP2S2 Coolant Pump 2 Status 2 States
    pub hvesscp2s2_voltage: f64,
    pub hvesscp2s2_current: f64,
    pub hvesscp2s2_hvil_status: u8,
    pub hvesscp2s2_percent_speed_status: u8,
    pub hvesscp2s2_percent_speed: f64,

    // HVESSTCH1 Thermal Channel 1 States
    pub hvesstch1_compressor_discharge_abs_pressure: u16,
    pub hvesstch1_compressor_suction_abs_pressure: u16,
    pub hvesstch1_outlet_coolant_temp: f64,
    pub hvesstch1_condenser_valve_position: f64,

    // HVESSTCH2 Thermal Channel 2 States
    pub hvesstch2_compressor_discharge_abs_pressure: u16,
    pub hvesstch2_compressor_suction_abs_pressure: u16,
    pub hvesstch2_outlet_coolant_temp: f64,
    pub hvesstch2_condenser_valve_position: f64,

    // HVESSTCH3 Thermal Channel 3 States
    pub hvesstch3_compressor_discharge_abs_pressure: u16,
    pub hvesstch3_compressor_suction_abs_pressure: u16,
    pub hvesstch3_outlet_coolant_temp: f64,
    pub hvesstch3_condenser_valve_position: f64,

    // HVESSHIST History/Lifetime States
    pub hvesshist_state_of_health: f64,           // HVESSHIST state of health (0-100%)
    pub hvesshist_contactor_open_under_load: u16, // HVESSHIST contactor open under load count
    pub hvesshist_total_energy_throughput: f64,    // HVESSHIST total energy throughput (kWh)
    pub hvesshist_total_accumulated_charge: f64,   // HVESSHIST total accumulated charge (Ah)
    pub hvesshist_lifetime_energy_input: u32,      // HVESSHIST total lifetime energy input (kWh)
    pub hvesshist_lifetime_energy_output: u32,     // HVESSHIST total lifetime energy output (kWh)
}

impl Default for HvessState {
    fn default() -> Self {
        Self {
            hvess_power_down_command: false,
            hvess_cell_balancing_command: false,
            hvess_discharge_power: 50.0,
            hvess_charge_power: 50.0,
            hvess_voltage_level: 800.0,
            hvess_current_level: 0.0,
            hvess_bus_voltage: 800.0,
            hvess_ignition_voltage: 12.0,
            hvess_coolant_temp: 25.0,
            hvess_electronics_temp: 35.0,
            hvess_fast_update_state_of_charge: 75.5,
            hvess_highest_cell_voltage: 4.2,
            hvess_lowest_cell_voltage: 4.0,
            hvess_cell_voltage_differential_status: 2,
            hvess_highest_cell_temperature: 45.0,
            hvess_lowest_cell_temperature: 35.0,
            hvess_average_cell_temperature: 40.0,
            hvess_cell_temp_differential_status: 1,
            hvess_fan_speed_status: 1,
            hvess_fan_status_reason_code: 0,
            hvess_fan_command_status: 1,
            hvess_fan_speed: 2400.0,
            hvess_fan_medium_temperature: 55.0,
            hvess_fan_power: 150.0,
            hvess_fan_service_indicator: 0,
            hvess_fan_operating_status: 1,
            hvess_fan_status1_instance: 1,

            // HVESSD4 Capacity & Cell Balancing defaults
            hvessd4_discharge_capacity: 200.0,     // 200 Ah available
            hvessd4_charge_capacity: 100.0,        // 100 Ah remaining capacity
            hvessd4_cell_balancing_count: 0,       // No cells being balanced

            // HVESSD5 Current Limits & Cell SOC defaults
            hvessd5_max_discharge_current_limit: -200.0, // -200A discharge limit
            hvessd5_max_charge_current_limit: 150.0,     // 150A charge limit
            hvessd5_min_cell_soc: 72.0,                  // 72% minimum cell SOC
            hvessd5_max_cell_soc: 78.0,                  // 78% maximum cell SOC

            // HVESSD7 Energy Capacity defaults
            hvessd7_discharge_energy_capacity: 40.0,     // 40 kWh discharge energy
            hvessd7_charge_energy_capacity: 20.0,        // 20 kWh charge energy
            hvessd7_max_charge_voltage_limit: 850.0,     // 850V charge limit

            // HVESSD8 Cell Voltage Location defaults
            hvessd8_highest_cell_voltage_module: 3,
            hvessd8_highest_cell_voltage_cell: 12,
            hvessd8_lowest_cell_voltage_module: 7,
            hvessd8_lowest_cell_voltage_cell: 5,
            hvessd8_average_cell_voltage: 4.1,           // 4.1V average

            // HVESSD9 Cell Temperature Location defaults
            hvessd9_highest_cell_temp_module: 2,
            hvessd9_highest_cell_temp_cell: 8,
            hvessd9_lowest_cell_temp_module: 10,
            hvessd9_lowest_cell_temp_cell: 3,
            hvessd9_thermal_event_detected: 0,           // No thermal event
            hvessd9_counter: 0,

            // HVESSD10 Cell SOC Location & Isolation defaults
            hvessd10_highest_cell_soc_module: 1,
            hvessd10_highest_cell_soc_cell: 6,
            hvessd10_lowest_cell_soc_module: 9,
            hvessd10_lowest_cell_soc_cell: 11,
            hvessd10_active_isolation_test: 500.0,       // 500 kOhm (good isolation)
            hvessd10_passive_isolation_test: 450.0,      // 450 kOhm (good isolation)

            // HVESSD11 Voltage & Energy defaults
            hvessd11_bus_voltage_neg_to_chassis: 400.0,
            hvessd11_voltage_neg_to_chassis: 400.0,
            hvessd11_actual_charge_rate: 0.0,
            hvessd11_total_stored_energy: 60.0,          // 60 kWh
            hvessd11_power_module_electronics_temp: 45.0,

            // HVESSD12 Coolant & Time Remaining defaults
            hvessd12_intake_coolant_pressure: 150.0,     // 150 kPa
            hvessd12_estimated_discharge_time: 120.0,    // 120 minutes
            hvessd12_estimated_charge_time: 60.0,        // 60 minutes
            hvessd12_hv_exposure_indicator: 0,           // No exposure
            hvessd12_power_hold_relay_status: 1,         // Closed
            hvessd12_positive_precharge_relay: 0,        // Open
            hvessd12_negative_precharge_relay: 0,        // Open

            // HVESSD13 Extended Range defaults
            hvessd13_discharge_power_extended: 50.0,
            hvessd13_charge_power_extended: 50.0,
            hvessd13_voltage_extended: 800.0,
            hvessd13_current_extended: 0.0,

            // HVESSD14 Extended Range Current & Voltage defaults
            hvessd14_max_discharge_current_extended: -200.0,
            hvessd14_max_charge_current_extended: 150.0,
            hvessd14_bus_voltage_extended: 800.0,
            hvessd14_min_discharge_voltage_limit: 650.0,

            // HVESSD15 Nominal Current Limits defaults
            hvessd15_nominal_discharge_current_limit: -150.0,
            hvessd15_nominal_charge_current_limit: 100.0,

            // HVESSIS1 Internal Segment 1 defaults
            hvessis1_internal_voltage_1: 400.0,
            hvessis1_internal_current_1: 0.0,
            hvessis1_internal_voltage_2: 400.0,
            hvessis1_internal_current_2: 0.0,

            // HVESSIS2 Internal Segment 2 defaults
            hvessis2_internal_voltage_3: 400.0,
            hvessis2_internal_current_3: 0.0,
            hvessis2_internal_voltage_4: 400.0,
            hvessis2_internal_current_4: 0.0,

            // HVESSIS3 Internal Segment 3 defaults
            hvessis3_internal_voltage_5: 400.0,
            hvessis3_internal_current_5: 0.0,
            hvessis3_internal_voltage_6: 400.0,
            hvessis3_internal_current_6: 0.0,

            // HVESSIS4 Internal Segment 4 defaults
            hvessis4_internal_voltage_7: 400.0,
            hvessis4_internal_current_7: 0.0,
            hvessis4_internal_voltage_8: 400.0,
            hvessis4_internal_current_8: 0.0,

            // HVESSIS5 Internal Segment 5 defaults (contactors/relays closed, normal)
            hvessis5_positive_contactor_1_state: 1,  // Closed
            hvessis5_negative_contactor_1_state: 1,
            hvessis5_precharge_relay_1_state: 0,     // Open (not pre-charging)
            hvessis5_inline_heater_1_status: 0,      // Off
            hvessis5_bus_voltage_1: 400.0,
            hvessis5_positive_contactor_2_state: 1,
            hvessis5_negative_contactor_2_state: 1,
            hvessis5_precharge_relay_2_state: 0,
            hvessis5_inline_heater_2_status: 0,
            hvessis5_bus_voltage_2: 400.0,

            // HVESSIS6 Internal Segment 6 defaults
            hvessis6_positive_contactor_3_state: 1,
            hvessis6_negative_contactor_3_state: 1,
            hvessis6_precharge_relay_3_state: 0,
            hvessis6_inline_heater_3_status: 0,
            hvessis6_bus_voltage_3: 400.0,
            hvessis6_positive_contactor_4_state: 1,
            hvessis6_negative_contactor_4_state: 1,
            hvessis6_precharge_relay_4_state: 0,
            hvessis6_inline_heater_4_status: 0,
            hvessis6_bus_voltage_4: 400.0,

            // HVESSIS7 defaults
            hvessis7_number_of_internal_circuits: 4,

            // HVESSMS1 Module Status defaults (all modules operational = 1)
            hvessms1_module_status: [1; 32],
            hvessms2_module_status: [1; 32],
            hvessms3_module_status: [1; 32],

            // HVESSS1 System Status 1 defaults
            hvesss1_positive_contactor_state: 1,     // Closed
            hvesss1_negative_contactor_state: 1,     // Closed
            hvesss1_disconnect_forewarning: 0,       // No warning
            hvesss1_precharge_relay_state: 0,        // Open
            hvesss1_center_of_pack_contactor: 0,     // Open
            hvesss1_active_isolation_test_status: 1, // Pass
            hvesss1_passive_isolation_test_status: 1,// Pass
            hvesss1_hvil_status: 0,                  // OK
            hvesss1_inertia_switch_status: 0,        // Normal
            hvesss1_soc_status: 1,                   // Normal
            hvesss1_cell_balance_status: 0,          // Not balancing
            hvesss1_cell_balancing_active: 0,        // Inactive
            hvesss1_internal_charger_status: 0,      // Off
            hvesss1_counter: 0,
            hvesss1_bus_connection_status: 1,        // Connected
            hvesss1_operational_status: 2,           // Active
            hvesss1_num_packs_ready: 1,              // 1 pack ready
            hvesss1_crc: 0,

            // HVESSS2 System Status 2 defaults (no derating)
            hvesss2_discharge_limit_soc: 0,
            hvesss2_discharge_limit_temp: 0,
            hvesss2_discharge_limit_diag: 0,
            hvesss2_discharge_limit_voltage: 0,
            hvesss2_discharge_limit_current: 0,
            hvesss2_discharge_limit_undefined: 0,
            hvesss2_discharge_limit_electronics_temp: 0,
            hvesss2_charge_limit_soc: 0,
            hvesss2_charge_limit_temp: 0,
            hvesss2_charge_limit_diag: 0,
            hvesss2_charge_limit_voltage: 0,
            hvesss2_charge_limit_current: 0,
            hvesss2_charge_limit_undefined: 0,
            hvesss2_charge_limit_electronics_temp: 0,

            // HVESSFS2 Fan Status 2 defaults
            hvessfs2_fan_voltage: 12.0,
            hvessfs2_fan_current: 5.0,
            hvessfs2_fan_hvil_status: 0,              // OK
            hvessfs2_fan_status_2_instance: 1,
            hvessfs2_fan_percent_speed_status: 1,     // Normal
            hvessfs2_fan_percent_speed: 60.0,         // 60% speed

            // HVESSFC Fan Command defaults
            hvessfc_fan_enable_command: 1,            // Enabled
            hvessfc_fan_power_hold: 0,                // No hold
            hvessfc_fan_speed_command: 2400.0,        // 2400 rpm
            hvessfc_fan_percent_speed_command: 60.0,  // 60%

            // HVESSCFG Configuration defaults
            hvesscfg_nominal_voltage: 800.0,
            hvesscfg_min_operating_voltage: 650.0,
            hvesscfg_max_operating_voltage: 850.0,
            hvesscfg_min_soc: 10.0,                   // 10% minimum SOC
            hvesscfg_max_soc: 95.0,                   // 95% maximum SOC
            hvesscfg_max_operating_temp: 55.0,        // 55C max
            hvesscfg_min_operating_temp: -20.0,       // -20C min
            hvesscfg_cell_max_voltage: 4.2,           // 4.2V max
            hvesscfg_cell_min_voltage: 2.5,           // 2.5V min
            hvesscfg_num_packs: 1,                    // 1 pack
            hvesscfg_nominal_capacity: 300.0,         // 300 Ah

            // HVESSCP1C Coolant Pump 1 Command defaults
            hvesscp1c_enable_command: 1,              // Enabled
            hvesscp1c_power_hold: 0,
            hvesscp1c_speed_command: 2000.0,          // 2000 rpm
            hvesscp1c_percent_speed_command: 65.0,    // 65%

            // HVESSCP1S1 Coolant Pump 1 Status 1 defaults
            hvesscp1s1_motor_speed_status: 1,         // Normal
            hvesscp1s1_controller_status_reason: 0,   // No issue
            hvesscp1s1_controller_command_status: 1,  // Active
            hvesscp1s1_motor_speed: 2000.0,           // 2000 rpm
            hvesscp1s1_control_temperature: 40.0,     // 40C
            hvesscp1s1_power: 120.0,                  // 120W
            hvesscp1s1_service_indicator: 0,          // No service
            hvesscp1s1_operating_status: 1,           // Normal

            // HVESSCP1S2 Coolant Pump 1 Status 2 defaults
            hvesscp1s2_voltage: 12.0,
            hvesscp1s2_current: 10.0,
            hvesscp1s2_hvil_status: 0,                // OK
            hvesscp1s2_percent_speed_status: 1,       // Normal
            hvesscp1s2_percent_speed: 65.0,           // 65%

            // HVESSCP2C Coolant Pump 2 Command defaults
            hvesscp2c_enable_command: 1,
            hvesscp2c_power_hold: 0,
            hvesscp2c_speed_command: 1800.0,
            hvesscp2c_percent_speed_command: 55.0,

            // HVESSCP2S1 Coolant Pump 2 Status 1 defaults
            hvesscp2s1_motor_speed_status: 1,
            hvesscp2s1_controller_status_reason: 0,
            hvesscp2s1_controller_command_status: 1,
            hvesscp2s1_motor_speed: 1800.0,
            hvesscp2s1_control_temperature: 38.0,
            hvesscp2s1_power: 100.0,
            hvesscp2s1_service_indicator: 0,
            hvesscp2s1_operating_status: 1,

            // HVESSCP2S2 Coolant Pump 2 Status 2 defaults
            hvesscp2s2_voltage: 12.0,
            hvesscp2s2_current: 8.0,
            hvesscp2s2_hvil_status: 0,
            hvesscp2s2_percent_speed_status: 1,
            hvesscp2s2_percent_speed: 55.0,

            // HVESSTCH1 Thermal Channel 1 defaults
            hvesstch1_compressor_discharge_abs_pressure: 1200,
            hvesstch1_compressor_suction_abs_pressure: 400,
            hvesstch1_outlet_coolant_temp: 25.0,
            hvesstch1_condenser_valve_position: 50.0,

            // HVESSTCH2 Thermal Channel 2 defaults
            hvesstch2_compressor_discharge_abs_pressure: 1100,
            hvesstch2_compressor_suction_abs_pressure: 380,
            hvesstch2_outlet_coolant_temp: 26.0,
            hvesstch2_condenser_valve_position: 48.0,

            // HVESSTCH3 Thermal Channel 3 defaults
            hvesstch3_compressor_discharge_abs_pressure: 1000,
            hvesstch3_compressor_suction_abs_pressure: 350,
            hvesstch3_outlet_coolant_temp: 27.0,
            hvesstch3_condenser_valve_position: 45.0,

            // HVESSHIST History/Lifetime defaults
            hvesshist_state_of_health: 95.0,          // 95% SOH
            hvesshist_contactor_open_under_load: 2,   // 2 events
            hvesshist_total_energy_throughput: 5000.0, // 5000 kWh
            hvesshist_total_accumulated_charge: 6000.0, // 6000 Ah
            hvesshist_lifetime_energy_input: 250000,   // 250000 kWh
            hvesshist_lifetime_energy_output: 240000,  // 240000 kWh
        }
    }
}
