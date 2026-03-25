//! TUI event loop and terminal management
//!
//! This module handles the terminal setup, event loop, and cleanup for the
//! interactive message builder.

use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;

use super::{AppState, FieldInfo, Screen, data, j1939_builder, screens};
use cando_core::field_name_converter::to_rust_field_name;

/// Main entry point for the TUI builder
pub fn run_builder(
    config: cando_config::CandoConfig,
    config_path: Option<String>,
    environment: Option<String>,
) -> Result<()> {
    // Setup terminal
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    // Create app state
    let mut app = AppState::new(config, config_path, environment);

    // Run the main loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    // Print final command if generated
    if let Some(command) = &app.generated_command {
        println!("\n{}", command);
    }

    result
}

/// Main application loop
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    loop {
        // Render current screen
        terminal.draw(|f| {
            if let Err(e) = screens::render(f, app) {
                app.error_message = Some(format!("Render error: {}", e));
            }
        })?;

        // Handle input with timeout
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && !handle_input(app, key)?
        {
            return Ok(()); // Exit requested
        }
    }
}

/// Handle keyboard input based on current screen
/// Returns false if user wants to quit
fn handle_input(app: &mut AppState, key: KeyEvent) -> Result<bool> {
    // Global help toggle
    if matches!(key.code, KeyCode::Char('?')) {
        app.show_help = !app.show_help;
        app.error_message = None; // Clear any error messages
        return Ok(true);
    }

    // If help is showing, Esc closes it
    if app.show_help && matches!(key.code, KeyCode::Esc) {
        app.show_help = false;
        return Ok(true);
    }

    // Don't process other keys when help is showing
    if app.show_help {
        return Ok(true);
    }

    // Global quit keys
    if matches!(key.code, KeyCode::Char('q') | KeyCode::Char('Q')) {
        return Ok(false);
    }

    match app.screen {
        Screen::DeviceSelection => handle_device_selection(app, key),
        Screen::MessageSelection => handle_message_selection(app, key),
        Screen::OpcodeSelection => {
            // OpcodeSelection is not used in cando-rs, skip to field entry
            app.next_screen();
            Ok(true)
        }
        Screen::FieldEntry => handle_field_entry(app, key),
        Screen::CommandGenerated => handle_command_generated(app, key),
    }
}

/// Handle input on device selection screen
fn handle_device_selection(app: &mut AppState, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            // Move selection up
            if let Some(idx) = app.selected_device_idx {
                if idx > 0 {
                    app.selected_device_idx = Some(idx - 1);
                }
            } else if !app.devices.is_empty() {
                app.selected_device_idx = Some(app.devices.len() - 1);
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Move selection down
            if let Some(idx) = app.selected_device_idx {
                if idx < app.devices.len() - 1 {
                    app.selected_device_idx = Some(idx + 1);
                }
            } else if !app.devices.is_empty() {
                app.selected_device_idx = Some(0);
            }
        }
        KeyCode::Enter => {
            // Select device and load messages
            if let Some(idx) = app.selected_device_idx
                && let Some(device) = app.devices.get(idx)
            {
                match data::load_messages_for_device(device) {
                    Ok(messages) => {
                        app.messages = messages;
                        app.error_message = None;
                        app.status_message = None;
                        app.next_screen();
                    }
                    Err(e) => {
                        app.error_message = Some(format!("Failed to load messages: {}", e));
                    }
                }
            }
        }
        KeyCode::Esc => {
            // Exit
            return Ok(false);
        }
        _ => {}
    }

    Ok(true)
}

