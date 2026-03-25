//! Multi-interface CAN reader with async frame aggregation.
//!
//! This module handles reading CAN frames from multiple interfaces simultaneously
//! (both live SocketCAN and log replay) and aggregating them into a single stream.

use crate::error::{InterfaceError, Result};
use crate::replay::LogReplayer;
use crate::types::{CanInterfaceConfig, SourcedFrame};
use socketcan::{CanSocket, Frame, Socket};
use std::io::ErrorKind;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use tracing::{debug, error, info, trace, warn};

/// Multi-interface CAN reader that aggregates frames from multiple sources.
///
/// Spawns separate tasks for each interface (live or replay) and aggregates
/// frames into a single async stream. Handles interface failures gracefully
/// by continuing to read from remaining interfaces.
///
/// # Example
///
/// ```no_run
/// use cando_can_monitor::{MultiInterfaceReader, CanInterfaceConfig};
///
/// #[tokio::main]
/// async fn main() {
///     let configs = vec![
///         CanInterfaceConfig::live("can0"),
///         CanInterfaceConfig::live("vcan0"),
///     ];
///
///     let mut reader = MultiInterfaceReader::new(configs).await.unwrap();
///
///     while let Some(frame) = reader.next_frame().await {
///         println!("Frame from {}: {:?}", frame.source, frame.frame);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct MultiInterfaceReader {
    /// Channel receiver for aggregated frames
    receiver: mpsc::Receiver<SourcedFrame>,
    /// Task handles for interface readers
    tasks: Vec<JoinHandle<()>>,
    /// Number of active interfaces
    active_count: usize,
}

impl MultiInterfaceReader {
    /// Create a new multi-interface reader.
    ///
    /// # Arguments
    ///
    /// * `configs` - List of interface configurations to monitor
    ///
    /// # Errors
    ///
    /// Returns error if all interfaces fail to initialize.
    pub async fn new(configs: Vec<CanInterfaceConfig>) -> Result<Self> {
        if configs.is_empty() {
            warn!("No interfaces configured");
        }

        // Create channel for frame aggregation
        let (sender, receiver) = mpsc::channel::<SourcedFrame>(1000);

        let mut tasks = Vec::new();
        let mut failed_count = 0;

        // Spawn a task for each interface
        for config in &configs {
            let sender = sender.clone();
            let _config_name = config.name();

            let task = match config.clone() {
                CanInterfaceConfig::Live { interface } => {
                    match Self::spawn_live_reader(interface.clone(), sender).await {
                        Ok(task) => {
                            info!("Started live reader for {}", interface);
                            Some(task)
                        }
                        Err(e) => {
                            error!("Failed to start live reader for {}: {}", interface, e);
                            failed_count += 1;
                            None
                        }
                    }
                }
                CanInterfaceConfig::Replay {
                    path,
                    rate,
                    loop_at_end,
                } => {
                    match Self::spawn_replay_reader(path.clone(), rate, loop_at_end, sender).await {
                        Ok(task) => {
                            info!("Started replay reader for {:?}", path);
                            Some(task)
                        }
                        Err(e) => {
                            error!("Failed to start replay reader for {:?}: {}", path, e);
                            failed_count += 1;
                            None
                        }
                    }
                }
            };

            if let Some(task) = task {
                tasks.push(task);
            }
        }

        let active_count = tasks.len();

        if active_count == 0 && !configs.is_empty() {
            return Err(InterfaceError::AllInterfacesFailed {
                count: configs.len(),
            }
            .into());
        }

        if failed_count > 0 {
            warn!(
                "{} of {} interfaces failed to initialize (continuing with {})",
                failed_count,
                configs.len(),
                active_count
            );
        }

        info!(
            "MultiInterfaceReader started with {} active interfaces",
            active_count
        );

        Ok(Self {
            receiver,
            tasks,
            active_count,
        })
    }

    /// Spawn a task to read from a live SocketCAN interface.
    async fn spawn_live_reader(
        interface: String,
        sender: mpsc::Sender<SourcedFrame>,
    ) -> Result<JoinHandle<()>> {
        // Try to open the socket (blocking operation)
        let socket = Arc::new(
            tokio::task::spawn_blocking({
                let interface = interface.clone();
                move || {
                    CanSocket::open(&interface).map_err(|e| {
                        // Try to provide helpful error messages
                        match e.kind() {
                            ErrorKind::NotFound => InterfaceError::NotFound {
                                interface: interface.clone(),
                            },
                            ErrorKind::PermissionDenied => InterfaceError::PermissionDenied {
                                interface: interface.clone(),
                            },
                            _ => InterfaceError::OpenFailed {
                                interface: interface.clone(),
                                source: e,
                            },
                        }
                    })
                }
            })
            .await
            .map_err(|e| InterfaceError::OpenFailed {
                interface: interface.clone(),
                source: std::io::Error::other(e.to_string()),
            })??,
        );

        // Set non-blocking mode
        socket
            .as_ref()
            .set_nonblocking(true)
            .map_err(|e| InterfaceError::OpenFailed {
                interface: interface.clone(),
                source: e,
            })?;

        debug!("Opened CAN socket for {}", interface);

        // Spawn task to read frames
        let task = tokio::spawn(async move {
            Self::read_live_frames(interface, socket, sender).await;
        });

        Ok(task)
    }

