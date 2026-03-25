mod aftertreatment;
mod braking;
mod crash;
mod dcdc;
mod diagnostics;
mod engine;
mod ev_charging;
mod hvess;
mod motor;
mod power_supply;
mod sensors;
mod thermal;
mod transmission;
mod vehicle;

use crate::SimulatorState;
use cando_messages::common::DeviceId;
use socketcan::CanFrame;

impl SimulatorState {
    pub fn generate_can_frames(&self) -> Vec<CanFrame> {
        let mut frames = Vec::new();

        // Convert device_id to DeviceId enum
        // This properly handles all valid device IDs (0x00-0xFF)
        let device_id = DeviceId::from(self.device_id as u8);

        self.generate_crash_frames(&mut frames, device_id);
        self.generate_sensor_frames(&mut frames, device_id);
        self.generate_motor_frames(&mut frames, device_id);
        self.generate_hvess_frames(&mut frames, device_id);
        self.generate_engine_frames(&mut frames, device_id);
        self.generate_transmission_frames(&mut frames, device_id);
        self.generate_braking_frames(&mut frames, device_id);
        self.generate_dcdc_frames(&mut frames, device_id);
        self.generate_power_supply_frames(&mut frames, device_id);
        self.generate_thermal_frames(&mut frames, device_id);
        self.generate_diagnostics_frames(&mut frames, device_id);
        self.generate_vehicle_frames(&mut frames, device_id);
        self.generate_aftertreatment_frames(&mut frames, device_id);
        self.generate_ev_charging_frames(&mut frames, device_id);

        frames
    }
}
