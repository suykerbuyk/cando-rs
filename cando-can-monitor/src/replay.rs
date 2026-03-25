//! Candump log file replay functionality.
//!
//! This module handles parsing and replaying candump format log files with
//! configurable rate control and looping.

use crate::error::{ReplayError, Result};
use crate::types::{ReplayRate, SourcedFrame};

use socketcan::{CanFrame, EmbeddedFrame, ExtendedId, Id, StandardId};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, trace, warn};

/// Candump log replayer with rate control and looping support.
///
/// Parses candump format log files and replays them with configurable timing.
///
/// # Candump Format
///
/// Expected format (one of the following):
/// ```text
/// (1698765432.123456) can0 18FECA88#A72E007DFFFFFFFF
/// can0 18FECA88 [8] A7 2E 00 7D FF FF FF FF
/// ```
///
/// # Example
///
/// ```no_run
/// use cando_can_monitor::{LogReplayer, ReplayRate};
/// use std::path::PathBuf;
///
/// #[tokio::main]
/// async fn main() {
///     let replayer = LogReplayer::new(
///         PathBuf::from("traffic.log"),
///         ReplayRate::Fixed(100),
///         true, // loop at end
///     ).await.unwrap();
///
///     // Get frames one at a time
///     // while let Some(frame) = replayer.next_frame().await { ... }
/// }
/// ```
pub struct LogReplayer {
    /// Path to the log file (kept for debugging/logging purposes)
    _path: PathBuf,
    /// Parsed log entries
    entries: Vec<LogEntry>,
    /// Current position in the log
    current_index: usize,
    /// Replay rate configuration
    rate: ReplayRate,
    /// Whether to loop back to start at end
    loop_at_end: bool,
    /// Start time of replay (for timing calculations)
    start_time: Option<Instant>,
    /// Total frames replayed (for performance tracking)
    frames_replayed: u64,
    /// Source name for frames
    source_name: String,
}

/// A single entry from a candump log file.
#[derive(Debug, Clone)]
struct LogEntry {
    /// Timestamp (if present in log)
    timestamp: Option<f64>,
    /// Interface name (parsed but not currently used in replay)
    _interface: String,
    /// CAN ID
    can_id: u32,
    /// Frame data
    data: Vec<u8>,
    /// Original line number (for error reporting)
    line_number: usize,
}

impl LogReplayer {
    /// Create a new log replayer by parsing the specified log file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to candump log file
    /// * `rate` - Replay rate configuration
    /// * `loop_at_end` - Whether to loop back to start when reaching end
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - File not found
    /// - File cannot be read
    /// - File has invalid format
    /// - File is empty
    pub async fn new(path: PathBuf, rate: ReplayRate, loop_at_end: bool) -> Result<Self> {
        // Validate rate
        rate.validate()
            .map_err(|_e| ReplayError::InvalidRate { rate: 0 })?;

        // Check file exists
        if !path.exists() {
            return Err(ReplayError::FileNotFound { path: path.clone() }.into());
        }

        info!("Loading candump log: {:?}", path);

        // Parse the log file
        let entries = Self::parse_log_file(&path).await?;

        if entries.is_empty() {
            return Err(ReplayError::EmptyLog { path: path.clone() }.into());
        }

        let source_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("replay")
            .to_string();

        info!(
            "Loaded {} entries from {} (rate: {:?}, loop: {})",
            entries.len(),
            source_name,
            rate,
            loop_at_end
        );

        Ok(Self {
            _path: path,
            entries,
            current_index: 0,
            rate,
            loop_at_end,
            start_time: None,
            frames_replayed: 0,
            source_name,
        })
    }

    /// Parse a candump log file into entries.
    async fn parse_log_file(path: &Path) -> Result<Vec<LogEntry>> {
        let file = File::open(path).map_err(|e| ReplayError::ReadFailed {
            path: path.to_path_buf(),
            source: e,
        })?;

        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for (line_number, line_result) in reader.lines().enumerate() {
            let line = line_result.map_err(|e| ReplayError::ReadFailed {
                path: path.to_path_buf(),
                source: e,
            })?;

            let line_num = line_number + 1; // 1-based for user display

            // Skip empty lines and comments
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            match Self::parse_log_line(&line, line_num) {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    warn!("Skipping malformed line {}: {}", line_num, e);
                    // Continue parsing, don't fail on single bad line
                }
            }
        }

