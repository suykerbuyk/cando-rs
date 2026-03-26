//! CAN-Do Code Generator - Standalone DBC to Rust Code Generator
//!
//! This tool generates Rust code from DBC (Database CAN) files for the CAN-Do project.
//! It tracks checksums to avoid unnecessary regeneration and works independently of the build system.
//!
//! # Usage
//!
//! Generate a specific protocol:
//! ```bash
//! cargo run --bin cando-codegen -- generate --protocol j1939
//! ```
//!
//! Generate all protocols (only if changed):
//! ```bash
//! cargo run --bin cando-codegen -- generate-all
//! ```
//!
//! Force regeneration:
//! ```bash
//! cargo run --bin cando-codegen -- generate --protocol j1939 --force
//! ```
//!
//! Check status of all protocols:
//! ```bash
//! cargo run --bin cando-codegen -- status
//! ```
//!
//! Validate that generated code matches DBC files:
//! ```bash
//! cargo run --bin cando-codegen -- validate
//! ```

mod field_name_converter;
mod generator;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Maximum number of history entries to keep in .checksums.json
/// This prevents unbounded growth of the history array over time.
/// Oldest entries are removed when limit is exceeded (FIFO).
const MAX_HISTORY_ENTRIES: usize = 20;

#[derive(Parser)]
#[command(
    name = "cando-codegen",
    about = "Generate Rust code from DBC files for CAN-Do",
    version,
    long_about = "A standalone code generator that transforms CAN DBC files into type-safe Rust code.\n\
                  \n\
                  OVERVIEW:\n\
                  This tool generates Rust protocol implementations from proprietary DBC (Database CAN) files.\n\
                  It uses SHA-256 checksums to intelligently regenerate only when source files change,\n\
                  following the \"C compiler model\" - only recompile what's needed.\n\
                  \n\
                  LICENSING MODEL:\n\
                  - DBC files: Proprietary (not redistributable)\n\
                  - Generated Rust: Derivative work (open-source, MIT/Apache-2.0)\n\
                  - This enables open-source distribution without requiring proprietary files\n\
                  \n\
                  FOR OPEN-SOURCE USERS:\n\
                  You don't need DBC files or this tool! Generated code is already included in the repository.\n\
                  Just run 'cargo build' normally.\n\
                  \n\
                  FOR MAINTAINERS:\n\
                  Use this tool when you have licensed DBC files and need to regenerate protocol implementations.\n\
                  Changes are tracked via checksums stored in dbc/.checksums.json\n\
                  \n\
                  Set CANDO_DBC_PATH to use DBC files from a different location (default: ./dbc).\n\
                  \n\
                  See doc/guide-codegen-quick-ref.md for complete documentation on the code generation workflow."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Generate man page and exit (internal use)
    #[arg(long = "generate-manpage", hide = true)]
    generate_manpage: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate code for a specific protocol
    ///
    /// Regenerates Rust code from a DBC file for the specified protocol.
    /// Uses SHA-256 checksums to detect changes - only regenerates if the
    /// DBC file content has changed since last generation.
    ///
    /// Example: cando-codegen generate --protocol j1939
    Generate {
        /// Protocol name (j1939, j1939-73)
        ///
        /// Available protocols:
        ///   j1939     - SAE J1939 vehicle bus (19 MB, 2,146 messages)
        ///   j1939-73  - J1939-73 diagnostics (1.7 MB)
        #[arg(short, long)]
        protocol: String,

        /// Force regeneration even if unchanged
        ///
        /// Bypasses checksum validation and regenerates the protocol
        /// regardless of whether the DBC file has changed.
        /// Useful after updating the code generator itself.
        #[arg(short, long)]
        force: bool,
    },

    /// Generate all protocols (only if changed)
    ///
    /// Intelligently regenerates Rust code for all protocols where
    /// the DBC file has changed since last generation. Uses SHA-256
    /// checksums stored in dbc/.checksums.json for change detection.
    ///
    /// This is the recommended command for daily maintenance.
    ///
    /// Example: cando-codegen generate-all
    GenerateAll {
        /// Force regeneration of all protocols
        ///
        /// Regenerates all protocols regardless of whether their
        /// DBC files have changed. Useful after generator updates.
        #[arg(short, long)]
        force: bool,
    },

    /// Validate that generated code matches DBC files
    ///
    /// Checks that the SHA-256 checksums of all DBC files match
    /// the checksums recorded in dbc/.checksums.json. Fails with
    /// non-zero exit code if any protocol is out of sync.
    ///
    /// This command is useful for:
    /// - CI/CD pipelines (validate before build)
    /// - Pre-commit hooks (ensure generated code is current)
    /// - Maintenance checks (verify everything is in sync)
    ///
    /// If DBC files are missing (normal for open-source users),
    /// this command gracefully reports the situation.
    ///
    /// Example: cando-codegen validate
    Validate,

    /// Show status of all protocols
    ///
    /// Displays a comprehensive table showing:
    /// - Whether DBC file exists
    /// - Whether generated Rust file exists
    /// - Whether checksum is recorded
    /// - Sync status (up-to-date vs needs regeneration)
    ///
    /// This is the recommended starting point for understanding
    /// the current state of code generation.
    ///
    /// Example: cando-codegen status
    Status,

    /// List all available protocols
    ///
    /// Shows all protocols that can be generated, along with
    /// their DBC file paths and descriptions.
    ///
    /// Example: cando-codegen list
    List,

    /// Detect algorithm evolution (DBC unchanged, output changed)
    ///
    /// Checks all protocols for the specific scenario where:
    /// - DBC file hash matches recorded hash (unchanged)
    /// - Generated output hash differs from recorded hash (changed)
    ///
    /// This scenario indicates codegen algorithm changes that require
    /// validation of dependent code (test scripts, documentation).
    ///
    /// Exit codes:
    ///   0 - No algorithm evolution detected
    ///   1 - Algorithm evolution detected (validation needed)
    ///   2 - Error during detection
    ///
    /// Example: cando-codegen detect-changes
    DetectChanges,
}

