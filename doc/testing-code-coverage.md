# Code Coverage Guidelines for Cando-RS

**Last Updated**: 2024-01-21 (Session 63)  
**Status**: Active

---

## Overview

This document defines code coverage expectations and practices for the Cando-RS project. We maintain different coverage standards for hand-written versus auto-generated code.

## Coverage Targets

### Hand-Written Code (Primary Focus)

**Target: 80%+ line coverage**

Hand-written code in the following modules should maintain at least 80% test coverage:

- `cando-messages/src/common.rs` - Core types and utilities
- `cando-messages/src/metadata.rs` - Metadata structures
- `cando-messages/src/encoder.rs` - Encoding/decoding logic
- `cando-messages/src/lib.rs` - Library integration
- `cando-messages/src/j1939/` - J1939 helper modules (non-generated)
- `cando-codegen/src/` - Code generation logic
- `cando-core/src/` - Core utilities

**Rationale**: Hand-written code contains business logic, error handling, and edge cases that require thorough testing.

### Generated Code (Secondary Focus)

**Target: 20-40% line coverage**

Auto-generated protocol files have lower coverage expectations:

- `cando-messages/src/generated/j1939.rs` - 200+ messages
- `cando-messages/src/generated/j1939_73.rs` - 50+ diagnostic messages
- `cando-messages/src/generated/emp_j1939.rs` - 9 EMP messages
- `cando-messages/src/generated/hvpc.rs` - 4 HVPC messages
- `cando-messages/src/generated/udc.rs` - 11 UDC messages

**Rationale**: 
1. **Uniform Implementation**: All generated code follows identical patterns
2. **Generator Testing**: The code generator itself is tested, ensuring correct output
3. **Sample Testing**: Representative samples validate the pattern works correctly
4. **ROI**: Testing every generated message provides diminishing returns

### Integration Tests

**Requirement**: Must validate generated code patterns work correctly

Integration tests should:
- Exercise the `DynamicCanMessage` trait implementations
- Test field access (get/set) for sample messages
- Validate encoding/decoding round-trips
- Test trait object usage (Box<dyn DynamicCanMessage>)
- Cover error cases (invalid fields, out-of-range values)

---

## Current Coverage Status

### Session 63 Results

| Module | Line Coverage | Function Coverage | Notes |
|--------|--------------|-------------------|-------|
| `common.rs` | **60.58%** ✅ | 66.67% | Improved from 32.82% |
| `metadata.rs` | **93.79%** ✅ | 92.00% | Excellent |
| `lib.rs` | **97.73%** ✅ | 93.33% | Excellent |
| `j1939/diagnostics/dm03_helpers.rs` | **98.54%** ✅ | 100.00% | Excellent |
| `j1939/diagnostics/command_helpers.rs` | **88.57%** ✅ | 93.55% | Good |
| `j1939/diagnostics/dtc_helpers.rs` | **87.57%** ✅ | 100.00% | Good |
| `encoder.rs` | 41.14% ⚠️ | 51.11% | Needs improvement |
| `generated/j1939.rs` | 1.10% 📊 | 0.44% | Generated (acceptable) |
| `generated/j1939_73.rs` | 7.77% 📊 | 4.57% | Generated (acceptable) |
| `generated/emp_j1939.rs` | 18.28% 📊 | 26.53% | Generated (acceptable) |

**Overall Package**: 2.16% line coverage (heavily skewed by large generated files)

---

## Running Coverage Analysis

### Install Coverage Tool

```bash
cargo install cargo-llvm-cov
```

### Run Coverage for cando-messages

```bash
# Basic coverage report (terminal output)
cargo llvm-cov --package cando-messages --all-targets

# Generate HTML report
cargo llvm-cov --package cando-messages --all-targets --html
open target/llvm-cov/html/index.html

# Generate LCOV for CI integration
cargo llvm-cov --package cando-messages --lcov --output-path coverage.lcov
```

### Run Coverage for Specific Module

```bash
# Test specific test file
cargo test -p cando-messages --test common_static_coverage

# With coverage
cargo llvm-cov --package cando-messages --test common_static_coverage
```

---

## Testing Strategy

### 1. Comprehensive Static Type Testing

**Goal**: Achieve 80%+ coverage on hand-written types

Test files:
- `tests/common_static_coverage.rs` - Tests for DeviceId, Percentage, MotorSpeed, normalize_field_name

Coverage includes:
- ✅ Valid value construction
- ✅ Boundary value testing
- ✅ Out-of-range validation
- ✅ Type conversion (From/Into traits)
- ✅ Equality and ordering
- ✅ Clone and Copy semantics
- ✅ Debug formatting
- ✅ Hash trait (for HashMap keys)
- ✅ Error message clarity

