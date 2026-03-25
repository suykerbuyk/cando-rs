mod dcdc;
mod engine;
mod ev_charging;
mod hvess;
mod motor;
mod transmission;
mod vehicle;

use chrono::Utc;

use crate::SimulatorState;

impl SimulatorState {
    pub fn update_physics(&mut self, delta_time: f64) {
        self.update_sensor_physics(delta_time);
        self.update_sensor_physics_batch3(delta_time);
        self.update_motor_physics(delta_time);
        self.update_hvess_physics(delta_time);
        self.update_dcdc_physics(delta_time);
        self.update_engine_physics(delta_time);
        self.update_transmission_physics(delta_time);
        self.update_braking_physics(delta_time);
        self.update_vehicle_physics(delta_time);
        self.update_ev_charging_physics(delta_time);
        self.update_counters();

        // Update crash checksum if crash detected
        if self.crash.crash_detected {
            self.crash.crash_checksum = self.calculate_checksum();
        }

        self.last_update_time = Some(Utc::now());
    }

    /// Calculate checksum for crash notification
    fn calculate_checksum(&self) -> u8 {
        // Simple checksum: sum of type and counter modulo 16
        ((self.crash.crash_type as u64 + self.crash.crash_counter as u64) % 16) as u8
    }

    fn update_sensor_physics(&mut self, delta_time: f64) {
        // Gradually move wand angle to target (simulate physical movement)
        let angle_diff = self.sensors.target_wand_angle - self.sensors.wand_angle;
        if angle_diff.abs() > 0.01 {
            let step = angle_diff.clamp(-50.0 * delta_time, 50.0 * delta_time); // 50 deg/sec
            self.sensors.wand_angle += step;
        } else {
            self.sensors.wand_angle = self.sensors.target_wand_angle;
        }

        // Clamp wand angle to valid range
        self.sensors.wand_angle = self.sensors.wand_angle.clamp(-250.0, 252.19);

        // Gradually move displacement to target
        let disp_diff = self.sensors.target_displacement - self.sensors.linear_displacement;
        if disp_diff.abs() > 0.1 {
            let step = disp_diff.clamp(-500.0 * delta_time, 500.0 * delta_time); // 500 mm/sec
            self.sensors.linear_displacement += step;
        } else {
            self.sensors.linear_displacement = self.sensors.target_displacement;
        }

        // Clamp displacement to valid range
        self.sensors.linear_displacement = self.sensors.linear_displacement.clamp(0.0, 6425.5);
    }

