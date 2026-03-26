# Code Generator Quick Reference

**Last Updated**: 2026-03-26
**Purpose**: Complete reference for cando-codegen DBC → Rust code generation

---

## For Open Source Users (No DBC Files)

You don't need DBC files! Generated code is already in the repository.

```bash
git clone https://github.com/suykerbuyk/cando-rs.git
cd cando-rs
cargo build --release
```

That's it! Skip to the [Troubleshooting](#troubleshooting) section if you have build issues.

---

## For Maintainers (With DBC Files)

### DBC Repository Setup

DBC files live in a separate private repository and are gitignored from
cando-rs. The `CANDO_DBC_PATH` environment variable controls where the
codegen tool looks for DBC files (default: `./dbc`).

**Option A — Clone into the default location:**

```bash
make dbc-init                  # clones into ./dbc (gitignored)
make codegen-status            # verify DBC files are detected
```

**Option B — Use DBC files from an external path:**

```bash
export CANDO_DBC_PATH=~/code/cando-dbc-private
make codegen-status            # reads from $CANDO_DBC_PATH
```

**Keeping DBC files up to date:**

```bash
make dbc-pull                  # git pull --ff-only inside the DBC repo
make dbc-status                # show DBC repo branch/commit
```

### First Time Code Generation

```bash
# After dbc-init or setting CANDO_DBC_PATH:
cargo run -p cando-codegen -- generate-all
```

### Daily Workflow

```bash
# Check what needs regeneration
cargo run -p cando-codegen -- status

# Generate changed protocols (smart - only if DBC or generator changed)
cargo run -p cando-codegen -- generate-all

# Generate specific protocol
cargo run -p cando-codegen -- generate --protocol j1939

# Force regeneration (bypasses change detection)
cargo run -p cando-codegen -- generate --protocol emp --force
```

### After Editing a DBC File

```bash
# 1. Edit DBC file
vim dbc/j1939.dbc

# 2. Regenerate (automatic detection of changes)
cargo run -p cando-codegen -- generate --protocol j1939

# 3. Review changes
git diff cando-messages/src/generated/j1939.rs

# 4. Test
cargo test -p cando-messages

# 5. Commit (ONLY generated code, NOT DBC file!)
git add cando-messages/src/generated/j1939.rs
git add dbc/.checksums.json
git commit -m "Update J1939 protocol definitions"
```

### After Modifying the Generator

The generator now automatically detects when its own source code changes:

```bash
# 1. Edit generator code
vim cando-codegen/src/generator.rs

# 2. Regenerate all protocols (automatically detects generator changes)
cargo run -p cando-codegen -- generate-all

# The generator will show:
#   ⚠️ emp algorithm evolution detected!
#   This indicates codegen algorithm changes.

# 3. Review ALL generated files (all protocols affected)
git diff cando-messages/src/generated/

# 4. Test thoroughly
cargo test --workspace

# 5. Commit generator + ALL generated code
git add cando-codegen/src/
git add cando-messages/src/generated/
git add dbc/.checksums.json
git commit -m "fix(codegen): improve generated code quality"
```

**Important**: When the generator changes, ALL protocols must be regenerated to maintain consistency.

---

## 📋 Command Reference

### List Protocols

```bash
cargo run -p cando-codegen -- list
```

Output: `j1939, j1939-73`

### Check Status

```bash
cargo run -p cando-codegen -- status
```

Shows:
- Which protocols are up-to-date
- Which need regeneration (DBC changed)
- Which have algorithm evolution (generator changed)
- Which are missing DBC files

### Generate Specific Protocol

```bash
cargo run -p cando-codegen -- generate --protocol <name> [--force]
```

- Without `--force`: Only regenerates if DBC file OR generator source changed
- With `--force`: Always regenerates (useful for testing)

### Generate All Changed

```bash
cargo run -p cando-codegen -- generate-all [--force]
```

Intelligently regenerates only protocols that need it based on:
1. DBC file hash changes
2. Generator source code hash changes
3. Manual edits to generated files

### Validate Checksums

```bash
cargo run -p cando-codegen -- validate
```

Checks if generated code matches DBC files. Fails if out of sync.

### Detect Algorithm Changes

```bash
cargo run -p cando-codegen -- detect-changes
```