    /// Read frames from a live CAN interface.
    async fn read_live_frames(
        interface: String,
        socket: Arc<CanSocket>,
        sender: mpsc::Sender<SourcedFrame>,
    ) {
        let mut consecutive_errors = 0;
        const MAX_CONSECUTIVE_ERRORS: u32 = 10;

        info!("Reading frames from live interface: {}", interface);

        loop {
            // Read with timeout (non-blocking socket + tokio)
            let result = tokio::task::spawn_blocking({
                let socket = Arc::clone(&socket);
                move || socket.read_frame()
            })
            .await;

            match result {
                Ok(Ok(frame)) => {
                    consecutive_errors = 0;

                    let sourced = SourcedFrame::new(frame, interface.clone());

                    trace!(
                        "Read frame from {}: CAN ID 0x{:08X}",
                        interface,
                        sourced.frame.raw_id()
                    );

                    if sender.send(sourced).await.is_err() {
                        debug!("Channel closed, stopping reader for {}", interface);
                        break;
                    }
                }
                Ok(Err(e)) if e.kind() == ErrorKind::WouldBlock => {
                    // No data available, sleep briefly
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                Ok(Err(e)) => {
                    consecutive_errors += 1;
                    warn!(
                        "Error reading from {} ({}): {}",
                        interface, consecutive_errors, e
                    );

                    if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                        error!(
                            "Too many consecutive errors on {}, stopping reader",
                            interface
                        );
                        break;
                    }

                    // Brief delay before retry
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                Err(e) => {
                    error!("Task error reading from {}: {}", interface, e);
                    break;
                }
            }
        }

        info!("Stopped reading from {}", interface);
    }

    /// Spawn a task to replay frames from a log file.
    async fn spawn_replay_reader(
        path: std::path::PathBuf,
        rate: crate::types::ReplayRate,
        loop_at_end: bool,
        sender: mpsc::Sender<SourcedFrame>,
    ) -> Result<JoinHandle<()>> {
        let mut replayer = LogReplayer::new(path, rate, loop_at_end).await?;

        let task = tokio::spawn(async move {
            while let Some(frame) = replayer.next_frame().await {
                if sender.send(frame).await.is_err() {
                    debug!("Channel closed, stopping replay reader");
                    break;
                }
            }

            info!("Replay completed");
        });

        Ok(task)
    }

    /// Get the next frame from any interface.
    ///
    /// Returns `None` when all interfaces have finished (only possible with
    /// non-looping replay sources).
    pub async fn next_frame(&mut self) -> Option<SourcedFrame> {
        self.receiver.recv().await
    }

    /// Get the number of active interfaces.
    pub fn active_count(&self) -> usize {
        self.active_count
    }

    /// Shutdown all readers and wait for tasks to complete.
    pub async fn shutdown(mut self) {
        debug!("Shutting down MultiInterfaceReader");

        // Close the receiver to signal tasks to stop
        self.receiver.close();

        // Wait for all tasks to complete
        while let Some(task) = self.tasks.pop() {
            let _ = task.await;
        }

        info!("MultiInterfaceReader shutdown complete");
    }
}

impl Drop for MultiInterfaceReader {
    fn drop(&mut self) {
        debug!("MultiInterfaceReader dropped");
        // Tasks will detect closed channel and stop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_multi_interface_reader_replay_only() {
        // Create a test log file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "can0 18FECA88#A72E007D").unwrap();
        writeln!(file, "can0 18F00488#00000000").unwrap();
        writeln!(file, "can0 18FECA00#FFFFFFFF").unwrap();
        file.flush().unwrap();

        let configs = vec![CanInterfaceConfig::replay(
            file.path().to_path_buf(),
            1000,
            false,
        )];

        let mut reader = MultiInterfaceReader::new(configs).await.unwrap();

        assert_eq!(reader.active_count(), 1);

        // Read frames (with timeout to prevent hanging)
        let mut count = 0;
        let result = timeout(Duration::from_secs(2), async {
            while let Some(_frame) = reader.next_frame().await {
                count += 1;
                if count >= 3 {
                    break;
                }
            }
        })
        .await;

        assert!(result.is_ok(), "Test timed out");
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn test_multi_interface_reader_multiple_replays() {
        // Create two test log files
        let mut file1 = NamedTempFile::new().unwrap();
        writeln!(file1, "can0 123#01").unwrap();
        file1.flush().unwrap();

        let mut file2 = NamedTempFile::new().unwrap();
        writeln!(file2, "vcan0 456#02").unwrap();
        file2.flush().unwrap();

        let configs = vec![
            CanInterfaceConfig::replay(file1.path().to_path_buf(), 1000, false),
            CanInterfaceConfig::replay(file2.path().to_path_buf(), 1000, false),
        ];

        let mut reader = MultiInterfaceReader::new(configs).await.unwrap();

        assert_eq!(reader.active_count(), 2);

        // Read frames from both sources (with timeout)
        let mut count = 0;
        let mut sources = std::collections::HashSet::new();

        let result = timeout(Duration::from_secs(2), async {
            while let Some(frame) = reader.next_frame().await {
                sources.insert(frame.source.clone());
                count += 1;
                if count >= 2 {
                    break;
                }
            }
        })
        .await;

        assert!(result.is_ok(), "Test timed out");
        assert_eq!(count, 2);
        assert_eq!(sources.len(), 2); // Should have frames from both sources
    }

    #[tokio::test]
    async fn test_multi_interface_reader_replay_looping() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "can0 123#01").unwrap();
        file.flush().unwrap();

