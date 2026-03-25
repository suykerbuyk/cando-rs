//! Timestamp utilities for CAN frame timing
//!
//! This module provides utilities for handling various timestamp modes
//! and time conversions used by cando-dump.

use std::time::SystemTime;

/// Valid timestamp modes for cando-dump
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TimestampMode {
    /// Absolute timestamp (seconds since epoch)
    Absolute,
    /// Delta timestamp (time since last frame)
    Delta,
    /// Zero-based timestamp (time since start)
    Zero,
    /// Absolute timestamp with date formatting
    AbsoluteWithDate,
}

impl TimestampMode {
    /// Parse timestamp mode from character
    ///
    /// # Arguments
    /// * `c` - Character representing the mode ('a', 'd', 'z', 'A')
    ///
    /// # Returns
    /// Some(TimestampMode) if valid, None otherwise
    #[allow(dead_code)]
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a' => Some(TimestampMode::Absolute),
            'd' => Some(TimestampMode::Delta),
            'z' => Some(TimestampMode::Zero),
            'A' => Some(TimestampMode::AbsoluteWithDate),
            _ => None,
        }
    }

    /// Convert timestamp mode to character
    #[allow(dead_code)]
    pub fn to_char(self) -> char {
        match self {
            TimestampMode::Absolute => 'a',
            TimestampMode::Delta => 'd',
            TimestampMode::Zero => 'z',
            TimestampMode::AbsoluteWithDate => 'A',
        }
    }

    /// Get description of the timestamp mode
    #[allow(dead_code)]
    pub fn description(self) -> &'static str {
        match self {
            TimestampMode::Absolute => "Absolute timestamp (seconds since epoch)",
            TimestampMode::Delta => "Delta timestamp (time since last frame)",
            TimestampMode::Zero => "Zero-based timestamp (time since start)",
            TimestampMode::AbsoluteWithDate => "Absolute timestamp with date",
        }
    }
}

/// Convert SystemTime to floating-point seconds since epoch
#[allow(dead_code)]
pub fn system_time_to_f64(time: SystemTime) -> f64 {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(_) => 0.0,
    }
}

/// Format a timestamp as a string with microsecond precision
#[allow(dead_code)]
pub fn format_timestamp(timestamp: f64, precision: usize) -> String {
    format!("{:.prec$}", timestamp, prec = precision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_mode_from_char() {
        assert_eq!(TimestampMode::from_char('a'), Some(TimestampMode::Absolute));
        assert_eq!(TimestampMode::from_char('d'), Some(TimestampMode::Delta));
        assert_eq!(TimestampMode::from_char('z'), Some(TimestampMode::Zero));
        assert_eq!(
            TimestampMode::from_char('A'),
            Some(TimestampMode::AbsoluteWithDate)
        );
        assert_eq!(TimestampMode::from_char('x'), None);
    }

    #[test]
    fn test_timestamp_mode_to_char() {
        assert_eq!(TimestampMode::Absolute.to_char(), 'a');
        assert_eq!(TimestampMode::Delta.to_char(), 'd');
        assert_eq!(TimestampMode::Zero.to_char(), 'z');
        assert_eq!(TimestampMode::AbsoluteWithDate.to_char(), 'A');
    }

    #[test]
    fn test_timestamp_mode_roundtrip() {
        for mode_char in ['a', 'd', 'z', 'A'] {
            let mode = TimestampMode::from_char(mode_char).unwrap();
            assert_eq!(mode.to_char(), mode_char);
        }
    }

    #[test]
    fn test_system_time_to_f64() {
        let now = SystemTime::now();
        let timestamp = system_time_to_f64(now);

        // Should be a reasonable timestamp (after 2020-01-01)
        assert!(timestamp > 1577836800.0);

        // Should have fractional seconds
        assert_ne!(timestamp.fract(), 0.0);
    }

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(1234567890.123456, 6), "1234567890.123456");
        assert_eq!(format_timestamp(1234567890.123456, 3), "1234567890.123");
        assert_eq!(format_timestamp(0.001234, 6), "0.001234");
        assert_eq!(format_timestamp(123.456789, 6), "123.456789");
    }

    #[test]
    fn test_timestamp_mode_description() {
        assert!(!TimestampMode::Absolute.description().is_empty());
        assert!(!TimestampMode::Delta.description().is_empty());
        assert!(!TimestampMode::Zero.description().is_empty());
        assert!(!TimestampMode::AbsoluteWithDate.description().is_empty());
    }
}