/// Enhanced checksum structure (v2.0) with output tracking and history
#[derive(Debug, Serialize, Deserialize)]
struct DbcChecksums {
    /// Schema version (currently "2.0", defaults to "1.0" if missing)
    #[serde(default = "default_version")]
    version: String,

    /// Codegen tool version
    #[serde(skip_serializing_if = "Option::is_none")]
    codegen_version: Option<String>,

    /// Hash of generator source files (to detect algorithm changes)
    #[serde(skip_serializing_if = "Option::is_none")]
    generator_source_hash: Option<String>,

    /// Last update timestamp (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<String>,

    /// Last forced regeneration timestamp (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    last_forced: Option<String>,

    /// Legacy v1 checksums field (for backward compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    checksums: Option<BTreeMap<String, String>>,

    /// DBC source file information
    #[serde(skip_serializing_if = "Option::is_none")]
    dbc_sources: Option<BTreeMap<String, DbcSourceInfo>>,

    /// Generated output file information (protocol -> file -> info)
    #[serde(skip_serializing_if = "Option::is_none")]
    generated_outputs: Option<BTreeMap<String, BTreeMap<String, GeneratedOutputInfo>>>,

    /// Codegen change history
    #[serde(skip_serializing_if = "Option::is_none")]
    codegen_history: Option<Vec<CodegenHistoryEntry>>,
}

/// Information about a DBC source file
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DbcSourceInfo {
    /// Relative path to DBC file
    path: String,

    /// SHA-256 hash of DBC file content
    hash: String,

    /// Last modification time (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<String>,

    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u64>,
}

/// Information about a generated output file
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GeneratedOutputInfo {
    /// SHA-256 hash of generated file content
    hash: String,

    /// File size in bytes
    size: u64,

    /// Generation timestamp (ISO 8601)
    generated: String,
}

/// Entry in the codegen change history
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CodegenHistoryEntry {
    /// Codegen version
    version: String,

    /// Change date (ISO 8601)
    date: String,

    /// Type of change (dbc_update, algorithm_enhancement, bug_fix, etc.)
    change_type: String,

    /// Human-readable description
    description: String,

    /// List of affected protocol names
    affected_protocols: Vec<String>,

    /// Whether this is a breaking change requiring validation
    breaking: bool,

    /// Optional migration notes for users
    #[serde(skip_serializing_if = "Option::is_none")]
    migration_notes: Option<String>,
}

/// Classification of codegen changes
#[derive(Debug, PartialEq, Eq)]
enum CodegenChangeType {
    /// DBC changed, output changed (expected)
    NormalDbc,
    /// DBC unchanged, output changed (algorithm evolution - needs validation)
    AlgorithmEvolution,
    /// Everything matches (clean state)
    Clean,
    /// Unexpected state (manual intervention needed)
    Unexpected,
}

/// Default version for v1 compatibility
fn default_version() -> String {
    "1.0".to_string()
}

impl Default for DbcChecksums {
    fn default() -> Self {
        Self {
            version: "2.0".to_string(),
            codegen_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            generator_source_hash: None,
            timestamp: Some(get_iso8601_timestamp()),
            last_forced: None,
            checksums: None, // Legacy field, not used in v2
            dbc_sources: Some(BTreeMap::new()),
            generated_outputs: Some(BTreeMap::new()),
            codegen_history: Some(Vec::new()),
        }
    }
}

impl DbcChecksums {
    /// Check if this is a v1 checksum file
    fn is_v1(&self) -> bool {
        // v1 has either no version field (defaults to "1.0"), or has checksums but no dbc_sources
        self.version == "1.0" || (self.checksums.is_some() && self.dbc_sources.is_none())
    }

    /// Migrate from v1 to v2 format
    fn migrate_from_v1(self) -> Self {
        if !self.is_v1() {
            return self;
        }

        let mut v2 = DbcChecksums::default();

        // Migrate legacy checksums to dbc_sources
        if let Some(legacy_checksums) = self.checksums {
            let mut dbc_sources = BTreeMap::new();

            for (protocol, hash) in legacy_checksums {
                // Find the corresponding DBC file path
                if let Some(config) = find_protocol_config(&protocol) {
                    dbc_sources.insert(
                        protocol.clone(),
                        DbcSourceInfo {
                            path: resolve_dbc_path(config.dbc_file).display().to_string(),
                            hash: hash.clone(),
                            modified: None,
                            size: None,
                        },
                    );
                }
            }

            v2.dbc_sources = Some(dbc_sources);
        }

        // Add migration history entry
        if let Some(ref mut history) = v2.codegen_history {
            history.push(CodegenHistoryEntry {
                version: env!("CARGO_PKG_VERSION").to_string(),
                date: get_iso8601_timestamp(),
                change_type: "migration".to_string(),
                description: "Migrated from v1.0 to v2.0 checksum format".to_string(),
                affected_protocols: vec![],
                breaking: false,
                migration_notes: Some(
                    "Automatic migration from legacy format. Output tracking now enabled."
                        .to_string(),
                ),
            });
        }

        v2
    }