/// Handle input on message selection screen
fn handle_message_selection(app: &mut AppState, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            // Move selection up
            if let Some(idx) = app.selected_message_idx {
                if idx > 0 {
                    app.selected_message_idx = Some(idx - 1);
                }
            } else if !app.messages.is_empty() {
                app.selected_message_idx = Some(app.messages.len() - 1);
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Move selection down
            if let Some(idx) = app.selected_message_idx {
                if idx < app.messages.len() - 1 {
                    app.selected_message_idx = Some(idx + 1);
                }
            } else if !app.messages.is_empty() {
                app.selected_message_idx = Some(0);
            }
        }
        KeyCode::Enter => {
            // Select message and initialize fields
            if app.selected_message().is_some() {
                app.field_values.clear();
                app.selected_field_idx = 0;
                // No opcode selection needed for J1939-only messages
                app.selected_opcode = None;
                app.available_opcodes.clear();
                app.next_screen();
            }
        }
        KeyCode::Esc => {
            // Go back to device selection
            app.error_message = None;
            app.status_message = None;
            app.previous_screen();
        }
        KeyCode::Char(c) => {
            // Add to filter text
            app.filter_text.push(c);
        }
        KeyCode::Backspace => {
            // Remove from filter text
            app.filter_text.pop();
        }
        _ => {}
    }

    Ok(true)
}

/// Handle input on field entry screen
fn handle_field_entry(app: &mut AppState, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            // Move to previous field
            if app.selected_field_idx > 0 {
                app.selected_field_idx -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Move to next field
            if let Some(msg) = app.selected_message()
                && app.selected_field_idx < msg.signals.len().saturating_sub(1)
            {
                app.selected_field_idx += 1;
            }
        }
        KeyCode::Tab => {
            // Move to next field (wrap around)
            if let Some(msg) = app.selected_message()
                && !msg.signals.is_empty()
            {
                app.selected_field_idx = (app.selected_field_idx + 1) % msg.signals.len();
            }
        }
        KeyCode::BackTab => {
            // Move to previous field (wrap around)
            if let Some(msg) = app.selected_message()
                && !msg.signals.is_empty()
            {
                if app.selected_field_idx == 0 {
                    app.selected_field_idx = msg.signals.len() - 1;
                } else {
                    app.selected_field_idx -= 1;
                }
            }
        }
        KeyCode::Char(' ') => {
            // Toggle sentinel value checkbox
            if let Some(msg) = app.selected_message()
                && let Some(field) = msg.signals.get(app.selected_field_idx)
                && field.sentinel_value.is_some()
            {
                let field_name = field.name.clone();
                let _ = msg; // Drop the borrow before mutating
                let current = app.field_use_sentinel.entry(field_name).or_insert(false);
                *current = !*current;
                app.error_message = None;
            }
        }
        KeyCode::Char(c) if c.is_ascii_digit() || c == '.' || c == '-' => {
            // Add digit to current field value
            if let Some(msg) = app.selected_message()
                && let Some(field) = msg.signals.get(app.selected_field_idx)
            {
                let field_name = field.name.clone();
                let _ = msg; // Drop the borrow before mutating
                let current = app.field_values.entry(field_name).or_default();
                current.push(c);
            }
        }
        KeyCode::Backspace => {
            // Remove digit from current field value
            if let Some(msg) = app.selected_message()
                && let Some(field) = msg.signals.get(app.selected_field_idx)
            {
                let field_name = field.name.clone();
                let _ = msg; // Drop the borrow before mutating
                if let Some(current) = app.field_values.get_mut(&field_name) {
                    current.pop();
                }
            }
        }
        KeyCode::Char('l') | KeyCode::Char('L') => {
            // Load preset for current device/message
            if let (Some(device), Some(message)) = (app.selected_device(), app.selected_message()) {
                match j1939_builder::load_preset(&device.name, &message.name) {
                    Ok(preset) => {
                        // Load field values from preset
                        app.field_values = preset.fields;
                        app.status_message = Some("Preset loaded".to_string());
                    }
                    Err(e) => {
                        app.error_message = Some(format!("No preset found: {}", e));
                    }
                }
            }
        }
        KeyCode::Enter => {
            // Generate command
            match generate_command(app) {
                Ok(command) => {
                    app.generated_command = Some(command);
                    app.error_message = None;
                    app.status_message = None;
                    app.next_screen();
                }
                Err(e) => {
                    app.error_message = Some(format!("Failed to generate command: {}", e));
                }
            }
        }
        KeyCode::Esc => {
            // If error or status message is showing, just dismiss it
            if app.error_message.is_some() || app.status_message.is_some() {
                app.error_message = None;
                app.status_message = None;
                // Stay on current screen to allow user to correct values
            } else {
                // No dialog showing - go back to message selection
                app.previous_screen();
            }
        }
        _ => {}
    }

    Ok(true)
}

