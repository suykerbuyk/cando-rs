//! Message dispatcher for routing decoded CAN messages to registered callbacks.
//!
//! The dispatcher routes messages by device ID (not by interface), allowing
//! multiple callbacks to be registered for the same device and supporting
//! concurrent access from multiple threads.

use crate::types::{DecodedMessage, MessageCallback};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, trace};

/// Message dispatcher that routes decoded messages to registered callbacks.
///
/// The dispatcher maintains a registry of callbacks keyed by device ID.
/// When a message is dispatched, all callbacks registered for that device
/// are invoked.
///
/// # Thread Safety
///
/// The dispatcher uses `Arc<RwLock<>>` internally, making it safe to share
/// across threads. Multiple readers can access the callback registry
/// concurrently, and writes (registrations/unregistrations) are exclusive.
///
/// # Example
///
/// ```
/// use cando_can_monitor::{MessageDispatcher, DecodedMessage};
///
/// let mut dispatcher = MessageDispatcher::new();
///
/// // Register a callback for device 0x88
/// dispatcher.register(0x88, |msg: DecodedMessage| {
///     println!("Device 0x88: {}", msg.message_name);
/// });
///
/// // Dispatch messages to registered callbacks
/// // dispatcher.dispatch(decoded_message);
/// ```
#[derive(Clone)]
pub struct MessageDispatcher {
    /// Callbacks registered by device ID
    callbacks: Arc<RwLock<HashMap<u8, Vec<MessageCallback>>>>,

    /// Wildcard callbacks that receive all messages
    wildcard_callbacks: Arc<RwLock<Vec<MessageCallback>>>,
}

impl MessageDispatcher {
    /// Create a new message dispatcher.
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(RwLock::new(HashMap::new())),
            wildcard_callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a callback for a specific device ID.
    ///
    /// Multiple callbacks can be registered for the same device ID.
    ///
    /// # Arguments
    ///
    /// * `device_id` - The device ID to listen for
    /// * `callback` - Function to call when a message from this device is received
    ///
    /// # Example
    ///
    /// ```
    /// use cando_can_monitor::MessageDispatcher;
    ///
    /// let mut dispatcher = MessageDispatcher::new();
    /// dispatcher.register(0x8A, |msg| {
    ///     println!("Device: 0x{:02X}", msg.device_id);
    /// });
    /// ```
    pub fn register<F>(&mut self, device_id: u8, callback: F)
    where
        F: Fn(DecodedMessage) + Send + Sync + 'static,
    {
        let callback = Arc::new(callback);
        let mut callbacks = self.callbacks.write().unwrap();

        callbacks.entry(device_id).or_default().push(callback);

        debug!(
            "Registered callback for device 0x{:02X} (total: {} callbacks)",
            device_id,
            callbacks.get(&device_id).map(|v| v.len()).unwrap_or(0)
        );
    }

    /// Register a wildcard callback that receives all messages.
    ///
    /// Wildcard callbacks are invoked for every message, regardless of device ID.
    /// This is useful for logging, metrics, or debugging.
    ///
    /// # Example
    ///
    /// ```
    /// use cando_can_monitor::MessageDispatcher;
    ///
    /// let mut dispatcher = MessageDispatcher::new();
    /// dispatcher.register_wildcard(|msg| {
    ///     println!("All messages: 0x{:02X} - {}", msg.device_id, msg.message_name);
    /// });
    /// ```
    pub fn register_wildcard<F>(&mut self, callback: F)
    where
        F: Fn(DecodedMessage) + Send + Sync + 'static,
    {
        let callback = Arc::new(callback);
        let mut wildcards = self.wildcard_callbacks.write().unwrap();
        wildcards.push(callback);

        debug!("Registered wildcard callback (total: {})", wildcards.len());
    }

    /// Unregister all callbacks for a specific device ID.
    ///
    /// # Returns
    ///
    /// The number of callbacks that were removed.
    pub fn unregister(&mut self, device_id: u8) -> usize {
        let mut callbacks = self.callbacks.write().unwrap();

        if let Some(removed) = callbacks.remove(&device_id) {
            let count = removed.len();
            debug!(
                "Unregistered {} callbacks for device 0x{:02X}",
                count, device_id
            );
            count
        } else {
            0
        }
    }

    /// Clear all registered callbacks (both device-specific and wildcard).
    pub fn clear(&mut self) {
        let mut callbacks = self.callbacks.write().unwrap();
        let mut wildcards = self.wildcard_callbacks.write().unwrap();

        let device_count = callbacks.len();
        let wildcard_count = wildcards.len();

        callbacks.clear();
        wildcards.clear();

        debug!(
            "Cleared all callbacks ({} device-specific, {} wildcard)",
            device_count, wildcard_count
        );
    }

