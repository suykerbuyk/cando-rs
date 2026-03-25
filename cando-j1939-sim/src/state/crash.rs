use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CrashState {
    pub crash_detected: bool,
    pub crash_type: u8,
    pub crash_counter: u8,
    pub crash_checksum: u8,
}
