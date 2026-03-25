//! Field name conversion from DBC format to idiomatic Rust snake_case
//!
//! This module converts field names from various DBC patterns to Rust-idiomatic snake_case:
//! - PascalCase → snake_case (e.g., MotorSpeedCommand → motor_speed_command)
//! - UPPER_SNAKE_CASE → lower_snake_case (e.g., HVPC_Command_Opcode → hvpc_command_opcode)
//! - Compressed abbreviations → snake_case with word boundaries (e.g., MG1IC → mg_1_ic)
//!
//! # Algorithm
//!
//! 1. If name contains underscores, assume already word-delimited → just lowercase
//! 2. Otherwise (PascalCase/compressed), insert underscores at word boundaries:
//!    - Before uppercase letters (except at start or after uppercase)
//!    - Before digits (except after digits)
//!    - Handle consecutive capitals (HTTP → http, not h_t_t_p)
//! 3. Lowercase the result
//!
//! # Examples
//!
//! ```
//! use cando_core::field_name_converter::to_rust_field_name;
//!
//! assert_eq!(to_rust_field_name("MotorSpeedCommand"), "motor_speed_command");
//! assert_eq!(to_rust_field_name("HVPC_Command_Opcode"), "hvpc_command_opcode");
//! assert_eq!(to_rust_field_name("MG1IC"), "mg_1_ic");
//! assert_eq!(to_rust_field_name("HTTPServer"), "http_server");
//! ```

use std::collections::HashMap;

/// Convert DBC field name to Rust snake_case
///
/// Handles multiple input formats:
/// - PascalCase: MotorSpeedCommand → motor_speed_command
/// - Already underscored: HVPC_Command_Opcode → hvpc_command_opcode
/// - With numbers: Motor1Command → motor_1_command
/// - Consecutive capitals: HTTPServer → http_server
/// - Compressed: LnrDsplmnt → lnr_dsplmnt
///
/// # Examples
///
/// ```
/// use cando_core::field_name_converter::to_rust_field_name;
///
/// assert_eq!(to_rust_field_name("MotorSpeed"), "motor_speed");
/// assert_eq!(to_rust_field_name("HVPC_Command"), "hvpc_command");
/// ```
pub fn to_rust_field_name(dbc_field_name: &str) -> String {
    // Empty string edge case
    if dbc_field_name.is_empty() {
        return String::new();
    }

    // If already has underscores, assume it's delimited - just lowercase it
    if has_word_delimiters(dbc_field_name) {
        return dbc_field_name.to_lowercase();
    }

    // Otherwise, it's PascalCase or compressed - insert underscores at word boundaries
    let mut result = String::with_capacity(dbc_field_name.len() + 10); // Reserve extra for underscores
    let chars: Vec<char> = dbc_field_name.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        // Determine if we need an underscore before this character
        let needs_underscore = if i == 0 {
            false // Never at start
        } else {
            let prev_char = chars[i - 1];
            let next_char = chars.get(i + 1);

            // Insert underscore before uppercase letter if:
            // - Previous char is lowercase (camelCase boundary)
            // - Previous char is digit (e.g., Motor1Speed → motor_1_speed)
            // - Previous char is uppercase AND next char is lowercase (HTTPServer → http_server, not h_t_t_p_server)
            if ch.is_uppercase() {
                prev_char.is_lowercase()
                    || prev_char.is_ascii_digit()
                    || (prev_char.is_uppercase() && next_char.is_some_and(|c| c.is_lowercase()))
            }
            // Insert underscore before digit if previous char is letter
            else if ch.is_ascii_digit() {
                prev_char.is_alphabetic()
            } else {
                false
            }
        };

        if needs_underscore {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }

    result
}

/// Detect if a name already has word delimiters (underscores)
///
/// If a name has underscores, we assume it's already properly delimited
/// and just needs lowercasing, rather than PascalCase parsing.
fn has_word_delimiters(name: &str) -> bool {
    name.contains('_')
}

/// Represents a naming collision after conversion
///
/// Two different DBC names that convert to the same Rust snake_case name.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Used in Phase 3
pub struct Collision {
    /// The Rust snake_case name that both DBC names convert to
    pub rust_name: String,
    /// First DBC name
    pub dbc_name1: String,
    /// Second DBC name that collides
    pub dbc_name2: String,
}

