//! Message Tracking Module
//!
//! Provides standardized message tracking infrastructure for simulator test verification.
//!
//! This module enables deterministic message verification in integration tests by tracking
//! CAN messages as they are received and processed by simulators. Tests can use the
//! `WaitForMessage` WebSocket command to verify messages were received and processed
//! instead of using arbitrary time-based delays.
//!
//! # Architecture
//!
//! - `ReceivedMessage`: Tracks individual CAN messages with timestamp and processing status
//! - `MessageTracking`: Trait providing standard message tracking operations
//!
//! # Usage
//!
//! Simulators implement the `MessageTracking` trait and call:
//! - `record_message()` when a CAN message is received
//! - `mark_last_message_processed()` after successfully decoding
//! - `find_message()` to search message history
//!
//! # Example
//!
//! ```rust
//! use std::collections::VecDeque;
//! use std::time::Instant;
//! use cando_simulator_common::message_tracking::{ReceivedMessage, MessageTracking};
//!
//! struct MySimulator {
//!     recent_messages: VecDeque<ReceivedMessage>,
//!     start_time: Instant,
//! }
//!
//! impl MessageTracking for MySimulator {
//!     fn get_recent_messages(&self) -> &VecDeque<ReceivedMessage> {
//!         &self.recent_messages
//!     }
//!
//!     fn get_recent_messages_mut(&mut self) -> &mut VecDeque<ReceivedMessage> {
//!         &mut self.recent_messages
//!     }
//!
//!     fn get_simulator_start_time(&self) -> Instant {
//!         self.start_time
//!     }
//! }
//!
//! let mut sim = MySimulator {
//!     recent_messages: VecDeque::new(),
//!     start_time: Instant::now(),
//! };
//!
//! // Record a message
//! sim.record_message(0x18FCCC0F);
//!
//! // Mark it as processed
//! sim.mark_last_message_processed();
//!
//! // Find it
//! assert!(sim.find_message(0x18FCCC0F).is_some());
//! ```

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Instant;

/// Default maximum number of messages to track in history
pub const DEFAULT_MESSAGE_BUFFER_SIZE: usize = 100;

/// A received CAN message tracked for test verification
///
/// This struct records metadata about CAN messages as they are received by the simulator,
/// enabling deterministic test verification without time-based delays.
///
/// # Fields
///
/// - `can_id`: The complete 29-bit CAN ID (including device ID)
/// - `timestamp_ms`: Milliseconds since simulator start when message was received
/// - `processed`: Whether the message was successfully decoded and applied to state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReceivedMessage {
    /// The CAN ID of the received message (29-bit extended format)
    pub can_id: u32,

    /// Timestamp when the message was received (milliseconds since simulator start)
    pub timestamp_ms: u64,

    /// Whether the message was successfully processed (decoded and state updated)
    pub processed: bool,
}

impl ReceivedMessage {
    /// Create a new received message record
    ///
    /// # Arguments
    ///
    /// * `can_id` - The CAN ID of the received message
    /// * `timestamp_ms` - Milliseconds since simulator start
    ///
    /// # Example
    ///
    /// ```
    /// use cando_simulator_common::message_tracking::ReceivedMessage;
    ///
    /// let msg = ReceivedMessage::new(0x18FCCC0F, 1234);
    /// assert_eq!(msg.can_id, 0x18FCCC0F);
    /// assert_eq!(msg.timestamp_ms, 1234);
    /// assert_eq!(msg.processed, false);
    /// ```
    pub fn new(can_id: u32, timestamp_ms: u64) -> Self {
        Self {
            can_id,
            timestamp_ms,
            processed: false,
        }
    }
}

