use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle CN - Crash Notification
    pub(crate) fn handle_cn(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // CN - Crash Notification
        // FIXED: Was incorrectly matching on CCVS1 range (0x18FEF100)
        // Now correctly matches on CN::BASE_CAN_ID (0x00F02B00)
        // This allows external systems to trigger crash simulation
        match CN::decode(can_id, data) {
            Ok(msg) => {
                self.crash.crash_detected = true;
                self.crash.crash_type = msg.crash_type;
                println!("🚨 Received CN: Crash type = {}", self.crash.crash_type);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode CN: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }
}
