//! CAN ID filtering for selective frame capture
//!
//! This module implements candump-compatible filter syntax for filtering CAN frames
//! by ID. Supports include/exclude filters, error frames, and per-interface filtering.
//!
//! # Filter Syntax
//!
//! - `id:mask` - Include filter: (received_id & mask) == (id & mask)
//! - `id~mask` - Exclude filter: (received_id & mask) != (id & mask)
//! - `#error` - Error frame filter
//! - `[j|J]` - Join mode: AND logic for multiple filters
//!
//! # Examples
//!
//! ```text
//! can0,123:7FF           # Only ID 0x123 on can0
//! can0,100~700:7FF       # Exclude IDs 0x100-0x1FF on can0
//! can0,123:7FF,j         # Multiple filters with AND logic
//! vcan0,#error           # Only error frames on vcan0
//! ```

use anyhow::{Context, Result};
use socketcan::{CanFrame, EmbeddedFrame, Frame, Id};

/// A single CAN ID filter specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterSpec {
    /// Include filter: matches if (id & mask) == (filter_id & mask)
    Include { id: u32, mask: u32 },

    /// Exclude filter: matches if (id & mask) != (filter_id & mask)
    Exclude { id: u32, mask: u32 },

    /// Error frame filter
    ErrorFrame,
}

impl FilterSpec {
    /// Parse a filter specification from a string
    pub fn parse(spec: &str) -> Result<Self> {
        let spec = spec.trim();

        // Check for error frame filter
        if spec == "#error" {
            return Ok(FilterSpec::ErrorFrame);
        }

        // Check for include filter (id:mask)
        if let Some((id_str, mask_str)) = spec.split_once(':') {
            let id = parse_hex_id(id_str)?;
            let mask = parse_hex_id(mask_str)?;
            return Ok(FilterSpec::Include { id, mask });
        }

        // Check for exclude filter (id~mask)
        if let Some((id_str, mask_str)) = spec.split_once('~') {
            let id = parse_hex_id(id_str)?;
            let mask = parse_hex_id(mask_str)?;
            return Ok(FilterSpec::Exclude { id, mask });
        }

        anyhow::bail!(
            "Invalid filter specification: '{}'. Expected format: id:mask, id~mask, or #error",
            spec
        )
    }

    /// Check if a CAN frame matches this filter
    pub fn matches(&self, frame: &CanFrame) -> bool {
        match self {
            FilterSpec::Include { id, mask } => {
                let frame_id = get_raw_id(frame);
                (frame_id & mask) == (*id & mask)
            }
            FilterSpec::Exclude { id, mask } => {
                let frame_id = get_raw_id(frame);
                (frame_id & mask) != (*id & mask)
            }
            FilterSpec::ErrorFrame => frame.is_error_frame(),
        }
    }
}

/// Join mode for combining multiple filters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinMode {
    /// OR logic: frame passes if ANY filter matches
    Or,
    /// AND logic: frame passes if ALL filters match
    And,
}

/// Filter configuration for a single interface
#[derive(Debug, Clone)]
pub struct InterfaceFilter {
    /// Interface name (e.g., "can0", "vcan0")
    pub interface: String,

    /// List of filter specifications
    pub filters: Vec<FilterSpec>,

    /// How to combine multiple filters
    pub join_mode: JoinMode,
}

impl InterfaceFilter {
    /// Create a new interface filter with OR join mode (default)
    pub fn new(interface: String) -> Self {
        Self {
            interface,
            filters: Vec::new(),
            join_mode: JoinMode::Or,
        }
    }

    /// Create an interface filter with specific join mode
    #[allow(dead_code)]
    pub fn with_join_mode(interface: String, join_mode: JoinMode) -> Self {
        Self {
            interface,
            filters: Vec::new(),
            join_mode,
        }
    }

    /// Add a filter specification
    pub fn add_filter(&mut self, filter: FilterSpec) {
        self.filters.push(filter);
    }