        Ok(entries)
    }

    /// Parse a single candump log line.
    ///
    /// Supports multiple formats:
    /// - `(1698765432.123456) can0 18FECA88#A72E007DFFFFFFFF`
    /// - `can0 18FECA88 [8] A7 2E 00 7D FF FF FF FF`
    /// - `can0 18FECA88#A72E007DFFFFFFFF`
    fn parse_log_line(line: &str, line_number: usize) -> Result<LogEntry> {
        let line = line.trim();

        // Try to parse timestamp if present
        let (timestamp, rest) = if line.starts_with('(') {
            let end_paren = line
                .find(')')
                .ok_or_else(|| ReplayError::invalid_format(line_number, "Missing closing ')'"))?;

            let ts_str = &line[1..end_paren];
            let timestamp = ts_str.parse::<f64>().ok();
            (timestamp, line[end_paren + 1..].trim())
        } else {
            (None, line)
        };

        // Parse interface and CAN ID + data
        let parts: Vec<&str> = rest.split_whitespace().collect();

        if parts.len() < 2 {
            return Err(ReplayError::invalid_format(
                line_number,
                "Expected at least interface and CAN ID",
            )
            .into());
        }

        let interface = parts[0].to_string();
        let can_id_part = parts[1];

        // Check for compact format (CAN_ID#DATA)
        if can_id_part.contains('#') {
            let mut split = can_id_part.split('#');
            let can_id_str = split
                .next()
                .ok_or_else(|| ReplayError::invalid_format(line_number, "Missing CAN ID"))?;
            let data_str = split
                .next()
                .ok_or_else(|| ReplayError::invalid_format(line_number, "Missing data"))?;

            let can_id = u32::from_str_radix(can_id_str, 16).map_err(|_| {
                ReplayError::invalid_format(line_number, format!("Invalid CAN ID: {}", can_id_str))
            })?;

            let data = Self::parse_hex_data(data_str, line_number)?;

            Ok(LogEntry {
                timestamp,
                _interface: interface,
                can_id,
                data,
                line_number,
            })
        } else {
            // Extended format: can0 18FECA88 [8] A7 2E 00 7D ...
            let can_id = u32::from_str_radix(can_id_part, 16).map_err(|_| {
                ReplayError::invalid_format(line_number, format!("Invalid CAN ID: {}", can_id_part))
            })?;

            // Skip DLC if present (e.g., [8])
            let data_start = if parts.len() > 2 && parts[2].starts_with('[') {
                3
            } else {
                2
            };

            if parts.len() <= data_start {
                return Err(ReplayError::invalid_format(line_number, "Missing frame data").into());
            }

            let data_parts = &parts[data_start..];
            let mut data = Vec::new();

            for byte_str in data_parts {
                let byte = u8::from_str_radix(byte_str, 16).map_err(|_| {
                    ReplayError::malformed_frame(
                        line_number,
                        format!("Invalid hex byte: {}", byte_str),
                    )
                })?;
                data.push(byte);
            }

            Ok(LogEntry {
                timestamp,
                _interface: interface,
                can_id,
                data,
                line_number,
            })
        }
    }

    /// Parse compact hex data string (e.g., "A72E007DFFFFFFFF")
    fn parse_hex_data(data_str: &str, line_number: usize) -> Result<Vec<u8>> {
        if !data_str.len().is_multiple_of(2) {
            return Err(ReplayError::malformed_frame(
                line_number,
                "Hex data must have even length",
            )
            .into());
        }

        let mut data = Vec::new();
        for i in (0..data_str.len()).step_by(2) {
            let byte_str = &data_str[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16).map_err(|_| {
                ReplayError::malformed_frame(line_number, format!("Invalid hex byte: {}", byte_str))
            })?;
            data.push(byte);
        }

        Ok(data)
    }

    /// Get the next frame from the replay.
    ///
    /// Handles timing based on the configured replay rate and loops back
    /// to the beginning if `loop_at_end` is true.
    ///
    /// # Returns
    ///
    /// - `Some(SourcedFrame)` - Next frame with appropriate timing
    /// - `None` - End of replay (when not looping)
    pub async fn next_frame(&mut self) -> Option<SourcedFrame> {
        if self.entries.is_empty() {
            return None;
        }

        loop {
            // Check if we've reached the end
            if self.current_index >= self.entries.len() {
                if self.loop_at_end {
                    debug!("Looping back to start of replay");
                    self.current_index = 0;
                    self.start_time = None; // Reset timing for new loop
                } else {
                    info!(
                        "Replay complete: {} frames from {}",
                        self.frames_replayed, self.source_name
                    );
                    return None;
                }
            }

            let entry = &self.entries[self.current_index];

            // Handle timing
            if let Some(start) = self.start_time {
                match self.rate {
                    ReplayRate::Fixed(rate) => {
                        // Calculate expected time for this frame
                        let frames_sent = self.current_index as u64;
                        let expected_duration =
                            Duration::from_millis(frames_sent * 1000 / rate as u64);
                        let elapsed = start.elapsed();

                        if elapsed < expected_duration {
                            let delay = expected_duration - elapsed;
                            sleep(delay).await;
                        }
                    }
                    ReplayRate::Timed => {
                        // Use original timestamps if available
                        if let (Some(prev_ts), Some(curr_ts)) = (
                            self.entries
                                .get(self.current_index.saturating_sub(1))
                                .and_then(|e| e.timestamp),
                            entry.timestamp,
                        ) {
                            let delay_secs = (curr_ts - prev_ts).max(0.0);
                            if delay_secs > 0.0 && delay_secs < 10.0 {
                                // Sanity check
                                sleep(Duration::from_secs_f64(delay_secs)).await;
                            }
                        }
                    }
                }
            } else {
                self.start_time = Some(Instant::now());
            }

            // Create CAN frame - convert u32 to proper ID type
            let can_id = if entry.can_id <= 0x7FF {
                Id::Standard(StandardId::new(entry.can_id as u16).unwrap())
            } else {
                Id::Extended(ExtendedId::new(entry.can_id).unwrap())
            };

            let frame = match CanFrame::new(can_id, &entry.data) {
                Some(f) => f,
                None => {
                    warn!(
                        "Failed to create frame at line {}: invalid data length or ID",
                        entry.line_number
                    );
                    self.current_index += 1;
                    continue; // Skip to next frame
                }
            };

            // Create sourced frame
            let sourced = SourcedFrame::new(frame, self.source_name.clone());

            self.current_index += 1;
            self.frames_replayed += 1;

            // Log progress periodically
            if self.frames_replayed.is_multiple_of(1000) {
                trace!("Replayed {} frames", self.frames_replayed);
            }

            return Some(sourced);
        }
    }

    /// Get replay statistics.
    pub fn stats(&self) -> ReplayStats {
        ReplayStats {
            total_entries: self.entries.len(),
            current_index: self.current_index,
            frames_replayed: self.frames_replayed,
            has_timestamps: self.entries.iter().any(|e| e.timestamp.is_some()),
        }
    }

    /// Reset replay to the beginning.
    pub fn reset(&mut self) {
        self.current_index = 0;
        self.start_time = None;
        debug!("Replay reset to beginning");
    }

    /// Get the total number of entries in the log.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Check if replay has finished (and not looping).
    pub fn is_finished(&self) -> bool {
        !self.loop_at_end && self.current_index >= self.entries.len()
    }
}

