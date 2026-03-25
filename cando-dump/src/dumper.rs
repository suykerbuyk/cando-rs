//! Core CAN dumping functionality
//!
//! This module implements the main CAN frame dumping logic, utilizing
//! the cando-can-monitor library for multi-interface frame reading.

use anyhow::{Context, Result};
use cando_can_monitor::{CanInterfaceConfig, MultiInterfaceReader, SourcedFrame};
use socketcan::{EmbeddedFrame, Id};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;

use crate::filter::FilterSet;
use crate::formatter::Formatter;

/// Statistics tracker for CAN frames
#[derive(Debug, Clone, Default)]
pub struct Statistics {
    /// Total frames received
    pub total_frames: usize,
    /// Total frames filtered out
    pub filtered_frames: usize,
    /// Frames per interface
    pub frames_per_interface: HashMap<String, usize>,
    /// Frames per CAN ID
    pub frames_per_id: HashMap<u32, usize>,
}

impl Statistics {
    /// Create a new statistics tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a frame
    pub fn record_frame(&mut self, frame: &SourcedFrame) {
        self.total_frames += 1;

        // Count by interface
        *self
            .frames_per_interface
            .entry(frame.source.clone())
            .or_insert(0) += 1;

        // Count by CAN ID
        let raw_id = match frame.frame.id() {
            Id::Standard(id) => id.as_raw() as u32,
            Id::Extended(id) => id.as_raw(),
        };
        *self.frames_per_id.entry(raw_id).or_insert(0) += 1;
    }

    /// Record a filtered frame
    pub fn record_filtered(&mut self) {
        self.filtered_frames += 1;
    }

    /// Display statistics summary
    pub fn display(&self) {
        eprintln!("\n=== CAN Frame Statistics ===");
        eprintln!("Total frames received: {}", self.total_frames);
        if self.filtered_frames > 0 {
            eprintln!("Frames filtered out: {}", self.filtered_frames);
        }

        if !self.frames_per_interface.is_empty() {
            eprintln!("\nFrames per interface:");
            let mut interfaces: Vec<_> = self.frames_per_interface.iter().collect();
            interfaces.sort_by_key(|(name, _)| *name);
            for (interface, count) in interfaces {
                eprintln!("  {}: {}", interface, count);
            }
        }

        if !self.frames_per_id.is_empty() {
            eprintln!("\nTop 10 CAN IDs by frame count:");
            let mut ids: Vec<_> = self.frames_per_id.iter().collect();
            ids.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
            for (id, count) in ids.iter().take(10) {
                eprintln!("  0x{:X}: {}", id, count);
            }
        }

        eprintln!("============================\n");
    }
}

/// Configuration for the CAN dumper
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DumperConfig {
    /// List of CAN interfaces to monitor
    pub interfaces: Vec<String>,

    /// Timestamp mode: 'a' (absolute), 'd' (delta), 'z' (zero), 'A' (absolute with date)
    pub timestamp_mode: char,

    /// Optional log file path
    pub log_file: Option<PathBuf>,

    /// Optional limit on number of frames to capture
    pub frame_count: Option<usize>,

    /// Filter set for CAN ID filtering
    pub filter_set: FilterSet,

    /// Color output level (0 = off, 1+ = increasing color)
    pub color_level: u8,

    /// Enable ASCII output mode
    pub ascii_mode: bool,

    /// Silent mode level (0 = normal, 1 = no animation, 2 = silent)
    pub silent_mode: u8,

    /// Timeout in milliseconds (terminate if no frames for this duration)
    pub timeout_ms: Option<u64>,

    /// Enable statistics mode
    pub stats_mode: bool,
}

/// Main CAN frame dumper
pub struct CanDumper {
    /// Multi-interface CAN frame reader
    reader: MultiInterfaceReader,

    /// Formatter for output
    formatter: Box<dyn Formatter>,

    /// Configuration
    config: DumperConfig,

    /// Optional file handle for logging
    log_file: Option<File>,

    /// Start time for relative timestamps
    start_time: std::time::SystemTime,

    /// Last frame time for delta timestamps
    last_frame_time: Option<f64>,

    /// Number of frames processed
    frames_processed: usize,

    /// Number of frames filtered out
    frames_filtered: usize,

    /// Statistics tracker
    statistics: Statistics,
}

impl CanDumper {
    /// Create a new CAN dumper
    pub async fn new(config: DumperConfig, formatter: Box<dyn Formatter>) -> Result<Self> {
        // Open log file if specified
        let log_file = if let Some(path) = &config.log_file {
            let file = File::create(path)
                .with_context(|| format!("Failed to create log file: {}", path.display()))?;

            // Only print if not in silent mode
            if config.silent_mode < 2 {
                eprintln!("Logging to: {}", path.display());
            }
            Some(file)
        } else {
            None
        };

        // Convert string interface names to CanInterfaceConfig
        let configs: Vec<CanInterfaceConfig> = config
            .interfaces
            .iter()
            .map(|name| CanInterfaceConfig::live(name.clone()))
            .collect();

        // Create multi-interface reader
        let reader = MultiInterfaceReader::new(configs)
            .await
            .context("Failed to create multi-interface CAN reader")?;

        // Only print if not in silent mode
        if config.silent_mode < 2 {
            eprintln!("Monitoring interfaces: {}", config.interfaces.join(", "));

            if !config.filter_set.is_empty() {
                eprintln!(
                    "Active filters on {} interface(s)",
                    config.filter_set.interface_count()
                );
            }
        }

        Ok(Self {
            reader,
            formatter,
            config,
            log_file,
            start_time: std::time::SystemTime::now(),
            last_frame_time: None,
            frames_processed: 0,
            frames_filtered: 0,
            statistics: Statistics::new(),
        })
    }