Reports protocols where:
- DBC unchanged BUT output changed (algorithm evolution or manual edits)
- Useful for detecting unintended changes

---

## 🏗️ Architecture

### The "C Compiler Model"

```
DBC files     → Generated Rust → Binary
(not in git)    (in git)        (not in git)
```

Just like:
- `.c` source → `.o` object → executable
- Only regenerate when source changes
- Object files can be distributed

### Change Detection System

The generator tracks THREE types of changes:

1. **DBC File Changes** (SHA-256 hash)
   - Stored in `dbc/.checksums.json`
   - Compared before each generation
   
2. **Generator Source Changes** (SHA-256 hash of all generator .rs files)
   - Tracks: `generator.rs`, `encoder.rs`, `field_name_converter.rs`, `main.rs`
   - Combined hash stored in `.checksums.json`
   - Automatically triggers regeneration when generator code changes
   
3. **Generated Output Changes**
   - SHA-256 hash of generated .rs files
   - Used to detect manual edits or unexpected changes

**Change Types**:
- `NormalDbc`: DBC file changed → regenerate
- `AlgorithmEvolution`: Generator changed OR output differs → regenerate
- `Clean`: Everything in sync → skip
- `Unexpected`: First generation or manual edits detected

### Licensing Model

- **DBC files**: Proprietary, not redistributable
- **Generated Rust**: Derivative work, IS redistributable (MIT/Apache-2.0)
- Similar to: compiled binaries from source code

### Generated Code Structure

For each message, generates:

```rust
// 1. Message struct (fields in bit-position order!)
#[derive(Debug, Clone, PartialEq)]
pub struct EEC12 {
    pub device_id: DeviceId,
    pub engn_exhst_1_gs_snsr_1_pwr_sppl: u64,  // Bit 0-1
    pub aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl: u64,  // Bit 2-3
    pub engn_exhst_2_gs_snsr_1_pwr_sppl: u64,  // Bit 4-5
    // ... more fields in bit position order
}

// 2. Implementation with encode/decode
impl EEC12 {
    pub const BASE_CAN_ID: u32 = 0x18FCCC00;  // Device byte masked!
    pub const DLC: u8 = 8;
    
    pub fn decode(can_id: u32, data: &[u8]) -> Result<Self, DecodeError> {
        // Extract signals from CAN data bytes
        // ...
    }
    
    pub fn encode(&self) -> Result<(u32, [u8; 8]), DecodeError> {
        // Pack signals into CAN data bytes
        // ...
    }
}

// 3. Message metadata
pub static EEC12_METADATA: MessageMetadata = MessageMetadata {
    name: "EEC12",
    can_id: 0x18FCCC00,
    dlc: 8,
    signals: &[ /* ... */ ],
};
```

**Critical Design Decisions**:

1. **Fields in Bit-Position Order**: Struct fields match the bit layout in the CAN message, NOT alphabetical DBC order. This ensures decode() can unpack directly into the struct.

2. **Device Byte Masked**: `BASE_CAN_ID` has the device ID byte (bits 7-0) cleared to `0x00`. This allows pattern matching and dynamic device ID embedding.

3. **Type Selection**:
   - Scaled signals (factor ≠ 1.0 or offset ≠ 0): `f64`
   - Unscaled signed: `i64`
   - Unscaled unsigned: `u64`

### Multiplexed Messages

For messages with mode-switching (like UDC_Command), generates:

```rust
// Payload enum with variants for each mode
pub enum UDC_CommandPayload {
    Convert {
        udc_convertcmd_convdir: u64,
        udc_convertcmd_prevstate: u64,
        // ... mode-specific fields
    },
    Safe {
        udc_reserved_2a: u64,
        udc_reserved_2b: u64,
    },
    NedReset,
    Shutdown,
}

// Message struct with multiplexor + payload
pub struct UDC_Command {
    pub device_id: DeviceId,
    pub udc_command_opcode: u64,  // The multiplexor field
    pub payload: UDC_CommandPayload,
}
```

---

## File Locations