    /// Get DBC hash for a protocol (works with both v1 and v2)
    fn get_dbc_hash(&self, protocol: &str) -> Option<String> {
        // Try v2 format first
        if let Some(ref sources) = self.dbc_sources
            && let Some(info) = sources.get(protocol)
        {
            return Some(info.hash.clone());
        }

        // Fall back to v1 format
        if let Some(ref checksums) = self.checksums {
            return checksums.get(protocol).cloned();
        }

        None
    }

    /// Set DBC hash for a protocol (v2 format)
    fn set_dbc_hash(&mut self, protocol: &str, hash: String, path: &str) {
        let sources = self.dbc_sources.get_or_insert_with(BTreeMap::new);

        // Get file metadata if possible
        let (modified, size) = get_file_metadata(path);

        sources.insert(
            protocol.to_string(),
            DbcSourceInfo {
                path: path.to_string(),
                hash,
                modified,
                size,
            },
        );

        self.timestamp = Some(get_iso8601_timestamp());
    }

    /// Get output hash for a specific file
    fn get_output_hash(&self, protocol: &str, file_path: &str) -> Option<String> {
        if let Some(ref outputs) = self.generated_outputs
            && let Some(protocol_outputs) = outputs.get(protocol)
            && let Some(info) = protocol_outputs.get(file_path)
        {
            return Some(info.hash.clone());
        }
        None
    }

    /// Set output hash for a specific file
    fn set_output_hash(&mut self, protocol: &str, file_path: &str, hash: String, size: u64) {
        let outputs = self.generated_outputs.get_or_insert_with(BTreeMap::new);
        let protocol_outputs = outputs.entry(protocol.to_string()).or_default();

        protocol_outputs.insert(
            file_path.to_string(),
            GeneratedOutputInfo {
                hash,
                size,
                generated: get_iso8601_timestamp(),
            },
        );

        self.timestamp = Some(get_iso8601_timestamp());
    }

    /// Add entry to codegen history with limit enforcement
    ///
    /// Maintains a maximum of MAX_HISTORY_ENTRIES entries using FIFO:
    /// - Oldest entries are removed first when limit is exceeded
    /// - Recent history is always preserved
    fn add_history_entry(&mut self, entry: CodegenHistoryEntry) {
        let history = self.codegen_history.get_or_insert_with(Vec::new);
        history.push(entry);

        // Enforce history limit (keep most recent entries)
        if history.len() > MAX_HISTORY_ENTRIES {
            let excess = history.len() - MAX_HISTORY_ENTRIES;
            history.drain(0..excess);
        }

        self.timestamp = Some(get_iso8601_timestamp());
    }
}

/// Get current timestamp in ISO 8601 format
fn get_iso8601_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time before UNIX epoch");

    // Format as ISO 8601: YYYY-MM-DDTHH:MM:SSZ
    let secs = now.as_secs();
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    // Days since Unix epoch (1970-01-01) to approximate date
    let year = 1970 + (days / 365);
    let day_of_year = days % 365;
    let month = (day_of_year / 30) + 1;
    let day = (day_of_year % 30) + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

/// Get file metadata (modification time and size)
fn get_file_metadata(path: &str) -> (Option<String>, Option<u64>) {
    if let Ok(metadata) = fs::metadata(path) {
        let size = Some(metadata.len());

        // Try to get modification time
        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|duration| {
                let secs = duration.as_secs();
                let days = secs / 86400;
                let remaining = secs % 86400;
                let hours = remaining / 3600;
                let minutes = (remaining % 3600) / 60;
                let seconds = remaining % 60;

                let year = 1970 + (days / 365);
                let day_of_year = days % 365;
                let month = (day_of_year / 30) + 1;
                let day = (day_of_year % 30) + 1;

                format!(
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                    year, month, day, hours, minutes, seconds
                )
            });

        (modified, size)
    } else {
        (None, None)
    }
}

/// Protocol configuration mapping protocol names to their DBC files and output paths.
/// `dbc_file` is relative to the DBC base directory (see [`dbc_base_dir`]).
struct ProtocolConfig {
    name: &'static str,
    dbc_file: &'static str,
    output_file: &'static str,
}

const PROTOCOLS: &[ProtocolConfig] = &[
    ProtocolConfig {
        name: "j1939",
        dbc_file: "j1939.dbc",
        output_file: "cando-messages/src/generated/j1939.rs",
    },
    ProtocolConfig {
        name: "j1939-73",
        dbc_file: "j1939-73-DTCs-split.dbc",
        output_file: "cando-messages/src/generated/j1939_73.rs",
    },
];

/// Returns the DBC base directory, respecting the `CANDO_DBC_PATH` environment
/// variable. Defaults to `"dbc"` (relative to the working directory).
fn dbc_base_dir() -> PathBuf {
    match std::env::var("CANDO_DBC_PATH") {
        Ok(val) if !val.is_empty() => PathBuf::from(val),
        _ => PathBuf::from("dbc"),
    }
}

/// Resolves a DBC-relative filename (e.g. `"j1939.dbc"`) against the base dir.
fn resolve_dbc_path(filename: &str) -> PathBuf {
    dbc_base_dir().join(filename)
}

/// Returns the path to the checksums file inside the DBC directory.
fn checksum_file() -> PathBuf {
    dbc_base_dir().join(".checksums.json")
}

