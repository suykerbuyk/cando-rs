use crate::SimulatorState;

impl SimulatorState {
    /// EV Charging state machine and physics simulation
    ///
    /// States: 0=Idle, 1=Communication, 2=PreCharge, 3=Charging, 4=Complete
    pub(crate) fn update_ev_charging_physics(&mut self, delta_time: f64) {
        let soc = self.hvess.hvess_fast_update_state_of_charge;

        match self.ev_charging.ev_charging_state {
            0 => {
                // Idle - no charging activity
                self.ev_charging.evsedcs1_dc_charging_state = 0; // Idle
                self.ev_charging.evsedcs1_status = 0; // Not ready
                self.ev_charging.evsedcs1_present_voltage = 0.0;
                self.ev_charging.evsedcs1_present_current = 0.0;
                self.ev_charging.evdccip_bulk_charging_complete = 0;
                self.ev_charging.evdccip_full_charging_complete = 0;

                // HV bus contactors open in idle
                self.ev_charging.hvbcs1_positive_contactor_states[0] = 0; // Open
                self.ev_charging.hvbcs1_negative_contactor_states[0] = 0; // Open
                self.ev_charging.evse1cs1_contactor_1_state = 0; // Open

                // HVBI subsystem availability - not connected in idle
                self.ev_charging.hvbi_dc_bus_availability = 0;
                self.ev_charging.hvbi_off_board_charger_availability = 0;

                // Transition to communication when EVSEC1 ev_ready is set
                if self.ev_charging.evsec1_ev_ready == 1 {
                    self.ev_charging.ev_charging_state = 1;
                }
            }
            1 => {
                // Communication - negotiating parameters
                self.ev_charging.evsedcs1_dc_charging_state = 5; // Authorizing
                self.ev_charging.evsedcs1_status = 1; // Ready
                self.ev_charging.evsedcs1_processing_state = 1; // Ongoing

                // HVBI shows bus is being prepared
                self.ev_charging.hvbi_dc_bus_availability = 1; // Connected
                self.ev_charging.hvbi_off_board_charger_availability = 1;

                // Adjust limits based on SOC and temperature
                let temp = self.hvess.hvess_average_cell_temperature;
                if temp > 45.0 {
                    self.ev_charging.evdclim1_max_current =
                        (self.ev_charging.evdclim1_max_current * 0.7).min(500.0);
                }
                if soc > 80.0 {
                    // Taper current limits at high SOC
                    let taper_factor = 1.0 - (soc - 80.0) / 20.0;
                    self.ev_charging.evdclim1_max_current =
                        (self.ev_charging.evdclim1_max_current * taper_factor.max(0.1)).min(500.0);
                }

                // After communication, move to pre-charge
                self.ev_charging.ev_charging_state = 2;
            }
            2 => {
                // Pre-charge - ramping voltage to match battery
                self.ev_charging.evsedcs1_dc_charging_state = 4; // Power Delivery (Start-up)
                self.ev_charging.evsedcs1_processing_state = 1; // Ongoing

                // Ramp present voltage toward target (battery voltage)
                let target_v = self
                    .hvess
                    .hvess_bus_voltage
                    .min(self.ev_charging.evdctgt_target_voltage);
                let v_diff = target_v - self.ev_charging.evsedcs1_present_voltage;
                if v_diff.abs() > 5.0 {
                    // Ramp at 200V/sec during pre-charge
                    let step = v_diff.clamp(-200.0 * delta_time, 200.0 * delta_time);
                    self.ev_charging.evsedcs1_present_voltage += step;
                } else {
                    self.ev_charging.evsedcs1_present_voltage = target_v;
                }

                // Update contactor input voltage to track
                self.ev_charging.evse1cs1_contactor_input_voltage =
                    self.ev_charging.evsedcs1_present_voltage;

                // HV bus contactors still open during pre-charge
                self.ev_charging.hvbcs1_positive_contactor_states[0] = 0;
                self.ev_charging.hvbcs1_negative_contactor_states[0] = 0;
                self.ev_charging.evse1cs1_contactor_1_state = 0;

                // Transition to charging when voltage is within 20V of target
                if (self.ev_charging.evsedcs1_present_voltage - target_v).abs() < 20.0 {
                    // Close contactors
                    self.ev_charging.hvbcs1_positive_contactor_states[0] = 1; // Closed
                    self.ev_charging.hvbcs1_negative_contactor_states[0] = 1; // Closed
                    self.ev_charging.evse1cs1_contactor_1_state = 1; // Closed
                    self.ev_charging.evse1cs1_charging_bus_voltage = self.hvess.hvess_bus_voltage;
                    self.ev_charging.ev_charging_state = 3;
                }
            }
            3 => {
                // Charging - active power transfer
                self.ev_charging.evsedcs1_dc_charging_state = 1; // Charging
                self.ev_charging.evsedcs1_status = 1; // Ready
                self.ev_charging.evsedcs1_processing_state = 0; // Finished (ready to charge)

                // Contactors closed
                self.ev_charging.hvbcs1_positive_contactor_states[0] = 1;
                self.ev_charging.hvbcs1_negative_contactor_states[0] = 1;
                self.ev_charging.evse1cs1_contactor_1_state = 1;

                // HVBI shows charging subsystems available
                self.ev_charging.hvbi_dc_bus_availability = 1;
                self.ev_charging.hvbi_off_board_charger_availability = 1;

                // Current ramp toward target, constrained by limits
                let max_current = self.ev_charging.evdclim1_max_current;
                let target_current = self.ev_charging.evdctgt_target_current.min(max_current);
                let current_diff = target_current - self.ev_charging.evsedcs1_present_current;
                if current_diff.abs() > 1.0 {
                    // Ramp at 50A/sec
                    let step = current_diff.clamp(-50.0 * delta_time, 50.0 * delta_time);
                    self.ev_charging.evsedcs1_present_current += step;
                } else {
                    self.ev_charging.evsedcs1_present_current = target_current;
                }
                self.ev_charging.evsedcs1_present_current = self
                    .ev_charging
                    .evsedcs1_present_current
                    .clamp(0.0, max_current);

                // Voltage follows target
                self.ev_charging.evsedcs1_present_voltage = self
                    .ev_charging
                    .evdctgt_target_voltage
                    .min(self.ev_charging.evdclim1_max_voltage);
                self.ev_charging.evse1cs1_contactor_input_voltage =
                    self.ev_charging.evsedcs1_present_voltage;
                self.ev_charging.evse1cs1_charging_bus_voltage = self.hvess.hvess_bus_voltage;

                // Limit achieved flags
                self.ev_charging.evsedcs1_voltage_limit_achieved = if (self
                    .ev_charging
                    .evsedcs1_present_voltage
                    - self.ev_charging.evdclim1_max_voltage)
                    .abs()
                    < 5.0
                {
                    1
                } else {
                    0
                };
                self.ev_charging.evsedcs1_current_limit_achieved =
                    if (self.ev_charging.evsedcs1_present_current - max_current).abs() < 1.0 {
                        1
                    } else {
                        0
                    };
                let power = self.ev_charging.evsedcs1_present_voltage
                    * self.ev_charging.evsedcs1_present_current
                    / 1000.0;
                self.ev_charging.evsedcs1_power_limit_achieved =
                    if (power - self.ev_charging.evdclim1_max_power).abs() < 1.0 {
                        1
                    } else {
                        0
                    };

                // SOC-based taper: reduce max current as SOC increases
                if soc > 80.0 {
                    let taper_factor = 1.0 - (soc - 80.0) / 20.0;
                    self.ev_charging.evdclim1_max_current =
                        (500.0 * taper_factor.max(0.1)).clamp(10.0, 500.0);
                }

                // Charging progress tracking
                self.ev_charging.evdccip_bulk_charging_complete =
                    if soc >= self.ev_charging.evdclim2_bulk_soc {
                        1
                    } else {
                        0
                    };
                self.ev_charging.evdccip_full_charging_complete =
                    if soc >= self.ev_charging.evdclim2_full_soc {
                        1
                    } else {
                        0
                    };

                // Estimate time remaining (simplified: energy / power)
                if power > 0.1 {
                    let energy_remaining = self.ev_charging.evdclim2_energy_capacity as f64
                        * (self.ev_charging.evdclim2_full_soc - soc)
                        / 100.0;
                    self.ev_charging.evdccip_full_charge_time_remaining =
                        (energy_remaining / power * 3600.0).max(0.0); // seconds
                    let bulk_energy_remaining = self.ev_charging.evdclim2_energy_capacity as f64
                        * (self.ev_charging.evdclim2_bulk_soc - soc).max(0.0)
                        / 100.0;
                    self.ev_charging.evdccip_bulk_charge_time_remaining =
                        (bulk_energy_remaining / power * 3600.0).max(0.0);
                }

                // Connector temperature rises during charging
                let temp_rise = power * 0.02 * delta_time;
                self.ev_charging.evses2_inlet_connector_temperature = (self
                    .ev_charging
                    .evses2_inlet_connector_temperature
                    + temp_rise
                    - 0.1 * delta_time)
                    .clamp(20.0, 90.0);
                self.ev_charging.evses2_connector_temperature_status =
                    if self.ev_charging.evses2_inlet_connector_temperature > 60.0 {
                        1
                    } else {
                        0
                    };

                // Transition to complete when SOC >= full_soc or ev_ready goes low
                if soc >= self.ev_charging.evdclim2_full_soc
                    || self.ev_charging.evsec1_ev_ready == 0
                {
                    self.ev_charging.ev_charging_state = 4;
                }
            }
            4 => {
                // Complete - charging finished, ramp down
                self.ev_charging.evsedcs1_dc_charging_state = 2; // Standby
                self.ev_charging.evsedcs1_status = 2; // Shutdown
                self.ev_charging.evsedcs1_processing_state = 0; // Finished

                // Ramp down current
                if self.ev_charging.evsedcs1_present_current > 0.5 {
                    self.ev_charging.evsedcs1_present_current -= 100.0 * delta_time; // 100A/sec ramp down
                    self.ev_charging.evsedcs1_present_current =
                        self.ev_charging.evsedcs1_present_current.max(0.0);
                } else {
                    self.ev_charging.evsedcs1_present_current = 0.0;

                    // Open contactors after current reaches zero
                    self.ev_charging.hvbcs1_positive_contactor_states[0] = 0;
                    self.ev_charging.hvbcs1_negative_contactor_states[0] = 0;
                    self.ev_charging.evse1cs1_contactor_1_state = 0;

                    // Ramp down voltage
                    if self.ev_charging.evsedcs1_present_voltage > 10.0 {
                        self.ev_charging.evsedcs1_present_voltage -= 200.0 * delta_time;
                    } else {
                        self.ev_charging.evsedcs1_present_voltage = 0.0;
                        // Return to idle
                        self.ev_charging.ev_charging_state = 0;
                    }
                }

                self.ev_charging.evdccip_full_charging_complete = 1;
                self.ev_charging.evdccip_bulk_charging_complete = 1;

                // HVBI returns to disconnected
                self.ev_charging.hvbi_off_board_charger_availability = 0;
            }
            _ => {
                // Invalid state, reset to idle
                self.ev_charging.ev_charging_state = 0;
            }
        }

        // Update EV charging counters
        self.ev_charging.evdctgt_counter = (self.ev_charging.evdctgt_counter + 1) % 16;
        self.ev_charging.hvbcs1_counter = (self.ev_charging.hvbcs1_counter + 1) % 16;
        self.ev_charging.hvbcs2_counter = (self.ev_charging.hvbcs2_counter + 1) % 16;
        self.ev_charging.hvbcs3_counter = (self.ev_charging.hvbcs3_counter + 1) % 16;
        self.ev_charging.hvbcc1_counter = (self.ev_charging.hvbcc1_counter + 1) % 16;
        self.ev_charging.hvbcc2_counter = (self.ev_charging.hvbcc2_counter + 1) % 16;

        // Update energy tracking
        if self.ev_charging.evsedcs1_present_current > 0.0
            && self.ev_charging.evsedcs1_present_voltage > 0.0
        {
            let power_kw = self.ev_charging.evsedcs1_present_voltage
                * self.ev_charging.evsedcs1_present_current
                / 1000.0;
            self.ev_charging.evei_total_trip_energy_consumed +=
                power_kw * delta_time / 3600.0; // kWh
        }

        // Remaining distance estimate based on SOC
        self.ev_charging.evoi1_estimated_remaining_distance =
            (soc / 100.0 * 500.0).clamp(0.0, 16063.75); // ~500km at 100% SOC
    }
}
