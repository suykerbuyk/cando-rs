# Binary Name Mapping: proteus-rs → cando-rs

This document maps every proteus-rs binary to its cando-rs equivalent.
Use this as the authoritative reference when porting code, updating
documentation, or adapting scripts that reference binary names.

## Retained Binaries

| proteus-rs | cando-rs | Crate | Purpose |
|------------|----------|-------|---------|
| `rust-can-util` | `cando-util` | `cando-util` | General-purpose CAN utility |
| `candump-rs` | `cando-dump` | `cando-dump` | Raw CAN frame dumper |
| `cansend-rs` | `cando-send` | `cando-send` | CAN frame sender |
| `monitor-can` | `cando-monitor` | `cando-monitor` | Live CAN bus monitor |
| `dump-messages` | `cando-dump-messages` | `cando-dump-messages` | Decoded message dumper |
| `proteus-cfg` | `cando-cfg` | `cando-cfg` | Configuration management |
| `proteus-codegen` | `cando-codegen` | `cando-codegen` | DBC-to-Rust code generator |
| `j1939-simulator` | `cando-j1939-sim` | `cando-j1939-sim` | J1939 protocol simulator |
| `log-analyzer` | `cando-log-analyzer` | `cando-log-analyzer` | CAN log analysis |
| `ws-query` | `cando-ws-query` | `cando-ws-query` | WebSocket query tool |

## Removed Binaries (Proprietary)

| proteus-rs | Reason for removal |
|------------|-------------------|
| `emp-simulator` | EMP protocol — proprietary |
| `hvpc-simulator` | HVPC protocol — proprietary |
| `udc-simulator` | UDC protocol — proprietary |
| `proteus-webui` | WebUI — primarily EMP/HVPC visualization |
| `count-hvpc-signals` | HVPC DBC signal counter — proprietary |
| `can-gateway` | CAN routing — tightly coupled to EMP/HVPC |
| `playwright-test` | WebUI test harness — depends on proteus-webui |

## String Substitution Patterns

When porting code or documentation, apply these substitutions:

| Pattern | Replacement | Context |
|---------|-------------|---------|
| `proteus-rs` | `cando-rs` | Package names, repo references |
| `proteus_rs` | `cando_rs` | Rust module paths, identifiers |
| `Proteus-RS` | `Cando-RS` | Display names, titles |
| `Proteus` (standalone) | `Cando` | Prose references |
| `proteus-meta` | `cando-meta` | Debian meta-package |
| `proteus-cfg` | `cando-cfg` | Config tool binary |
| `proteus-codegen` | `cando-codegen` | Codegen binary |
| `proteus-webui` | *(removed)* | WebUI binary |
| `proteus.yaml` | `cando.yaml` | Config file name |
| `rust-can-util` | `cando-util` | CAN utility binary |
| `candump-rs` | `cando-dump` | Dump binary |
| `cansend-rs` | `cando-send` | Send binary |
| `monitor-can` | `cando-monitor` | Monitor binary |
| `dump-messages` | `cando-dump-messages` | Message dump binary |
| `j1939-simulator` | `cando-j1939-sim` | J1939 simulator binary |
| `log-analyzer` | `cando-log-analyzer` | Log analysis binary |
| `ws-query` | `cando-ws-query` | WebSocket query binary |
| `99-proteus-can.rules` | `99-cando-can.rules` | Udev rules file |
| `proteus-testing` | `cando-testing` | Sudoers file |

## Crate Name Mapping

| proteus-rs crate | cando-rs crate |
|-----------------|----------------|
| `proteus-meta` | `cando-meta` |
| `proteus-cfg` | `cando-cfg` |
| `proteus-codegen` | `cando-codegen` |
| `proteus-messages` | `cando-messages` |
| `proteus-simulator` | `cando-simulator` |
| `proteus-common` | `cando-common` |
| `proteus-can` | `cando-can` |
| `proteus-webui` | *(removed)* |

## Notes

- The `cando-` prefix was chosen for consistency. All binaries share the
  prefix, making them discoverable via tab completion (`cando-<TAB>`).
- proteus-rs used inconsistent naming (`rust-can-util`, `candump-rs`,
  `cansend-rs`, `monitor-can`). cando-rs standardizes on `cando-<function>`.
- The `dump-messages` → `cando-dump-messages` rename adds the prefix but
  preserves the descriptive name, since this tool's identity is "dump
  decoded messages" as distinct from `cando-dump` (raw frames).