    /// Check if a frame passes this interface's filters
    pub fn matches(&self, frame: &CanFrame) -> bool {
        if self.filters.is_empty() {
            return true; // No filters = pass all
        }

        match self.join_mode {
            JoinMode::Or => self.filters.iter().any(|f| f.matches(frame)),
            JoinMode::And => self.filters.iter().all(|f| f.matches(frame)),
        }
    }
}

/// Parse interface filter specification from command-line format
///
/// # Format
///
/// `interface[,filter1][,filter2][,...][,j|J]`
///
/// # Examples
///
/// - `can0` - No filters (pass all)
/// - `can0,123:7FF` - Single include filter
/// - `can0,100~700:7FF,200~700:7FF` - Multiple exclude filters
/// - `can0,123:7FF,456:7FF,j` - Multiple filters with AND logic
pub fn parse_interface_filter(spec: &str) -> Result<(String, InterfaceFilter)> {
    let parts: Vec<&str> = spec.split(',').collect();

    if parts.is_empty() {
        anyhow::bail!("Empty interface specification");
    }

    let interface = parts[0].to_string();
    let mut filter = InterfaceFilter::new(interface.clone());

    // Process remaining parts as filters
    for part in &parts[1..] {
        let part = part.trim();

        // Check for join mode specifier
        if part == "j" || part == "J" {
            filter.join_mode = JoinMode::And;
            continue;
        }

        // Parse as filter spec
        let spec = FilterSpec::parse(part)
            .with_context(|| format!("Failed to parse filter specification: '{}'", part))?;

        filter.add_filter(spec);
    }

    Ok((interface, filter))
}

/// Collection of filters for all interfaces
#[derive(Debug, Clone)]
pub struct FilterSet {
    /// Filters keyed by interface name
    filters: std::collections::HashMap<String, InterfaceFilter>,
}

impl FilterSet {
    /// Create an empty filter set
    pub fn new() -> Self {
        Self {
            filters: std::collections::HashMap::new(),
        }
    }

    /// Add an interface filter
    pub fn add_interface_filter(&mut self, filter: InterfaceFilter) {
        self.filters.insert(filter.interface.clone(), filter);
    }

    /// Check if a frame from a specific interface passes filters
    pub fn matches(&self, interface: &str, frame: &CanFrame) -> bool {
        match self.filters.get(interface) {
            Some(filter) => filter.matches(frame),
            None => true, // No filter for this interface = pass all
        }
    }

    /// Get the number of interfaces with filters configured
    pub fn interface_count(&self) -> usize {
        self.filters.len()
    }

    /// Check if any filters are configured
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }
}

impl Default for FilterSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a hex CAN ID from a string
fn parse_hex_id(s: &str) -> Result<u32> {
    let s = s.trim();

    // Remove 0x prefix if present
    let s = s.strip_prefix("0x").unwrap_or(s);
    let s = s.strip_prefix("0X").unwrap_or(s);

    u32::from_str_radix(s, 16)
        .with_context(|| format!("Invalid hex ID: '{}'", s))
        .and_then(|id| {
            // Validate CAN ID range (29-bit max)
            if id > 0x1FFFFFFF {
                anyhow::bail!("CAN ID 0x{:X} exceeds maximum (0x1FFFFFFF)", id);
            }
            Ok(id)
        })
}