        let configs = vec![CanInterfaceConfig::replay(
            file.path().to_path_buf(),
            1000,
            true, // loop
        )];

        let mut reader = MultiInterfaceReader::new(configs).await.unwrap();

        // Should be able to read more frames than in the file (with timeout)
        let mut count = 0;
        let result = timeout(Duration::from_secs(2), async {
            while let Some(_frame) = reader.next_frame().await {
                count += 1;
                if count >= 5 {
                    // More than the 1 frame in the file
                    break;
                }
            }
        })
        .await;

        assert!(result.is_ok(), "Test timed out");
        assert_eq!(count, 5);
    }

    #[tokio::test]
    async fn test_multi_interface_reader_empty_config() {
        let configs = vec![];
        let reader = MultiInterfaceReader::new(configs).await;

        // Should succeed with 0 active interfaces (empty config is allowed)
        assert!(reader.is_ok());
        assert_eq!(reader.unwrap().active_count(), 0);
    }

    #[tokio::test]
    async fn test_multi_interface_reader_all_interfaces_fail() {
        let configs = vec![
            CanInterfaceConfig::replay(std::path::PathBuf::from("/nonexistent1.log"), 100, false),
            CanInterfaceConfig::replay(std::path::PathBuf::from("/nonexistent2.log"), 100, false),
        ];

        let result = MultiInterfaceReader::new(configs).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            crate::error::MonitorError::Interface(InterfaceError::AllInterfacesFailed {
                count,
            }) => {
                assert_eq!(count, 2);
            }
            _ => panic!("Expected AllInterfacesFailed error"),
        }
    }

    #[tokio::test]
    async fn test_multi_interface_reader_partial_failure() {
        // One valid, one invalid
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "can0 123#01").unwrap();
        file.flush().unwrap();

        let configs = vec![
            CanInterfaceConfig::replay(file.path().to_path_buf(), 1000, false),
            CanInterfaceConfig::replay(std::path::PathBuf::from("/nonexistent.log"), 100, false),
        ];

        let mut reader = MultiInterfaceReader::new(configs).await.unwrap();

        // Should succeed with 1 active interface
        assert_eq!(reader.active_count(), 1);

        // Should be able to read from the valid interface (with timeout)
        let result = timeout(Duration::from_secs(2), reader.next_frame()).await;
        assert!(result.is_ok(), "Test timed out");
        let frame = result.unwrap();
        assert!(frame.is_some());
    }

    #[tokio::test]
    async fn test_multi_interface_reader_shutdown() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "can0 123#01").unwrap();
        writeln!(file, "can0 456#02").unwrap();
        file.flush().unwrap();

        let configs = vec![CanInterfaceConfig::replay(
            file.path().to_path_buf(),
            1000,
            false, // don't loop (so it finishes quickly)
        )];

        let reader = MultiInterfaceReader::new(configs).await.unwrap();

        // Shutdown should complete without hanging (with timeout)
        let result = timeout(Duration::from_secs(2), reader.shutdown()).await;
        assert!(result.is_ok(), "Shutdown timed out");
    }

    #[tokio::test]
    async fn test_config_name_extraction() {
        let live_config = CanInterfaceConfig::live("can0");
        assert_eq!(live_config.name(), "can0");

        let replay_config =
            CanInterfaceConfig::replay(std::path::PathBuf::from("/path/to/test.log"), 100, false);
        assert_eq!(replay_config.name(), "test.log");
    }
}
