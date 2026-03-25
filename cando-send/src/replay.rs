//! File replay functionality for cando-send
//!
//! Supports replaying CAN frames from log files with rate control:
//! - candump ASCII format: (timestamp) interface CANID#DATA
//! - candump JSON format: structured JSON objects
//!
//! Rate control options:
//! - Fixed rate (messages per second)
//! - Fixed interval between messages
//! - No delay (send all frames immediately)

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::frame::{BuiltFrame, build_frame};
use crate::parser::parse_frame;

/// Replay statistics
#[derive(Debug, Default)]
pub struct ReplayStats {
    /// Total frames read from file
    pub frames_read: usize,
    /// Frames successfully sent
    pub frames_sent: usize,
    /// Frames skipped due to parsing errors
    pub frames_skipped: usize,
    /// Frames skipped due to interface mismatch
    pub interface_mismatch: usize,
}

/// Replay configuration
pub struct ReplayConfig {
    /// Target interface name
    pub interface: String,
    /// Replay rate (messages per second), None = no delay
    pub rate: Option<f64>,
    /// Fixed interval between frames (overrides rate)
    pub interval: Option<Duration>,
    /// Verbose output
    pub verbose: bool,
}

/// File format detection result
#[derive(Debug, PartialEq)]
enum FileFormat {
    CandumpAscii,
    CandumpJson,
}

/// JSON frame structure (from candump JSON output)
#[derive(Debug, Deserialize)]
struct JsonFrame {
    #[allow(dead_code)]
    timestamp: f64,
    interface: String,
    #[allow(dead_code)]
    can_id: String,
    can_id_decimal: u32,
    extended: bool,
    #[allow(dead_code)]
    dlc: u8,
    #[allow(dead_code)]
    data: Vec<u8>,
    data_hex: String,
}

/// Parse a single line from candump ASCII format
///
/// Format: (timestamp) interface CANID#DATA
/// Example: (1699564800.123456) vcan0 123#DEADBEEF
fn parse_candump_line(line: &str) -> Result<(String, String)> {
    // Skip empty lines and comments
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        anyhow::bail!("Empty or comment line");
    }

    // Parse: (timestamp) interface frame_spec
    // We need to extract interface and frame_spec

    // Find the closing parenthesis for timestamp
    let timestamp_end = line
        .find(')')
        .context("Invalid candump format: missing ')'")?;

    // Skip past the timestamp and whitespace
    let after_timestamp = line[timestamp_end + 1..].trim_start();

    // Split by whitespace to get interface and frame_spec
    let mut parts = after_timestamp.splitn(2, char::is_whitespace);

    let interface = parts
        .next()
        .context("Invalid candump format: missing interface")?
        .to_string();

    let frame_spec = parts
        .next()
        .context("Invalid candump format: missing frame specification")?
        .trim()
        .to_string();

    Ok((interface, frame_spec))
}

/// Parse a single line from candump JSON format
fn parse_json_line(line: &str) -> Result<(String, String)> {
    let json_frame: JsonFrame = serde_json::from_str(line).context("Failed to parse JSON frame")?;

    // Reconstruct frame specification from JSON data
    // We need to determine if it's standard or extended ID
    let can_id = if json_frame.extended {
        // Extended ID: 8 hex digits
        format!("{:08X}", json_frame.can_id_decimal)
    } else {
        // Standard ID: 3 hex digits
        format!("{:03X}", json_frame.can_id_decimal)
    };

    // Build frame specification: CANID#DATA
    let frame_spec = format!("{}#{}", can_id, json_frame.data_hex);

    Ok((json_frame.interface, frame_spec))
}

/// Detect file format by examining the first non-comment line
fn detect_format<P: AsRef<Path>>(path: P) -> Result<FileFormat> {
    let file = File::open(path).context("Failed to open file")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.context("Failed to read line")?;
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Check if it looks like JSON
        if trimmed.starts_with('{') {
            return Ok(FileFormat::CandumpJson);
        }

        // Check if it looks like candump ASCII
        if trimmed.starts_with('(') {
            return Ok(FileFormat::CandumpAscii);
        }

        anyhow::bail!("Unable to detect file format from line: {}", trimmed);
    }

    anyhow::bail!("File contains no valid data lines");
}

/// Parse a log file and extract frame specifications
fn parse_log_file<P: AsRef<Path>>(path: P, config: &ReplayConfig) -> Result<Vec<(String, String)>> {
    let format = detect_format(&path)?;

    if config.verbose {
        let format_name = match format {
            FileFormat::CandumpAscii => "candump ASCII",
            FileFormat::CandumpJson => "candump JSON",
        };
        eprintln!("Detected format: {}", format_name);
    }

    let file = File::open(path).context("Failed to open file")?;
    let reader = BufReader::new(file);
    let mut frames = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.context("Failed to read line")?;
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Parse based on detected format
        let result = match format {
            FileFormat::CandumpAscii => parse_candump_line(trimmed),
            FileFormat::CandumpJson => parse_json_line(trimmed),
        };

        match result {
            Ok((interface, frame_spec)) => {
                frames.push((interface, frame_spec));
            }
            Err(e) => {
                if config.verbose {
                    eprintln!("Warning: Skipping line {}: {}", line_num + 1, e);
                }
            }
        }
    }

    Ok(frames)
}