### 2. Pattern Validation for Generated Code

**Goal**: Validate generated patterns work correctly with sample messages

Test files:
- `tests/dynamic_can_message_integration.rs` - DynamicCanMessage trait testing

Coverage includes:
- ✅ Trait object creation and usage
- ✅ Field access by name (get/set)
- ✅ Case-insensitive field matching
- ✅ Field aliasing (user-friendly names)
- ✅ Encoding/decoding through trait
- ✅ Metadata access
- ✅ Device ID operations
- ✅ Clone via trait object
- ✅ Downcasting (as_any)

### 3. Protocol-Specific Integration Tests

**Goal**: Validate actual protocol behavior

Test files:
- `tests/j1939_command_helpers.rs` - J1939 diagnostic commands
- `tests/j1939_dtc_helpers.rs` - J1939 DTC parsing
- `tests/dm03_comprehensive.rs` - DM03 maintenance workflows

---

## What NOT to Test

### ❌ Every Generated Message

**Don't**: Create tests for all 200+ J1939 messages individually

**Why**: 
- Provides diminishing returns
- Generated code follows uniform patterns
- If the pattern works for 5 messages, it works for 200
- Wastes development time on redundant tests

**Instead**: Test a representative sample from each category (engine, transmission, diagnostic)

### ❌ Generated Boilerplate Code

**Don't**: Test auto-generated struct definitions, field accessors, or metadata constants

**Why**:
- No business logic to validate
- Generator correctness is tested separately
- Rust compiler ensures type safety

**Instead**: Focus on the code generator tests in `cando-codegen`

### ❌ Third-Party Library Code

**Don't**: Test dependencies like `thiserror`, `serde`, `pest`

**Why**: These are thoroughly tested by their maintainers

---

## Adding New Tests

When adding new hand-written code:

1. **Write tests first** (TDD approach recommended)
2. **Aim for 80%+ coverage** of new code
3. **Test edge cases** (null, zero, max, invalid inputs)
4. **Test error paths** (validation failures, decode errors)
5. **Run coverage** to verify targets met

Example workflow:

```bash
# Add new function to common.rs
vim cando-messages/src/common.rs

# Add tests
vim cando-messages/tests/common_static_coverage.rs

# Run tests
cargo test -p cando-messages --test common_static_coverage

# Check coverage
cargo llvm-cov --package cando-messages --all-targets | grep common.rs
```

---

## CI Integration

### GitHub Actions Example

```yaml
- name: Run tests with coverage
  run: |
    cargo llvm-cov --package cando-messages --all-targets --lcov --output-path coverage.lcov
    
- name: Upload coverage to Codecov
  uses: codecov/codecov-action@v3
  with:
    files: coverage.lcov
    fail_ci_if_error: true
```

### Coverage Gates

- **Block PR if**: Hand-written code coverage drops below 75%
- **Allow PR if**: Generated code coverage is low (20-40% acceptable)
- **Manual review**: If overall package coverage drops significantly

---

## Rationale: Why Different Standards?

### Generated Code is Different

1. **No business logic**: Just struct definitions and field accessors
2. **Uniform patterns**: All messages follow identical implementation
3. **Compiler-verified**: Rust's type system ensures correctness
4. **Tested by proxy**: Generator tests validate output correctness

### Example: J1939 Messages

If `EEC1` (Engine Controller 1) implements `DynamicCanMessage` correctly:
- And `CCVS` (Cruise Control) implements it correctly
- And `LFE` (Fuel Economy) implements it correctly
- **Then**: All 200+ messages implement it correctly (same generator, same pattern)

### Testing the Generator is Key

Instead of testing every generated message, we:
1. ✅ Test the code generator thoroughly (`cando-codegen/src/generator.rs`)
2. ✅ Test representative samples from each protocol
3. ✅ Validate the pattern works end-to-end
4. ✅ Trust the Rust compiler to catch issues

---

## References

- **Session 62**: Initial DynamicCanMessage trait implementation
- **Session 63**: Coverage analysis and static type testing
- **MESSAGE-REGISTRY-IMPLEMENTATION-PLAN.md**: Phase 1-4 roadmap
- **AI-WORKFLOW-GUIDE.md**: Quality standards and testing requirements

---

## Questions?

If you're unsure whether code needs tests:

1. **Is it hand-written?** → Yes, write comprehensive tests (80%+ target)
2. **Is it generated?** → No, rely on generator tests + samples
3. **Is it critical error handling?** → Yes, test all paths
4. **Is it a simple getter?** → No explicit test needed

**When in doubt, err on the side of more testing for hand-written code.**