/// Handle input on command generated screen (Enhanced with execute, save, preset)
fn handle_command_generated(app: &mut AppState, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Enter | KeyCode::Char('e') | KeyCode::Char('E') => {
            // Execute command directly
            if let Some(command) = &app.generated_command {
                app.status_message = Some("Executing command...".to_string());
                match j1939_builder::execute_command(command) {
                    Ok(output) => {
                        app.status_message = Some(output);
                        // Add to history
                        if let (Some(device), Some(message)) =
                            (app.selected_device(), app.selected_message())
                        {
                            let _ = j1939_builder::add_to_history(
                                command.clone(),
                                device.name.clone(),
                                message.name.clone(),
                                app.max_history_size,
                            );
                        }
                    }
                    Err(e) => {
                        app.error_message = Some(format!("Execution failed: {}", e));
                    }
                }
            }
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            // Save command to file
            if let Some(command) = &app.generated_command {
                let file_path = j1939_builder::get_commands_file();
                match j1939_builder::save_command_to_file(command, &file_path) {
                    Ok(_) => {
                        app.status_message = Some(format!("Saved to {}", file_path.display()));
                        // Add to history
                        if let (Some(device), Some(message)) =
                            (app.selected_device(), app.selected_message())
                        {
                            let _ = j1939_builder::add_to_history(
                                command.clone(),
                                device.name.clone(),
                                message.name.clone(),
                                app.max_history_size,
                            );
                        }
                    }
                    Err(e) => {
                        app.error_message = Some(format!("Save failed: {}", e));
                    }
                }
            }
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            // Save current field values as preset
            if let (Some(device), Some(message)) = (app.selected_device(), app.selected_message()) {
                let preset = j1939_builder::MessagePreset {
                    name: format!("{} - {}", device.name, message.name),
                    device: device.name.clone(),
                    message: message.name.clone(),
                    fields: app.field_values.clone(),
                    description: Some("Saved from TUI".to_string()),
                };

                match j1939_builder::save_preset(&preset) {
                    Ok(_) => {
                        app.status_message = Some("Preset saved".to_string());
                    }
                    Err(e) => {
                        app.error_message = Some(format!("Preset save failed: {}", e));
                    }
                }
            }
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            // Copy to clipboard (multi-strategy)
            if let Some(command) = &app.generated_command {
                match copy_to_clipboard(command) {
                    Ok(msg) => {
                        app.status_message = Some(msg);
                    }
                    Err(e) => {
                        app.error_message = Some(format!("Copy failed: {}", e));
                    }
                }
            }
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            // Reset and start over
            app.error_message = None;
            app.status_message = None;
            app.reset();
        }
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
            // Exit
            return Ok(false);
        }
        _ => {}
    }

    Ok(true)
}

/// Convert DBC field name to Rust snake_case
///
/// Converts from PascalCase, underscored, or compressed naming patterns
/// to idiomatic Rust snake_case using the proper field name converter.
///
/// Examples:
/// - "OnOffDirectionCommand" -> "on_off_direction_command"
/// - "MotorSpeed" -> "motor_speed"
/// - "LnrDsplmntSnsr" -> "lnr_dsplmnt_snsr"
fn to_snake_case(s: &str) -> String {
    to_rust_field_name(s)
}

