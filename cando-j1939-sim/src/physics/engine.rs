use crate::SimulatorState;

impl SimulatorState {
    pub(crate) fn update_engine_physics(&mut self, delta_time: f64) {
        // Engine Physics Simulation
        // Engine speed response to load
        let engine_load_demand =
            (self.motor.mg1_actual_speed.abs() + self.motor.mg2_actual_speed.abs()) * 0.01;
        let target_rpm = 800.0 + (engine_load_demand * 2000.0); // Idle + load-based RPM
        let rpm_diff = target_rpm - self.engine.engine_speed;
        if rpm_diff.abs() > 1.0 {
            let step = rpm_diff.clamp(-500.0 * delta_time, 500.0 * delta_time); // 500 RPM/sec
            self.engine.engine_speed += step;
        }
        self.engine.engine_speed = self.engine.engine_speed.clamp(600.0, 6000.0);

        // Engine torque and load based on RPM
        self.engine.engine_load =
            ((self.engine.engine_speed - 800.0) / 2000.0 * 100.0).clamp(0.0, 100.0);
        self.engine.engine_torque = (self.engine.engine_load * 0.8).clamp(0.0, 100.0);

        // Engine temperatures based on load
        let load_temp_factor = self.engine.engine_load * 0.001 * delta_time;
        self.engine.engine_coolant_temp =
            (self.engine.engine_coolant_temp + load_temp_factor - (0.05 * delta_time))
                .clamp(75.0, 105.0);
        self.engine.engine_exhaust_temp =
            (200.0 + self.engine.engine_load * 4.0).clamp(150.0, 800.0);

        // Fuel rate based on load
        self.engine.engine_fuel_rate = (2.0 + self.engine.engine_load * 0.1).clamp(1.0, 15.0);

        // Turbo speed based on load (if applicable)
        if self.engine.engine_load > 30.0 {
            self.engine.turbo_speed =
                ((self.engine.engine_load - 30.0) * 1000.0).clamp(0.0, 50000.0);
        } else {
            self.engine.turbo_speed *= 0.9; // Decay when not under load
        }

        // =====================================================================
        // Core Engine Electronics Physics (Batch 1)
        // =====================================================================

        // EEC1 engine speed responds to EEC2 accelerator pedal position
        let accel_demand = self.engine.eec2_accelerator_pedal_1_position;
        if accel_demand > 0.5 {
            // Engine speed increases towards rated speed based on pedal position
            let target_speed = 800.0 + (self.engine.eec4_engine_rated_speed - 800.0) * (accel_demand / 100.0);
            let speed_diff = target_speed - self.engine.eec1_engine_speed;
            let ramp = speed_diff.clamp(-500.0 * delta_time, 500.0 * delta_time);
            self.engine.eec1_engine_speed = (self.engine.eec1_engine_speed + ramp).clamp(0.0, 8031.0);
        } else {
            // Return to idle
            let speed_diff = 800.0 - self.engine.eec1_engine_speed;
            let ramp = speed_diff.clamp(-300.0 * delta_time, 300.0 * delta_time);
            self.engine.eec1_engine_speed = (self.engine.eec1_engine_speed + ramp).clamp(0.0, 8031.0);
        }

        // EEC1 torque responds to engine load
        self.engine.eec1_actual_engine_percent_torque =
            (accel_demand * 1.2).clamp(0.0, 125.0);
        self.engine.eec1_drvr_s_dmnd_engn_prnt_trq = accel_demand.clamp(0.0, 125.0);

        // EEC3 friction torque inversely tracks coolant temperature
        // Colder engine = more friction; warmer engine = less friction
        let coolant = self.engine.engine_coolant_temp;
        self.engine.eec3_nominal_friction_percent_torque =
            (30.0 - (coolant - 20.0) * 0.2).clamp(-125.0, 125.0);
        self.engine.eec3_engine_s_desired_operating_speed = self.engine.eec1_engine_speed;

        // EEC3 exhaust flow tracks engine speed and load
        self.engine.eec3_aftrtrtmnt_1_exhst_gs_mss_flw_rt =
            (self.engine.eec1_engine_speed * 0.1 + accel_demand * 2.0).clamp(0.0, 12851.0);

        // ETC1 output shaft speed = engine speed / gear ratio
        let gear_ratio = if self.transmission.etc2_transmission_actual_gear_ratio > 0.1 {
            self.transmission.etc2_transmission_actual_gear_ratio
        } else {
            1.0 // Prevent division by zero
        };
        self.engine.etc1_transmission_output_shaft_speed =
            (self.engine.eec1_engine_speed / gear_ratio).clamp(0.0, 8031.0);
        self.engine.etc1_transmission_input_shaft_speed = self.engine.eec1_engine_speed;

        // Turbo boost (EEC5-7) responds to engine load
        let load_factor = accel_demand / 100.0;
        // EEC5: turbine temperatures increase with load
        self.engine.eec5_engn_trhrgr_1_clltd_trn_intk_tmprtr =
            (200.0 + load_factor * 500.0).clamp(-273.0, 1734.0);
        self.engine.eec5_engn_trhrgr_1_clltd_trn_otlt_tmprtr =
            (150.0 + load_factor * 400.0).clamp(-273.0, 1734.0);
        // VGT position opens with load
        self.engine.eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn =
            (20.0 + load_factor * 70.0).clamp(0.0, 100.0);

        // EEC6: compressor bypass closes as load increases (more boost needed)
        self.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_pstn =
            (80.0 - load_factor * 70.0).clamp(0.0, 100.0);
        self.engine.eec6_engn_vrl_gmtr_trhrgr_attr_1 =
            (20.0 + load_factor * 60.0).clamp(0.0, 100.0);

        // EEC7: intake manifold pressure increases with boost
        self.engine.eec7_engn_intk_mnfld_cmmndd_prssr =
            (100.0 + load_factor * 200.0).clamp(0.0, 8031.0);

        // EEC14: fuel mass flow rate follows engine load
        self.engine.eec14_engine_fuel_mass_flow_rate =
            (1.0 + load_factor * 50.0).clamp(0.0, 321.27);

        // EEC20: parasitic losses and air mass track engine speed
        let speed_factor = self.engine.eec1_engine_speed / 2200.0;
        self.engine.eec20_aslt_engn_ld_prnt_ar_mss =
            (20.0 + speed_factor * 80.0 + load_factor * 200.0).clamp(0.0, 1606.0);
    }
}
