use crate::SimulatorState;

impl SimulatorState {
    pub(crate) fn update_transmission_physics(&mut self, delta_time: f64) {
        // TC1 -> ETC3/ETC2 Physics: Gear shift with delay
        // When TC1 requests a gear, ETC2 current gear gradually follows
        let requested = self.transmission.tc1_transmission_requested_gear;
        let current = self.transmission.etc2_transmission_current_gear;
        let gear_diff = requested - current;
        if gear_diff.abs() > 0.1 {
            // Shift takes ~0.5 seconds (2 gears/sec rate)
            let step = gear_diff.clamp(-2.0 * delta_time, 2.0 * delta_time);
            self.transmission.etc2_transmission_current_gear += step;
            // During shift: clutch engaged, shift finger active
            self.transmission.etc3_shift_finger_engagement_indicator = 1; // On - shifting
            self.transmission.etc3_shift_finger_neutral_indicator = 0;     // Off - not in neutral
            self.transmission.etc3_clutch_actuator = 1;                    // On - clutch active
        } else {
            self.transmission.etc2_transmission_current_gear = requested;
            // Shift complete: clutch disengaged, shift finger at rest
            if requested.abs() < 0.01 {
                self.transmission.etc3_shift_finger_neutral_indicator = 1; // On - in neutral
            } else {
                self.transmission.etc3_shift_finger_neutral_indicator = 0; // Off - in gear
            }
            self.transmission.etc3_shift_finger_engagement_indicator = 0; // Off - not shifting
            self.transmission.etc3_clutch_actuator = 0;                    // Off - clutch released
        }

        // ETC14 Clutch temperature physics: rises with engine load, cools over time
        let clutch_heat = self.engine.engine_load * 0.02 * delta_time; // Heat from load
        let clutch_cool = 0.5 * delta_time;                             // Natural cooling
        self.transmission.etc14_clutch_1_temperature =
            (self.transmission.etc14_clutch_1_temperature + clutch_heat - clutch_cool)
                .clamp(20.0, 300.0);
        // Overheat indicator when temperature exceeds 200°C
        self.transmission.etc14_clutch_1_overheat_indicator =
            if self.transmission.etc14_clutch_1_temperature > 200.0 {
                1 // On - overheating
            } else {
                0 // Off - normal
            };

        // ETC8 Torque converter input speed follows engine speed
        self.transmission.etc8_clutch_converter_input_speed = self.engine.engine_speed;

        // ETC15 counter increments each physics tick
        self.transmission.etc15_counter =
            ((self.transmission.etc15_counter as u16 + 1) % 16) as u8;
    }
}