fn generate_command(app: &AppState) -> Result<String> {
    let device = app.selected_device().context("No device selected")?;
    let message = app.selected_message().context("No message selected")?;

    // Process all fields for the message
    let fields_to_process: Vec<&FieldInfo> = message.signals.iter().collect();

    // Build field string and collect values for CAN frame generation
    let mut field_parts = Vec::new();
    let mut field_values = std::collections::HashMap::new();

    for field in &fields_to_process {
        // Check if using sentinel value for this field
        let using_sentinel = app
            .field_use_sentinel
            .get(&field.name)
            .copied()
            .unwrap_or(false);

        let value = if using_sentinel && field.sentinel_value.is_some() {
            // Use sentinel value (convert from u64 to f64)
            field.sentinel_value.unwrap() as f64
        } else {
            // Use normal field value
            let parsed_value = app
                .field_values
                .get(&field.name)
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(0.0);

            // Validate value (only when not using sentinel)
            field.validate(parsed_value)?;

            parsed_value
        };

        // Convert field name to snake_case for encoder compatibility
        let snake_case_name = to_snake_case(&field.name);

        field_values.insert(snake_case_name.clone(), value);
        field_parts.push(format!("{}={}", snake_case_name, value));
    }

    let fields_str = field_parts.join(",");

    // Generate CAN frame for display (candump format)
    let can_frame = generate_can_frame(device, message, &field_values)?;

    // Generate command with CAN frame comment
    // Build command with config and environment if they were specified
    let mut cmd_parts = vec!["cando-util".to_string()];

    // Add config and environment flags to ensure command uses same configuration
    if let Some(config_path) = &app.config_path {
        cmd_parts.push(format!("--config \"{}\"", config_path));
    }
    if let Some(environment) = &app.environment {
        cmd_parts.push(format!("--environment \"{}\"", environment));
    }

    cmd_parts.push(format!("--device \"{}\"", device.name));
    cmd_parts.push(format!("--message {}", message.name));
    cmd_parts.push(format!("--fields \"{}\"", fields_str));
    cmd_parts.push(format!("--send-interface {}", device.interface));

    let command = format!(
        "{}\n\n# CAN Frame (candump/cansend format):\n# {}",
        cmd_parts.join(" \\\n  "),
        can_frame
    );

    Ok(command)
}

/// Generate CAN frame representation in candump/cansend format
fn generate_can_frame(
    device: &super::DeviceInfo,
    message: &super::MessageInfo,
    field_values: &std::collections::HashMap<String, f64>,
) -> Result<String> {
    use crate::encoder;
    use cando_core::parse_device_id;

    // Parse device ID from string (e.g., "0x82" -> DeviceId)
    let device_id = parse_device_id(&device.device_id)?;

    // Use the encoder module to encode the message
    let encoded = encoder::encode_message(&message.name, device_id, field_values)?;

    // Format as candump/cansend: CANID#DATA
    let can_id_str = format!("{:08X}", encoded.can_id);
    let data_str = encoded
        .data
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join("");

    Ok(format!("{}#{}", can_id_str, data_str))
}

/// Copy text to clipboard using multiple strategies
///
/// Tries methods in order:
/// 1. OSC 52 escape sequences (works over SSH + tmux)
/// 2. tmux set-buffer (if inside tmux)
/// 3. File fallback (always works)
///
/// Returns a success message describing what worked.
fn copy_to_clipboard(text: &str) -> Result<String> {
    let mut successes: Vec<String> = Vec::new();
    let mut errors = Vec::new();

    // Strategy 1: Try OSC 52 (works over SSH, tmux, modern terminals)
    match copy_via_osc52(text) {
        Ok(_) => successes.push("System clipboard (OSC 52)".to_string()),
        Err(e) => errors.push(format!("OSC 52: {}", e)),
    }

    // Strategy 2: Try tmux if inside tmux session
    if is_inside_tmux() {
        match copy_via_tmux(text) {
            Ok(_) => successes.push("tmux buffer (Prefix+] to paste)".to_string()),
            Err(e) => errors.push(format!("tmux: {}", e)),
        }
    }

    // Strategy 3: Always save to file as backup
    match copy_via_file(text) {
        Ok(path) => successes.push(format!("File: {}", path)),
        Err(e) => errors.push(format!("File: {}", e)),
    }

    if successes.is_empty() {
        anyhow::bail!("All copy methods failed:\n{}", errors.join("\n"));
    }

    Ok(format!("Copied to:\n  {}", successes.join("\n  ")))
}