    fn update_braking_physics(&mut self, delta_time: f64) {
        // AEBS System Physics
        if self.braking.aebs_enabled && self.braking.aebs_brake_demand > 0.0 {
            // Simulate brake response
            self.braking.aebs_target_deceleration = self.braking.aebs_brake_demand * 0.1; // 10% of demand
            self.braking.aebs_status = 1; // Active
        } else {
            self.braking.aebs_target_deceleration = 0.0;
            self.braking.aebs_status = 0; // Inactive
        }

        // ============================================================================
        // Braking & Stability Physics (Batch 5)
        // ============================================================================

        // EBC1: Brake pedal sets brake pressure and switch states
        if self.braking.ebc1_brake_pedal_position > 0.0 {
            self.braking.ebc1_ebs_brake_switch = 1; // Pedal pressed
            self.braking.ebc1_engine_retarder_selection =
                (self.braking.ebc1_brake_pedal_position * 0.5).clamp(0.0, 100.0);
            // Set brake pressures proportional to pedal position
            let pressure = self.braking.ebc1_brake_pedal_position * 12.5; // 0-1250 kPa
            self.braking.ebc3_pressure_front_left = pressure.clamp(0.0, 1250.0);
            self.braking.ebc3_pressure_front_right = pressure.clamp(0.0, 1250.0);
            self.braking.ebc3_pressure_rear1_left = (pressure * 0.8).clamp(0.0, 1250.0);
            self.braking.ebc3_pressure_rear1_right = (pressure * 0.8).clamp(0.0, 1250.0);
            // Foundation brakes in use
            self.braking.ebc5_foundation_brake_use = 1;
            // Driver brake demand from pedal
            self.braking.ebc5_driver_brake_demand =
                (-self.braking.ebc1_brake_pedal_position * 0.125).clamp(-12.5, 0.0); // m/s^2
        } else {
            self.braking.ebc1_ebs_brake_switch = 0;
            // Decay brake pressures
            self.braking.ebc3_pressure_front_left *= 0.9_f64.powf(delta_time);
            self.braking.ebc3_pressure_front_right *= 0.9_f64.powf(delta_time);
            self.braking.ebc3_pressure_rear1_left *= 0.9_f64.powf(delta_time);
            self.braking.ebc3_pressure_rear1_right *= 0.9_f64.powf(delta_time);
            if self.braking.ebc3_pressure_front_left < 1.0 {
                self.braking.ebc3_pressure_front_left = 0.0;
                self.braking.ebc3_pressure_front_right = 0.0;
                self.braking.ebc3_pressure_rear1_left = 0.0;
                self.braking.ebc3_pressure_rear1_right = 0.0;
            }
            self.braking.ebc5_foundation_brake_use = 0;
            self.braking.ebc5_driver_brake_demand = 0.0;
        }

        // EBC2: Wheel speeds track vehicle speed (from engine RPM proxy)
        let vehicle_speed_kmh = (self.engine.engine_speed - 600.0).max(0.0) * 0.05; // Crude RPM->speed
        self.braking.ebc2_front_axle_speed = vehicle_speed_kmh.clamp(0.0, 250.0);

        // ABS: detect wheel speed differences (simulated)
        // If braking hard and speed > threshold, simulate small wheel speed variations
        if self.braking.ebc1_brake_pedal_position > 50.0 && vehicle_speed_kmh > 20.0 {
            // Simulate slight wheel speed differences under heavy braking
            let variation = (self.uptime_seconds as f64 * 0.1).sin() * 0.5;
            self.braking.ebc2_rel_speed_front_left = variation;
            self.braking.ebc2_rel_speed_front_right = -variation;
            // ABS activates if variation is significant
            if variation.abs() > 0.3 {
                self.braking.ebc1_abs_active = 1;
                self.braking.ebc1_atc_asr_information_signal = 1;
            } else {
                self.braking.ebc1_abs_active = 0;
                self.braking.ebc1_atc_asr_information_signal = 0;
            }
        } else {
            self.braking.ebc2_rel_speed_front_left = 0.0;
            self.braking.ebc2_rel_speed_front_right = 0.0;
            self.braking.ebc1_abs_active = 0;
            self.braking.ebc1_atc_asr_information_signal = 0;
        }

        // XBR: External brake request from ACC/AEBS feeds into EBC5
        if self.braking.xbr_control_mode > 0 && self.braking.xbr_acceleration_demand < 0.0 {
            self.braking.ebc5_xbr_active_control_mode = self.braking.xbr_control_mode;
            self.braking.ebc5_overall_brake_demand = self.braking.xbr_acceleration_demand;
            // Emergency braking if high urgency XBR
            if self.braking.xbr_priority == 0 {
                self.braking.ebc5_emergency_braking_active = 1;
            }
        } else if self.braking.ebc1_brake_pedal_position > 0.0 {
            self.braking.ebc5_xbr_active_control_mode = 1; // Driver brake demand
            self.braking.ebc5_overall_brake_demand = self.braking.ebc5_driver_brake_demand;
            self.braking.ebc5_emergency_braking_active = 0;
        } else {
            self.braking.ebc5_xbr_active_control_mode = 0;
            self.braking.ebc5_overall_brake_demand = 0.0;
            self.braking.ebc5_emergency_braking_active = 0;
        }

        // ERC1: Retarder activates above certain speeds when demanded
        if self.braking.erc1_selection_non_engine > 0.0 && vehicle_speed_kmh > 30.0 {
            self.braking.erc1_retarder_torque_mode = 1; // Active
            self.braking.erc1_actual_retarder_torque =
                (self.braking.erc1_selection_non_engine * 1.25).clamp(0.0, 125.0);
            self.braking.erc1_intended_retarder_torque = self.braking.erc1_actual_retarder_torque;
            if self.braking.erc1_actual_retarder_torque > 10.0 {
                self.braking.erc1_requesting_brake_light = 1;
            }
        } else {
            // Decay retarder torque
            self.braking.erc1_actual_retarder_torque *= 0.9_f64.powf(delta_time);
            if self.braking.erc1_actual_retarder_torque < 1.0 {
                self.braking.erc1_actual_retarder_torque = 0.0;
                self.braking.erc1_retarder_torque_mode = 0;
                self.braking.erc1_requesting_brake_light = 0;
            }
        }

        // ACCS: Track longitudinal acceleration from braking
        self.braking.accs_longitudinal_acceleration = self.braking.ebc5_overall_brake_demand;

        // XBR message counter
        self.braking.xbr_message_counter = (self.braking.xbr_message_counter.wrapping_add(1)) % 16;
    }

