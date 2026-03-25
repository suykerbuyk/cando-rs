use chrono::Utc;

use crate::SimulatorState;

impl SimulatorState {
    pub(crate) fn update_vehicle_physics(&mut self, delta_time: f64) {
        // CCVS1 vehicle speed derived from engine speed and gear ratio
        // speed = engine_speed / gear_ratio * tire_radius_factor
        let gear_ratio = self.transmission.etc2_transmission_actual_gear_ratio;
        if gear_ratio > 0.1 {
            let tire_radius_factor = 0.003; // Calibration factor: RPM -> km/h through gear ratio
            self.vehicle.ccvs1_vehicle_speed =
                (self.engine.engine_speed / gear_ratio * tire_radius_factor).clamp(0.0, 251.0);
        } else {
            // Neutral or park - no speed
            self.vehicle.ccvs1_vehicle_speed = 0.0;
        }

        // CCVS5 directional speed tracks CCVS1 (positive = forward)
        self.vehicle.ccvs5_directional_speed = self.vehicle.ccvs1_vehicle_speed;

        // VDS navigation speed approximately equals vehicle speed
        self.vehicle.vds_nav_speed = self.vehicle.ccvs1_vehicle_speed;

        // HRW - wheel speeds track vehicle speed with small per-wheel noise
        let base_speed = self.vehicle.ccvs1_vehicle_speed;
        // Use uptime to create stable but varied per-wheel offsets
        let time_phase = (self.uptime_seconds as f64) * 0.1;
        self.vehicle.hrw_front_left_speed =
            (base_speed + 0.05 * time_phase.sin()).clamp(0.0, 251.0);
        self.vehicle.hrw_front_right_speed =
            (base_speed + 0.04 * (time_phase + 1.0).sin()).clamp(0.0, 251.0);
        self.vehicle.hrw_rear_left_speed =
            (base_speed + 0.06 * (time_phase + 2.0).sin()).clamp(0.0, 251.0);
        self.vehicle.hrw_rear_right_speed =
            (base_speed + 0.03 * (time_phase + 3.0).sin()).clamp(0.0, 251.0);

        // VD - distance accumulates from speed * dt (speed is in km/h, dt in seconds)
        let distance_increment = self.vehicle.ccvs1_vehicle_speed * delta_time / 3600.0; // km
        self.vehicle.vd_trip_distance += distance_increment;
        self.vehicle.vd_total_distance += distance_increment;

        // VEP1 - battery voltage tracks alternator output
        if self.engine.engine_speed > 400.0 {
            // Engine running: alternator charging
            let alt_output =
                14.0 + (self.engine.engine_speed - 800.0).clamp(0.0, 2000.0) * 0.0002;
            self.vehicle.vep1_charging_voltage = alt_output.clamp(13.5, 14.8);
            self.vehicle.vep1_battery_potential = (self.vehicle.vep1_battery_potential * 0.99
                + self.vehicle.vep1_charging_voltage * 0.01)
                .clamp(11.0, 15.0);
            self.vehicle.vep1_key_switch_voltage = self.vehicle.vep1_battery_potential - 0.2;
        } else {
            // Engine off: voltage slowly drops
            self.vehicle.vep1_charging_voltage = 0.0;
            self.vehicle.vep1_battery_potential =
                (self.vehicle.vep1_battery_potential - 0.0001 * delta_time).clamp(10.0, 13.0);
            self.vehicle.vep1_key_switch_voltage = self.vehicle.vep1_battery_potential - 0.2;
        }

        // AS1 - alternator speed proportional to engine speed (belt ratio ~2.5:1)
        self.vehicle.as1_alternator_speed = self.engine.engine_speed * 2.5;

        // AS2 - alternator output voltage tracks charging voltage
        self.vehicle.as2_output_voltage = self.vehicle.vep1_charging_voltage;

        // VEP3 - high resolution values mirror standard VEP1
        self.vehicle.vep3_alternator_current_hr = self.vehicle.vep1_alternator_current as f64;
        self.vehicle.vep3_battery_current_hr = self.vehicle.vep1_battery_current;

        // TD - broadcast real clock time
        let now = Utc::now();
        self.vehicle.td_seconds = now.timestamp_subsec_millis() as f64 / 1000.0
            + (now.format("%S").to_string().parse::<f64>().unwrap_or(0.0));
        self.vehicle.td_minutes = now
            .format("%M")
            .to_string()
            .parse::<u8>()
            .unwrap_or(0);
        self.vehicle.td_hours = now
            .format("%H")
            .to_string()
            .parse::<u8>()
            .unwrap_or(0);
        self.vehicle.td_day = now
            .format("%d")
            .to_string()
            .parse::<f64>()
            .unwrap_or(1.0);
        self.vehicle.td_month = now
            .format("%m")
            .to_string()
            .parse::<u8>()
            .unwrap_or(1);
        self.vehicle.td_year = now
            .format("%Y")
            .to_string()
            .parse::<f64>()
            .unwrap_or(2026.0);

        // TIRE1 - tire temperature increases slightly with speed
        self.vehicle.tire1_temperature =
            (25.0 + self.vehicle.ccvs1_vehicle_speed * 0.1).clamp(15.0, 80.0);

        // SSI - small pitch/roll from vehicle dynamics (simplified)
        // Under braking, pitch forward slightly
        if self.vehicle.ccvs1_brake_switch > 0 {
            self.vehicle.ssi_pitch_angle =
                (self.vehicle.ssi_pitch_angle * 0.9 + -0.1).clamp(-10.0, 10.0);
        } else {
            self.vehicle.ssi_pitch_angle =
                (self.vehicle.ssi_pitch_angle * 0.95).clamp(-10.0, 10.0);
        }
        self.vehicle.ssi_roll_angle =
            (self.vehicle.ssi_roll_angle * 0.95).clamp(-10.0, 10.0);

        // GFI1 - fuel accumulation based on fuel rate
        self.vehicle.gfi1_total_fuel_used +=
            self.engine.engine_fuel_rate * delta_time / 3600.0; // kg
        self.vehicle.gfi1_trip_average_fuel_rate = self.engine.engine_fuel_rate;
    }
}
