//! Code Generator Module - DBC to Rust Code Generation
//!
//! This module contains the actual code generation logic for transforming
//! DBC (Database CAN) files into type-safe Rust code.
//!
//! ## Architecture Overview
//!
//! The generation process transforms DBC files through several stages:
//! 1. **DBC Parsing**: Uses can-dbc crate to parse DBC text format
//! 2. **Code Generation**: Creates Rust structs, enums, and metadata
//! 3. **Field Ordering**: Sorts struct fields by bit position (CRITICAL FIX)
//! 4. **File Output**: Writes generated code to specified path
//!
//! ## Key Fix: Bit-Order-Based Field Generation
//!
//! **Problem**: Struct fields were generated in alphabetical order (DBC order),
//! but encode/decode functions expected bit position order, causing field mismatches.
//!
//! **Solution**: Sort signals by start_bit before generating struct fields.
//! This aligns struct field order with encode/decode parameter order.
//!
//! ## Generated Code Structure
//!
//! For each DBC file, generates:
//! - Message structs with typed fields (in bit position order)
//! - Payload enums for multiplexed messages
//! - Value enums from DBC VAL_ descriptions
//! - Static metadata for runtime introspection
//! - Protocol-level metadata for tooling integration
use crate::field_name_converter::to_rust_field_name;
use anyhow::Result;
use can_dbc::{Dbc, MultiplexIndicator, Signal};
use quote::{format_ident, quote};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
const BASE_ID_MASK: u32 = 0x1FFFFF00;

/// Parse full 32-bit message IDs from DBC text
///
/// The can-dbc parser has a limitation where it truncates message IDs to 16 bits.
/// This function parses the BO_ lines directly from the DBC text to get the full IDs.
///
/// # Arguments
/// * `dbc_text` - The full DBC file content as a string
///
/// # Returns
/// HashMap mapping message names to their full 32-bit message IDs
fn parse_message_ids_from_dbc(dbc_text: &str) -> std::collections::HashMap<String, u32> {
    let mut message_ids = std::collections::HashMap::new();

    for line in dbc_text.lines() {
        let line = line.trim();
        if line.starts_with("BO_ ") {
            // Parse: BO_ <id> <name>: <dlc> <node>
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let Ok(msg_id) = parts[1].parse::<u32>() {
                    let name = parts[2].trim_end_matches(':');
                    message_ids.insert(name.to_string(), msg_id);
                }
            }
        }
    }

    message_ids
}