fn main() -> Result<()> {
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.contains(&"--generate-manpage".to_string()) {
        #[cfg(feature = "manpages")]
        {
            use clap::CommandFactory;
            use clap_mangen::Man;
            let man = Man::new(Cli::command());
            man.render(&mut std::io::stdout())
                .expect("Failed to render man page to stdout");
            return Ok(());
        }
        #[cfg(not(feature = "manpages"))]
        {
            eprintln!("Error: manpages feature not enabled at compile time");
            eprintln!("Please rebuild with: cargo build --features=manpages");
            std::process::exit(1);
        }
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { protocol, force } => {
            generate_protocol(&protocol, force)?;
        }
        Commands::GenerateAll { force } => {
            generate_all_protocols(force)?;
        }
        Commands::Validate => {
            validate_all()?;
        }
        Commands::Status => {
            show_status()?;
        }
        Commands::List => {
            list_protocols();
        }
        Commands::DetectChanges => {
            detect_changes()?;
        }
    }

    Ok(())
}

fn find_protocol_config(protocol: &str) -> Option<&'static ProtocolConfig> {
    PROTOCOLS.iter().find(|p| p.name == protocol)
}

fn generate_protocol(protocol: &str, force: bool) -> Result<()> {
    let config = find_protocol_config(protocol)
        .ok_or_else(|| anyhow::anyhow!("Unknown protocol: {}", protocol))?;

    let dbc_path = resolve_dbc_path(config.dbc_file);
    let cksum_path = checksum_file();

    println!("Processing protocol: {}", config.name);

    // Check if DBC file exists
    if !dbc_path.exists() {
        eprintln!("DBC file not found: {}", dbc_path.display());
        eprintln!(
            "This is normal for open-source users without access to proprietary DBC files."
        );
        eprintln!(
            "   The project will use existing generated code from: {}",
            config.output_file
        );
        eprintln!("   No action needed - cargo build will work normally.");
        if dbc_base_dir() != Path::new("dbc") {
            eprintln!("   (CANDO_DBC_PATH={})", dbc_base_dir().display());
        }
        return Ok(());
    }

    let dbc_path_str = dbc_path.display().to_string();

    // Compute current DBC hash
    let current_dbc_hash = compute_file_hash(&dbc_path_str)
        .with_context(|| format!("Failed to compute hash for {}", dbc_path_str))?;

    // Load existing checksums
    let checksums = load_checksums(&cksum_path.display().to_string())?;

    // Detect what type of change has occurred
    let change_type = detect_codegen_change(config, &checksums, &current_dbc_hash)?;

    // Check if regeneration needed
    if !force {
        match change_type {
            CodegenChangeType::Clean => {
                println!(
                    "{} is up to date (DBC: {}..., output: in sync)",
                    config.name,
                    &current_dbc_hash[..12]
                );
                return Ok(());
            }
            CodegenChangeType::NormalDbc => {
                if let Some(saved_hash) = checksums.get_dbc_hash(config.name) {
                    println!(
                        "{} DBC changed (old: {}... -> new: {}...)",
                        config.name,
                        &saved_hash[..12],
                        &current_dbc_hash[..12]
                    );
                } else {
                    println!("{} has no recorded checksum, generating...", config.name);
                }
            }
            CodegenChangeType::AlgorithmEvolution => {
                println!("{} algorithm evolution detected!", config.name);
                println!(
                    "   DBC unchanged ({}...) but output differs",
                    &current_dbc_hash[..12]
                );
                println!("   This indicates codegen algorithm changes.");
                println!("   Consider running validation: scripts/validate_test_fields.sh");
            }
            CodegenChangeType::Unexpected => {
                println!(
                    "{} in unexpected state (first generation or manual edit)",
                    config.name
                );
            }
        }
    } else {
        println!("Force regeneration requested for {}", config.name);

        // Check for algorithm evolution even when forcing
        if change_type == CodegenChangeType::AlgorithmEvolution {
            println!("   Note: Algorithm evolution detected (DBC unchanged, output changed)");
        }
    }

    // Generate the code
    println!("Generating {} from {}...", config.name, dbc_path_str);

    // This would call the actual generation logic
    generate_rust_from_dbc(&dbc_path_str, config.output_file)
        .with_context(|| format!("Failed to generate code for {}", config.name))?;

    // Update DBC checksum
    let mut updated_checksums = checksums;
    updated_checksums.set_dbc_hash(config.name, current_dbc_hash.clone(), &dbc_path_str);

    // Calculate and store output hash
    if Path::new(config.output_file).exists() {
        let output_hash = compute_file_hash(config.output_file)
            .with_context(|| format!("Failed to compute output hash for {}", config.output_file))?;
        let output_size = fs::metadata(config.output_file)
            .map(|m| m.len())
            .unwrap_or(0);
        updated_checksums.set_output_hash(
            config.name,
            config.output_file,
            output_hash,
            output_size,
        );
    }

    // Mark as forced if applicable
    if force {
        updated_checksums.last_forced = Some(get_iso8601_timestamp());
    }

    // Update generator source hash
    if let Ok(generator_hash) = compute_generator_source_hash() {
        updated_checksums.generator_source_hash = Some(generator_hash);
    }

    // Check and track codegen version changes
    check_and_track_version_change(&mut updated_checksums);

    // Add history entry for this generation operation
    let change_description = match change_type {
        CodegenChangeType::NormalDbc => {
            format!("Regenerated {} due to DBC file changes", config.name)
        }
        CodegenChangeType::AlgorithmEvolution => {
            format!(
                "Regenerated {} due to codegen algorithm changes (DBC unchanged)",
                config.name
            )
        }
        CodegenChangeType::Unexpected | CodegenChangeType::Clean => {
            if force {
                format!("Force regenerated {}", config.name)
            } else {
                format!(
                    "Generated {} (first time or after manual changes)",
                    config.name
                )
            }
        }
    };

    updated_checksums.add_history_entry(CodegenHistoryEntry {
        version: env!("CARGO_PKG_VERSION").to_string(),
        date: get_iso8601_timestamp(),
        change_type: if force {
            "forced_regeneration".to_string()
        } else {
            match change_type {
                CodegenChangeType::NormalDbc => "dbc_update".to_string(),
                CodegenChangeType::AlgorithmEvolution => "algorithm_enhancement".to_string(),
                CodegenChangeType::Unexpected => "initial_generation".to_string(),
                CodegenChangeType::Clean => "regeneration".to_string(),
            }
        },
        description: change_description,
        affected_protocols: vec![config.name.to_string()],
        breaking: change_type == CodegenChangeType::AlgorithmEvolution,
        migration_notes: if change_type == CodegenChangeType::AlgorithmEvolution {
            Some("Algorithm changes may affect dependent code. Validate test scripts.".to_string())
        } else {
            None
        },
    });

    save_checksums(&cksum_path.display().to_string(), &updated_checksums)?;

    println!(
        "Successfully generated {} (DBC hash: {}...)",
        config.output_file,
        &current_dbc_hash[..12]
    );

    Ok(())
}