/// Trait for simulators that support message tracking
///
/// Provides standardized message tracking operations for test verification.
/// Simulators implement this trait to enable deterministic message verification
/// in integration tests via the `WaitForMessage` WebSocket command.
///
/// # Required Methods
///
/// Implementors must provide access to:
/// - Recent message buffer (VecDeque<ReceivedMessage>)
/// - Simulator start time (for timestamp calculation)
///
/// # Provided Methods
///
/// The trait provides default implementations for:
/// - `record_message()`: Add message to tracking buffer
/// - `mark_last_message_processed()`: Mark most recent message as decoded
/// - `find_message()`: Search for a specific CAN ID in message history
/// - `clear_message_history()`: Clear all tracked messages
///
/// # Example
///
/// ```
/// use std::collections::VecDeque;
/// use std::time::Instant;
/// use cando_simulator_common::message_tracking::{ReceivedMessage, MessageTracking};
///
/// struct Simulator {
///     recent_messages: VecDeque<ReceivedMessage>,
///     start_time: Instant,
/// }
///
/// impl MessageTracking for Simulator {
///     fn get_recent_messages(&self) -> &VecDeque<ReceivedMessage> {
///         &self.recent_messages
///     }
///
///     fn get_recent_messages_mut(&mut self) -> &mut VecDeque<ReceivedMessage> {
///         &mut self.recent_messages
///     }
///
///     fn get_simulator_start_time(&self) -> Instant {
///         self.start_time
///     }
/// }
/// ```
pub trait MessageTracking {
    /// Get immutable reference to recent messages buffer
    fn get_recent_messages(&self) -> &VecDeque<ReceivedMessage>;

    /// Get mutable reference to recent messages buffer
    fn get_recent_messages_mut(&mut self) -> &mut VecDeque<ReceivedMessage>;

    /// Get the simulator start time for timestamp calculation
    fn get_simulator_start_time(&self) -> Instant;

    /// Get the maximum buffer size (default: 100)
    fn get_max_buffer_size(&self) -> usize {
        DEFAULT_MESSAGE_BUFFER_SIZE
    }

    /// Record a received CAN message
    ///
    /// Adds the message to the tracking buffer with the current timestamp.
    /// If the buffer exceeds max size, the oldest message is removed.
    ///
    /// Call this method when a CAN frame is received, before attempting to decode it.
    ///
    /// # Arguments
    ///
    /// * `can_id` - The CAN ID of the received message
    fn record_message(&mut self, can_id: u32) {
        let timestamp_ms = self.get_simulator_start_time().elapsed().as_millis() as u64;
        let max_size = self.get_max_buffer_size();
        let messages = self.get_recent_messages_mut();

        messages.push_back(ReceivedMessage::new(can_id, timestamp_ms));

        // Maintain buffer size limit
        while messages.len() > max_size {
            messages.pop_front();
        }
    }

    /// Mark the most recently recorded message as successfully processed
    ///
    /// Call this method after successfully decoding a message and updating simulator state.
    /// This indicates to tests that the message was not just received, but also processed.
    fn mark_last_message_processed(&mut self) {
        if let Some(msg) = self.get_recent_messages_mut().back_mut() {
            msg.processed = true;
        }
    }

    /// Find a processed message by CAN ID
    ///
    /// Searches the message history (most recent first) for a message with the given
    /// CAN ID that has been marked as processed. Returns the first match found.
    ///
    /// This is used by the `WaitForMessage` WebSocket command to verify message reception.
    ///
    /// # Arguments
    ///
    /// * `can_id` - The CAN ID to search for
    ///
    /// # Returns
    ///
    /// - `Some(&ReceivedMessage)` if a processed message with this CAN ID exists
    /// - `None` if no matching processed message is found
    fn find_message(&self, can_id: u32) -> Option<&ReceivedMessage> {
        self.get_recent_messages()
            .iter()
            .rev() // Search most recent first
            .find(|msg| msg.can_id == can_id && msg.processed)
    }

    /// Clear all tracked messages
    ///
    /// Removes all messages from the tracking buffer. Useful for test isolation
    /// when resetting simulator state between tests.
    fn clear_message_history(&mut self) {
        self.get_recent_messages_mut().clear();
    }