/// Detect naming collisions after conversion
///
/// Returns a list of collisions where different DBC names convert to the same Rust name.
///
/// # Examples
///
/// ```
/// use cando_core::field_name_converter::detect_collisions;
///
/// let names = vec!["MotorSpeed", "MotorPower"];
/// assert!(detect_collisions(&names).is_empty());
///
/// // Hypothetical collision
/// let names = vec!["motorSpeed", "MotorSpeed"];
/// let collisions = detect_collisions(&names);
/// assert_eq!(collisions.len(), 1);
/// ```
#[allow(dead_code)] // Used in Phase 3
pub fn detect_collisions(dbc_names: &[&str]) -> Vec<Collision> {
    let mut seen: HashMap<String, &str> = HashMap::new();
    let mut collisions = Vec::new();

    for &name in dbc_names {
        let converted = to_rust_field_name(name);

        if let Some(&original) = seen.get(&converted) {
            // Collision found - only add if not already recorded
            if !collisions.iter().any(|c: &Collision| {
                c.rust_name == converted
                    && ((c.dbc_name1 == original && c.dbc_name2 == name)
                        || (c.dbc_name1 == name && c.dbc_name2 == original))
            }) {
                collisions.push(Collision {
                    rust_name: converted.clone(),
                    dbc_name1: original.to_string(),
                    dbc_name2: name.to_string(),
                });
            }
        } else {
            seen.insert(converted, name);
        }
    }

    collisions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_pascalcase() {
        assert_eq!(
            to_rust_field_name("MotorSpeedCommand"),
            "motor_speed_command"
        );
        assert_eq!(
            to_rust_field_name("OnOffDirectionStatus"),
            "on_off_direction_status"
        );
        assert_eq!(to_rust_field_name("MotorSpeed"), "motor_speed");
        assert_eq!(to_rust_field_name("WandAngle"), "wand_angle");
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(to_rust_field_name("Motor1Command"), "motor_1_command");
        assert_eq!(to_rust_field_name("MG1IC"), "mg_1_ic");
        assert_eq!(to_rust_field_name("MG2IC2"), "mg_2_ic_2");
        assert_eq!(to_rust_field_name("Test123Value"), "test_123_value");
        assert_eq!(to_rust_field_name("HTML2PDF"), "html_2_pdf");
    }

    #[test]
    fn test_already_underscored() {
        assert_eq!(
            to_rust_field_name("HVPC_Command_Opcode"),
            "hvpc_command_opcode"
        );
        assert_eq!(to_rust_field_name("HVPC_reserved_1a"), "hvpc_reserved_1a");
        assert_eq!(to_rust_field_name("motor_speed"), "motor_speed");
        assert_eq!(to_rust_field_name("UPPER_CASE"), "upper_case");
    }

    #[test]
    fn test_consecutive_capitals() {
        // HTTPServer should become http_server, not h_t_t_p_server
        assert_eq!(to_rust_field_name("HTTPServer"), "http_server");
        assert_eq!(to_rust_field_name("IOError"), "io_error");
        assert_eq!(to_rust_field_name("XMLParser"), "xml_parser");
        assert_eq!(to_rust_field_name("PDFDocument"), "pdf_document");
    }

    #[test]
    fn test_compressed_abbreviations() {
        // Compressed names like LnrDsplmntSnsr become more parseable
        assert_eq!(to_rust_field_name("LnrDsplmntSnsr"), "lnr_dsplmnt_snsr");
        assert_eq!(
            to_rust_field_name("GnrtrCrrntBstAtvStts"),
            "gnrtr_crrnt_bst_atv_stts"
        );
        // Even without vowels, at least we get word boundaries at capitals
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(to_rust_field_name("a"), "a");
        assert_eq!(to_rust_field_name("A"), "a");
        assert_eq!(to_rust_field_name(""), "");
        assert_eq!(to_rust_field_name("ABC"), "abc");
        assert_eq!(to_rust_field_name("123"), "123");
        assert_eq!(to_rust_field_name("_test"), "_test"); // Already has underscore
    }

    #[test]
    fn test_real_world_j1939_names() {
        // Real examples from j1939.dbc
        assert_eq!(
            to_rust_field_name("MtrGnrtr1InvrtrCntrlStpntRqst"),
            "mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst"
        );
        assert_eq!(
            to_rust_field_name("MtrGnrtr1InvrtrCntrlPrntTrq"),
            "mtr_gnrtr_1_invrtr_cntrl_prnt_trq"
        );
        assert_eq!(
            to_rust_field_name("MtrGnrtr1InvrtrCntrlPrtyActvtnStts"),
            "mtr_gnrtr_1_invrtr_cntrl_prty_actvtn_stts"
        );
        assert_eq!(
            to_rust_field_name("MeasuredLinearDisplacement"),
            "measured_linear_displacement"
        );
    }

    #[test]
    fn test_no_collisions() {
        let names = vec!["MotorSpeed", "MotorPower", "MotorStatus"];
        let collisions = detect_collisions(&names);
        assert!(collisions.is_empty());
    }

    #[test]
    fn test_detects_collision() {
        // These should collide (both become "motor_speed")
        let names = vec!["motorSpeed", "MotorSpeed"];
        let collisions = detect_collisions(&names);
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0].rust_name, "motor_speed");
        assert!(
            (collisions[0].dbc_name1 == "motorSpeed" && collisions[0].dbc_name2 == "MotorSpeed")
                || (collisions[0].dbc_name1 == "MotorSpeed"
                    && collisions[0].dbc_name2 == "motorSpeed")
        );
    }

    #[test]
    fn test_multiple_collisions() {
        let names = vec!["Test", "TEST", "test", "Motor", "MOTOR"];
        let collisions = detect_collisions(&names);
        // "Test", "TEST", "test" all become "test" (2 collisions)
        // "Motor", "MOTOR" all become "motor" (1 collision)
        assert_eq!(collisions.len(), 3);
    }

    #[test]
    fn test_has_word_delimiters() {
        assert!(has_word_delimiters("HVPC_Command"));
        assert!(has_word_delimiters("motor_speed"));
        assert!(has_word_delimiters("_leading"));
        assert!(!has_word_delimiters("MotorSpeed"));
        assert!(!has_word_delimiters("motorspeed"));
        assert!(!has_word_delimiters(""));
    }
}