/// Generate Rust code from a single DBC file
///
/// This is the main entry point for code generation. It reads a DBC file,
/// generates comprehensive Rust code with proper field ordering, and writes
/// the output to the specified path.
///
/// ## Critical Fix: Bit-Order Sorting
///
/// This implementation sorts struct fields by bit position (start_bit) instead
/// of alphabetical order, ensuring alignment with encode/decode functions.
///
/// # Arguments
///
/// * `dbc_path` - Path to the input DBC file
/// * `output_path` - Path where generated Rust code will be written
///
/// # Returns
///
/// `Ok(())` on success, error on failure
pub fn generate_for_dbc(dbc_path: &str, output_path: impl AsRef<Path>) -> Result<()> {
    let dbc_content = fs::read_to_string(dbc_path)?;

    // Parse full 32-bit message IDs directly from DBC text (workaround for can-dbc parser limitation)
    let full_message_ids = parse_message_ids_from_dbc(&dbc_content);

    let dbc = Dbc::try_from(dbc_content.as_str())
        .map_err(|e| anyhow::anyhow!("Failed to parse DBC: {:?}", e))?;
    eprintln!("DEBUG: === ALL VALUE DESCRIPTIONS IN DBC ===");
    for message in &dbc.messages {
        for signal in &message.signals {
            if let Some(val_descs) = dbc.value_descriptions_for_signal(message.id, &signal.name) {
                eprintln!(
                    "DEBUG: Message ID {} (0x{:X}), Signal '{}': {} value descriptions",
                    message.id.raw(),
                    message.id.raw(),
                    signal.name,
                    val_descs.len()
                );
            }
        }
    }
    eprintln!("DEBUG: === END VALUE DESCRIPTIONS ===");
    let mut file_content = String::new();
    file_content.push_str("// Automatically generated code from DBC file\n");
    file_content.push_str("use crate::common::*;\n");
    file_content.push_str("use crate::metadata::*;\n");
    file_content.push_str("use bitvec::prelude::*;\n\n");
    let protocol_name = Path::new(dbc_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.replace("-", "_").to_uppercase())
        .unwrap_or_else(|| "UNKNOWN".to_string());
    let mut generated_enums = HashSet::new();
    let mut message_names: Vec<String> = Vec::new();
    for message in &dbc.messages {
        let msg_name = format_ident!("{}", message.name);
        let msg_name_str = &message.name;
        message_names.push(msg_name_str.clone());

        // Use full message ID from our parser, fall back to can-dbc parser if not found
        let raw_id = full_message_ids
            .get(msg_name_str)
            .copied()
            .unwrap_or_else(|| message.id.raw());
        let base_id = raw_id & BASE_ID_MASK;
        let source_addr = (raw_id & 0xFF) as u8;

        if msg_name_str.contains("J1939")
            || msg_name_str.contains("UDC")
            || msg_name_str.contains("Status")
        {
            eprintln!(
                "DEBUG: Message '{}': raw_id=0x{:08X} ({}), base_id=0x{:08X}",
                msg_name_str, raw_id, raw_id, base_id
            );
        }
        let dlc = message.size as u8;
        let msg_doc = generate_message_doc(&dbc, message, &full_message_ids);
        let mut fields: Vec<proc_macro2::TokenStream> = Vec::new();
        let multiplexer = message
            .signals
            .iter()
            .find(|s| matches!(s.multiplexer_indicator, MultiplexIndicator::Multiplexor));
        let is_multiplexed = multiplexer.is_some();
        if is_multiplexed {
            let mux_field = multiplexer.unwrap();
            let mux_field_name = format_ident!("{}", to_rust_field_name(&mux_field.name));
            let mux_type = get_rust_type(mux_field);
            fields.push(quote! {
                pub # mux_field_name : # mux_type
            });
            let payload_enum_name = format_ident!("{}Payload", message.name);
            let mut payload_variants = Vec::new();
            let mut mux_values: std::collections::BTreeMap<u64, Vec<&Signal>> =
                std::collections::BTreeMap::new();

            // Pre-populate mux_values with ALL defined opcode values from VAL_
            // This ensures we generate payload variants even for opcodes with no signals
            if let Some(val_descs) = dbc.value_descriptions_for_signal(message.id, &mux_field.name)
            {
                for desc in val_descs.iter() {
                    mux_values.entry(desc.id as u64).or_default();
                }
            }

            // Now populate signals for opcodes that have them
            for signal in &message.signals {
                if let MultiplexIndicator::MultiplexedSignal(mux_val) = signal.multiplexer_indicator
                {
                    mux_values.entry(mux_val).or_default().push(signal);
                }
            }
            for (mux_val, signals) in &mux_values {
                let variant_name = if msg_name_str == "UDC_Command" {
                    match mux_val {
                        0 => format_ident!("Convert"),
                        2 => format_ident!("Safe"),
                        3 => format_ident!("NedReset"),
                        4 => format_ident!("Shutdown"),
                        _ => format_ident!("Opcode{}", mux_val),
                    }
                } else {
                    format_ident!("Variant{}", mux_val)
                };
                let mut variant_fields: Vec<proc_macro2::TokenStream> = Vec::new();
                for signal in signals {
                    let field_name = format_ident!("{}", to_rust_field_name(&signal.name));
                    let field_type = get_rust_type(signal);
                    let field_doc = generate_signal_doc(&dbc, message, signal);
                    variant_fields.push(quote! {
                        #[doc = # field_doc] # field_name : # field_type
                    });
                }
                if variant_fields.is_empty() {
                    payload_variants.push(quote! {
                        # variant_name
                    });
                } else {
                    payload_variants.push(quote! {
                        # variant_name { # (# variant_fields,) * }
                    });
                }
            }
            file_content
                .push_str(
                    &format!(
                        "# [allow (non_camel_case_types)] # [derive (Debug , Clone , PartialEq)] pub enum {} {{ {} }}",
                        payload_enum_name, payload_variants.iter().map(| v | v
                        .to_string()).collect::< Vec < _ >> ().join(",")
                    ),
                );
            fields.push(quote! {
                pub payload : # payload_enum_name
            });
            let struct_item = quote! {
                #[allow(non_camel_case_types)] #[derive(Debug, Clone, PartialEq)] #[doc =
                # msg_doc] pub struct # msg_name { pub device_id : DeviceId, # (#
                fields),* }
            };
            file_content.push_str(&struct_item.to_string());
            generate_message_metadata(
                &mut file_content,
                &dbc,
                message,
                &msg_name,
                &full_message_ids,
            );
            let impl_item = generate_multiplexed_encode_decode_impl(
                &msg_name,
                &payload_enum_name,
                mux_field,
                &mux_field_name,
                &mux_values,
                base_id,
                dlc,
                source_addr,
            );
            file_content.push_str(&impl_item.to_string());
        } else {
            let mut signals: Vec<&Signal> = message.signals.iter().collect();
            signals.sort_by_key(|s| s.start_bit);
            let mut field_inits = Vec::new();
            for signal in &signals {
                let field_name = format_ident!("{}", to_rust_field_name(&signal.name));
                let field_type = get_rust_type(signal);
                let field_doc = generate_signal_doc(&dbc, message, signal);
                fields.push(quote! {
                    #[doc = # field_doc] pub # field_name : # field_type
                });
                let placeholder_value = if signal.factor != 1.0 || signal.offset != 0.0 {
                    quote! {
                        0.0
                    }
                } else {
                    let is_signed = signal.value_type == can_dbc::ValueType::Signed;
                    let bit_size = signal.size;
                    match (bit_size, is_signed) {
                        (1..=8, false) => {
                            quote! {
                                0u8
                            }
                        }
                        (1..=8, true) => {
                            quote! {
                                0i8
                            }
                        }
                        (9..=16, false) => {
                            quote! {
                                0u16
                            }
                        }
                        (9..=16, true) => {
                            quote! {
                                0i16
                            }
                        }
                        (17..=32, false) => {
                            quote! {
                                0u32
                            }
                        }
                        (17..=32, true) => {
                            quote! {
                                0i32
                            }
                        }
                        (33..=64, false) => {
                            quote! {
                                0u64
                            }
                        }
                        (33..=64, true) => {
                            quote! {
                                0i64
                            }
                        }
                        _ => {
                            quote! {
                                0u64
                            }
                        }
                    }
                };
                field_inits.push(quote! {
                    # field_name : # placeholder_value
                });
            }
            let struct_item = quote! {
                #[allow(non_camel_case_types)] #[derive(Debug, Clone, PartialEq)] #[doc =
                # msg_doc] pub struct # msg_name { pub device_id : DeviceId, # (#
                fields),* }
            };
            file_content.push_str(&struct_item.to_string());
            generate_message_metadata(
                &mut file_content,
                &dbc,
                message,
                &msg_name,
                &full_message_ids,
            );
            let impl_item = generate_encode_decode_impl(&msg_name, &signals, base_id, dlc, source_addr);
            file_content.push_str(&impl_item.to_string());
        }
        for signal in &message.signals {
            let signal_name = &signal.name;
            let enum_key = format!("{}_{}", message.name, signal_name);
            if generated_enums.contains(&enum_key) {
                continue;
            }
            if let Some(val_desc) = dbc.value_descriptions_for_signal(message.id, signal_name) {
                generated_enums.insert(enum_key.clone());
                let enum_name = format_ident!("{}{}Enum", message.name, signal_name);
                let mut variants = Vec::new();
                for desc in val_desc.iter() {
                    let value = desc.id;
                    let description = &desc.description;
                    let sanitized_name = description
                        .chars()
                        .map(|c| {
                            if c.is_alphanumeric() || c == '_' {
                                c
                            } else {
                                '_'
                            }
                        })
                        .collect::<String>();
                    let unique_var_name = format!(
                        "{}_{}_{}_{}",
                        message.name, signal_name, sanitized_name, value
                    );
                    let var_name = format_ident!("{}", unique_var_name);
                    let value_lit = syn::LitInt::new(
                        &format!("0x{:02X}", value),
                        proc_macro2::Span::call_site(),
                    );
                    variants.push(quote! {
                        #[allow(non_camel_case_types)] # var_name = # value_lit
                    });
                }
                let enum_item = quote! {
                    #[allow(non_camel_case_types)] #[derive(Debug, Clone, Copy,
                    PartialEq, Eq)] #[repr(u32)] pub enum # enum_name { # (# variants),*
                    }
                };
                file_content.push_str(&enum_item.to_string());
            }
        }
    }
    generate_protocol_metadata(&mut file_content, &protocol_name, &message_names);

    // Get path string before creating file (to avoid borrow issues)
    let output_path_str = output_path
        .as_ref()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid output path"))?;

    let mut file = File::create(output_path.as_ref())?;
    file.write_all(file_content.as_bytes())?;
    drop(file); // Close file before running rustfmt

    // Format the generated code with rustfmt for human readability
    eprintln!("   🎨 Formatting generated code with rustfmt...");
    let rustfmt_result = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(output_path_str)
        .output();

    match rustfmt_result {
        Ok(output) if output.status.success() => {
            eprintln!("   ✅ Code formatted successfully");
        }
        Ok(output) => {
            eprintln!(
                "   ⚠️  rustfmt failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            eprintln!("   📝 Generated code saved but not formatted");
        }
        Err(e) => {
            eprintln!("   ⚠️  Could not run rustfmt: {}", e);
            eprintln!("   📝 Generated code saved but not formatted");
        }
    }

    Ok(())
}
/// Generate protocol-level metadata constant
fn generate_protocol_metadata(
    file_content: &mut String,
    protocol_name: &str,
    message_names: &[String],
) {
    let mut message_refs = Vec::new();
    for msg_name in message_names {
        let metadata_const_name = format!("{}_METADATA", msg_name.to_uppercase());
        message_refs.push(format!("        &{}", metadata_const_name));
    }
    let messages_array = if message_refs.is_empty() {
        String::from("&[]")
    } else {
        format!("&[\n{}\n    ]", message_refs.join(",\n"))
    };
    let protocol_constant = format!(
        r#"
/// Protocol-level metadata constant
pub const {}_METADATA: ProtocolMetadata = ProtocolMetadata {{
    name: "{}",
    version: "",
    messages: {},
    comment: "",
}};
"#,
        protocol_name, protocol_name, messages_array
    );
    file_content.push_str(&protocol_constant);
}
/// Generate documentation string for a message
fn generate_message_doc(
    dbc: &Dbc,
    message: &can_dbc::Message,
    full_message_ids: &std::collections::HashMap<String, u32>,
) -> String {
    let msg_name = &message.name;

    // Use full message ID from our parser, fall back to can-dbc parser if not found
    let raw_id = full_message_ids
        .get(msg_name)
        .copied()
        .unwrap_or_else(|| message.id.raw());
    let can_id = raw_id & BASE_ID_MASK;
    let comment = dbc
        .message_comment(message.id)
        .map(|c| c.trim())
        .filter(|c| !c.is_empty())
        .unwrap_or(msg_name);
    format!(
        "/// {} (CAN ID: 0x{:08X})\n///\n/// {}\n///\n/// # Example\n///\n/// ```\n/// use proteus_messages::common::{{DeviceId, DecodeError}};\n///\n/// # fn main() -> Result<(), DecodeError> {{\n/// let msg = {} {{\n///     device_id: DeviceId::HVPC_UNIT,\n/// }};\n/// assert_eq!(msg.device_id, DeviceId::HVPC_UNIT);\n/// # Ok(())\n/// # }}\n/// ```",
        msg_name, can_id, comment, msg_name
    )
}
/// Generate documentation string for a signal
fn generate_signal_doc(dbc: &Dbc, message: &can_dbc::Message, signal: &Signal) -> String {
    let signal_name = &signal.name;
    let min = signal.min;
    let max = signal.max;
    let unit = &signal.unit;
    let signal_comment = dbc.signal_comment(message.id, signal_name).unwrap_or("");
    let range_info = if !unit.is_empty() {
        format!(
            "/// Signal: {}\n/// Range: {:.2} to {:.2} {}",
            signal_name, min, max, unit
        )
    } else {
        format!(
            "/// Signal: {}\n/// Range: {:.2} to {:.2}",
            signal_name, min, max
        )
    };
    if !signal_comment.is_empty() {
        format!("{}\n/// {}", range_info, signal_comment)
    } else {
        range_info
    }
}
/// Generate static metadata for a message
fn generate_message_metadata(
    file_content: &mut String,
    dbc: &Dbc,
    message: &can_dbc::Message,
    msg_name: &proc_macro2::Ident,
    full_message_ids: &std::collections::HashMap<String, u32>,
) {
    let metadata_name = format!("{}_METADATA", msg_name.to_string().to_uppercase());
    let msg_name_str = &message.name;

    // Use full message ID from our parser, fall back to can-dbc parser if not found
    let raw_id = full_message_ids
        .get(msg_name_str)
        .copied()
        .unwrap_or_else(|| message.id.raw());
    let can_id = raw_id & BASE_ID_MASK;
    let dlc = message.size as u8;
    let comment = dbc.message_comment(message.id).unwrap_or("");
    let transmitter = String::new();
    let mut signal_metadata_items = Vec::new();
    for signal in &message.signals {
        let signal_name = &signal.name;
        let start_bit = signal.start_bit;
        let signal_size = signal.size;
        let byte_order = match &signal.byte_order {
            can_dbc::ByteOrder::LittleEndian => "LittleEndian",
            can_dbc::ByteOrder::BigEndian => "BigEndian",
        };
        let value_type = match &signal.value_type {
            can_dbc::ValueType::Signed => "Signed",
            can_dbc::ValueType::Unsigned => "Unsigned",
        };
        let factor = signal.factor;
        let offset = signal.offset;
        let min = signal.min;
        let max = signal.max;
        let unit = &signal.unit;
        let factor_str = if factor.fract() == 0.0 && factor.is_finite() {
            format!("{:.1}", factor)
        } else {
            format!("{}", factor)
        };
        let offset_str = if offset.fract() == 0.0 && offset.is_finite() {
            format!("{:.1}", offset)
        } else {
            format!("{}", offset)
        };
        let min_str = if min.fract() == 0.0 && min.is_finite() {
            format!("{:.1}", min)
        } else {
            format!("{}", min)
        };
        let max_str = if max.fract() == 0.0 && max.is_finite() {
            format!("{:.1}", max)
        } else {
            format!("{}", max)
        };
        let signal_comment = dbc.signal_comment(message.id, signal_name).unwrap_or("");
        let mut value_desc_items = Vec::new();
        eprintln!(
            "DEBUG: Looking up value descriptions for message_id={} (0x{:X}), signal='{}'",
            message.id.raw(),
            message.id.raw(),
            signal_name
        );
        if let Some(val_desc) = dbc.value_descriptions_for_signal(message.id, signal_name) {
            eprintln!(
                "DEBUG: Found {} value descriptions for signal '{}'",
                val_desc.len(),
                signal_name
            );
            for desc in val_desc.iter() {
                let value = desc.id;
                let description = &desc.description;
                value_desc_items.push(format!(
                    "({}, \"{}\")",
                    value,
                    description.replace("\"", "\\\"")
                ));
            }
        } else {
            eprintln!(
                "DEBUG: No value descriptions found for signal '{}'",
                signal_name
            );
        }
        let value_descriptions = if value_desc_items.is_empty() {
            "&[]".to_string()
        } else {
            format!("&[{}]", value_desc_items.join(", "))
        };
        signal_metadata_items.push(format!(
            r#"
                SignalMetadata {{
                    name: "{}",
                    start_bit: {},
                    signal_size: {},
                    byte_order: ByteOrder::{},
                    value_type: ValueType::{},
                    factor: {},
                    offset: {},
                    min: {},
                    max: {},
                    unit: "{}",
                    value_descriptions: {},
                    comment: "{}",
                }}"#,
            signal_name,
            start_bit,
            signal_size,
            byte_order,
            value_type,
            factor_str,
            offset_str,
            min_str,
            max_str,
            unit,
            value_descriptions,
            signal_comment.replace("\"", "\\\"")
        ));
    }
    let signals_array = if signal_metadata_items.is_empty() {
        "&[]".to_string()
    } else {
        format!("&[{}]", signal_metadata_items.join(","))
    };
    let is_multiplexed = message
        .signals
        .iter()
        .any(|s| matches!(s.multiplexer_indicator, MultiplexIndicator::Multiplexor));
    let metadata_def = format!(
        r#"
/// Metadata for {} message
pub static {}: MessageMetadata = MessageMetadata {{
    name: "{}",
    can_id: 0x{:08X},
    dlc: {},
    signals: {},
    is_multiplexed: {},
    comment: "{}",
    transmitter: "{}",
}};
"#,
        msg_name_str,
        metadata_name,
        msg_name_str,
        can_id,
        dlc,
        signals_array,
        is_multiplexed,
        comment.replace("\"", "\\\""),
        transmitter
    );
    file_content.push_str(&metadata_def);
    let metadata_impl = format!(
        r#"
impl HasMetadata for {} {{
    fn metadata() -> &'static MessageMetadata {{
        &{}
    }}
}}
"#,
        msg_name_str, metadata_name
    );
    file_content.push_str(&metadata_impl);
}
/// Determine the appropriate Rust type for a signal based on bit width and signedness
fn get_rust_type(signal: &Signal) -> proc_macro2::TokenStream {
    if signal.factor != 1.0 || signal.offset != 0.0 {
        return quote! {
            f64
        };
    }
    let is_signed = signal.value_type == can_dbc::ValueType::Signed;
    let bit_size = signal.size;
    match (bit_size, is_signed) {
        (1..=8, false) => {
            quote! {
                u8
            }
        }
        (1..=8, true) => {
            quote! {
                i8
            }
        }
        (9..=16, false) => {
            quote! {
                u16
            }
        }
        (9..=16, true) => {
            quote! {
                i16
            }
        }
        (17..=32, false) => {
            quote! {
                u32
            }
        }
        (17..=32, true) => {
            quote! {
                i32
            }
        }
        (33..=64, false) => {
            quote! {
                u64
            }
        }
        (33..=64, true) => {
            quote! {
                i64
            }
        }
        _ => {
            quote! {
                u64
            }
        }
    }
}
/// Generate full encode/decode implementation for a message
///
/// This function generates complete, working encode() and decode() methods
/// that replace the old stub implementations. It handles:
/// - Simple unsigned signals (factor=1.0, offset=0.0)
/// - Scaled float signals (with factor/offset)
/// - Signed signals
/// - Device ID embedding/extraction
fn generate_encode_decode_impl(
    msg_name: &proc_macro2::Ident,
    signals: &[&Signal],
    base_id: u32,
    dlc: u8,
    source_addr: u8,
) -> proc_macro2::TokenStream {
    let base_id_lit = syn::LitInt::new(&format!("{}", base_id), proc_macro2::Span::call_site());
    let dlc_lit = syn::LitInt::new(&format!("{}", dlc), proc_macro2::Span::call_site());
    let source_addr_lit = syn::LitInt::new(&format!("{}", source_addr), proc_macro2::Span::call_site());
    let mut decode_stmts = Vec::new();
    let mut struct_field_names = Vec::new();
    for signal in signals {
        let field_name = format_ident!("{}", to_rust_field_name(&signal.name));
        struct_field_names.push(field_name.clone());
        let start_bit = signal.start_bit as usize;
        let signal_size = signal.size as usize;
        let factor = signal.factor;
        let offset = signal.offset;
        let is_signed = signal.value_type == can_dbc::ValueType::Signed;
        if factor == 1.0 && offset == 0.0 {
            let bit_size = signal.size;
            let cast_type = match (bit_size, is_signed) {
                (1..=8, false) => {
                    quote! {
                        u8
                    }
                }
                (1..=8, true) => {
                    quote! {
                        i8
                    }
                }
                (9..=16, false) => {
                    quote! {
                        u16
                    }
                }
                (9..=16, true) => {
                    quote! {
                        i16
                    }
                }
                (17..=32, false) => {
                    quote! {
                        u32
                    }
                }
                (17..=32, true) => {
                    quote! {
                        i32
                    }
                }
                (33..=64, false) => {
                    quote! {
                        u64
                    }
                }
                (33..=64, true) => {
                    quote! {
                        i64
                    }
                }
                _ => {
                    quote! {
                        u64
                    }
                }
            };
            if bit_size <= 32 || is_signed {
                decode_stmts.push(quote! {
                    let # field_name = extract_signal(data, # start_bit, #
                    signal_size) ? as # cast_type;
                });
            } else {
                decode_stmts.push(quote! {
                    let # field_name = extract_signal(data, # start_bit, #
                    signal_size) ?;
                });
            }
        } else {
            let start_bit_lit =
                syn::LitInt::new(&format!("{}", start_bit), proc_macro2::Span::call_site());
            let signal_size_lit =
                syn::LitInt::new(&format!("{}", signal_size), proc_macro2::Span::call_site());
            let factor_str = format!("{}", factor);
            let factor_str = if !factor_str.contains('.') && !factor_str.contains('e') {
                format!("{}.0", factor_str)
            } else {
                factor_str
            };
            let offset_str = format!("{}", offset);
            let offset_str = if !offset_str.contains('.') && !offset_str.contains('e') {
                format!("{}.0", offset_str)
            } else {
                offset_str
            };
            let factor_lit = syn::LitFloat::new(&factor_str, proc_macro2::Span::call_site());
            let offset_lit = syn::LitFloat::new(&offset_str, proc_macro2::Span::call_site());
            decode_stmts.push(quote! {
                let # field_name = { let raw = extract_signal(data, #
                start_bit_lit, # signal_size_lit) ?; apply_scaling(raw, #
                factor_lit, # offset_lit, # is_signed, # signal_size_lit) };
            });
        }
    }
    let mut encode_stmts = Vec::new();
    for signal in signals {
        let field_name = format_ident!("{}", to_rust_field_name(&signal.name));
        let start_bit = signal.start_bit as usize;
        let signal_size = signal.size as usize;
        let factor = signal.factor;
        let offset = signal.offset;
        if factor == 1.0 && offset == 0.0 {
            let is_signed = signal.value_type == can_dbc::ValueType::Signed;
            let bit_size = signal.size;
            let needs_cast = is_signed || bit_size <= 32;
            if needs_cast {
                encode_stmts.push(quote! {
                    pack_signal(& mut data, # start_bit, # signal_size, self.#
                    field_name as u64) ?;
                });
            } else {
                encode_stmts.push(quote! {
                    pack_signal(& mut data, # start_bit, # signal_size, self.#
                    field_name) ?;
                });
            }
        } else {
            let start_bit_lit =
                syn::LitInt::new(&format!("{}", start_bit), proc_macro2::Span::call_site());
            let signal_size_lit =
                syn::LitInt::new(&format!("{}", signal_size), proc_macro2::Span::call_site());
            let factor_str = format!("{}", factor);
            let factor_str = if !factor_str.contains('.') && !factor_str.contains('e') {
                format!("{}.0", factor_str)
            } else {
                factor_str
            };
            let offset_str = format!("{}", offset);
            let offset_str = if !offset_str.contains('.') && !offset_str.contains('e') {
                format!("{}.0", offset_str)
            } else {
                offset_str
            };
            let factor_lit = syn::LitFloat::new(&factor_str, proc_macro2::Span::call_site());
            let offset_lit = syn::LitFloat::new(&offset_str, proc_macro2::Span::call_site());
            encode_stmts.push(quote! {
                { let raw = apply_inverse_scaling(self.# field_name, #
                factor_lit, # offset_lit, # signal_size_lit); pack_signal(& mut
                data, # start_bit_lit, # signal_size_lit, raw) ?; }
            });
        }
    }
    quote! {
        impl # msg_name { pub const BASE_CAN_ID : u32 = # base_id_lit; pub const DLC : u8
        = # dlc_lit; pub const SOURCE_ADDR : u8 = # source_addr_lit; pub fn decode(can_id : u32, data : & [u8]) -> Result < Self,
        DecodeError > { use crate ::encoder:: { extract_device_id, extract_signal,
        apply_scaling }; let device_id = extract_device_id(can_id); # (# decode_stmts) *
        Ok(Self { device_id, # (# struct_field_names),* }) } pub fn encode(& self) ->
        Result < (u32, [u8; # dlc_lit]), DecodeError > { use crate ::encoder:: {
        embed_device_id, pack_signal, apply_inverse_scaling }; let mut data = [0u8; #
        dlc_lit]; # (# encode_stmts) * let can_id = embed_device_id(Self::BASE_CAN_ID,
        self.device_id, Some(Self::SOURCE_ADDR)); Ok((can_id, data)) } }
    }
}
/// Generate full encode/decode implementation for a multiplexed message
///
/// Handles messages with multiplexor fields that switch between different payload variants
fn generate_multiplexed_encode_decode_impl(
    msg_name: &proc_macro2::Ident,
    payload_enum_name: &proc_macro2::Ident,
    mux_field: &Signal,
    mux_field_name: &proc_macro2::Ident,
    mux_values: &std::collections::BTreeMap<u64, Vec<&Signal>>,
    base_id: u32,
    dlc: u8,
    source_addr: u8,
) -> proc_macro2::TokenStream {
    let base_id_lit = syn::LitInt::new(&format!("{}", base_id), proc_macro2::Span::call_site());
    let dlc_lit = syn::LitInt::new(&format!("{}", dlc), proc_macro2::Span::call_site());
    let source_addr_lit = syn::LitInt::new(&format!("{}", source_addr), proc_macro2::Span::call_site());
    let mux_start_bit = mux_field.start_bit as usize;
    let mux_size = mux_field.size as usize;
    let mut decode_match_arms = Vec::new();
    for (mux_val, signals) in mux_values {
        let mux_val_lit = syn::LitInt::new(&format!("{}", mux_val), proc_macro2::Span::call_site());
        let variant_name = if *msg_name == "UDC_Command" {
            match mux_val {
                0 => format_ident!("Convert"),
                2 => format_ident!("Safe"),
                3 => format_ident!("NedReset"),
                4 => format_ident!("Shutdown"),
                _ => format_ident!("Opcode{}", mux_val),
            }
        } else {
            format_ident!("Variant{}", mux_val)
        };
        if signals.is_empty() {
            decode_match_arms.push(quote! {
                # mux_val_lit => # payload_enum_name::# variant_name,
            });
        } else {
            let mut sorted_signals = signals.clone();
            sorted_signals.sort_by_key(|s| s.start_bit);
            let mut field_decodes = Vec::new();
            let mut field_names = Vec::new();
            for signal in sorted_signals {
                let field_name = format_ident!("{}", to_rust_field_name(&signal.name));
                field_names.push(field_name.clone());
                let start_bit = signal.start_bit as usize;
                let signal_size = signal.size as usize;
                let factor = signal.factor;
                let offset = signal.offset;
                let is_signed = signal.value_type == can_dbc::ValueType::Signed;
                if factor == 1.0 && offset == 0.0 {
                    let bit_size = signal.size;
                    let cast_type = match (bit_size, is_signed) {
                        (1..=8, false) => {
                            quote! {
                                u8
                            }
                        }
                        (1..=8, true) => {
                            quote! {
                                i8
                            }
                        }
                        (9..=16, false) => {
                            quote! {
                                u16
                            }
                        }
                        (9..=16, true) => {
                            quote! {
                                i16
                            }
                        }
                        (17..=32, false) => {
                            quote! {
                                u32
                            }
                        }
                        (17..=32, true) => {
                            quote! {
                                i32
                            }
                        }
                        (33..=64, false) => {
                            quote! {
                                u64
                            }
                        }
                        (33..=64, true) => {
                            quote! {
                                i64
                            }
                        }
                        _ => {
                            quote! {
                                u64
                            }
                        }
                    };
                    if bit_size <= 32 || is_signed {
                        field_decodes.push(quote! {
                            let # field_name = extract_signal(data, # start_bit, #
                            signal_size) ? as # cast_type;
                        });
                    } else {
                        field_decodes.push(quote! {
                            let # field_name = extract_signal(data, # start_bit, #
                            signal_size) ?;
                        });
                    }
                } else {
                    let start_bit_lit =
                        syn::LitInt::new(&format!("{}", start_bit), proc_macro2::Span::call_site());
                    let signal_size_lit = syn::LitInt::new(
                        &format!("{}", signal_size),
                        proc_macro2::Span::call_site(),
                    );
                    let factor_str = format!("{}", factor);
                    let factor_str = if !factor_str.contains('.') && !factor_str.contains('e') {
                        format!("{}.0", factor_str)
                    } else {
                        factor_str
                    };
                    let offset_str = format!("{}", offset);
                    let offset_str = if !offset_str.contains('.') && !offset_str.contains('e') {
                        format!("{}.0", offset_str)
                    } else {
                        offset_str
                    };
                    let factor_lit =
                        syn::LitFloat::new(&factor_str, proc_macro2::Span::call_site());
                    let offset_lit =
                        syn::LitFloat::new(&offset_str, proc_macro2::Span::call_site());
                    field_decodes.push(quote! {
                        let # field_name = { let raw = extract_signal(data, #
                        start_bit_lit, # signal_size_lit) ?; apply_scaling(raw, #
                        factor_lit, # offset_lit, # is_signed, # signal_size_lit) };
                    });
                }
            }
            decode_match_arms.push(quote! {
                # mux_val_lit => { # (# field_decodes) * # payload_enum_name::#
                variant_name { # (# field_names),* } },
            });
        }
    }
    let mut encode_match_arms = Vec::new();
    for (mux_val, signals) in mux_values {
        let mux_val_lit = syn::LitInt::new(&format!("{}", mux_val), proc_macro2::Span::call_site());
        let variant_name = if *msg_name == "UDC_Command" {
            match mux_val {
                0 => format_ident!("Convert"),
                2 => format_ident!("Safe"),
                3 => format_ident!("NedReset"),
                4 => format_ident!("Shutdown"),
                _ => format_ident!("Opcode{}", mux_val),
            }
        } else {
            format_ident!("Variant{}", mux_val)
        };
        if signals.is_empty() {
            encode_match_arms.push(quote! {
                # payload_enum_name::# variant_name => { pack_signal(& mut data,
                # mux_start_bit, # mux_size, # mux_val_lit) ?; },
            });
        } else {
            let mut sorted_signals = signals.clone();
            sorted_signals.sort_by_key(|s| s.start_bit);
            let mut field_names = Vec::new();
            let mut field_encodes = Vec::new();
            for signal in sorted_signals {
                let field_name = format_ident!("{}", to_rust_field_name(&signal.name));
                field_names.push(field_name.clone());
                let start_bit = signal.start_bit as usize;
                let signal_size = signal.size as usize;
                let factor = signal.factor;
                let offset = signal.offset;
                if factor == 1.0 && offset == 0.0 {
                    let is_signed = signal.value_type == can_dbc::ValueType::Signed;
                    let bit_size = signal.size;
                    let needs_cast = is_signed || bit_size <= 32;
                    if needs_cast {
                        field_encodes.push(quote! {
                            pack_signal(& mut data, # start_bit, # signal_size, *#
                            field_name as u64) ?;
                        });
                    } else {
                        field_encodes.push(quote! {
                            pack_signal(& mut data, # start_bit, # signal_size, *#
                            field_name) ?;
                        });
                    }
                } else {
                    let start_bit_lit =
                        syn::LitInt::new(&format!("{}", start_bit), proc_macro2::Span::call_site());
                    let signal_size_lit = syn::LitInt::new(
                        &format!("{}", signal_size),
                        proc_macro2::Span::call_site(),
                    );
                    let factor_str = format!("{}", factor);
                    let factor_str = if !factor_str.contains('.') && !factor_str.contains('e') {
                        format!("{}.0", factor_str)
                    } else {
                        factor_str
                    };
                    let offset_str = format!("{}", offset);
                    let offset_str = if !offset_str.contains('.') && !offset_str.contains('e') {
                        format!("{}.0", offset_str)
                    } else {
                        offset_str
                    };
                    let factor_lit =
                        syn::LitFloat::new(&factor_str, proc_macro2::Span::call_site());
                    let offset_lit =
                        syn::LitFloat::new(&offset_str, proc_macro2::Span::call_site());
                    field_encodes.push(quote! {
                        { let raw = apply_inverse_scaling(*# field_name, #
                        factor_lit, # offset_lit, # signal_size_lit); pack_signal(&
                        mut data, # start_bit_lit, # signal_size_lit, raw) ?; }
                    });
                }
            }
            encode_match_arms.push(quote! {
                # payload_enum_name::# variant_name { # (# field_names),* } => {
                pack_signal(& mut data, # mux_start_bit, # mux_size, #
                mux_val_lit) ?; # (# field_encodes) * },
            });
        }
    }
    let mut opcode_constants = Vec::new();
    for mux_val in mux_values.keys() {
        let mux_val_lit = syn::LitInt::new(&format!("{}", mux_val), proc_macro2::Span::call_site());
        let variant_name = if *msg_name == "UDC_Command" {
            match mux_val {
                0 => "CONVERT",
                2 => "SAFE",
                3 => "NED_RESET",
                4 => "SHUTDOWN",
                _ => continue,
            }
        } else if *msg_name == "HVPC_Command" {
            match mux_val {
                0 => "CHANNEL_GROUP",
                1 => "GROUP_HVIL",
                2 => "NED_RESET",
                3 => "SHUTDOWN",
                4 => "VALVE",
                5 => "REPROGRAM_INIT",
                _ => continue,
            }
        } else {
            continue;
        };
        let const_name = format_ident!("OPCODE_{}", variant_name);
        opcode_constants.push(quote! {
            pub const # const_name : u64 = # mux_val_lit;
        });
    }
    let mux_type = get_rust_type(mux_field);
    quote! {
        impl # msg_name { pub const BASE_CAN_ID : u32 = # base_id_lit; pub const DLC : u8
        = # dlc_lit; pub const SOURCE_ADDR : u8 = # source_addr_lit; # (# opcode_constants) * pub fn decode(can_id : u32, data : & [u8])
        -> Result < Self, DecodeError > { use crate ::encoder:: { extract_device_id,
        extract_signal, apply_scaling }; let device_id = extract_device_id(can_id); let
        mux_value_raw = extract_signal(data, # mux_start_bit, # mux_size) ?; let payload
        = match mux_value_raw { # (# decode_match_arms) * _ => return
        Err(DecodeError::UnsupportedMux { mux : mux_value_raw }), }; Ok(Self { device_id,
        # mux_field_name : mux_value_raw as # mux_type, payload, }) } pub fn encode(&
        self) -> Result < (u32, [u8; # dlc_lit]), DecodeError > { use crate ::encoder:: {
        embed_device_id, pack_signal, apply_inverse_scaling }; let mut data = [0u8; #
        dlc_lit]; match & self.payload { # (# encode_match_arms) * } let can_id =
        embed_device_id(Self::BASE_CAN_ID, self.device_id, Some(Self::SOURCE_ADDR)); Ok((can_id, data)) } }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_signal_sorting_principle() {
        struct TestSignal {
            name: &'static str,
            start_bit: u64,
        }
        let mut signals = [
            TestSignal {
                name: "Aftrtrtmnt1Otlt1GsSnsrPwrSppl",
                start_bit: 2,
            },
            TestSignal {
                name: "Aftrtrtmnt1Otlt2GsSnsrPwrSppl",
                start_bit: 10,
            },
            TestSignal {
                name: "Aftrtrtmnt2Otlt1GsSnsrPwrSppl",
                start_bit: 6,
            },
            TestSignal {
                name: "EngnExhst1GsSnsr1PwrSppl",
                start_bit: 0,
            },
            TestSignal {
                name: "EngnExhst2GsSnsr1PwrSppl",
                start_bit: 4,
            },
            TestSignal {
                name: "EngnExhst1GsSnsr2PwrSppl",
                start_bit: 8,
            },
        ];
        let alphabetical: Vec<_> = signals.iter().map(|s| s.name).collect();
        assert_eq!(
            alphabetical,
            vec![
                "Aftrtrtmnt1Otlt1GsSnsrPwrSppl",
                "Aftrtrtmnt1Otlt2GsSnsrPwrSppl",
                "Aftrtrtmnt2Otlt1GsSnsrPwrSppl",
                "EngnExhst1GsSnsr1PwrSppl",
                "EngnExhst2GsSnsr1PwrSppl",
                "EngnExhst1GsSnsr2PwrSppl",
            ]
        );
        signals.sort_by_key(|s| s.start_bit);
        let bit_ordered: Vec<_> = signals.iter().map(|s| s.name).collect();
        assert_eq!(
            bit_ordered,
            vec![
                "EngnExhst1GsSnsr1PwrSppl",
                "Aftrtrtmnt1Otlt1GsSnsrPwrSppl",
                "EngnExhst2GsSnsr1PwrSppl",
                "Aftrtrtmnt2Otlt1GsSnsrPwrSppl",
                "EngnExhst1GsSnsr2PwrSppl",
                "Aftrtrtmnt1Otlt2GsSnsrPwrSppl",
            ]
        );
    }
}