    /// Run the dumper until completion or error
    pub async fn run(&mut self) -> Result<()> {
        loop {
            // Check if we've reached the frame limit
            if let Some(limit) = self.config.frame_count
                && self.frames_processed >= limit {
                    if self.config.silent_mode < 2 {
                        eprintln!("Frame limit reached ({}), stopping...", limit);
                    }
                    break;
                }

            // Read next frame with optional timeout
            let sourced_frame = if let Some(timeout_ms) = self.config.timeout_ms {
                match tokio::time::timeout(
                    Duration::from_millis(timeout_ms),
                    self.reader.next_frame(),
                )
                .await
                {
                    Ok(Some(frame)) => frame,
                    Ok(None) => {
                        if self.config.silent_mode < 2 {
                            eprintln!("All interfaces finished");
                        }
                        break;
                    }
                    Err(_) => {
                        if self.config.silent_mode < 2 {
                            eprintln!("Timeout: No frames received for {} ms", timeout_ms);
                        }
                        break;
                    }
                }
            } else {
                match self.reader.next_frame().await {
                    Some(frame) => frame,
                    None => {
                        if self.config.silent_mode < 2 {
                            eprintln!("All interfaces finished");
                        }
                        break;
                    }
                }
            };

            // Apply filters
            if !self
                .config
                .filter_set
                .matches(&sourced_frame.source, &sourced_frame.frame)
            {
                self.frames_filtered += 1;
                self.statistics.record_filtered();
                continue; // Skip this frame
            }

            // Calculate timestamp based on mode
            let timestamp = self.calculate_timestamp(&sourced_frame);

            // Record statistics
            self.statistics.record_frame(&sourced_frame);

            // Format the frame
            let formatted = self.formatter.format_frame(&sourced_frame, timestamp);

            // Write to stdout
            println!("{}", formatted);
            io::stdout().flush().context("Failed to flush stdout")?;

            // Write to log file if configured
            if let Some(ref mut file) = self.log_file {
                writeln!(file, "{}", formatted).context("Failed to write to log file")?;
            }

            self.frames_processed += 1;
        }

        Ok(())
    }

    /// Display statistics summary
    pub fn display_stats(&self) {
        if self.config.stats_mode {
            self.statistics.display();
        }
    }

    /// Calculate timestamp based on configured mode
    fn calculate_timestamp(&mut self, frame: &SourcedFrame) -> f64 {
        // Convert DateTime<Utc> to Unix timestamp
        let current_time = frame.timestamp.timestamp() as f64
            + frame.timestamp.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;

        match self.config.timestamp_mode {
            'a' | 'A' => {
                // Absolute timestamp
                self.last_frame_time = Some(current_time);
                current_time
            }
            'd' => {
                // Delta timestamp (time since last frame)
                let delta = if let Some(last) = self.last_frame_time {
                    current_time - last
                } else {
                    0.0
                };
                self.last_frame_time = Some(current_time);
                delta
            }
            'z' => {
                // Zero timestamp (time since start)
                let elapsed = self
                    .start_time
                    .elapsed()
                    .unwrap_or(std::time::Duration::from_secs(0));
                elapsed.as_secs_f64()
            }
            _ => {
                // Default to absolute (should have been caught in validation)
                current_time
            }
        }
    }

    /// Get number of frames processed so far
    #[allow(dead_code)]
    pub fn frames_processed(&self) -> usize {
        self.frames_processed
    }

    /// Get number of frames filtered out
    #[allow(dead_code)]
    pub fn frames_filtered(&self) -> usize {
        self.frames_filtered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::FilterSet;

    #[test]
    fn test_config_creation() {
        let config = DumperConfig {
            interfaces: vec!["vcan0".to_string()],
            timestamp_mode: 'a',
            log_file: None,
            frame_count: Some(100),
            filter_set: FilterSet::new(),
            color_level: 0,
            ascii_mode: false,
            silent_mode: 0,
            timeout_ms: None,
            stats_mode: false,
        };

        assert_eq!(config.interfaces.len(), 1);
        assert_eq!(config.timestamp_mode, 'a');
        assert_eq!(config.frame_count, Some(100));
    }

    #[test]
    fn test_config_with_multiple_interfaces() {
        let config = DumperConfig {
            interfaces: vec!["vcan0".to_string(), "vcan1".to_string()],
            timestamp_mode: 'd',
            log_file: Some(PathBuf::from("test.log")),
            frame_count: None,
            filter_set: FilterSet::new(),
            color_level: 0,
            ascii_mode: false,
            silent_mode: 0,
            timeout_ms: None,
            stats_mode: false,
        };

        assert_eq!(config.interfaces.len(), 2);
        assert_eq!(config.log_file, Some(PathBuf::from("test.log")));
    }

    // Note: Integration tests for the actual dumper will be in tests/
    // as they require actual CAN interfaces or mocking
}