| Path | Description | In Git? |
|------|-------------|---------|
| `dbc/*.dbc` | Source DBC files (or `$CANDO_DBC_PATH/*.dbc`) | No (private repo) |
| `dbc/.checksums.json` | SHA-256 checksums (DBC + generator + output) | Yes |
| `cando-messages/src/generated/*.rs` | Generated protocol code | Yes |
| `cando-codegen/src/` | Generator source code | Yes |
| `cando-codegen/src/generator.rs` | Main generation logic | Yes |
| `cando-codegen/src/encoder.rs` | Bit packing/unpacking utilities | Yes |
| `cando-codegen/src/field_name_converter.rs` | DBC name → snake_case | Yes |

---

## 🔍 Troubleshooting

### "DBC file not found"

**For users**: This is normal! You don't need DBC files. Generated code is already in the repo.

**For maintainers**: Clone the private DBC repository or set the path:
```bash
make dbc-init                  # clone into ./dbc
# OR
export CANDO_DBC_PATH=/path/to/your/dbc/files
cargo run -p cando-codegen -- generate --protocol j1939
```

### "Generated code out of sync"

```bash
cargo run -p cando-codegen -- generate-all
```

This regenerates any protocol where:
- DBC file changed
- Generator source changed
- Output differs from expected

### "No checksum recorded"

First time setup - run generation:
```bash
cargo run -p cando-codegen -- generate-all
```

This creates `.checksums.json` with current hashes.

### "Algorithm evolution detected"

This is normal when the generator source code changes. It means:
- Generator logic was modified
- All protocols should be regenerated for consistency

Action:
```bash
cargo run -p cando-codegen -- generate-all --force
cargo test --workspace  # Verify everything still works
```

### Build is slow

**First build**: 13-15s is normal (compiling 21 MB of generated code).

**Subsequent builds**: Should be <1s if nothing changed.

If every build is slow:
- Check for uncommitted changes in `src/generated/`
- Run `cargo clean` and rebuild
- Ensure all protocols were regenerated after generator changes

### Clippy warnings in generated code

If you see warnings like "unnecessary cast" in generated code:
1. Fix the generator source (e.g., `cando-codegen/src/generator.rs`)
2. Regenerate ALL protocols: `cargo run -p cando-codegen -- generate-all --force`
3. The generator hash will update and prevent false "up to date" status

### Field values are wrong

If encode/decode works but values are incorrect:
1. Check struct field order matches bit positions (use `--verbose` during generation)
2. Verify DBC signal definitions (bit positions, byte order)
3. Check for little-endian vs big-endian issues
4. Review the BASE_CAN_ID masking (device byte should be 0x00)

---

## 🎯 Best Practices

### DO ✅

- Commit generated Rust code
- Commit `.checksums.json`
- Review generated code changes before committing
- Test after regeneration
- Regenerate ALL protocols when generator changes
- Use `--force` when testing generator changes
- Keep field names in snake_case (automatic via field_name_converter)

### DON'T ❌

