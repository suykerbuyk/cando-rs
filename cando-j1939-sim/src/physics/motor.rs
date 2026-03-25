use crate::SimulatorState;

impl SimulatorState {
    pub(crate) fn update_motor_physics(&mut self, delta_time: f64) {
        // EMP Motor 1 Physics - Realistic motor response
        let mg1_speed_diff = self.motor.mg1_speed_setpoint - self.motor.mg1_actual_speed;
        if mg1_speed_diff.abs() > 0.1 {
            let step = mg1_speed_diff.clamp(-20.0 * delta_time, 20.0 * delta_time); // 20%/sec ramp rate
            self.motor.mg1_actual_speed += step;
        } else {
            self.motor.mg1_actual_speed = self.motor.mg1_speed_setpoint;
        }
        self.motor.mg1_actual_speed = self.motor.mg1_actual_speed.clamp(-125.0, 125.0);

        // Motor torque follows speed with load simulation
        let load_factor = 0.3; // 30% base load
        self.motor.mg1_actual_torque =
            self.motor.mg1_actual_speed * load_factor + (self.motor.mg1_torque_setpoint * 0.1);
        self.motor.mg1_actual_torque = self.motor.mg1_actual_torque.clamp(-125.0, 125.0);

        // Motor current and voltage based on torque (P = I*V)
        self.motor.mg1_current = (self.motor.mg1_actual_torque.abs() * 0.8).clamp(0.0, 100.0);
        self.motor.mg1_voltage = 48.0 - (self.motor.mg1_current * 0.1); // Voltage drop under load

        // EMP Motor 2 Physics - Similar to Motor 1 but with coordination
        let mg2_speed_diff = self.motor.mg2_speed_setpoint - self.motor.mg2_actual_speed;
        if mg2_speed_diff.abs() > 0.1 {
            let step = mg2_speed_diff.clamp(-20.0 * delta_time, 20.0 * delta_time);
            self.motor.mg2_actual_speed += step;
        } else {
            self.motor.mg2_actual_speed = self.motor.mg2_speed_setpoint;
        }
        self.motor.mg2_actual_speed = self.motor.mg2_actual_speed.clamp(-125.0, 125.0);

        self.motor.mg2_actual_torque =
            self.motor.mg2_actual_speed * 0.25 + (self.motor.mg2_torque_setpoint * 0.1);
        self.motor.mg2_actual_torque = self.motor.mg2_actual_torque.clamp(-125.0, 125.0);
        self.motor.mg2_current = (self.motor.mg2_actual_torque.abs() * 0.8).clamp(0.0, 100.0);
        self.motor.mg2_voltage = 48.0 - (self.motor.mg2_current * 0.1);

        // ============================================================================
        // Batch 8: Extended Motor/Generator Physics
        // ============================================================================

        let ambient_temp = 25.0;
        let load_factor = 0.3; // 30% base load

        // MG1/MG2 Temperature Physics - temps rise with current, cool toward ambient
        let mg1_heat = self.motor.mg1_current.abs() * 0.15 * delta_time;
        let mg1_cool = (self.motor.mg1_inverter_temp1 - ambient_temp) * 0.02 * delta_time;
        self.motor.mg1_inverter_temp1 = (self.motor.mg1_inverter_temp1 + mg1_heat - mg1_cool).clamp(-40.0, 210.0);
        self.motor.mg1_inverter_temp2 = (self.motor.mg1_inverter_temp1 - 1.0).clamp(-40.0, 210.0);
        self.motor.mg1_inverter_temp3 = (self.motor.mg1_inverter_temp1 - 2.0).clamp(-40.0, 210.0);
        self.motor.mg1_inverter_temp4 = (self.motor.mg1_inverter_temp1 - 3.0).clamp(-40.0, 210.0);
        self.motor.mg1_inverter_temp5 = (self.motor.mg1_inverter_temp1 - 4.0).clamp(-40.0, 210.0);

        let mg2_heat = self.motor.mg2_current.abs() * 0.15 * delta_time;
        let mg2_cool = (self.motor.mg2_inverter_temp1 - ambient_temp) * 0.02 * delta_time;
        self.motor.mg2_inverter_temp1 = (self.motor.mg2_inverter_temp1 + mg2_heat - mg2_cool).clamp(-40.0, 210.0);
        self.motor.mg2_inverter_temp2 = (self.motor.mg2_inverter_temp1 - 1.0).clamp(-40.0, 210.0);
        self.motor.mg2_inverter_temp3 = (self.motor.mg2_inverter_temp1 - 2.0).clamp(-40.0, 210.0);
        self.motor.mg2_inverter_temp4 = (self.motor.mg2_inverter_temp1 - 3.0).clamp(-40.0, 210.0);
        self.motor.mg2_inverter_temp5 = (self.motor.mg2_inverter_temp1 - 4.0).clamp(-40.0, 210.0);

        // MG1/MG2 Motor Angle Physics - incrementing rotor angle (resolver)
        let mg1_angle_rate = self.motor.mg1_actual_speed.abs() * 3.6;
        self.motor.mg1_motor_angle = (self.motor.mg1_motor_angle + mg1_angle_rate * delta_time) % 360.0;
        let mg2_angle_rate = self.motor.mg2_actual_speed.abs() * 3.6;
        self.motor.mg2_motor_angle = (self.motor.mg2_motor_angle + mg2_angle_rate * delta_time) % 360.0;

        // MG1/MG2 Power limits derate when temperature high
        let mg1_temp_derate = if self.motor.mg1_inverter_temp1 > 150.0 {
            1.0 - ((self.motor.mg1_inverter_temp1 - 150.0) / 60.0).clamp(0.0, 0.5)
        } else {
            1.0
        };
        self.motor.mg1_power_limit_mech_max = 100.0 * mg1_temp_derate;
        self.motor.mg1_power_limit_mech_min = -100.0 * mg1_temp_derate;
        self.motor.mg1_power_limit_dc_max = 100.0 * mg1_temp_derate;
        self.motor.mg1_power_limit_dc_min = -100.0 * mg1_temp_derate;
        self.motor.mg1_max_torque = (100.0 * mg1_temp_derate).clamp(-125.0, 125.0);
        self.motor.mg1_min_torque = (-100.0 * mg1_temp_derate).clamp(-125.0, 125.0);

        let mg2_temp_derate = if self.motor.mg2_inverter_temp1 > 150.0 {
            1.0 - ((self.motor.mg2_inverter_temp1 - 150.0) / 60.0).clamp(0.0, 0.5)
        } else {
            1.0
        };
        self.motor.mg2_power_limit_mech_max = 100.0 * mg2_temp_derate;
        self.motor.mg2_power_limit_mech_min = -100.0 * mg2_temp_derate;
        self.motor.mg2_power_limit_dc_max = 100.0 * mg2_temp_derate;
        self.motor.mg2_power_limit_dc_min = -100.0 * mg2_temp_derate;
        self.motor.mg2_max_torque = (100.0 * mg2_temp_derate).clamp(-125.0, 125.0);
        self.motor.mg2_min_torque = (-100.0 * mg2_temp_derate).clamp(-125.0, 125.0);

        // MG3 Physics - reuses MG1 physics model (speed ramp, torque with load)
        let mg3_speed_diff = self.motor.mg3_speed_setpoint - self.motor.mg3_actual_speed;
        if mg3_speed_diff.abs() > 0.1 {
            let step = mg3_speed_diff.clamp(-20.0 * delta_time, 20.0 * delta_time);
            self.motor.mg3_actual_speed += step;
        } else {
            self.motor.mg3_actual_speed = self.motor.mg3_speed_setpoint;
        }
        self.motor.mg3_actual_speed = self.motor.mg3_actual_speed.clamp(-125.0, 125.0);

        self.motor.mg3_actual_torque =
            self.motor.mg3_actual_speed * load_factor + (self.motor.mg3_torque_setpoint * 0.1);
        self.motor.mg3_actual_torque = self.motor.mg3_actual_torque.clamp(-125.0, 125.0);
        self.motor.mg3_current = (self.motor.mg3_actual_torque.abs() * 0.8).clamp(0.0, 100.0);
        self.motor.mg3_voltage = 48.0 - (self.motor.mg3_current * 0.1);

        // MG3 temperature physics
        let mg3_heat = self.motor.mg3_current.abs() * 0.15 * delta_time;
        let mg3_cool = (self.motor.mg3_inverter_temp1 - ambient_temp) * 0.02 * delta_time;
        self.motor.mg3_inverter_temp1 = (self.motor.mg3_inverter_temp1 + mg3_heat - mg3_cool).clamp(-40.0, 210.0);
        self.motor.mg3_inverter_temp2 = (self.motor.mg3_inverter_temp1 - 1.0).clamp(-40.0, 210.0);
        self.motor.mg3_inverter_temp3 = (self.motor.mg3_inverter_temp1 - 2.0).clamp(-40.0, 210.0);
        self.motor.mg3_inverter_temp4 = (self.motor.mg3_inverter_temp1 - 3.0).clamp(-40.0, 210.0);
        self.motor.mg3_inverter_temp5 = (self.motor.mg3_inverter_temp1 - 4.0).clamp(-40.0, 210.0);

        // MG3 motor angle (resolver)
        let mg3_angle_rate = self.motor.mg3_actual_speed.abs() * 3.6;
        self.motor.mg3_motor_angle = (self.motor.mg3_motor_angle + mg3_angle_rate * delta_time) % 360.0;

        // MG3 power limit derate with temperature
        let mg3_temp_derate = if self.motor.mg3_inverter_temp1 > 150.0 {
            1.0 - ((self.motor.mg3_inverter_temp1 - 150.0) / 60.0).clamp(0.0, 0.5)
        } else {
            1.0
        };
        self.motor.mg3_power_limit_mech_max = 100.0 * mg3_temp_derate;
        self.motor.mg3_power_limit_mech_min = -100.0 * mg3_temp_derate;
        self.motor.mg3_power_limit_dc_max = 100.0 * mg3_temp_derate;
        self.motor.mg3_power_limit_dc_min = -100.0 * mg3_temp_derate;
        self.motor.mg3_max_torque = (100.0 * mg3_temp_derate).clamp(-125.0, 125.0);
        self.motor.mg3_min_torque = (-100.0 * mg3_temp_derate).clamp(-125.0, 125.0);

        // MG4 Physics - reuses MG2 physics model
        let mg4_speed_diff = self.motor.mg4_speed_setpoint - self.motor.mg4_actual_speed;
        if mg4_speed_diff.abs() > 0.1 {
            let step = mg4_speed_diff.clamp(-20.0 * delta_time, 20.0 * delta_time);
            self.motor.mg4_actual_speed += step;
        } else {
            self.motor.mg4_actual_speed = self.motor.mg4_speed_setpoint;
        }
        self.motor.mg4_actual_speed = self.motor.mg4_actual_speed.clamp(-125.0, 125.0);

        self.motor.mg4_actual_torque = self.motor.mg4_actual_speed * 0.25 + (self.motor.mg4_torque_setpoint * 0.1);
        self.motor.mg4_actual_torque = self.motor.mg4_actual_torque.clamp(-125.0, 125.0);
        self.motor.mg4_current = (self.motor.mg4_actual_torque.abs() * 0.8).clamp(0.0, 100.0);
        self.motor.mg4_voltage = 48.0 - (self.motor.mg4_current * 0.1);

        self.motor.mg4_max_torque = 100.0_f64.clamp(-125.0, 125.0);
        self.motor.mg4_min_torque = (-100.0_f64).clamp(-125.0, 125.0);

    }
}
