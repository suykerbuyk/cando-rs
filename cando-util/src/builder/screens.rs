//! Screen rendering for the TUI message builder
//!
//! This module handles rendering all screens in the interactive builder.

use anyhow::Result;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use super::{AppState, FieldInfo, Screen, data};

/// Main render function - dispatches to appropriate screen
pub fn render(f: &mut Frame, app: &AppState) -> Result<()> {
    // Render the current screen
    match app.screen {
        Screen::DeviceSelection => render_device_selection(f, app)?,
        Screen::MessageSelection => render_message_selection(f, app)?,
        Screen::OpcodeSelection => render_opcode_selection(f, app)?,
        Screen::FieldEntry => render_field_entry(f, app)?,
        Screen::CommandGenerated => render_command_generated(f, app)?,
    }

    // Render help overlay on top if enabled
    if app.show_help {
        render_help_overlay(f);
    }

    Ok(())
}

/// Render device selection screen
fn render_device_selection(f: &mut Frame, app: &AppState) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(5),    // Device list
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("CAN Message Builder - Device Selection")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Device list
    let items: Vec<ListItem> = app
        .devices
        .iter()
        .enumerate()
        .map(|(idx, device)| {
            let protocol_display = data::get_protocol_display_name(device);
            let content = format!(
                "{} - {} ({})",
                device.name, device.friendly_name, protocol_display
            );

            let style = if Some(idx) == app.selected_device_idx {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let device_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Select Device"),
    );
    f.render_widget(device_list, chunks[1]);

    // Help text
    let help = Paragraph::new("[↑/↓] Navigate  [Enter] Select  [?] Help  [q] Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);

    // Error message if any
    if let Some(error) = &app.error_message {
        render_error_popup(f, error);
    }

    Ok(())
}

/// Render message selection screen
fn render_message_selection(f: &mut Frame, app: &AppState) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Filter
            Constraint::Length(4), // Message type legend
            Constraint::Min(5),    // Message list
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Title with device info
    let device_name = app
        .selected_device()
        .map(|d| format!("{} ({})", d.friendly_name, d.protocol))
        .unwrap_or_else(|| "Unknown Device".to_string());

    let title = Paragraph::new(format!("Message Selection - {}", device_name))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Filter input
    let filter_text = if app.filter_text.is_empty() {
        "(type to filter)".to_string()
    } else {
        app.filter_text.clone()
    };
    let filter = Paragraph::new(format!("Filter: {}", filter_text))
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(filter, chunks[1]);

    // Message type legend
    let legend_lines = vec![Line::from(vec![
        Span::styled("Message Types: ", Style::default().fg(Color::Yellow)),
        Span::styled("CMD/Command", Style::default().fg(Color::Green)),
        Span::raw(" = Send commands TO device  "),
        Span::styled("ST/Status", Style::default().fg(Color::Cyan)),
        Span::raw(" = Receive status FROM device (read-only)"),
    ])];
    let legend = Paragraph::new(legend_lines)
        .style(Style::default())
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(legend, chunks[2]);

    // Message list (filtered)
    let filtered_messages = data::filter_messages(&app.messages, &app.filter_text);
    let items: Vec<ListItem> = filtered_messages
        .iter()
        .enumerate()
        .map(|(idx, message)| {
            let content = format!(
                "{:30} (CAN ID: 0x{:08X}, {} bytes)",
                message.name, message.can_id, message.dlc
            );

            let style = if Some(idx) == app.selected_message_idx {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let message_list = List::new(items).block(Block::default().borders(Borders::ALL).title(
        format!("Select Message ({} messages)", filtered_messages.len()),
    ));
    f.render_widget(message_list, chunks[3]);

    // Help text
    let help = Paragraph::new("[↑/↓] Navigate  [Enter] Select  [?] Help  [Esc] Back  [q] Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[4]);

    // Error message if any
    if let Some(error) = &app.error_message {
        render_error_popup(f, error);
    }

    Ok(())
}

/// Render opcode selection screen (placeholder - not used in cando-rs)
fn render_opcode_selection(f: &mut Frame, _app: &AppState) -> Result<()> {
    let area = f.area();
    let text = Paragraph::new("Opcode selection is not available in this build.")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(text, area);
    Ok(())
}

/// Render field entry screen
fn render_field_entry(f: &mut Frame, app: &AppState) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(5),    // Fields
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Title with message info
    let message_name = app
        .selected_message()
        .map(|m| format!("{} (CAN ID: 0x{:08X})", m.name, m.can_id))
        .unwrap_or_else(|| "Unknown Message".to_string());

    let title = Paragraph::new(format!("Field Entry - {}", message_name))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Fields
    if let Some(message) = app.selected_message() {
        // Show all fields for the selected message
        let visible_fields: Vec<&FieldInfo> = message.signals.iter().collect();

        let field_items: Vec<ListItem> = visible_fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                // Check if using sentinel value for this field
                let using_sentinel = app
                    .field_use_sentinel
                    .get(&field.name)
                    .copied()
                    .unwrap_or(false);

                let value = app
                    .field_values
                    .get(&field.name)
                    .cloned()
                    .unwrap_or_else(|| "0.0".to_string());

                // Check if value is valid (allow sentinel if checkbox is checked)
                let is_valid = if using_sentinel && field.sentinel_value.is_some() {
                    true // Sentinel value is always valid when checkbox is checked
                } else {
                    value
                        .parse::<f64>()
                        .ok()
                        .and_then(|v| field.validate(v).ok())
                        .is_some()
                };

                let status = if is_valid { "✓" } else { "✗" };
                let status_color = if is_valid { Color::Green } else { Color::Red };

                // Format field info
                let mut lines = Vec::new();

                // Field name and value
                let name_style = if idx == app.selected_field_idx {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                // Show sentinel value if using it
                let display_value = if using_sentinel && field.sentinel_value.is_some() {
                    format!("0x{:X}", field.sentinel_value.unwrap())
                } else {
                    value.clone()
                };

                lines.push(Line::from(vec![
                    Span::styled(format!("{}: ", field.name), name_style),
                    Span::styled(
                        format!("[{:10}] ", display_value),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled(status, Style::default().fg(status_color)),
                ]));

                // Range and unit info (or sentinel info)
                let range_unit = if field.is_enum() {
                    format!(
                        "  Enum: {}",
                        field
                            .value_descriptions
                            .iter()
                            .map(|(val, desc)| format!("{}={}", val, desc))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                } else if field.sentinel_value.is_some() {
                    format!(
                        "  Range: {:.2} - {:.2}{} (or 0x{:X} for special value)",
                        field.min,
                        field.max,
                        if field.unit.is_empty() {
                            String::new()
                        } else {
                            format!(" {}", field.unit)
                        },
                        field.sentinel_value.unwrap()
                    )
                } else {
                    format!(
                        "  Range: {:.2} - {:.2}{}",
                        field.min,
                        field.max,
                        if field.unit.is_empty() {
                            String::new()
                        } else {
                            format!(" {}", field.unit)
                        }
                    )
                };

                lines.push(Line::from(Span::styled(
                    range_unit,
                    Style::default().fg(Color::Gray),
                )));

                // Show sentinel checkbox if available
                if let Some(sentinel) = field.sentinel_value {
                    let checkbox = if using_sentinel { "[✓]" } else { "[ ]" };
                    let checkbox_style = if idx == app.selected_field_idx {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::Gray)
                    };

                    let sentinel_text = format!(
                        "  {} Use Sentinel Value (0x{:X} = {})",
                        checkbox,
                        sentinel,
                        if field.sentinel_description.is_empty() {
                            "special value"
                        } else {
                            &field.sentinel_description
                        }
                    );

                    lines.push(Line::from(Span::styled(sentinel_text, checkbox_style)));
                }

                // Comment if available
                if !field.comment.is_empty() {
                    lines.push(Line::from(Span::styled(
                        format!("  {}", field.comment),
                        Style::default().fg(Color::DarkGray),
                    )));
                }

                ListItem::new(lines)
            })
            .collect();

        let field_list = List::new(field_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Enter Field Values"),
        );
        f.render_widget(field_list, chunks[1]);
    }

    // Help text
    let help_text = "[↑/↓] Navigate  [Space] Toggle Sentinel  [l] Load Preset  [Enter] Generate  [?] Help  [Esc] Back  [q] Quit".to_string();

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);

    // Error message if any
    if let Some(error) = &app.error_message {
        render_error_popup(f, error);
    }

    // Status message if any
    if let Some(message) = &app.status_message {
        render_status_popup(f, message);
    }

    Ok(())
}

/// Render command generated screen
fn render_command_generated(f: &mut Frame, app: &AppState) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(5),    // Command
            Constraint::Length(5), // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Command Generated")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Generated command
    if let Some(command) = &app.generated_command {
        let command_text = Paragraph::new(command.as_str())
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Command"))
            .wrap(Wrap { trim: false });
        f.render_widget(command_text, chunks[1]);
    }

    // Help text (Phase 3: Enhanced)
    let help_lines = vec![
        Line::from("[e/Enter] Execute  [s] Save to File  [p] Save Preset"),
        Line::from("[c] Copy to Clipboard  [r] Reset"),
        Line::from("[?] Help  [q] Quit"),
    ];
    let help = Paragraph::new(help_lines)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Actions"));
    f.render_widget(help, chunks[2]);

    // Error message if any
    if let Some(message) = &app.error_message {
        render_error_popup(f, message);
    }

    // Status message if any
    if let Some(message) = &app.status_message {
        render_status_popup(f, message);
    }

    Ok(())
}

/// Render error popup in the center of the screen
fn render_error_popup(f: &mut Frame, message: &str) {
    let area = centered_rect(60, 20, f.area());

    let popup = Paragraph::new(message)
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Message")
                .style(Style::default().bg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(popup, area);
}

/// Render status/success popup in the center of the screen
fn render_status_popup(f: &mut Frame, message: &str) {
    let area = centered_rect(60, 20, f.area());

    let popup = Paragraph::new(message)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .wrap(Wrap { trim: false });

    f.render_widget(popup, area);
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Render comprehensive help overlay
fn render_help_overlay(f: &mut Frame) {
    let area = centered_rect(80, 85, f.area());

    let help_text = vec![
        Line::from(vec![Span::styled(
            "CAN Message Builder - Help",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "OVERVIEW",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  The message builder guides you through composing CAN messages with"),
        Line::from("  a progressive disclosure interface:"),
        Line::from(
            "    1. Select Device → 2. Select Message → 3. Enter Fields → 4. Generate Command",
        ),
        Line::from(""),
        Line::from(vec![Span::styled(
            "UNDERSTANDING CAN MESSAGES",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  Command (CMD) Messages",
            Style::default().fg(Color::Green),
        )]),
        Line::from("    • You SEND these TO the device to control it"),
        Line::from("    • Examples: MCM_MotorCommandMessage, Set_Speed, Control_Position"),
        Line::from("    • Use these when you want to: start/stop, change speed, set direction"),
        Line::from(vec![Span::styled(
            "  Status (ST) Messages",
            Style::default().fg(Color::Cyan),
        )]),
        Line::from("    • Device SENDS these FROM itself (you receive/monitor them)"),
        Line::from("    • Examples: MSM1_MotorStatusMessage1, Temperature_Report"),
        Line::from("    • These are READ-ONLY - you don't send status messages"),
        Line::from("    • Note: Status messages typically can't be sent via this tool"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "GLOBAL SHORTCUTS",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  q         ", Style::default().fg(Color::Green)),
            Span::raw("Quit the application"),
        ]),
        Line::from(vec![
            Span::styled("  ?         ", Style::default().fg(Color::Green)),
            Span::raw("Toggle this help screen"),
        ]),
        Line::from(vec![
            Span::styled("  Esc       ", Style::default().fg(Color::Green)),
            Span::raw("Go back to previous screen (or close help)"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "DEVICE SELECTION SCREEN",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ↑/↓ or k/j", Style::default().fg(Color::Green)),
            Span::raw("Navigate through device list"),
        ]),
        Line::from(vec![
            Span::styled("  Enter     ", Style::default().fg(Color::Green)),
            Span::raw("Select device and load its messages"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "MESSAGE SELECTION SCREEN",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ↑/↓ or k/j", Style::default().fg(Color::Green)),
            Span::raw("Navigate through message list"),
        ]),
        Line::from(vec![
            Span::styled("  Enter     ", Style::default().fg(Color::Green)),
            Span::raw("Select message and proceed to field entry"),
        ]),
        Line::from(vec![
            Span::styled("  Type text ", Style::default().fg(Color::Green)),
            Span::raw("Filter messages by name (case-insensitive)"),
        ]),
        Line::from(vec![
            Span::styled("  Backspace ", Style::default().fg(Color::Green)),
            Span::raw("Remove last character from filter"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "FIELD ENTRY SCREEN",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ↑/↓ or k/j", Style::default().fg(Color::Green)),
            Span::raw("Navigate between fields"),
        ]),
        Line::from(vec![
            Span::styled("  Tab       ", Style::default().fg(Color::Green)),
            Span::raw("Move to next field (wraps around)"),
        ]),
        Line::from(vec![
            Span::styled("  Shift+Tab ", Style::default().fg(Color::Green)),
            Span::raw("Move to previous field (wraps around)"),
        ]),
        Line::from(vec![
            Span::styled("  0-9, ., - ", Style::default().fg(Color::Green)),
            Span::raw("Enter numeric values for selected field"),
        ]),
        Line::from(vec![
            Span::styled("  Backspace ", Style::default().fg(Color::Green)),
            Span::raw("Delete last digit from selected field"),
        ]),
        Line::from(vec![
            Span::styled("  Space     ", Style::default().fg(Color::Green)),
            Span::raw("Toggle sentinel value (special values like 0xFF for N/A)"),
        ]),
        Line::from(vec![
            Span::styled("  l         ", Style::default().fg(Color::Green)),
            Span::raw("Load preset (previously saved field values)"),
        ]),
        Line::from(vec![
            Span::styled("  Enter     ", Style::default().fg(Color::Green)),
            Span::raw("Generate command (validates all fields)"),
        ]),
        Line::from(""),
        Line::from("  Fields display:"),
        Line::from(vec![
            Span::styled("    ✓ ", Style::default().fg(Color::Green)),
            Span::raw("Valid value within range"),
        ]),
        Line::from(vec![
            Span::styled("    ✗ ", Style::default().fg(Color::Red)),
            Span::raw("Invalid value (out of range)"),
        ]),
        Line::from(""),
        Line::from("  Sentinel values:"),
        Line::from("    Some fields support special sentinel values (e.g., 0xFF = Not Available)."),
        Line::from(
            "    When available, a checkbox appears below the field. Press Space to toggle.",
        ),
        Line::from("    When checked, the sentinel value is used instead of the numeric input."),
        Line::from(""),
        Line::from(vec![Span::styled(
            "COMMAND GENERATED SCREEN",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  e/Enter   ", Style::default().fg(Color::Green)),
            Span::raw("Execute command on CAN interface (uses full path)"),
        ]),
        Line::from(vec![
            Span::styled("  s         ", Style::default().fg(Color::Green)),
            Span::raw("Save command to commands.sh"),
        ]),
        Line::from(vec![
            Span::styled("  p         ", Style::default().fg(Color::Green)),
            Span::raw("Save current field values as preset"),
        ]),
        Line::from(vec![
            Span::styled("  c         ", Style::default().fg(Color::Green)),
            Span::raw("Copy command to clipboard"),
        ]),
        Line::from(vec![
            Span::styled("  r         ", Style::default().fg(Color::Green)),
            Span::raw("Reset and start over (return to device selection)"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "TIPS",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • Use filter on message screen to quickly find messages"),
        Line::from("  • All field values default to 0.0 if not entered"),
        Line::from("  • Enum fields show valid values with descriptions"),
        Line::from("  • Save presets for frequently used field configurations"),
        Line::from("  • Execute replaces 'cando-util' with full executable path"),
        Line::from("  • Commands saved to file include timestamps"),
        Line::from("  • Command is also printed to stdout when you quit"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Press ? or Esc to close this help",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        )]),
    ];

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .style(Style::default().bg(Color::Black)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(help, area);
}