- Commit DBC files (licensing!)
- Edit generated code manually (it'll be overwritten)
- Assume `cargo build` regenerates code (use `cando-codegen`)
- Share DBC files publicly
- Regenerate only some protocols after generator changes (maintain consistency)
- Ignore "algorithm evolution" warnings

---

## 🔧 For Generator Developers

### Generator Source Files Tracked

The system automatically hashes these files:
```
cando-codegen/src/generator.rs
cando-codegen/src/encoder.rs  
cando-codegen/src/field_name_converter.rs
cando-codegen/src/main.rs
```

When ANY of these change, all protocols are marked for regeneration.

### Making Generator Changes

1. **Make your changes** to generator source
2. **Test on small protocol** first:
   ```bash
   cargo run -p cando-codegen -- generate --protocol emp --force
   cargo test -p cando-messages
   ```
3. **Review generated diff** carefully
4. **Regenerate all** when satisfied:
   ```bash
   cargo run -p cando-codegen -- generate-all --force
   ```
5. **Test everything**:
   ```bash
   cargo test --workspace
   cargo clippy --workspace
   ```

### Common Generator Fixes

**Unnecessary type casts**:
```rust
// BAD: Casts u64 field to u64 (redundant)
pack_signal(&mut data, start, size, self.field as u64)?;

// GOOD: Only cast i64 to u64
if is_signed {
    pack_signal(&mut data, start, size, self.field as u64)?;
} else {
    pack_signal(&mut data, start, size, self.field)?;
}
```

**Field ordering**:
```rust
// CRITICAL: Sort by bit position, not alphabetically
let mut signals: Vec<&Signal> = message.signals().collect();
signals.sort_by_key(|s| s.start_bit);
```

**Device ID masking**:
```rust
// BASE_CAN_ID must have device byte cleared
const BASE_ID_MASK: u32 = 0x1FFFFF00;  // 29-bit + clear device byte
let base_id = message.id().raw() & BASE_ID_MASK;
```

**Source address extraction and propagation** (Session 86):
```rust
// Extract source address from raw CAN ID (bits 7-0)
let raw_id = message.id().raw();
let source_addr = (raw_id & 0xFF) as u8;

// Pass source_addr through generation pipeline:
// 1. For non-multiplexed messages:
let impl_item = generate_encode_decode_impl(&msg_name, &signals, base_id, dlc, source_addr);

// 2. For multiplexed messages:
let impl_item = generate_multiplexed_encode_decode_impl(
    &msg_name, &payload_enum_name, mux_field, &mux_field_name,
    &mux_values, base_id, dlc, source_addr  // Added parameter
);

// 3. Generate SOURCE_ADDR constant in impl block:
impl MessageName {
    pub const BASE_CAN_ID: u32 = 0x18EF0000;
    pub const DLC: u8 = 8;
    pub const SOURCE_ADDR: u8 = 128;  // NEW - extracted from DBC
}

// 4. Update embed_device_id call to pass source address:
let can_id = embed_device_id(Self::BASE_CAN_ID, self.device_id, Some(Self::SOURCE_ADDR));
// Changed from: embed_device_id(Self::BASE_CAN_ID, self.device_id, None)
```

**Why this matters**: J1939 PDU1 messages require both destination address (DA) and source address (SA) to be embedded in the CAN ID. The SA must be extracted from the DBC file's raw CAN ID and propagated through the generated code to ensure proper encoding.

**Example**: HVPC_Command has raw CAN ID 0x18EF8D80 (DA=0x8D, SA=0x80). Without this fix, generated code used default SA=0x0F, producing incorrect CAN ID 0x18EF8D0F instead of 0x18EF8D80.

**J1939 PDU1 vs PDU2 message matching**:
```rust
// CRITICAL: Different masks for PDU1 vs PDU2 when comparing received messages
// PDU1 (PF < 0xF0): PS byte is Destination Address, not part of PGN
// PDU2 (PF >= 0xF0): PS byte is part of PGN

// For PDU1 messages (e.g., PGN 32000 = 0x7D00, PF=0x7D):
let frame_base_id = can_id & 0xFFFF0000;  // Mask out DA and SA
if frame_base_id == PDU1_BASE_CAN_ID { /* ... */ }

// For PDU2 messages (e.g., PGN 64513 = 0xFC01, PF=0xFC):
let frame_base_id = can_id & 0xFFFFFF00;  // Mask out SA only
if frame_base_id == PDU2_BASE_CAN_ID { /* ... */ }

// Example: PGN 32000 (PDU1) command message
// BASE_CAN_ID = 0x187D0000
// Received CAN ID = 0x187D820F (DA=0x82, SA=0x0F)
// With 0xFFFF0000 mask: 0x187D820F & 0xFFFF0000 = 0x187D0000 ✅ MATCH
// With 0xFFFFFF00 mask: 0x187D820F & 0xFFFFFF00 = 0x187D8200 ❌ NO MATCH
```

### Testing Changes

**Unit tests**: Add to `cando-codegen/src/generator.rs`:
```rust
#[test]
fn test_my_generator_change() {
    // Test your specific change
}
```

**Integration tests**: Use actual protocols:
```bash
# Generate and compare
cargo run -p cando-codegen -- generate --protocol emp --force
git diff cando-messages/src/generated/emp.rs

# Run protocol tests
cargo test -p cando-messages
```

**Full validation**:
```bash
cargo test --workspace
make tier1  # If integration tests available
```

---

## 🚨 Emergency: Regenerate Everything

If something's wrong, regenerate from scratch:

```bash
# With DBC files present
cargo run -p cando-codegen -- generate-all --force

# Verify checksums updated
git diff dbc/.checksums.json

# Test everything
cargo test --workspace

# If tests fail:
# 1. Check if you edited DBC files incorrectly
# 2. Review generator changes
# 3. Restore from backup: git checkout HEAD -- dbc/.checksums.json cando-messages/src/generated/
```

---

## 📊 Project Statistics

### Generated Code Sizes

| Protocol | Messages | Generated Size | Notes |
|----------|----------|----------------|-------|
| emp_j1939 | 7 | ~15 KB | EMP J1939 (EAMC_EMP, EAMS1, EAMS2, DM01, DM11, DM13, DM19) |
| hvpc | 6 | ~165 KB | High voltage power |
| udc | 2 (complex) | ~43 KB | DC-DC converter |
| j1939 | 2,146 | ~19.5 MB | SAE J1939 vehicle bus |
| j1939-73 | ~100 | ~1.7 MB | J1939 diagnostics |

**Total**: ~21.4 MB across 2,259+ messages

### Build Times

- **First build**: 13-15s (compiling all generated code)
- **Incremental**: <1s (only changed crates)
- **After full regeneration**: 10-12s (all protocols recompiled)

---

## 📚 Historical Context

### Major Changes

**2026-02-04** (Session 86): Fixed source address handling for J1939 PDU1 messages
- Extract SA from DBC raw CAN ID (bits 7-0)
- Propagate SA through generation pipeline to encode/decode functions
- Generate SOURCE_ADDR constant in message impl blocks
- Pass Some(SOURCE_ADDR) to embed_device_id() instead of None
- Fixed HVPC encoder producing wrong CAN IDs (0x18EF8D0F → 0x18EF8D80)
- All 4 GE tool commands now encode perfectly

**2025-11-16**: Added generator source hash tracking
- System now detects when generator code changes
- Prevents "silent algorithm drift"
- Automatically marks protocols for regeneration

**2024-12**: Fixed BASE_CAN_ID device byte masking
- Device ID byte now properly cleared to 0x00
- Enables proper pattern matching in simulators
- Fixed 100% J1939 integration test failures

**2024-11**: Implemented bit-position field ordering
- Struct fields now match CAN bit layout
- Fixed field value mismatches in decode
- Improved decode performance (direct struct unpacking)

**2024-11**: Moved from build.rs to standalone tool
- Separated code generation from build process
- Added checksum tracking system
- Enabled "C compiler model" workflow

### Lessons Learned

1. **Multi-layer change detection needed**: Track DBC files, generator sources, and outputs
2. **Bit positions matter**: Field order affects decode correctness
3. **Device byte masking critical**: BASE_CAN_ID must be maskable for pattern matching
4. **Generator changes affect everything**: All protocols must maintain consistency
5. **Deterministic generation essential**: Same inputs must produce same outputs

---

## ⚙️ Optional: Build-Time Validation

Enable checksum validation during builds (maintainers only):

```bash
CANDO_VALIDATE_GENERATED=1 cargo build
```

Warns if generated code is out of sync with DBC files.

---

## 🔗 Related Documentation

- **Main README**: Project overview and getting started
- **cando-messages/README.md**: Generated code usage
- **doc/license-summary.md**: Licensing model (DBC proprietary, generated code MIT/Apache-2.0)

---

## ❓ FAQ

**Q: Do I need DBC files to use this project?**  
A: No! Generated code is in the repo. Only maintainers with licensed DBCs need them.

**Q: Why are DBC files not in git?**  
A: They're proprietary/licensed. Generated code is derivative work (legal to distribute).

**Q: How do I know if I need to regenerate?**  
A: Run `cargo run -p cando-codegen -- status`. It will tell you.

**Q: Can I edit generated code?**  
A: Don't. Changes will be lost on next regeneration. Modify the generator instead.

**Q: What if I change the generator?**  
A: Regenerate ALL protocols with `generate-all --force` to maintain consistency.

**Q: Why does the generator track its own source hash?**  
A: To detect algorithm changes and prevent "silent drift" where generated code becomes inconsistent.

**Q: How do I test generator changes before committing?**  
A: Test on small protocol (emp), review diff, then regenerate all, run full tests.

---

**Questions?** Open an issue or check the main README.md

**Last Updated**: 2026-03-26 (CANDO_DBC_PATH env var, private DBC repo workflow)