fn generate_all_protocols(force: bool) -> Result<()> {
    println!("Generating all protocols...\n");

    let mut success_count = 0;
    let mut skip_count = 0;
    let mut error_count = 0;

    for config in PROTOCOLS {
        match generate_protocol(config.name, force) {
            Ok(_) => {
                if resolve_dbc_path(config.dbc_file).exists() {
                    success_count += 1;
                } else {
                    skip_count += 1;
                }
            }
            Err(e) => {
                eprintln!("Error generating {}: {}", config.name, e);
                error_count += 1;
            }
        }
        println!();
    }

    println!("Summary:");
    println!("   Generated: {}", success_count);
    println!("   Skipped (no DBC): {}", skip_count);
    println!("   Errors: {}", error_count);

    if error_count > 0 {
        anyhow::bail!("Generation completed with {} errors", error_count);
    }

    Ok(())
}

fn validate_all() -> Result<()> {
    println!("Validating generated code against DBC files...\n");

    let checksums = load_checksums(&checksum_file().display().to_string())?;
    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut missing_count = 0;

    for config in PROTOCOLS {
        let dbc_path = resolve_dbc_path(config.dbc_file);
        if !dbc_path.exists() {
            println!(
                "{}: DBC file not present (OK for open-source users)",
                config.name
            );
            missing_count += 1;
            continue;
        }

        let current_hash = compute_file_hash(&dbc_path.display().to_string())?;

        match checksums.get_dbc_hash(config.name) {
            Some(saved_hash) if saved_hash == current_hash => {
                println!(
                    "{}: Valid (hash: {}...)",
                    config.name,
                    &current_hash[..12]
                );
                valid_count += 1;
            }
            Some(saved_hash) => {
                println!(
                    "{}: OUT OF SYNC! Saved: {}... Current: {}...",
                    config.name,
                    &saved_hash[..12],
                    &current_hash[..12]
                );
                println!(
                    "   Run: cargo run --bin cando-codegen -- generate --protocol {}",
                    config.name
                );
                invalid_count += 1;
            }
            None => {
                println!("{}: No checksum recorded", config.name);
                println!(
                    "   Run: cargo run --bin cando-codegen -- generate --protocol {}",
                    config.name
                );
                invalid_count += 1;
            }
        }
    }

    println!("\nValidation Summary:");
    println!("   Valid: {}", valid_count);
    println!("   Out of sync: {}", invalid_count);
    println!("   Missing DBC: {}", missing_count);

    if invalid_count > 0 {
        anyhow::bail!(
            "{} protocol(s) out of sync. Run generate-all to update.",
            invalid_count
        );
    }

    Ok(())
}

fn show_status() -> Result<()> {
    println!("CAN-Do Code Generator Status\n");

    let dbc_dir = dbc_base_dir();
    if dbc_dir != Path::new("dbc") {
        println!("DBC path: {}\n", dbc_dir.display());
    }

    let checksums = load_checksums(&checksum_file().display().to_string())?;

    println!("Protocol Status:");
    println!(
        "{:<15} {:<20} {:<15} Status",
        "Name", "DBC Present", "Generated"
    );
    println!("{}", "-".repeat(80));

    for config in PROTOCOLS {
        let dbc_path = resolve_dbc_path(config.dbc_file);
        let dbc_exists = dbc_path.exists();
        let output_exists = Path::new(config.output_file).exists();
        let has_checksum = checksums.get_dbc_hash(config.name).is_some();

        let status = if !dbc_exists && output_exists {
            "Ready (no DBC)"
        } else if dbc_exists && output_exists && has_checksum {
            let current_hash = compute_file_hash(&dbc_path.display().to_string())?;
            let saved_hash = checksums.get_dbc_hash(config.name).unwrap();
            if saved_hash == current_hash {
                "Up to date"
            } else {
                "Needs regen"
            }
        } else if !output_exists {
            "Missing output"
        } else {
            "Unknown"
        };

        println!(
            "{:<15} {:<20} {:<15} {}",
            config.name,
            if dbc_exists { "yes" } else { "no" },
            if output_exists { "yes" } else { "no" },
            status
        );
    }

    println!("\nTips:");
    println!("   - Run 'make codegen-all' to update all changed protocols");
    println!("   - Run 'make codegen-validate' to check for out-of-sync files");
    println!("   - Missing DBC files is normal for open-source users");

    Ok(())
}