/// Statistics about replay progress.
#[derive(Debug, Clone)]
pub struct ReplayStats {
    /// Total number of entries in the log
    pub total_entries: usize,
    /// Current position in the log
    pub current_index: usize,
    /// Total frames replayed (including loops)
    pub frames_replayed: u64,
    /// Whether the log contains timestamps
    pub has_timestamps: bool,
}

impl ReplayStats {
    /// Get the progress percentage (0.0 to 1.0).
    pub fn progress(&self) -> f64 {
        if self.total_entries == 0 {
            0.0
        } else {
            (self.current_index as f64) / (self.total_entries as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_log_line_compact_format() {
        let line = "can0 18FECA88#A72E007DFFFFFFFF";
        let entry = LogReplayer::parse_log_line(line, 1).unwrap();

        assert_eq!(entry._interface, "can0");
        assert_eq!(entry.can_id, 0x18FECA88);
        assert_eq!(
            entry.data,
            vec![0xA7, 0x2E, 0x00, 0x7D, 0xFF, 0xFF, 0xFF, 0xFF]
        );
        assert!(entry.timestamp.is_none());
    }

    #[test]
    fn test_parse_log_line_extended_format() {
        let line = "can0 18FECA88 [8] A7 2E 00 7D FF FF FF FF";
        let entry = LogReplayer::parse_log_line(line, 1).unwrap();

        assert_eq!(entry._interface, "can0");
        assert_eq!(entry.can_id, 0x18FECA88);
        assert_eq!(
            entry.data,
            vec![0xA7, 0x2E, 0x00, 0x7D, 0xFF, 0xFF, 0xFF, 0xFF]
        );
    }

    #[test]
    fn test_parse_log_line_with_timestamp() {
        let line = "(1698765432.123456) can0 18FECA88#A72E007D";
        let entry = LogReplayer::parse_log_line(line, 1).unwrap();

        assert_eq!(entry._interface, "can0");
        assert_eq!(entry.can_id, 0x18FECA88);
        assert_eq!(entry.data, vec![0xA7, 0x2E, 0x00, 0x7D]);
        assert_eq!(entry.timestamp, Some(1698765432.123456));
    }

    #[test]
    fn test_parse_log_line_without_dlc() {
        let line = "vcan0 123 01 02 03";
        let entry = LogReplayer::parse_log_line(line, 1).unwrap();

        assert_eq!(entry._interface, "vcan0");
        assert_eq!(entry.can_id, 0x123);
        assert_eq!(entry.data, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_parse_log_line_invalid() {
        // Missing data
        assert!(LogReplayer::parse_log_line("can0", 1).is_err());

        // Invalid CAN ID
        assert!(LogReplayer::parse_log_line("can0 ZZZZ#0102", 1).is_err());

        // Invalid hex data
        assert!(LogReplayer::parse_log_line("can0 123 [2] ZZ", 1).is_err());

        // Odd length hex in compact format
        assert!(LogReplayer::parse_log_line("can0 123#ABC", 1).is_err());
    }

    #[tokio::test]
    async fn test_log_replayer_basic() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "can0 18FECA88#A72E007D").unwrap();
        writeln!(file, "can0 18F00488#00000000").unwrap();
        writeln!(file, "vcan0 18FECA00#FFFFFFFF").unwrap();
        file.flush().unwrap();

        let mut replayer =
            LogReplayer::new(file.path().to_path_buf(), ReplayRate::Fixed(1000), false)
                .await
                .unwrap();

        assert_eq!(replayer.entry_count(), 3);

        // Get first frame
        let frame1 = replayer.next_frame().await.unwrap();
        assert_eq!(
            frame1.source,
            file.path().file_name().unwrap().to_str().unwrap()
        );

        // Get remaining frames
        let _frame2 = replayer.next_frame().await.unwrap();
        let _frame3 = replayer.next_frame().await.unwrap();

        // Should be finished (no looping)
        assert!(replayer.next_frame().await.is_none());
        assert!(replayer.is_finished());
    }

    #[tokio::test]
    async fn test_log_replayer_looping() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "can0 123#01").unwrap();
        writeln!(file, "can0 456#02").unwrap();
        file.flush().unwrap();

        let mut replayer = LogReplayer::new(
            file.path().to_path_buf(),
            ReplayRate::Fixed(1000),
            true, // loop
        )
        .await
        .unwrap();

        // First pass
        assert!(replayer.next_frame().await.is_some());
        assert!(replayer.next_frame().await.is_some());

        // Should loop back
        assert!(replayer.next_frame().await.is_some());
        assert!(!replayer.is_finished());
    }

    #[tokio::test]
    async fn test_log_replayer_empty_file() {
        let file = NamedTempFile::new().unwrap();

        let result =
            LogReplayer::new(file.path().to_path_buf(), ReplayRate::Fixed(100), false).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_log_replayer_skip_comments() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "# This is a comment").unwrap();
        writeln!(file, "can0 123#01").unwrap();
        writeln!(file).unwrap(); // empty line
        writeln!(file, "can0 456#02").unwrap();
        file.flush().unwrap();

        let replayer = LogReplayer::new(file.path().to_path_buf(), ReplayRate::Fixed(100), false)
            .await
            .unwrap();

        assert_eq!(replayer.entry_count(), 2);
    }