    /// Get count of tracked messages
    ///
    /// Returns the current number of messages in the tracking buffer.
    fn message_count(&self) -> usize {
        self.get_recent_messages().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test simulator implementation
    struct TestSimulator {
        recent_messages: VecDeque<ReceivedMessage>,
        start_time: Instant,
    }

    impl TestSimulator {
        fn new() -> Self {
            Self {
                recent_messages: VecDeque::new(),
                start_time: Instant::now(),
            }
        }
    }

    impl MessageTracking for TestSimulator {
        fn get_recent_messages(&self) -> &VecDeque<ReceivedMessage> {
            &self.recent_messages
        }

        fn get_recent_messages_mut(&mut self) -> &mut VecDeque<ReceivedMessage> {
            &mut self.recent_messages
        }

        fn get_simulator_start_time(&self) -> Instant {
            self.start_time
        }
    }

    #[test]
    fn test_received_message_new() {
        let msg = ReceivedMessage::new(0x18FCCC0F, 1234);
        assert_eq!(msg.can_id, 0x18FCCC0F);
        assert_eq!(msg.timestamp_ms, 1234);
        assert!(!msg.processed);
    }

    #[test]
    fn test_record_message() {
        let mut sim = TestSimulator::new();

        sim.record_message(0x18FCCC0F);

        assert_eq!(sim.message_count(), 1);
        let msg = &sim.get_recent_messages()[0];
        assert_eq!(msg.can_id, 0x18FCCC0F);
        assert!(!msg.processed);
    }

    #[test]
    fn test_mark_last_message_processed() {
        let mut sim = TestSimulator::new();

        sim.record_message(0x18FCCC0F);
        assert!(!sim.get_recent_messages()[0].processed);

        sim.mark_last_message_processed();
        assert!(sim.get_recent_messages()[0].processed);
    }

    #[test]
    fn test_mark_processed_empty_buffer() {
        let mut sim = TestSimulator::new();

        // Should not panic on empty buffer
        sim.mark_last_message_processed();
        assert_eq!(sim.message_count(), 0);
    }

    #[test]
    fn test_find_message_found() {
        let mut sim = TestSimulator::new();

        sim.record_message(0x18FCCC0F);
        sim.mark_last_message_processed();

        let found = sim.find_message(0x18FCCC0F);
        assert!(found.is_some());
        assert_eq!(found.unwrap().can_id, 0x18FCCC0F);
    }

    #[test]
    fn test_find_message_not_found() {
        let mut sim = TestSimulator::new();

        sim.record_message(0x18FCCC0F);
        sim.mark_last_message_processed();

        let found = sim.find_message(0x18FCCC10);
        assert!(found.is_none());
    }

    #[test]
    fn test_find_message_not_processed() {
        let mut sim = TestSimulator::new();

        sim.record_message(0x18FCCC0F);
        // Don't mark as processed

        let found = sim.find_message(0x18FCCC0F);
        assert!(found.is_none()); // Should not find unprocessed messages
    }

    #[test]
    fn test_buffer_size_limit() {
        let mut sim = TestSimulator::new();

        // Add more than max buffer size
        for i in 0..150 {
            sim.record_message(0x18FCCC00 + i);
        }

        assert_eq!(sim.message_count(), 100); // Should be limited to 100

        // Oldest messages should be removed
        let first_msg = &sim.get_recent_messages()[0];
        assert_eq!(first_msg.can_id, 0x18FCCC00 + 50); // First 50 were removed
    }

    #[test]
    fn test_clear_message_history() {
        let mut sim = TestSimulator::new();

        sim.record_message(0x18FCCC0F);
        sim.record_message(0x18FCCC10);
        assert_eq!(sim.message_count(), 2);

        sim.clear_message_history();
        assert_eq!(sim.message_count(), 0);
    }

    #[test]
    fn test_multiple_messages_same_id() {
        let mut sim = TestSimulator::new();

        // Record same CAN ID twice
        sim.record_message(0x18FCCC0F);
        sim.mark_last_message_processed();

        std::thread::sleep(std::time::Duration::from_millis(10));

        sim.record_message(0x18FCCC0F);
        sim.mark_last_message_processed();

        // Should find most recent one
        let found = sim.find_message(0x18FCCC0F);
        assert!(found.is_some());

        // Should have 2 messages
        assert_eq!(sim.message_count(), 2);
    }

    #[test]
    fn test_message_timestamp_increases() {
        let mut sim = TestSimulator::new();

        sim.record_message(0x18FCCC0F);
        let ts1 = sim.get_recent_messages()[0].timestamp_ms;

        std::thread::sleep(std::time::Duration::from_millis(10));

        sim.record_message(0x18FCCC10);
        let ts2 = sim.get_recent_messages()[1].timestamp_ms;

        assert!(ts2 > ts1, "Second message should have later timestamp");
    }

    #[test]
    fn test_message_tracking_workflow() {
        let mut sim = TestSimulator::new();

        // Simulate receiving a CAN message
        let can_id = 0x18FCCC0F;
        sim.record_message(can_id);

        // Verify not yet found (not processed)
        assert!(sim.find_message(can_id).is_none());

        // Simulate successful decode
        sim.mark_last_message_processed();

        // Now should be found
        let found = sim.find_message(can_id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().can_id, can_id);
        assert!(found.unwrap().processed);
    }
}