/// Copy to clipboard using OSC 52 escape sequences
///
/// Works over SSH and through tmux with modern terminal emulators.
/// Supported terminals: Alacritty, kitty, iTerm2, WezTerm, tmux 3.2+
fn copy_via_osc52(text: &str) -> Result<()> {
    use base64::{Engine as _, engine::general_purpose};
    use std::io::{self, Write};

    // Base64 encode the text
    let encoded = general_purpose::STANDARD.encode(text.as_bytes());

    // Construct OSC 52 sequence: ESC ] 52 ; c ; <base64> BEL
    // 'c' = clipboard selection (p = primary, q = secondary, s = select)
    let osc52 = format!("\x1b]52;c;{}\x07", encoded);

    // Write to stderr (stdout might be captured by pagers)
    io::stderr()
        .write_all(osc52.as_bytes())
        .context("Failed to write OSC 52 sequence")?;
    io::stderr().flush()?;

    Ok(())
}

/// Check if running inside tmux
fn is_inside_tmux() -> bool {
    std::env::var("TMUX").is_ok()
}

/// Copy to tmux buffer
fn copy_via_tmux(text: &str) -> Result<()> {
    use std::process::Command;

    let output = Command::new("tmux")
        .arg("set-buffer")
        .arg(text)
        .output()
        .context("Failed to execute tmux set-buffer")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("tmux set-buffer failed: {}", error);
    }

    Ok(())
}

/// Copy to file as fallback
fn copy_via_file(text: &str) -> Result<String> {
    use std::fs;
    use std::path::PathBuf;

    // Use XDG_RUNTIME_DIR or fallback to /tmp
    let dir = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"));

    let file_path = dir.join("cando-util-last-command.txt");

    fs::write(&file_path, text).context("Failed to write command to file")?;

    // Make readable by user only (security)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&file_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&file_path, perms)?;
    }

    Ok(file_path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case_j1939_fields() {
        // J1939 field names with PascalCase
        assert_eq!(
            to_snake_case("OnOffDirectionCommand"),
            "on_off_direction_command"
        );
        assert_eq!(to_snake_case("MotorSpeedCommand"), "motor_speed_command");
        assert_eq!(to_snake_case("PowerHoldCommand"), "power_hold_command");
        assert_eq!(
            to_snake_case("PercentMotorSpeedCommand"),
            "percent_motor_speed_command"
        );
    }

    #[test]
    fn test_to_snake_case_already_lower() {
        assert_eq!(to_snake_case("already_snake_case"), "already_snake_case");
        // Proper converter inserts underscores before digits: test123 -> test_123
        assert_eq!(to_snake_case("test123"), "test_123");
    }

    #[test]
    fn test_to_snake_case_with_numbers() {
        // PascalCase with numbers: numbers get underscores before them
        assert_eq!(to_snake_case("Motor1Command"), "motor_1_command");
        assert_eq!(to_snake_case("MG1IC"), "mg_1_ic");
    }

    #[test]
    fn test_is_inside_tmux() {
        // Save original TMUX var
        let original = std::env::var("TMUX").ok();

        // Test when TMUX is set
        unsafe {
            std::env::set_var("TMUX", "/tmp/tmux-1000/default,12345,0");
        }
        assert!(is_inside_tmux());

        // Test when TMUX is not set
        unsafe {
            std::env::remove_var("TMUX");
        }
        assert!(!is_inside_tmux());

        // Restore original
        unsafe {
            if let Some(val) = original {
                std::env::set_var("TMUX", val);
            } else {
                std::env::remove_var("TMUX");
            }
        }
    }
}