fn list_protocols() {
    println!("Available Protocols:\n");
    for config in PROTOCOLS {
        let dbc_path = resolve_dbc_path(config.dbc_file);
        let marker = if dbc_path.exists() { "+" } else { "-" };
        println!("  {} {} ({})", marker, config.name, dbc_path.display());
    }
    println!("\n  + = DBC present, - = DBC missing (OK for open-source users)");
    if dbc_base_dir() != Path::new("dbc") {
        println!("  CANDO_DBC_PATH={}", dbc_base_dir().display());
    }
    println!("\nUsage:");
    println!("  cargo run --bin cando-codegen -- generate --protocol <name>");
}

fn detect_changes() -> Result<()> {
    println!("Detecting codegen algorithm changes...\n");

    let checksums = load_checksums(&checksum_file().display().to_string())?;
    let mut evolution_count = 0;
    let mut clean_count = 0;
    let mut normal_count = 0;
    let mut unexpected_count = 0;
    let mut evolution_protocols = Vec::new();

    for config in PROTOCOLS {
        let dbc_path = resolve_dbc_path(config.dbc_file);
        if !dbc_path.exists() {
            continue;
        }

        let current_dbc_hash = match compute_file_hash(&dbc_path.display().to_string()) {
            Ok(hash) => hash,
            Err(_) => continue,
        };

        let change_type = detect_codegen_change(config, &checksums, &current_dbc_hash)?;

        match change_type {
            CodegenChangeType::AlgorithmEvolution => {
                println!("{}: ALGORITHM EVOLUTION DETECTED", config.name);
                println!(
                    "   DBC unchanged ({}...), output changed",
                    &current_dbc_hash[..12]
                );
                evolution_count += 1;
                evolution_protocols.push(config.name);
            }
            CodegenChangeType::NormalDbc => {
                println!("{}: DBC file changed (normal update)", config.name);
                normal_count += 1;
            }
            CodegenChangeType::Clean => {
                println!("{}: Clean (in sync)", config.name);
                clean_count += 1;
            }
            CodegenChangeType::Unexpected => {
                println!("{}: Unexpected state", config.name);
                unexpected_count += 1;
            }
        }
    }

    println!("\nSummary:");
    println!("   Algorithm Evolution: {}", evolution_count);
    println!("   Normal Updates: {}", normal_count);
    println!("   Clean: {}", clean_count);
    println!("   Unexpected: {}", unexpected_count);

    if evolution_count > 0 {
        println!("\nACTION REQUIRED:");
        println!(
            "   Algorithm evolution detected in {} protocol(s)",
            evolution_count
        );
        println!("   Affected: {}", evolution_protocols.join(", "));
        println!("\n   Next steps:");
        println!("   1. Review generated code changes");
        println!("   2. Run: scripts/validate_test_fields.sh");
        println!("   3. Update test scripts if needed");
        println!("   4. Document changes in codegen_history");

        anyhow::bail!("Algorithm evolution requires validation");
    }

    println!("\nNo algorithm evolution detected - all clear!");
    Ok(())
}

fn compute_file_hash(path: &str) -> Result<String> {
    let content = fs::read(path).with_context(|| format!("Failed to read file: {}", path))?;
    let hash = Sha256::digest(&content);
    Ok(format!("{:x}", hash))
}

/// Compute combined hash of all generator source files
/// This allows detection of algorithm changes even when DBC files haven't changed
fn compute_generator_source_hash() -> Result<String> {
    use sha2::Digest;

    // Source files that affect code generation
    let source_files = [
        "cando-codegen/src/generator.rs",
        "cando-codegen/src/encoder.rs",
        "cando-codegen/src/field_name_converter.rs",
        "cando-codegen/src/main.rs",
    ];

    let mut combined_hasher = Sha256::new();

    for source_file in &source_files {
        if Path::new(source_file).exists() {
            let content = fs::read(source_file)
                .with_context(|| format!("Failed to read generator source: {}", source_file))?;
            combined_hasher.update(&content);
        }
    }

    let hash = combined_hasher.finalize();
    Ok(format!("{:x}", hash))
}

/// Detect what type of codegen change has occurred
/// Check if codegen tool version changed and record it in history
fn check_and_track_version_change(checksums: &mut DbcChecksums) {
    let current_version = env!("CARGO_PKG_VERSION");

    // Check if we have a saved version
    if let Some(ref saved_version) = checksums.codegen_version {
        // Version changed - record the upgrade
        if saved_version != current_version {
            println!(
                "Codegen tool upgraded: {} -> {}",
                saved_version, current_version
            );

            let entry = CodegenHistoryEntry {
                version: current_version.to_string(),
                date: get_iso8601_timestamp(),
                change_type: "tool_upgrade".to_string(),
                description: format!(
                    "Codegen tool upgraded from {} to {}",
                    saved_version, current_version
                ),
                affected_protocols: vec![],
                breaking: false,
                migration_notes: Some(
                    "Tool version changed. Review release notes for any breaking changes."
                        .to_string(),
                ),
            };

            checksums.add_history_entry(entry);
        }
    }

    // Update the stored version
    checksums.codegen_version = Some(current_version.to_string());
}