    /// Get the number of callbacks registered for a specific device.
    pub fn callback_count(&self, device_id: u8) -> usize {
        let callbacks = self.callbacks.read().unwrap();
        callbacks.get(&device_id).map(|v| v.len()).unwrap_or(0)
    }

    /// Get the total number of wildcard callbacks.
    pub fn wildcard_count(&self) -> usize {
        let wildcards = self.wildcard_callbacks.read().unwrap();
        wildcards.len()
    }

    /// Get the number of devices with registered callbacks.
    pub fn device_count(&self) -> usize {
        let callbacks = self.callbacks.read().unwrap();
        callbacks.len()
    }

    /// Dispatch a decoded message to all registered callbacks.
    ///
    /// Invokes callbacks in this order:
    /// 1. Device-specific callbacks for the message's device ID
    /// 2. Wildcard callbacks
    ///
    /// If a callback panics, the panic is caught and logged, and dispatch
    /// continues to remaining callbacks.
    ///
    /// # Arguments
    ///
    /// * `message` - The decoded message to dispatch
    ///
    /// # Returns
    ///
    /// The number of callbacks that were invoked.
    pub fn dispatch(&self, message: DecodedMessage) -> usize {
        let device_id = message.device_id;
        let mut invoked = 0;

        trace!(
            "Dispatching message from device 0x{:02X}: {}",
            device_id,
            message.message_name
        );

        // Invoke device-specific callbacks
        {
            let callbacks = self.callbacks.read().unwrap();
            if let Some(device_callbacks) = callbacks.get(&device_id) {
                for callback in device_callbacks {
                    if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        callback(message.clone());
                    })) {
                        tracing::error!(
                            "Callback panicked for device 0x{:02X}: {:?}",
                            device_id,
                            e
                        );
                    } else {
                        invoked += 1;
                    }
                }
            }
        }

        // Invoke wildcard callbacks
        {
            let wildcards = self.wildcard_callbacks.read().unwrap();
            for callback in wildcards.iter() {
                if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    callback(message.clone());
                })) {
                    tracing::error!("Wildcard callback panicked: {:?}", e);
                } else {
                    invoked += 1;
                }
            }
        }

        trace!(
            "Dispatched to {} callbacks for device 0x{:02X}",
            invoked,
            device_id
        );

        invoked
    }

    /// Dispatch multiple messages in batch.
    ///
    /// This is more efficient than calling `dispatch` individually when
    /// processing multiple messages.
    ///
    /// # Returns
    ///
    /// The total number of callback invocations across all messages.
    pub fn dispatch_batch(&self, messages: &[DecodedMessage]) -> usize {
        messages.iter().map(|msg| self.dispatch(msg.clone())).sum()
    }
}

