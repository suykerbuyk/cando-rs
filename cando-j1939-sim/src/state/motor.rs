use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorState {
    // Motor/Generator 1
    pub mg1_speed_setpoint: f64,
    pub mg1_actual_speed: f64,
    pub mg1_torque_setpoint: f64,
    pub mg1_actual_torque: f64,
    pub mg1_current: f64,
    pub mg1_voltage: f64,
    pub mg1_max_torque: f64,
    pub mg1_min_torque: f64,
    pub mg1_control_counter: u64,
    pub mg1_status_counter: u64,

    // Motor/Generator 2
    pub mg2_speed_setpoint: f64,
    pub mg2_actual_speed: f64,
    pub mg2_torque_setpoint: f64,
    pub mg2_actual_torque: f64,
    pub mg2_current: f64,
    pub mg2_voltage: f64,
    pub mg2_control_counter: u64,
    pub mg2_status_counter: u64,

    // Motor/Generator 1 Extended States (Batch 8)
    pub mg1_motor_angle: f64,           // MG1IS3 motor shaft angle (0-359.99 deg)
    pub mg1_inverter_temp1: f64,        // MG1IT inverter temperature 1 (degC)
    pub mg1_inverter_temp2: f64,        // MG1IT inverter temperature 2 (degC)
    pub mg1_inverter_temp3: f64,        // MG1IT inverter temperature 3 (degC)
    pub mg1_inverter_temp4: f64,        // MG1IT inverter temperature 4 (degC)
    pub mg1_inverter_temp5: f64,        // MG1IT inverter temperature 5 (degC)
    pub mg1_isolation_neg_voltage: f64, // MG1II DC side negative to chassis ground voltage
    pub mg1_ref_torque: f64,            // MG1IR1 reference torque (Nm)
    pub mg1_ref_speed: f64,             // MG1IR1 reference speed (rpm)
    pub mg1_ref_power: f64,             // MG1IR1 reference power (kW)
    pub mg1_ref_current: f64,           // MG1IR2 reference current (A)
    pub mg1_ref_voltage: f64,           // MG1IR2 reference voltage (V)
    pub mg1_power_limit_mech_max: f64,  // MG1IRP mechanical power max
    pub mg1_power_limit_mech_min: f64,  // MG1IRP mechanical power min
    pub mg1_power_limit_dc_max: f64,    // MG1IRP DC side power max
    pub mg1_power_limit_dc_min: f64,    // MG1IRP DC side power min

    // Motor/Generator 2 Extended States (Batch 8)
    pub mg2_motor_angle: f64,           // MG2IS3 motor shaft angle (0-359.99 deg)
    pub mg2_max_torque: f64,            // MG2IS2 available maximum torque
    pub mg2_min_torque: f64,            // MG2IS2 available minimum torque
    pub mg2_inverter_temp1: f64,        // MG2IT inverter temperature 1 (degC)
    pub mg2_inverter_temp2: f64,        // MG2IT inverter temperature 2 (degC)
    pub mg2_inverter_temp3: f64,        // MG2IT inverter temperature 3 (degC)
    pub mg2_inverter_temp4: f64,        // MG2IT inverter temperature 4 (degC)
    pub mg2_inverter_temp5: f64,        // MG2IT inverter temperature 5 (degC)
    pub mg2_isolation_neg_voltage: f64, // MG2II DC side negative to chassis ground voltage
    pub mg2_ref_torque: f64,            // MG2IR1 reference torque (Nm)
    pub mg2_ref_speed: f64,             // MG2IR1 reference speed (rpm)
    pub mg2_ref_power: f64,             // MG2IR1 reference power (kW)
    pub mg2_ref_current: f64,           // MG2IR2 reference current (A)
    pub mg2_ref_voltage: f64,           // MG2IR2 reference voltage (V)
    pub mg2_power_limit_mech_max: f64,  // MG2IRP mechanical power max
    pub mg2_power_limit_mech_min: f64,  // MG2IRP mechanical power min
    pub mg2_power_limit_dc_max: f64,    // MG2IRP DC side power max
    pub mg2_power_limit_dc_min: f64,    // MG2IRP DC side power min

    // Motor/Generator 3 (Tertiary Motor) Control and Status (Batch 8)
    pub mg3_speed_setpoint: f64,        // MG3IC control setpoint (-125 to 125%)
    pub mg3_actual_speed: f64,          // MG3IS1 actual speed feedback
    pub mg3_torque_setpoint: f64,       // MG3IC torque control
    pub mg3_actual_torque: f64,         // MG3IS1 actual torque feedback
    pub mg3_current: f64,               // MG3IS1 D-side current
    pub mg3_voltage: f64,               // MG3IS1 D-side voltage
    pub mg3_max_torque: f64,            // MG3IS2 available maximum torque
    pub mg3_min_torque: f64,            // MG3IS2 available minimum torque
    pub mg3_control_counter: u64,       // MG3IC counter
    pub mg3_status_counter: u64,        // MG3IS1/MG3IS2 counter
    pub mg3_motor_angle: f64,           // MG3IS3 motor shaft angle (0-359.99 deg)
    pub mg3_inverter_temp1: f64,        // MG3IT inverter temperature 1 (degC)
    pub mg3_inverter_temp2: f64,        // MG3IT inverter temperature 2 (degC)
    pub mg3_inverter_temp3: f64,        // MG3IT inverter temperature 3 (degC)
    pub mg3_inverter_temp4: f64,        // MG3IT inverter temperature 4 (degC)
    pub mg3_inverter_temp5: f64,        // MG3IT inverter temperature 5 (degC)
    pub mg3_isolation_neg_voltage: f64, // MG3II DC side negative to chassis ground voltage
    pub mg3_ref_torque: f64,            // MG3IR1 reference torque (Nm)
    pub mg3_ref_speed: f64,             // MG3IR1 reference speed (rpm)
    pub mg3_ref_power: f64,             // MG3IR1 reference power (kW)
    pub mg3_ref_current: f64,           // MG3IR2 reference current (A)
    pub mg3_ref_voltage: f64,           // MG3IR2 reference voltage (V)
    pub mg3_power_limit_mech_max: f64,  // MG3IRP mechanical power max
    pub mg3_power_limit_mech_min: f64,  // MG3IRP mechanical power min
    pub mg3_power_limit_dc_max: f64,    // MG3IRP DC side power max
    pub mg3_power_limit_dc_min: f64,    // MG3IRP DC side power min

    // Motor/Generator 4 (Quaternary Motor) Control and Status (Batch 8)
    pub mg4_speed_setpoint: f64,        // MG4IC control setpoint (-125 to 125%)
    pub mg4_actual_speed: f64,          // MG4IS1 actual speed feedback
    pub mg4_torque_setpoint: f64,       // MG4IC torque control
    pub mg4_actual_torque: f64,         // MG4IS1 actual torque feedback
    pub mg4_current: f64,               // MG4IS1 D-side current
    pub mg4_voltage: f64,               // MG4IS1 D-side voltage
    pub mg4_max_torque: f64,            // MG4IS2 available maximum torque
    pub mg4_min_torque: f64,            // MG4IS2 available minimum torque
    pub mg4_control_counter: u64,       // MG4IC counter
    pub mg4_status_counter: u64,        // MG4IS1/MG4IS2 counter
}