fn detect_codegen_change(
    config: &ProtocolConfig,
    checksums: &DbcChecksums,
    current_dbc_hash: &str,
) -> Result<CodegenChangeType> {
    // Get saved DBC hash
    let saved_dbc_hash = checksums.get_dbc_hash(config.name);

    // Determine if DBC changed
    let dbc_changed = match saved_dbc_hash {
        Some(ref saved) => saved != current_dbc_hash,
        None => true, // No saved hash means we treat it as changed
    };

    // Check if generator sources changed
    let current_generator_hash = compute_generator_source_hash().ok();
    let generator_changed = match (&checksums.generator_source_hash, &current_generator_hash) {
        (Some(saved), Some(current)) => saved != current,
        (None, Some(_)) => true, // No saved hash, treat as changed
        _ => false,
    };

    // Check if output file exists and get its hash
    let output_exists = Path::new(config.output_file).exists();
    let current_output_hash = if output_exists {
        compute_file_hash(config.output_file).ok()
    } else {
        None
    };

    // Get saved output hash
    let saved_output_hash = checksums.get_output_hash(config.name, config.output_file);

    // Determine if output changed
    let output_changed = match (saved_output_hash, current_output_hash) {
        (Some(saved), Some(current)) => saved != current,
        (None, Some(_)) => true, // No saved hash but file exists
        (Some(_), None) => true, // Had hash but file missing
        (None, None) => false,   // No saved hash, no file (clean for new protocol)
    };

    // Classify the change
    let change_type = match (dbc_changed, output_changed, generator_changed) {
        // DBC changed - normal update
        (true, _, _) => CodegenChangeType::NormalDbc,
        // DBC unchanged, but generator changed - algorithm evolution
        (false, _, true) => CodegenChangeType::AlgorithmEvolution,
        // DBC unchanged, generator unchanged, but output changed - unexpected/manual edit
        (false, true, false) => CodegenChangeType::AlgorithmEvolution,
        // Everything in sync
        (false, false, false) => {
            if saved_dbc_hash.is_some() {
                CodegenChangeType::Clean
            } else {
                CodegenChangeType::Unexpected // First time generation
            }
        }
    };

    Ok(change_type)
}