impl Default for MessageDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Protocol;
    use chrono::Utc;
    use socketcan::{CanFrame, EmbeddedFrame, ExtendedId, Id};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    fn create_test_message(device_id: u8, message_name: &str) -> DecodedMessage {
        let can_id_raw = 0x18FECA00 | device_id as u32;
        let can_id = Id::Extended(ExtendedId::new(can_id_raw).unwrap());
        let frame = CanFrame::new(can_id, &[1, 2, 3, 4]).unwrap();
        DecodedMessage::new(
            device_id,
            Protocol::J1939,
            message_name.to_string(),
            can_id_raw,
            vec![1, 2, 3, 4],
            Utc::now(),
            "can0".to_string(),
            frame,
        )
    }

    #[test]
    fn test_register_and_dispatch() {
        let mut dispatcher = MessageDispatcher::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        dispatcher.register(0x88, move |msg| {
            assert_eq!(msg.device_id, 0x88);
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        let msg = create_test_message(0x88, "DM01");
        let invoked = dispatcher.dispatch(msg);

        assert_eq!(invoked, 1);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_multiple_callbacks_same_device() {
        let mut dispatcher = MessageDispatcher::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter1 = counter.clone();
        dispatcher.register(0x8A, move |_| {
            counter1.fetch_add(1, Ordering::SeqCst);
        });

        let counter2 = counter.clone();
        dispatcher.register(0x8A, move |_| {
            counter2.fetch_add(10, Ordering::SeqCst);
        });

        let msg = create_test_message(0x8A, "DM01");
        dispatcher.dispatch(msg);

        assert_eq!(counter.load(Ordering::SeqCst), 11); // 1 + 10
        assert_eq!(dispatcher.callback_count(0x8A), 2);
    }

    #[test]
    fn test_different_device_ids() {
        let mut dispatcher = MessageDispatcher::new();
        let counter_88 = Arc::new(AtomicUsize::new(0));
        let counter_8a = Arc::new(AtomicUsize::new(0));

        let c88 = counter_88.clone();
        dispatcher.register(0x88, move |_| {
            c88.fetch_add(1, Ordering::SeqCst);
        });

        let c8a = counter_8a.clone();
        dispatcher.register(0x8A, move |_| {
            c8a.fetch_add(1, Ordering::SeqCst);
        });

        // Dispatch to 0x88
        dispatcher.dispatch(create_test_message(0x88, "DM01"));
        assert_eq!(counter_88.load(Ordering::SeqCst), 1);
        assert_eq!(counter_8a.load(Ordering::SeqCst), 0);

        // Dispatch to 0x8A
        dispatcher.dispatch(create_test_message(0x8A, "DM01"));
        assert_eq!(counter_88.load(Ordering::SeqCst), 1);
        assert_eq!(counter_8a.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_wildcard_callback() {
        let mut dispatcher = MessageDispatcher::new();
        let wildcard_counter = Arc::new(AtomicUsize::new(0));
        let device_counter = Arc::new(AtomicUsize::new(0));

        let wc = wildcard_counter.clone();
        dispatcher.register_wildcard(move |_| {
            wc.fetch_add(1, Ordering::SeqCst);
        });

        let dc = device_counter.clone();
        dispatcher.register(0x88, move |_| {
            dc.fetch_add(1, Ordering::SeqCst);
        });

        // Dispatch to 0x88 - should trigger both
        dispatcher.dispatch(create_test_message(0x88, "DM01"));
        assert_eq!(wildcard_counter.load(Ordering::SeqCst), 1);
        assert_eq!(device_counter.load(Ordering::SeqCst), 1);

        // Dispatch to 0x8A (no device callback) - should trigger wildcard only
        dispatcher.dispatch(create_test_message(0x8A, "DM01"));
        assert_eq!(wildcard_counter.load(Ordering::SeqCst), 2);
        assert_eq!(device_counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_unregister() {
        let mut dispatcher = MessageDispatcher::new();

        dispatcher.register(0x88, |_| {});
        dispatcher.register(0x88, |_| {});
        dispatcher.register(0x8A, |_| {});

        assert_eq!(dispatcher.callback_count(0x88), 2);
        assert_eq!(dispatcher.callback_count(0x8A), 1);

        let removed = dispatcher.unregister(0x88);
        assert_eq!(removed, 2);
        assert_eq!(dispatcher.callback_count(0x88), 0);
        assert_eq!(dispatcher.callback_count(0x8A), 1);

        // Unregister non-existent device
        let removed = dispatcher.unregister(0xFF);
        assert_eq!(removed, 0);
    }

    #[test]
    fn test_clear() {
        let mut dispatcher = MessageDispatcher::new();

        dispatcher.register(0x88, |_| {});
        dispatcher.register(0x8A, |_| {});
        dispatcher.register_wildcard(|_| {});

        assert_eq!(dispatcher.device_count(), 2);
        assert_eq!(dispatcher.wildcard_count(), 1);

        dispatcher.clear();

        assert_eq!(dispatcher.device_count(), 0);
        assert_eq!(dispatcher.wildcard_count(), 0);
    }

    #[test]
    fn test_dispatch_batch() {
        let mut dispatcher = MessageDispatcher::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();

        dispatcher.register_wildcard(move |_| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        let messages = vec![
            create_test_message(0x88, "DM01"),
            create_test_message(0x8A, "DM01"),
            create_test_message(0x82, "DM01"),
        ];

        let invoked = dispatcher.dispatch_batch(&messages);
        assert_eq!(invoked, 3);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_no_callbacks_registered() {
        let dispatcher = MessageDispatcher::new();
        let msg = create_test_message(0x88, "DM01");
        let invoked = dispatcher.dispatch(msg);
        assert_eq!(invoked, 0);
    }

    #[test]
    fn test_callback_panic_handling() {
        let mut dispatcher = MessageDispatcher::new();
        let success_counter = Arc::new(AtomicUsize::new(0));
        let c = success_counter.clone();

        // Register a callback that panics
        dispatcher.register(0x88, |_| {
            panic!("Intentional panic for testing");
        });

        // Register a callback that succeeds
        dispatcher.register(0x88, move |_| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        let msg = create_test_message(0x88, "DM01");

        // Should not panic, and second callback should still execute
        let invoked = dispatcher.dispatch(msg);

        // Only the non-panicking callback counts as invoked
        assert_eq!(invoked, 1);
        assert_eq!(success_counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let mut dispatcher = MessageDispatcher::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();

        dispatcher.register(0x88, move |_| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        let dispatcher_clone = dispatcher.clone();

        // Spawn multiple threads dispatching messages
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let d = dispatcher_clone.clone();
                thread::spawn(move || {
                    for _ in 0..10 {
                        let msg = create_test_message(0x88, "DM01");
                        d.dispatch(msg);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        // 10 threads * 10 messages = 100 invocations
        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn test_default() {
        let dispatcher = MessageDispatcher::default();
        assert_eq!(dispatcher.device_count(), 0);
        assert_eq!(dispatcher.wildcard_count(), 0);
    }
}
