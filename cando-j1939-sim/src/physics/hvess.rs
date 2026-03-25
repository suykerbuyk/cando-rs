use crate::SimulatorState;

impl SimulatorState {
    pub(crate) fn update_hvess_physics(&mut self, delta_time: f64) {
        // HVESS System Physics - Battery and power management
        let total_motor_power =
            (self.motor.mg1_current + self.motor.mg2_current) * 48.0 / 1000.0; // Convert to kW

        // Battery discharge/charge based on motor load
        if total_motor_power > 0.1 {
            self.hvess.hvess_discharge_power = (total_motor_power * 1.2).clamp(0.0, 100.0); // 20% overhead
            self.hvess.hvess_charge_power = (50.0 - total_motor_power).clamp(0.0, 50.0);
        } else {
            self.hvess.hvess_discharge_power = 1.0; // Standby power
            self.hvess.hvess_charge_power = 50.0; // Full charging available
        }

        // Voltage stability simulation
        let voltage_drop = total_motor_power * 2.0; // 2V drop per kW
        self.hvess.hvess_voltage_level = (800.0 - voltage_drop).clamp(750.0, 850.0);
        self.hvess.hvess_bus_voltage = self.hvess.hvess_voltage_level;

        // Temperature simulation based on power
        let temp_rise = total_motor_power * 0.5 * delta_time; // Gradual heating
        self.hvess.hvess_electronics_temp =
            (self.hvess.hvess_electronics_temp + temp_rise - (0.2 * delta_time)).clamp(25.0, 80.0);
        self.hvess.hvess_coolant_temp =
            (self.hvess.hvess_coolant_temp + temp_rise * 0.3 - (0.1 * delta_time))
                .clamp(20.0, 60.0);

        // HVESSD2 Cell Voltage & State of Charge Physics - Individual cell voltage and charge simulation
        let voltage_variance = 0.2; // Voltage spread between cells (V)
        let base_cell_voltage = 4.1 - (total_motor_power * 0.05); // Voltage drops under load
        self.hvess.hvess_highest_cell_voltage =
            (base_cell_voltage + voltage_variance).clamp(2.5, 4.3);
        self.hvess.hvess_lowest_cell_voltage =
            (base_cell_voltage - voltage_variance).clamp(2.0, 4.2);

        // State of charge simulation based on discharge/charge activity
        let charge_delta = if total_motor_power > 1.0 {
            -total_motor_power * 0.01 * delta_time // Discharging
        } else {
            0.005 * delta_time // Slow charging when not under load
        };
        self.hvess.hvess_fast_update_state_of_charge =
            (self.hvess.hvess_fast_update_state_of_charge + charge_delta).clamp(0.0, 100.0);

        // Update voltage differential status based on voltage spread
        let voltage_diff =
            self.hvess.hvess_highest_cell_voltage - self.hvess.hvess_lowest_cell_voltage;
        self.hvess.hvess_cell_voltage_differential_status = if voltage_diff > 0.4 {
            15 // High voltage differential
        } else if voltage_diff > 0.3 {
            10 // Moderate-high voltage differential
        } else if voltage_diff > 0.2 {
            5 // Moderate voltage differential
        } else if voltage_diff > 0.1 {
            2 // Normal voltage differential
        } else {
            0 // Low voltage differential
        };

        // HVESSD3 Cell Temperature Physics - Individual cell temperature simulation
        // NOTE: Cell temperature must be computed before fan status, since fan status
        // depends on hvess_highest_cell_temperature and hvess_average_cell_temperature.
        let cell_temp_variance = 2.0; // Temperature spread between cells (°C)
        let base_cell_temp = self.hvess.hvess_electronics_temp + (total_motor_power * 0.3);
        self.hvess.hvess_highest_cell_temperature =
            (base_cell_temp + cell_temp_variance).clamp(25.0, 85.0);
        self.hvess.hvess_lowest_cell_temperature =
            (base_cell_temp - cell_temp_variance).clamp(20.0, 80.0);
        self.hvess.hvess_average_cell_temperature = base_cell_temp.clamp(22.5, 82.5);

        // Update differential status based on temperature spread
        let temp_diff = self.hvess.hvess_highest_cell_temperature
            - self.hvess.hvess_lowest_cell_temperature;
        self.hvess.hvess_cell_temp_differential_status = if temp_diff > 8.0 {
            3 // High differential
        } else if temp_diff > 5.0 {
            2 // Moderate differential
        } else if temp_diff > 2.0 {
            1 // Normal differential
        } else {
            0 // Low differential
        };

        // HVESSFS1 Fan Status Physics - Fan performance simulation based on thermal load
        let thermal_load = (self.hvess.hvess_highest_cell_temperature - 25.0) / 50.0; // Thermal load factor
        let target_fan_speed = (thermal_load * 4000.0 + 1000.0).clamp(800.0, 5000.0); // Target speed based on thermal needs

        // Simulate fan response to thermal demands (realistic lag)
        let speed_diff = target_fan_speed - self.hvess.hvess_fan_speed;
        self.hvess.hvess_fan_speed =
            (self.hvess.hvess_fan_speed + speed_diff * 0.1).clamp(0.0, 32127.5);

        // Power consumption based on fan speed (cubic relationship for fans)
        let speed_ratio = self.hvess.hvess_fan_speed / 3000.0;
        self.hvess.hvess_fan_power =
            (speed_ratio * speed_ratio * speed_ratio * 200.0).clamp(10.0, 500.0);

        // Medium temperature influenced by fan effectiveness
        let cooling_effect = self.hvess.hvess_fan_speed * 0.01;
        self.hvess.hvess_fan_medium_temperature =
            (self.hvess.hvess_average_cell_temperature - cooling_effect).clamp(-10.0, 100.0);

        // Update fan status based on performance
        self.hvess.hvess_fan_speed_status = if self.hvess.hvess_fan_speed > 4000.0 {
            3 // High speed
        } else if self.hvess.hvess_fan_speed > 2000.0 {
            2 // Medium speed
        } else if self.hvess.hvess_fan_speed > 500.0 {
            1 // Low speed
        } else {
            0 // Stopped/fault
        };

        // Operating status based on system health
        self.hvess.hvess_fan_operating_status = if self.hvess.hvess_fan_power > 400.0 {
            3 // Overload warning
        } else if self.hvess.hvess_fan_medium_temperature > 80.0 {
            2 // High temperature warning
        } else {
            1 // Normal operation
        };

        // ============================================================================
        // Batch 7: Extended HVESS Physics
        // ============================================================================

        // HVESSD4: Capacity tracks SOC
        self.hvess.hvessd4_discharge_capacity =
            (self.hvess.hvesscfg_nominal_capacity * self.hvess.hvess_fast_update_state_of_charge / 100.0)
                .clamp(0.0, 642.55);
        self.hvess.hvessd4_charge_capacity =
            (self.hvess.hvesscfg_nominal_capacity * (100.0 - self.hvess.hvess_fast_update_state_of_charge) / 100.0)
                .clamp(0.0, 642.55);

        // HVESSD5: Cell SOC tracks system SOC with spread
        self.hvess.hvessd5_min_cell_soc =
            (self.hvess.hvess_fast_update_state_of_charge - 3.0).clamp(0.0, 100.0);
        self.hvess.hvessd5_max_cell_soc =
            (self.hvess.hvess_fast_update_state_of_charge + 3.0).clamp(0.0, 100.0);

        // HVESSD7: Energy capacity = voltage * Ah capacity / 1000
        self.hvess.hvessd7_discharge_energy_capacity =
            (self.hvess.hvessd4_discharge_capacity * self.hvess.hvess_voltage_level / 1000.0).clamp(0.0, 16449.0);
        self.hvess.hvessd7_charge_energy_capacity =
            (self.hvess.hvessd4_charge_capacity * self.hvess.hvess_voltage_level / 1000.0).clamp(0.0, 16449.0);

        // HVESSD8: Average cell voltage tracks system voltage / cell count (assume ~200 cells)
        self.hvess.hvessd8_average_cell_voltage =
            (self.hvess.hvess_voltage_level / 200.0).clamp(0.0, 64.25);

        // HVESSD9 counter increments
        self.hvess.hvessd9_counter = (self.hvess.hvessd9_counter.wrapping_add(1)) % 16;

        // HVESSD11: Electronics temp tracks main electronics temp
        self.hvess.hvessd11_power_module_electronics_temp = self.hvess.hvess_electronics_temp;

        // HVESSD13/D14: Extended range tracks primary values
        self.hvess.hvessd13_discharge_power_extended = self.hvess.hvess_discharge_power;
        self.hvess.hvessd13_charge_power_extended = self.hvess.hvess_charge_power;
        self.hvess.hvessd13_voltage_extended = self.hvess.hvess_voltage_level;
        self.hvess.hvessd14_bus_voltage_extended = self.hvess.hvess_bus_voltage;

        // HVESSIS1-4: Internal segment voltages track bus voltage / segments
        let segment_voltage = self.hvess.hvess_voltage_level / 4.0;
        let segment_current = self.hvess.hvess_current_level / 4.0;
        self.hvess.hvessis1_internal_voltage_1 = segment_voltage;
        self.hvess.hvessis1_internal_current_1 = segment_current;
        self.hvess.hvessis1_internal_voltage_2 = segment_voltage;
        self.hvess.hvessis1_internal_current_2 = segment_current;
        self.hvess.hvessis2_internal_voltage_3 = segment_voltage;
        self.hvess.hvessis2_internal_current_3 = segment_current;
        self.hvess.hvessis2_internal_voltage_4 = segment_voltage;
        self.hvess.hvessis2_internal_current_4 = segment_current;
        self.hvess.hvessis3_internal_voltage_5 = segment_voltage;
        self.hvess.hvessis3_internal_current_5 = segment_current;
        self.hvess.hvessis3_internal_voltage_6 = segment_voltage;
        self.hvess.hvessis3_internal_current_6 = segment_current;
        self.hvess.hvessis4_internal_voltage_7 = segment_voltage;
        self.hvess.hvessis4_internal_current_7 = segment_current;
        self.hvess.hvessis4_internal_voltage_8 = segment_voltage;
        self.hvess.hvessis4_internal_current_8 = segment_current;

        // HVESSIS5/6: Bus voltages track segment voltages
        self.hvess.hvessis5_bus_voltage_1 = segment_voltage;
        self.hvess.hvessis5_bus_voltage_2 = segment_voltage;
        self.hvess.hvessis6_bus_voltage_3 = segment_voltage;
        self.hvess.hvessis6_bus_voltage_4 = segment_voltage;

        // HVESSCP1 pump physics: speed responds to command with lag
        if self.hvess.hvesscp1c_enable_command > 0 {
            let target_speed = self.hvess.hvesscp1c_speed_command;
            let speed_diff = target_speed - self.hvess.hvesscp1s1_motor_speed;
            self.hvess.hvesscp1s1_motor_speed =
                (self.hvess.hvesscp1s1_motor_speed + speed_diff * 0.1 * delta_time).clamp(0.0, 32127.5);
            let speed_ratio = self.hvess.hvesscp1s1_motor_speed / target_speed.max(1.0);
            self.hvess.hvesscp1s1_power = (speed_ratio * speed_ratio * 200.0).clamp(0.0, 500.0);
            self.hvess.hvesscp1s2_percent_speed = (self.hvess.hvesscp1s1_motor_speed / 32127.5 * 100.0).clamp(0.0, 100.0);
            self.hvess.hvesscp1s1_motor_speed_status = 1; // Normal
            self.hvess.hvesscp1s1_operating_status = 1;   // Running
        } else {
            self.hvess.hvesscp1s1_motor_speed = (self.hvess.hvesscp1s1_motor_speed * 0.9).max(0.0);
            self.hvess.hvesscp1s1_power = 0.0;
            self.hvess.hvesscp1s1_motor_speed_status = 0; // Stopped
            self.hvess.hvesscp1s1_operating_status = 0;
        }

        // HVESSCP2 pump physics: speed responds to command with lag
        if self.hvess.hvesscp2c_enable_command > 0 {
            let target_speed = self.hvess.hvesscp2c_speed_command;
            let speed_diff = target_speed - self.hvess.hvesscp2s1_motor_speed;
            self.hvess.hvesscp2s1_motor_speed =
                (self.hvess.hvesscp2s1_motor_speed + speed_diff * 0.1 * delta_time).clamp(0.0, 32127.5);
            let speed_ratio = self.hvess.hvesscp2s1_motor_speed / target_speed.max(1.0);
            self.hvess.hvesscp2s1_power = (speed_ratio * speed_ratio * 180.0).clamp(0.0, 500.0);
            self.hvess.hvesscp2s2_percent_speed = (self.hvess.hvesscp2s1_motor_speed / 32127.5 * 100.0).clamp(0.0, 100.0);
            self.hvess.hvesscp2s1_motor_speed_status = 1;
            self.hvess.hvesscp2s1_operating_status = 1;
        } else {
            self.hvess.hvesscp2s1_motor_speed = (self.hvess.hvesscp2s1_motor_speed * 0.9).max(0.0);
            self.hvess.hvesscp2s1_power = 0.0;
            self.hvess.hvesscp2s1_motor_speed_status = 0;
            self.hvess.hvesscp2s1_operating_status = 0;
        }

        // HVESSFC -> HVESSFS2: Fan speed responds to fan command
        if self.hvess.hvessfc_fan_enable_command > 0 {
            let target_fan = self.hvess.hvessfc_fan_speed_command;
            let fan_diff = target_fan - self.hvess.hvess_fan_speed;
            self.hvess.hvess_fan_speed = (self.hvess.hvess_fan_speed + fan_diff * 0.15 * delta_time).clamp(0.0, 32127.5);
            self.hvess.hvessfs2_fan_percent_speed = (self.hvess.hvess_fan_speed / target_fan.max(1.0) * 100.0).clamp(0.0, 100.0);
        }

        // HVESSS1 counter
        self.hvess.hvesss1_counter = (self.hvess.hvesss1_counter.wrapping_add(1)) % 16;
    }
}