fn load_checksums(path: &str) -> Result<DbcChecksums> {
    if !Path::new(path).exists() {
        return Ok(DbcChecksums::default());
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read checksum file: {}", path))?;

    let mut checksums: DbcChecksums = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse checksum file: {}", path))?;

    // Auto-migrate v1 to v2 if needed
    if checksums.is_v1() {
        println!("Migrating checksum file from v1.0 to v2.0...");
        checksums = checksums.migrate_from_v1();
        println!("Migration complete - output tracking now enabled");
    }

    Ok(checksums)
}

fn save_checksums(path: &str, checksums: &DbcChecksums) -> Result<()> {
    // Ensure directory exists
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    let content =
        serde_json::to_string_pretty(checksums).context("Failed to serialize checksums to JSON")?;

    fs::write(path, content).with_context(|| format!("Failed to write checksum file: {}", path))?;

    Ok(())
}

/// Generate Rust code from a DBC file
///
/// This function invokes the actual code generation logic extracted from
/// the historical build.rs with the critical bit-order sorting fix applied.
///
/// ## Critical Fix Applied
///
/// This implementation sorts struct fields by bit position (start_bit) instead
/// of alphabetical order, fixing the J1939 field ordering bug that caused
/// 28/59 tier2 tests to fail (52% pass rate).
///
/// The fix ensures struct field order matches encode/decode parameter order,
/// eliminating field value mismatches in message encoding/decoding.
fn generate_rust_from_dbc(dbc_path: &str, output_path: &str) -> Result<()> {
    println!("   Generating Rust code from DBC file...");
    println!("   Input:  {}", dbc_path);
    println!("   Output: {}", output_path);

    // Call the actual generator module
    generator::generate_for_dbc(dbc_path, output_path)
        .with_context(|| format!("Failed to generate code from {}", dbc_path))?;

    println!("   Code generation complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_config_lookup() {
        let j1939 = find_protocol_config("j1939");
        assert!(j1939.is_some());
        assert_eq!(j1939.unwrap().name, "j1939");

        let invalid = find_protocol_config("invalid");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_checksum_serialization() {
        let mut checksums = DbcChecksums::default();
        checksums.set_dbc_hash("j1939", "abc123".to_string(), "dbc/j1939.dbc");

        let json = serde_json::to_string(&checksums).unwrap();
        let deserialized: DbcChecksums = serde_json::from_str(&json).unwrap();

        assert_eq!(
            checksums.get_dbc_hash("j1939"),
            deserialized.get_dbc_hash("j1939")
        );
    }

    #[test]
    fn test_v1_migration() {
        // Create a v1-style checksum structure
        let v1_json = r#"{
            "checksums": {
                "j1939": "abc123",
                "j1939-73": "def456"
            }
        }"#;

        let v1: DbcChecksums = serde_json::from_str(v1_json).unwrap();
        assert!(v1.is_v1());

        let v2 = v1.migrate_from_v1();
        assert_eq!(v2.version, "2.0");
        assert_eq!(v2.get_dbc_hash("j1939"), Some("abc123".to_string()));
        assert_eq!(v2.get_dbc_hash("j1939-73"), Some("def456".to_string()));
    }

    #[test]
    fn test_version_tracking() {
        // Create checksums with initial version
        let mut checksums = DbcChecksums {
            codegen_version: Some("0.1.0".to_string()),
            ..Default::default()
        };

        // Initially should have 1 history entry (from default())
        let initial_history_len = checksums.codegen_history.as_ref().unwrap().len();

        // Manually add a version upgrade entry to test the structure
        let upgrade_entry = CodegenHistoryEntry {
            version: "0.2.0".to_string(),
            date: get_iso8601_timestamp(),
            change_type: "tool_upgrade".to_string(),
            description: "Codegen tool upgraded from 0.1.0 to 0.2.0".to_string(),
            affected_protocols: vec![],
            breaking: false,
            migration_notes: Some(
                "Tool version changed. Review release notes for any breaking changes.".to_string(),
            ),
        };

        checksums.add_history_entry(upgrade_entry);

        // Verify history entry was added
        let history = checksums.codegen_history.as_ref().unwrap();
        assert_eq!(history.len(), initial_history_len + 1);

        // Verify the last entry is the upgrade
        let last_entry = history.last().unwrap();
        assert_eq!(last_entry.change_type, "tool_upgrade");
        assert_eq!(last_entry.version, "0.2.0");
        assert!(last_entry.description.contains("0.1.0"));
        assert!(last_entry.description.contains("0.2.0"));
        assert!(!last_entry.breaking);
    }

    #[test]
    fn test_history_limit_enforcement() {
        let mut checksums = DbcChecksums::default();

        // Add more than MAX_HISTORY_ENTRIES entries
        for i in 0..(MAX_HISTORY_ENTRIES + 5) {
            let entry = CodegenHistoryEntry {
                version: env!("CARGO_PKG_VERSION").to_string(),
                date: get_iso8601_timestamp(),
                change_type: "test_entry".to_string(),
                description: format!("Test entry {}", i),
                affected_protocols: vec!["test".to_string()],
                breaking: false,
                migration_notes: None,
            };
            checksums.add_history_entry(entry);
        }

        // Verify history was limited to MAX_HISTORY_ENTRIES
        let history = checksums.codegen_history.as_ref().unwrap();
        assert_eq!(
            history.len(),
            MAX_HISTORY_ENTRIES,
            "History should be limited to {} entries",
            MAX_HISTORY_ENTRIES
        );

        // Verify oldest entries were removed (FIFO)
        // The first entry should be "Test entry 5" (entries 0-4 were removed)
        assert_eq!(
            history[0].description, "Test entry 5",
            "Oldest entries should be removed first"
        );

        // Verify newest entries are preserved
        let last_entry = history.last().unwrap();
        assert_eq!(
            last_entry.description,
            format!("Test entry {}", MAX_HISTORY_ENTRIES + 4),
            "Most recent entries should be preserved"
        );
    }

    #[test]
    fn test_history_entry_types() {
        let mut checksums = DbcChecksums::default();

        // Test DBC update entry
        let dbc_entry = CodegenHistoryEntry {
            version: "0.1.0".to_string(),
            date: get_iso8601_timestamp(),
            change_type: "dbc_update".to_string(),
            description: "Regenerated j1939 due to DBC file changes".to_string(),
            affected_protocols: vec!["j1939".to_string()],
            breaking: false,
            migration_notes: None,
        };
        checksums.add_history_entry(dbc_entry);

        // Test algorithm enhancement entry
        let algo_entry = CodegenHistoryEntry {
            version: "0.1.0".to_string(),
            date: get_iso8601_timestamp(),
            change_type: "algorithm_enhancement".to_string(),
            description: "Regenerated j1939 due to codegen algorithm changes (DBC unchanged)"
                .to_string(),
            affected_protocols: vec!["j1939".to_string()],
            breaking: true,
            migration_notes: Some(
                "Algorithm changes may affect dependent code. Validate test scripts.".to_string(),
            ),
        };
        checksums.add_history_entry(algo_entry);

        // Test forced regeneration entry
        let forced_entry = CodegenHistoryEntry {
            version: "0.1.0".to_string(),
            date: get_iso8601_timestamp(),
            change_type: "forced_regeneration".to_string(),
            description: "Force regenerated j1939-73".to_string(),
            affected_protocols: vec!["j1939-73".to_string()],
            breaking: false,
            migration_notes: None,
        };
        checksums.add_history_entry(forced_entry);

        // Verify all entries were added
        let history = checksums.codegen_history.as_ref().unwrap();
        assert_eq!(history.len(), 3);

        // Verify entry types
        assert_eq!(history[0].change_type, "dbc_update");
        assert_eq!(history[1].change_type, "algorithm_enhancement");
        assert_eq!(history[2].change_type, "forced_regeneration");

        // Verify breaking flag is set correctly
        assert!(!history[0].breaking);
        assert!(history[1].breaking);
        assert!(!history[2].breaking);

        // Verify migration notes are present for algorithm changes
        assert!(history[0].migration_notes.is_none());
        assert!(history[1].migration_notes.is_some());
        assert!(history[2].migration_notes.is_none());
    }

    #[test]
    fn test_history_preserves_old_entries() {
        let mut checksums = DbcChecksums::default();

        // Add initial entries
        for i in 0..5 {
            let entry = CodegenHistoryEntry {
                version: "0.1.0".to_string(),
                date: get_iso8601_timestamp(),
                change_type: "initial".to_string(),
                description: format!("Initial entry {}", i),
                affected_protocols: vec![],
                breaking: false,
                migration_notes: None,
            };
            checksums.add_history_entry(entry);
        }

        // Verify we have 5 entries
        assert_eq!(checksums.codegen_history.as_ref().unwrap().len(), 5);

        // Add more entries (but not enough to trigger limit)
        for i in 5..10 {
            let entry = CodegenHistoryEntry {
                version: "0.2.0".to_string(),
                date: get_iso8601_timestamp(),
                change_type: "new".to_string(),
                description: format!("New entry {}", i),
                affected_protocols: vec![],
                breaking: false,
                migration_notes: None,
            };
            checksums.add_history_entry(entry);
        }

        // Verify all entries preserved (no limit reached)
        let history = checksums.codegen_history.as_ref().unwrap();
        assert_eq!(history.len(), 10);

        // Verify oldest entries still present
        assert_eq!(history[0].description, "Initial entry 0");
        assert_eq!(history[4].description, "Initial entry 4");

        // Verify newest entries present
        assert_eq!(history[9].description, "New entry 9");
    }
}