/// Replay frames from a log file
pub fn replay_file<P: AsRef<Path>, F>(
    path: P,
    config: &ReplayConfig,
    mut send_fn: F,
) -> Result<ReplayStats>
where
    F: FnMut(&BuiltFrame) -> Result<()>,
{
    let mut stats = ReplayStats::default();

    // Parse the log file
    let frames = parse_log_file(&path, config)?;
    stats.frames_read = frames.len();

    if config.verbose {
        eprintln!("Loaded {} frames from file", frames.len());
    }

    // Calculate delay between frames
    let delay = if let Some(interval) = config.interval {
        Some(interval)
    } else if let Some(rate) = config.rate {
        if rate > 0.0 {
            Some(Duration::from_secs_f64(1.0 / rate))
        } else {
            None
        }
    } else {
        None
    };

    if config.verbose {
        if let Some(d) = delay {
            eprintln!("Replay delay: {:?} per frame", d);
        } else {
            eprintln!("Replay delay: None (maximum speed)");
        }
    }

    // Replay frames
    for (idx, (interface, frame_spec)) in frames.iter().enumerate() {
        // Check interface match
        if interface != &config.interface {
            stats.interface_mismatch += 1;
            if config.verbose {
                eprintln!(
                    "Skipping frame {} (interface mismatch: {} != {})",
                    idx + 1,
                    interface,
                    config.interface
                );
            }
            continue;
        }

        // Parse and build frame
        match parse_frame(frame_spec) {
            Ok(parsed) => match build_frame(&parsed) {
                Ok(frame) => {
                    // Send the frame
                    if let Err(e) = send_fn(&frame) {
                        if config.verbose {
                            eprintln!("Warning: Failed to send frame {}: {}", idx + 1, e);
                        }
                        stats.frames_skipped += 1;
                    } else {
                        stats.frames_sent += 1;

                        if config.verbose && (idx + 1) % 100 == 0 {
                            eprintln!("Progress: {}/{} frames sent", idx + 1, frames.len());
                        }
                    }
                }
                Err(e) => {
                    if config.verbose {
                        eprintln!("Warning: Failed to build frame {}: {}", idx + 1, e);
                    }
                    stats.frames_skipped += 1;
                }
            },
            Err(e) => {
                if config.verbose {
                    eprintln!("Warning: Failed to parse frame {}: {}", idx + 1, e);
                }
                stats.frames_skipped += 1;
            }
        }

        // Apply rate control delay
        if let Some(d) = delay {
            thread::sleep(d);
        }
    }

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_candump_line_standard() {
        let line = "(1699564800.123456) vcan0 123#DEADBEEF";
        let result = parse_candump_line(line).unwrap();
        assert_eq!(result.0, "vcan0");
        assert_eq!(result.1, "123#DEADBEEF");
    }

    #[test]
    fn test_parse_candump_line_comment() {
        let line = "# This is a comment";
        let result = parse_candump_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_json_line_standard() {
        let line = r#"{"timestamp":1699564800.123456,"interface":"vcan0","can_id":"0x123","can_id_decimal":291,"extended":false,"dlc":4,"data":[222,173,190,239],"data_hex":"DEADBEEF"}"#;
        let result = parse_json_line(line).unwrap();
        assert_eq!(result.0, "vcan0");
        assert_eq!(result.1, "123#DEADBEEF");
    }

    #[test]
    fn test_replay_stats_default() {
        let stats = ReplayStats::default();
        assert_eq!(stats.frames_read, 0);
        assert_eq!(stats.frames_sent, 0);
        assert_eq!(stats.frames_skipped, 0);
        assert_eq!(stats.interface_mismatch, 0);
    }

    #[test]
    fn test_detect_format_candump_ascii() {
        use std::io::Write;
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(temp_file, "(1699564800.123456) vcan0 123#DEADBEEF").unwrap();
        temp_file.flush().unwrap();

        let format = detect_format(temp_file.path()).unwrap();
        assert_eq!(format, FileFormat::CandumpAscii);
    }

    #[test]
    fn test_detect_format_candump_json() {
        use std::io::Write;
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"{{"timestamp":1699564800.123456,"interface":"vcan0","can_id":"0x123","can_id_decimal":291,"extended":false,"dlc":4,"data":[222,173,190,239],"data_hex":"DEADBEEF"}}"#
        )
        .unwrap();
        temp_file.flush().unwrap();

        let format = detect_format(temp_file.path()).unwrap();
        assert_eq!(format, FileFormat::CandumpJson);
    }

    #[test]
    fn test_replay_file_interface_filtering() {
        use std::io::Write;
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(temp_file, "(1699564800.123456) vcan0 123#AA").unwrap();
        writeln!(temp_file, "(1699564800.223456) can0 456#BB").unwrap();
        writeln!(temp_file, "(1699564800.323456) vcan0 789#CC").unwrap();
        temp_file.flush().unwrap();

        let config = ReplayConfig {
            interface: "vcan0".to_string(),
            rate: None,
            interval: None,
            verbose: false,
        };

        let mut sent_count = 0;
        let stats = replay_file(temp_file.path(), &config, |_frame| {
            sent_count += 1;
            Ok(())
        })
        .unwrap();

        assert_eq!(stats.frames_read, 3);
        assert_eq!(stats.frames_sent, 2);
        assert_eq!(stats.interface_mismatch, 1);
        assert_eq!(sent_count, 2);
    }
}
