use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_crash_frames(&self, frames: &mut Vec<CanFrame>, device_id: DeviceId) {
        // CN message if crash detected
        if self.crash.crash_detected {
            let cn = CN {
                device_id,
                crash_checksum: self.crash.crash_checksum,
                crash_counter: self.crash.crash_counter,
                crash_type: self.crash.crash_type,
            };

            if let Ok((can_id, data)) = cn.encode()
                && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
            {
                frames.push(frame);
            }
        }
    }
}