/// Get raw CAN ID from a frame
fn get_raw_id(frame: &CanFrame) -> u32 {
    match frame.id() {
        Id::Standard(id) => id.as_raw() as u32,
        Id::Extended(id) => id.as_raw(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socketcan::{ExtendedId, StandardId};

    fn create_test_frame(id: u32, extended: bool) -> CanFrame {
        if extended {
            let ext_id = ExtendedId::new(id).unwrap();
            CanFrame::new(Id::Extended(ext_id), &[]).unwrap()
        } else {
            let std_id = StandardId::new(id as u16).unwrap();
            CanFrame::new(Id::Standard(std_id), &[]).unwrap()
        }
    }

    #[test]
    fn test_parse_hex_id() {
        assert_eq!(parse_hex_id("123").unwrap(), 0x123);
        assert_eq!(parse_hex_id("0x123").unwrap(), 0x123);
        assert_eq!(parse_hex_id("7FF").unwrap(), 0x7FF);
        assert_eq!(parse_hex_id("0X7FF").unwrap(), 0x7FF);
        assert_eq!(parse_hex_id("18FEF100").unwrap(), 0x18FEF100);
    }

    #[test]
    fn test_parse_hex_id_invalid() {
        assert!(parse_hex_id("ZZZ").is_err());
        assert!(parse_hex_id("20000000").is_err()); // Exceeds 29-bit max
        assert!(parse_hex_id("").is_err());
    }

    #[test]
    fn test_filter_spec_parse_include() {
        let filter = FilterSpec::parse("123:7FF").unwrap();
        assert_eq!(
            filter,
            FilterSpec::Include {
                id: 0x123,
                mask: 0x7FF
            }
        );
    }

    #[test]
    fn test_filter_spec_parse_exclude() {
        let filter = FilterSpec::parse("100~7FF").unwrap();
        assert_eq!(
            filter,
            FilterSpec::Exclude {
                id: 0x100,
                mask: 0x7FF
            }
        );
    }

    #[test]
    fn test_filter_spec_parse_error() {
        let filter = FilterSpec::parse("#error").unwrap();
        assert_eq!(filter, FilterSpec::ErrorFrame);
    }

    #[test]
    fn test_filter_spec_parse_invalid() {
        assert!(FilterSpec::parse("invalid").is_err());
        assert!(FilterSpec::parse("123").is_err()); // Missing mask
        assert!(FilterSpec::parse("").is_err());
    }

    #[test]
    fn test_filter_spec_include_matches() {
        let filter = FilterSpec::Include {
            id: 0x123,
            mask: 0x7FF,
        };

        let frame = create_test_frame(0x123, false);
        assert!(filter.matches(&frame));

        let frame = create_test_frame(0x124, false);
        assert!(!filter.matches(&frame));
    }

    #[test]
    fn test_filter_spec_exclude_matches() {
        let filter = FilterSpec::Exclude {
            id: 0x100,
            mask: 0x700,
        };

        // IDs in 0x100-0x1FF range should NOT match (excluded)
        let frame = create_test_frame(0x123, false);
        assert!(!filter.matches(&frame));

        // IDs outside range should match
        let frame = create_test_frame(0x456, false);
        assert!(filter.matches(&frame));
    }

    #[test]
    fn test_filter_spec_extended_id() {
        let filter = FilterSpec::Include {
            id: 0x18FEF100,
            mask: 0x1FFFFFFF,
        };

        let frame = create_test_frame(0x18FEF100, true);
        assert!(filter.matches(&frame));

        let frame = create_test_frame(0x18FEF101, true);
        assert!(!filter.matches(&frame));
    }

    #[test]
    fn test_interface_filter_no_filters() {
        let filter = InterfaceFilter::new("can0".to_string());
        let frame = create_test_frame(0x123, false);

        // No filters = pass all
        assert!(filter.matches(&frame));
    }

    #[test]
    fn test_interface_filter_or_mode() {
        let mut filter = InterfaceFilter::new("can0".to_string());
        filter.add_filter(FilterSpec::Include {
            id: 0x123,
            mask: 0x7FF,
        });
        filter.add_filter(FilterSpec::Include {
            id: 0x456,
            mask: 0x7FF,
        });

        // Should match if ANY filter matches (OR mode)
        let frame1 = create_test_frame(0x123, false);
        assert!(filter.matches(&frame1));

        let frame2 = create_test_frame(0x456, false);
        assert!(filter.matches(&frame2));

        let frame3 = create_test_frame(0x789, false);
        assert!(!filter.matches(&frame3));
    }

    #[test]
    fn test_interface_filter_and_mode() {
        let mut filter = InterfaceFilter::with_join_mode("can0".to_string(), JoinMode::And);

        // Include 0x1xx range
        filter.add_filter(FilterSpec::Include {
            id: 0x100,
            mask: 0x700,
        });

        // Exclude 0x123 specifically
        filter.add_filter(FilterSpec::Exclude {
            id: 0x123,
            mask: 0x7FF,
        });

        // 0x123 matches include but not exclude = fail AND
        let frame1 = create_test_frame(0x123, false);
        assert!(!filter.matches(&frame1));

        // 0x124 matches include and exclude = pass AND
        let frame2 = create_test_frame(0x124, false);
        assert!(filter.matches(&frame2));

        // 0x456 doesn't match include = fail AND
        let frame3 = create_test_frame(0x456, false);
        assert!(!filter.matches(&frame3));
    }

    #[test]
    fn test_parse_interface_filter_simple() {
        let (iface, filter) = parse_interface_filter("can0").unwrap();
        assert_eq!(iface, "can0");
        assert!(filter.filters.is_empty());
        assert_eq!(filter.join_mode, JoinMode::Or);
    }

    #[test]
    fn test_parse_interface_filter_with_filters() {
        let (iface, filter) = parse_interface_filter("can0,123:7FF,456:7FF").unwrap();
        assert_eq!(iface, "can0");
        assert_eq!(filter.filters.len(), 2);
        assert_eq!(filter.join_mode, JoinMode::Or);
    }

    #[test]
    fn test_parse_interface_filter_with_join() {
        let (iface, filter) = parse_interface_filter("can0,123:7FF,456:7FF,j").unwrap();
        assert_eq!(iface, "can0");
        assert_eq!(filter.filters.len(), 2);
        assert_eq!(filter.join_mode, JoinMode::And);
    }

    #[test]
    fn test_parse_interface_filter_mixed() {
        let (iface, filter) = parse_interface_filter("vcan0,100:7FF,200~7FF,#error,J").unwrap();
        assert_eq!(iface, "vcan0");
        assert_eq!(filter.filters.len(), 3);
        assert_eq!(filter.join_mode, JoinMode::And);
    }

    #[test]
    fn test_filter_set_empty() {
        let filter_set = FilterSet::new();
        let frame = create_test_frame(0x123, false);

        // No filters = pass all
        assert!(filter_set.matches("can0", &frame));
        assert!(filter_set.is_empty());
    }

    #[test]
    fn test_filter_set_with_interface_filter() {
        let mut filter_set = FilterSet::new();

        let mut can0_filter = InterfaceFilter::new("can0".to_string());
        can0_filter.add_filter(FilterSpec::Include {
            id: 0x123,
            mask: 0x7FF,
        });

        filter_set.add_interface_filter(can0_filter);

        // Frame that matches filter
        let frame1 = create_test_frame(0x123, false);
        assert!(filter_set.matches("can0", &frame1));

        // Frame that doesn't match filter
        let frame2 = create_test_frame(0x456, false);
        assert!(!filter_set.matches("can0", &frame2));

        // Different interface (no filter) = pass all
        assert!(filter_set.matches("can1", &frame2));
    }

    #[test]
    fn test_filter_set_multiple_interfaces() {
        let mut filter_set = FilterSet::new();

        let mut can0_filter = InterfaceFilter::new("can0".to_string());
        can0_filter.add_filter(FilterSpec::Include {
            id: 0x123,
            mask: 0x7FF,
        });

        let mut can1_filter = InterfaceFilter::new("can1".to_string());
        can1_filter.add_filter(FilterSpec::Include {
            id: 0x456,
            mask: 0x7FF,
        });

        filter_set.add_interface_filter(can0_filter);
        filter_set.add_interface_filter(can1_filter);

        assert_eq!(filter_set.interface_count(), 2);

        // Test can0 filter
        let frame1 = create_test_frame(0x123, false);
        assert!(filter_set.matches("can0", &frame1));
        assert!(!filter_set.matches("can1", &frame1));

        // Test can1 filter
        let frame2 = create_test_frame(0x456, false);
        assert!(!filter_set.matches("can0", &frame2));
        assert!(filter_set.matches("can1", &frame2));
    }
}
