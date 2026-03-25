//! Phase 3 helper functions for advanced TUI features
//!
//! This module provides functionality for:
//! - Executing commands directly on CAN interfaces
//! - Saving commands to files
//! - Loading/saving field value presets
//! - Managing command history with persistence

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Preset configuration for a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePreset {
    /// Preset name
    pub name: String,

    /// Device name
    pub device: String,

    /// Message name
    pub message: String,

    /// Field values
    pub fields: HashMap<String, String>,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Command history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Timestamp (RFC 3339 format)
    pub timestamp: String,

    /// Generated command
    pub command: String,

    /// Device name
    pub device: String,

    /// Message name
    pub message: String,
}

/// Replace "cando-util" in a command with the full path to the current executable
///
/// This ensures commands work even when cando-util is not in PATH
fn replace_with_exe_path(command: &str) -> Result<String> {
    let exe_path = std::env::current_exe().context("Failed to get current executable path")?;
    let exe_path_str = exe_path
        .to_str()
        .context("Executable path contains invalid UTF-8")?;

    Ok(command.replace("cando-util", exe_path_str))
}

/// Execute a cando-util command directly
///
/// This runs the command as a subprocess and captures output.
/// Replaces "cando-util" with the full path to the current executable.
pub fn execute_command(command: &str) -> Result<String> {
    // Replace "cando-util" with the full path to the current executable
    // This ensures the command works even when cando-util is not in PATH
    let command_with_full_path = replace_with_exe_path(command)?;

    // Execute via shell
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command_with_full_path)
        .output()
        .context("Failed to execute command")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(format!("✓ Command executed successfully\n{}", stdout))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!(
            "Command failed (exit code: {})\n{}",
            output.status.code().unwrap_or(-1),
            stderr
        ))
    }
}

/// Save command to a file (append mode)
pub fn save_command_to_file(command: &str, file_path: &Path) -> Result<()> {
    // Create parent directory if needed
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).context("Failed to create directory for commands file")?;
    }

    // Open file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .context("Failed to open commands file")?;

    // Write timestamp and command
    let timestamp = chrono::Utc::now().to_rfc3339();
    writeln!(file, "# {}", timestamp)?;
    writeln!(file, "{}", command)?;
    writeln!(file)?;

    Ok(())
}

/// Get the default presets directory
pub fn get_presets_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/cando/presets")
}

/// Get the default history file path
pub fn get_history_file() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/cando/history.json")
}

/// Get the default commands file path (current working directory)
pub fn get_commands_file() -> PathBuf {
    PathBuf::from("commands.sh")
}

/// Save a preset to disk
pub fn save_preset(preset: &MessagePreset) -> Result<()> {
    let presets_dir = get_presets_dir();
    fs::create_dir_all(&presets_dir).context("Failed to create presets directory")?;

    // Create filename from device and message
    let filename = format!("{}_{}.json", preset.device, preset.message)
        .replace(" ", "_")
        .replace("/", "_");
    let file_path = presets_dir.join(filename);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&preset).context("Failed to serialize preset")?;

    // Write to file
    fs::write(&file_path, json).context("Failed to write preset file")?;

    Ok(())
}

/// Load a preset from disk
pub fn load_preset(device: &str, message: &str) -> Result<MessagePreset> {
    let presets_dir = get_presets_dir();
    let filename = format!("{}_{}.json", device, message)
        .replace(" ", "_")
        .replace("/", "_");
    let file_path = presets_dir.join(filename);

    // Read file
    let json = fs::read_to_string(&file_path).context("Failed to read preset file")?;

    // Deserialize
    let preset: MessagePreset =
        serde_json::from_str(&json).context("Failed to parse preset file")?;

    Ok(preset)
}

/// List all available presets
#[allow(dead_code)]
pub fn list_presets() -> Result<Vec<MessagePreset>> {
    let presets_dir = get_presets_dir();

    // Return empty list if directory doesn't exist
    if !presets_dir.exists() {
        return Ok(Vec::new());
    }

    let mut presets = Vec::new();

    // Read all .json files in the directory
    for entry in fs::read_dir(&presets_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json")
            && let Ok(json) = fs::read_to_string(&path)
                && let Ok(preset) = serde_json::from_str::<MessagePreset>(&json) {
                    presets.push(preset);
                }
    }

    Ok(presets)
}