impl Default for MotorState {
    fn default() -> Self {
        Self {
            mg1_speed_setpoint: 0.0,
            mg1_actual_speed: 0.0,
            mg1_torque_setpoint: 0.0,
            mg1_actual_torque: 0.0,
            mg1_current: 0.0,
            mg1_voltage: 48.0,
            mg1_max_torque: 100.0,
            mg1_min_torque: -100.0,
            mg1_control_counter: 0,
            mg1_status_counter: 0,
            mg2_speed_setpoint: 0.0,
            mg2_actual_speed: 0.0,
            mg2_torque_setpoint: 0.0,
            mg2_actual_torque: 0.0,
            mg2_current: 0.0,
            mg2_voltage: 48.0,
            mg2_control_counter: 0,
            mg2_status_counter: 0,

            // MG1 extended defaults (Batch 8)
            mg1_motor_angle: 0.0,
            mg1_inverter_temp1: 35.0,
            mg1_inverter_temp2: 34.0,
            mg1_inverter_temp3: 33.0,
            mg1_inverter_temp4: 32.0,
            mg1_inverter_temp5: 31.0,
            mg1_isolation_neg_voltage: 0.0,
            mg1_ref_torque: 400.0,
            mg1_ref_speed: 8192.0,
            mg1_ref_power: 100.0,
            mg1_ref_current: 200.0,
            mg1_ref_voltage: 400.0,
            mg1_power_limit_mech_max: 100.0,
            mg1_power_limit_mech_min: -100.0,
            mg1_power_limit_dc_max: 100.0,
            mg1_power_limit_dc_min: -100.0,

            // MG2 extended defaults (Batch 8)
            mg2_motor_angle: 0.0,
            mg2_max_torque: 100.0,
            mg2_min_torque: -100.0,
            mg2_inverter_temp1: 35.0,
            mg2_inverter_temp2: 34.0,
            mg2_inverter_temp3: 33.0,
            mg2_inverter_temp4: 32.0,
            mg2_inverter_temp5: 31.0,
            mg2_isolation_neg_voltage: 0.0,
            mg2_ref_torque: 400.0,
            mg2_ref_speed: 8192.0,
            mg2_ref_power: 100.0,
            mg2_ref_current: 200.0,
            mg2_ref_voltage: 400.0,
            mg2_power_limit_mech_max: 100.0,
            mg2_power_limit_mech_min: -100.0,
            mg2_power_limit_dc_max: 100.0,
            mg2_power_limit_dc_min: -100.0,

            // MG3 defaults (Batch 8)
            mg3_speed_setpoint: 0.0,
            mg3_actual_speed: 0.0,
            mg3_torque_setpoint: 0.0,
            mg3_actual_torque: 0.0,
            mg3_current: 0.0,
            mg3_voltage: 48.0,
            mg3_max_torque: 100.0,
            mg3_min_torque: -100.0,
            mg3_control_counter: 0,
            mg3_status_counter: 0,
            mg3_motor_angle: 0.0,
            mg3_inverter_temp1: 35.0,
            mg3_inverter_temp2: 34.0,
            mg3_inverter_temp3: 33.0,
            mg3_inverter_temp4: 32.0,
            mg3_inverter_temp5: 31.0,
            mg3_isolation_neg_voltage: 0.0,
            mg3_ref_torque: 400.0,
            mg3_ref_speed: 8192.0,
            mg3_ref_power: 100.0,
            mg3_ref_current: 200.0,
            mg3_ref_voltage: 400.0,
            mg3_power_limit_mech_max: 100.0,
            mg3_power_limit_mech_min: -100.0,
            mg3_power_limit_dc_max: 100.0,
            mg3_power_limit_dc_min: -100.0,

            // MG4 defaults (Batch 8)
            mg4_speed_setpoint: 0.0,
            mg4_actual_speed: 0.0,
            mg4_torque_setpoint: 0.0,
            mg4_actual_torque: 0.0,
            mg4_current: 0.0,
            mg4_voltage: 48.0,
            mg4_max_torque: 100.0,
            mg4_min_torque: -100.0,
            mg4_control_counter: 0,
            mg4_status_counter: 0,
        }
    }
}