    fn update_counters(&mut self) {
        self.motor.mg1_control_counter = (self.motor.mg1_control_counter + 1) % 16;
        self.motor.mg1_status_counter = (self.motor.mg1_status_counter + 1) % 16;
        self.motor.mg2_control_counter = (self.motor.mg2_control_counter + 1) % 16;
        self.motor.mg2_status_counter = (self.motor.mg2_status_counter + 1) % 16;
        self.motor.mg3_control_counter = (self.motor.mg3_control_counter + 1) % 16;
        self.motor.mg3_status_counter = (self.motor.mg3_status_counter + 1) % 16;
        self.motor.mg4_control_counter = (self.motor.mg4_control_counter + 1) % 16;
        self.motor.mg4_status_counter = (self.motor.mg4_status_counter + 1) % 16;
        self.dcdc.dcdc_control_counter = (self.dcdc.dcdc_control_counter + 1) % 16;

        for i in 0..8 {
            self.engine.engine_control_counters[i] =
                (self.engine.engine_control_counters[i] + 1) % 16;
        }
    }

    // ============================================================================
    // Batch 3: Engine Temps, Fluids & Sensors Physics
    // ============================================================================
    fn update_sensor_physics_batch3(&mut self, delta_time: f64) {
        // ET1: Coolant temperature rises with engine load, cools toward ambient
        let coolant_target = 75.0 + self.engine.engine_load * 0.3; // 75-105C based on load
        let coolant_diff = coolant_target - self.sensors.et1_coolant_temp;
        self.sensors.et1_coolant_temp += coolant_diff * 0.02 * delta_time;
        self.sensors.et1_coolant_temp = self.sensors.et1_coolant_temp.clamp(-40.0, 210.0);

        // Fuel temp rises slowly with load, cools toward ambient
        let fuel_target = self.sensors.amb_ambient_temp + 20.0 + self.engine.engine_load * 0.15;
        self.sensors.et1_fuel_temp += (fuel_target - self.sensors.et1_fuel_temp) * 0.01 * delta_time;
        self.sensors.et1_fuel_temp = self.sensors.et1_fuel_temp.clamp(-40.0, 210.0);

        // Oil temp tracks coolant but higher
        self.sensors.et1_oil_temp = self.sensors.et1_coolant_temp + 10.0 + self.engine.engine_load * 0.05;
        self.sensors.et1_oil_temp = self.sensors.et1_oil_temp.clamp(-273.0, 1735.0);

        // Turbo oil temp based on turbo speed
        self.sensors.et1_turbo_oil_temp = self.sensors.et1_oil_temp + (self.engine.turbo_speed / 1000.0) * 2.0;
        self.sensors.et1_turbo_oil_temp = self.sensors.et1_turbo_oil_temp.clamp(-273.0, 1735.0);

        // Intercooler temp between ambient and charge air temp
        self.sensors.et1_intercooler_temp = self.sensors.amb_ambient_temp + 15.0 + self.engine.engine_load * 0.1;
        self.sensors.et1_intercooler_temp = self.sensors.et1_intercooler_temp.clamp(-40.0, 210.0);

        // Charge air cooler thermostat: opens more as coolant heats up
        self.sensors.et1_charge_air_cooler_thermostat = if self.sensors.et1_coolant_temp > 90.0 {
            ((self.sensors.et1_coolant_temp - 90.0) / 15.0 * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };

        // ET2: Secondary sensors track primary with offsets
        self.sensors.et2_oil_temp_2 = self.sensors.et1_oil_temp - 3.0;
        self.sensors.et2_ecu_temp = 35.0 + self.engine.engine_load * 0.2;
        self.sensors.et2_ecu_temp = self.sensors.et2_ecu_temp.clamp(-40.0, 210.0);
        self.sensors.et2_egr_diff_pressure = self.engine.engine_load * 0.15;
        self.sensors.et2_egr_temp = 100.0 + self.engine.engine_load * 2.0;
        self.sensors.et2_egr_temp = self.sensors.et2_egr_temp.clamp(-40.0, 210.0);

        // ET3: High resolution versions of primary sensors
        self.sensors.et3_intake_manifold_temp_hr = self.sensors.ic1_intake_manifold_temp;
        self.sensors.et3_coolant_temp_hr = self.sensors.et1_coolant_temp;
        self.sensors.et3_intake_valve_oil_temp = self.sensors.et1_oil_temp - 15.0;
        self.sensors.et3_charge_air_cooler_outlet_temp = self.sensors.et1_intercooler_temp - 8.0;

        // ET4: Additional coolant sensors track ET1 with small offsets
        self.sensors.et4_coolant_temp_2 = self.sensors.et1_coolant_temp - 1.0;
        self.sensors.et4_coolant_pump_outlet_temp = self.sensors.et1_coolant_temp - 3.0;
        self.sensors.et4_coolant_thermostat_opening = self.sensors.et1_charge_air_cooler_thermostat;
        self.sensors.et4_exhaust_valve_oil_temp = self.sensors.et1_oil_temp + 5.0;
        self.sensors.et4_egr_mixer_intake_temp = self.sensors.et2_egr_temp - 60.0;
        self.sensors.et4_coolant_temp_3 = self.sensors.et1_coolant_temp - 2.0;

        // ET5: EGR2 system tracks EGR1
        self.sensors.et5_egr2_temp = self.sensors.et2_egr_temp - 5.0;
        self.sensors.et5_egr2_mixer_intake_temp = self.sensors.et4_egr_mixer_intake_temp - 5.0;
        self.sensors.et5_coolant_temp_2_hr = self.sensors.et4_coolant_temp_2;

        // ET6: Charge air cooler coolant temperatures
        self.sensors.et6_charge_air_cooler_intake_coolant_temp = self.sensors.et1_coolant_temp - 50.0;
        self.sensors.et6_charge_air_cooler_outlet_coolant_temp = self.sensors.et1_coolant_temp - 40.0;
        self.sensors.et6_intake_coolant_temp = self.sensors.et1_coolant_temp - 47.0;
        self.sensors.et6_intake_mixed_air_side_coolant_temp = self.sensors.et1_coolant_temp - 45.0;

        // LFE1: Fuel rate tracks engine load
        self.sensors.lfe1_fuel_rate = 2.0 + self.engine.engine_load * 0.1; // 2-12 L/h
        self.sensors.lfe1_fuel_rate = self.sensors.lfe1_fuel_rate.clamp(0.0, 3212.75);
        // Fuel economy inversely proportional to fuel rate (simplified)
        if self.sensors.lfe1_fuel_rate > 0.1 {
            self.sensors.lfe1_instant_fuel_economy = (80.0 / self.sensors.lfe1_fuel_rate).clamp(0.0, 125.0);
        }
        // Average economy drifts slowly toward instantaneous
        self.sensors.lfe1_average_fuel_economy += (self.sensors.lfe1_instant_fuel_economy - self.sensors.lfe1_average_fuel_economy) * 0.001 * delta_time;
        self.sensors.lfe1_average_fuel_economy = self.sensors.lfe1_average_fuel_economy.clamp(0.0, 125.0);
        self.sensors.lfe1_throttle_valve_1_pos = self.engine.engine_load.clamp(0.0, 100.0);
        self.sensors.lfe1_throttle_valve_2_pos = (self.engine.engine_load * 0.5).clamp(0.0, 100.0);

        // LFE2: High resolution fuel rate
        self.sensors.lfe2_fuel_rate_hr = self.sensors.lfe1_fuel_rate;
        self.sensors.lfe2_diesel_fuel_demand_rate = self.sensors.lfe1_fuel_rate * 0.95;

        // IC1: Intake conditions track turbo boost and engine load
        self.sensors.ic1_intake_manifold_pressure = self.sensors.amb_barometric_pressure + (self.engine.turbo_speed / 1000.0) * 5.0;
        self.sensors.ic1_intake_manifold_pressure = self.sensors.ic1_intake_manifold_pressure.clamp(0.0, 500.0);
        self.sensors.ic1_intake_manifold_temp = self.sensors.amb_ambient_temp + 15.0 + (self.engine.turbo_speed / 5000.0) * 10.0;
        self.sensors.ic1_intake_manifold_temp = self.sensors.ic1_intake_manifold_temp.clamp(-40.0, 210.0);
        self.sensors.ic1_intake_air_pressure = self.sensors.amb_barometric_pressure;
        self.sensors.ic1_air_filter_diff_pressure = 1.0 + self.engine.engine_load * 0.05; // Increases with airflow
        self.sensors.ic1_exhaust_temp = 200.0 + self.engine.engine_load * 5.0;
        self.sensors.ic1_exhaust_temp = self.sensors.ic1_exhaust_temp.clamp(-273.0, 1735.0);
        self.sensors.ic1_coolant_filter_diff_pressure = 5.0 + self.engine.engine_load * 0.1;
        self.sensors.ic1_aftertreatment_intake_pressure = self.sensors.ic1_intake_manifold_pressure - 2.0;

        // IC2: Additional intake sensors
        self.sensors.ic2_air_filter_2_diff_pressure = self.sensors.ic1_air_filter_diff_pressure - 0.3;
        self.sensors.ic2_air_filter_3_diff_pressure = self.sensors.ic1_air_filter_diff_pressure - 0.5;
        self.sensors.ic2_air_filter_4_diff_pressure = self.sensors.ic1_air_filter_diff_pressure - 0.7;
        self.sensors.ic2_intake_manifold_2_pressure = self.sensors.ic1_intake_manifold_pressure - 1.0;
        self.sensors.ic2_intake_manifold_1_abs_pressure = self.sensors.ic1_intake_manifold_pressure;
        self.sensors.ic2_intake_manifold_1_abs_pressure_hr = self.sensors.ic1_intake_manifold_pressure;
        self.sensors.ic2_intake_manifold_2_abs_pressure = self.sensors.ic2_intake_manifold_2_pressure;

        // IC3: Mixer intake pressures
        self.sensors.ic3_mixer_1_intake_pressure = self.sensors.ic1_intake_manifold_pressure - 2.0;
        self.sensors.ic3_mixer_2_intake_pressure = self.sensors.ic1_intake_manifold_pressure - 3.0;
        self.sensors.ic3_intake_manifold_2_abs_pressure_hr = self.sensors.ic2_intake_manifold_2_pressure;
        self.sensors.ic3_desired_intake_manifold_pressure_high_limit = 200.0 + self.engine.engine_load;

        // AMB: Ambient is relatively stable, slight warming from engine proximity
        // (ambient drifts very slowly - effectively constant in short simulations)
        self.sensors.amb_intake_air_temp = self.sensors.amb_ambient_temp + 5.0 + self.engine.engine_load * 0.05;
        self.sensors.amb_intake_air_temp = self.sensors.amb_intake_air_temp.clamp(-40.0, 210.0);

        // AMB2-4: Derived ambient sensors
        self.sensors.amb2_calculated_ambient_temp = self.sensors.amb_ambient_temp;
        self.sensors.amb3_intake_2_air_temp = self.sensors.amb_intake_air_temp + 2.0;

        // FD2: Fan speed ramps when coolant exceeds thermostat threshold
        let fan_thermostat_threshold = 90.0;
        if self.sensors.et1_coolant_temp > fan_thermostat_threshold {
            let overheat_factor = (self.sensors.et1_coolant_temp - fan_thermostat_threshold) / 15.0;
            let target_fan_pct = (overheat_factor * 100.0).clamp(0.0, 100.0);
            self.sensors.fd2_estimated_fan_2_speed_pct += (target_fan_pct - self.sensors.fd2_estimated_fan_2_speed_pct) * 0.1 * delta_time;
            self.sensors.fd2_fan_2_speed = self.sensors.fd2_estimated_fan_2_speed_pct / 100.0 * 5000.0;
            self.sensors.fd2_fan_2_drive_state = if self.sensors.fd2_estimated_fan_2_speed_pct > 80.0 { 3 } else if self.sensors.fd2_estimated_fan_2_speed_pct > 40.0 { 2 } else { 1 };
        } else {
            // Fan idles when below threshold
            self.sensors.fd2_estimated_fan_2_speed_pct = (self.sensors.fd2_estimated_fan_2_speed_pct * 0.95).max(10.0);
            self.sensors.fd2_fan_2_speed = self.sensors.fd2_estimated_fan_2_speed_pct / 100.0 * 5000.0;
            self.sensors.fd2_fan_2_drive_state = 1; // Normal idle
        }
        self.sensors.fd2_fan_2_speed = self.sensors.fd2_fan_2_speed.clamp(0.0, 32127.5);
        self.sensors.fd2_hydraulic_fan_2_pressure = self.sensors.fd2_fan_2_speed * 0.5; // Proportional

        // DD2: Fuel level decreases per fuel rate
        // Assume 200L tank capacity, lfe1_fuel_rate is L/h, delta_time in seconds
        let fuel_consumed_liters = self.sensors.lfe1_fuel_rate * delta_time / 3600.0;
        self.sensors.dd2_fuel_2_tank_1_level -= (fuel_consumed_liters / 200.0) * 100.0;
        self.sensors.dd2_fuel_2_tank_1_level = self.sensors.dd2_fuel_2_tank_1_level.clamp(0.0, 100.0);
        self.sensors.dd2_oil_filter_diff_pressure_ext = 10.0 + self.engine.engine_load * 0.1;

        // HOURS: Increments real-time (uses uptime_seconds)
        self.sensors.hours_engine_total_hours = 12500.5 + (self.uptime_seconds as f64 / 3600.0);
        self.sensors.hours_total_revolutions += self.engine.engine_speed * delta_time / 60.0; // RPM -> revolutions

        // HOURS2: Idle time increments when engine load is low
        if self.engine.engine_load < 10.0 {
            self.sensors.hours2_idle_management_active_total_time += delta_time / 3600.0;
        }

        // IO: Idle fuel and hours accumulate when engine is at low load
        if self.engine.engine_load < 10.0 {
            self.sensors.io_total_idle_fuel_used += self.sensors.lfe1_fuel_rate * delta_time / 3600.0;
            self.sensors.io_total_idle_hours += delta_time / 3600.0;
        }

        // LFC1: Trip and total fuel consumption
        self.sensors.lfc1_trip_fuel += fuel_consumed_liters;
        self.sensors.lfc1_total_fuel_used += fuel_consumed_liters;
    }
}