/// Delete a preset
#[allow(dead_code)]
pub fn delete_preset(device: &str, message: &str) -> Result<()> {
    let presets_dir = get_presets_dir();
    let filename = format!("{}_{}.json", device, message)
        .replace(" ", "_")
        .replace("/", "_");
    let file_path = presets_dir.join(filename);

    if file_path.exists() {
        fs::remove_file(&file_path).context("Failed to delete preset file")?;
    }

    Ok(())
}

/// Save command history to disk
pub fn save_history(history: &[HistoryEntry]) -> Result<()> {
    let history_file = get_history_file();

    // Create parent directory if needed
    if let Some(parent) = history_file.parent() {
        fs::create_dir_all(parent).context("Failed to create directory for history file")?;
    }

    // Serialize to JSON
    let json = serde_json::to_string_pretty(history).context("Failed to serialize history")?;

    // Write to file
    fs::write(&history_file, json).context("Failed to write history file")?;

    Ok(())
}

/// Load command history from disk
pub fn load_history() -> Result<Vec<HistoryEntry>> {
    let history_file = get_history_file();

    // Return empty history if file doesn't exist
    if !history_file.exists() {
        return Ok(Vec::new());
    }

    // Read file
    let json = fs::read_to_string(&history_file).context("Failed to read history file")?;

    // Deserialize
    let history: Vec<HistoryEntry> =
        serde_json::from_str(&json).context("Failed to parse history file")?;

    Ok(history)
}

/// Add entry to history and save to disk
pub fn add_to_history(
    command: String,
    device: String,
    message: String,
    max_size: usize,
) -> Result<()> {
    // Load existing history
    let mut history = load_history().unwrap_or_default();

    // Create new entry
    let entry = HistoryEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        command,
        device,
        message,
    };

    // Add to front
    history.insert(0, entry);

    // Trim to max size
    if history.len() > max_size {
        history.truncate(max_size);
    }

    // Save
    save_history(&history)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_serialization() {
        let mut fields = HashMap::new();
        fields.insert("speed".to_string(), "1000".to_string());
        fields.insert("direction".to_string(), "1".to_string());

        let preset = MessagePreset {
            name: "Test Preset".to_string(),
            device: "EMP Test Device".to_string(),
            message: "EMP_CMD_Message".to_string(),
            fields,
            description: Some("Test description".to_string()),
        };

        let json = serde_json::to_string_pretty(&preset).unwrap();
        let deserialized: MessagePreset = serde_json::from_str(&json).unwrap();

        assert_eq!(preset.name, deserialized.name);
        assert_eq!(preset.device, deserialized.device);
        assert_eq!(preset.message, deserialized.message);
    }

    #[test]
    fn test_history_entry_serialization() {
        let entry = HistoryEntry {
            timestamp: "2025-01-21T12:00:00Z".to_string(),
            command: "cando-util test".to_string(),
            device: "Test Device".to_string(),
            message: "Test Message".to_string(),
        };

        let json = serde_json::to_string_pretty(&entry).unwrap();
        let deserialized: HistoryEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(entry.command, deserialized.command);
        assert_eq!(entry.device, deserialized.device);
    }

    #[test]
    fn test_replace_with_exe_path() {
        let test_command = "cando-util --device Test --message TestMsg --fields 1,2,3";
        let result = replace_with_exe_path(test_command).unwrap();

        // Verify that "cando-util" was replaced
        assert!(!result.contains("cando-util "));

        // Verify the rest of the command is intact
        assert!(result.contains("--device Test"));
        assert!(result.contains("--message TestMsg"));
        assert!(result.contains("--fields 1,2,3"));

        // Verify it contains a path separator (either / or \)
        assert!(result.contains('/') || result.contains('\\'));
    }
}