    #[tokio::test]
    async fn test_log_replayer_file_not_found() {
        let result = LogReplayer::new(
            PathBuf::from("/nonexistent/file.log"),
            ReplayRate::Fixed(100),
            false,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_replay_stats() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "can0 123#01").unwrap();
        writeln!(file, "can0 456#02").unwrap();
        writeln!(file, "can0 789#03").unwrap();
        file.flush().unwrap();

        let mut replayer =
            LogReplayer::new(file.path().to_path_buf(), ReplayRate::Fixed(1000), false)
                .await
                .unwrap();

        let stats = replayer.stats();
        assert_eq!(stats.total_entries, 3);
        assert_eq!(stats.current_index, 0);
        assert_eq!(stats.progress(), 0.0);

        replayer.next_frame().await;
        let stats = replayer.stats();
        assert_eq!(stats.current_index, 1);
        assert!((stats.progress() - 0.333).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_replay_reset() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "can0 123#01").unwrap();
        writeln!(file, "can0 456#02").unwrap();
        file.flush().unwrap();

        let mut replayer =
            LogReplayer::new(file.path().to_path_buf(), ReplayRate::Fixed(1000), false)
                .await
                .unwrap();

        replayer.next_frame().await;
        replayer.next_frame().await;
        assert!(replayer.next_frame().await.is_none());

        replayer.reset();
        assert_eq!(replayer.current_index, 0);
        assert!(replayer.next_frame().await.is_some());
    }

    #[test]
    fn test_replay_rate_validation() {
        assert!(ReplayRate::Fixed(100).validate().is_ok());
        assert!(ReplayRate::Fixed(1).validate().is_ok());
        assert!(ReplayRate::Fixed(10000).validate().is_ok());
        assert!(ReplayRate::Timed.validate().is_ok());

        assert!(ReplayRate::Fixed(0).validate().is_err());
        assert!(ReplayRate::Fixed(10001).validate().is_err());
    }
}
