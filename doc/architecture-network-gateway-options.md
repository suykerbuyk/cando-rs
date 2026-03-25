# Network Technology Options for Control System Gateways in Cando-RS

## Purpose

This document surveys implementation and technology options for building a TSN-capable network gateway in the Cando-RS project. The gateway will aggregate CAN bus messages and enable control/monitoring from remote clients over Ethernet, connecting "Brain Board" devices running Cando-RS binaries. It draws from prior discussions to recommend a practical, incremental path forward.

## Background

Industrial control systems require reliable, low-latency communication for command, control, and monitoring. Protocols often separate **physical transports** (e.g., twisted-pair serial, Ethernet) from **logical/application layers** (e.g., data structures like CANopen messages). A common pattern is embedding one protocol's data (e.g., CANopen) into another's transport (e.g., Ethernet TCP/IP).

Cando-RS currently handles CAN bus analysis and simulation (e.g., EMP/HVPC protocols via DBC files). Adding Ethernet connectivity means tunneling CANopen services over IP while supporting TSN for time-sensitive networking. Options range from simple TCP/IP bridges to real-time EtherCAT, balancing ease, performance, and future-proofing.

## Executive Summary and Recommended Approach

The ultimate goal is **CoE (CANopen over EtherCAT)** for native TSN support, real-time determinism, and true publisher/subscriber (pub/sub) models. However, CoE's complexity (e.g., real-time kernel patches) exceeds our current timeline and expertise.

- **CiA 309-3 as CAN bus transport**: a lightweight, user-space (no kernel mods), and directly maps CANopen services (SDO, PDO, NMT) over TCP/IP. It handles non-real-time tasks like configuration and monitoring without disrupting local CAN PDOs.
- **Adding CiA 309-5 WebSockets for pub/sub emulation**: 309-5 builds on 309-3 with REST/JSON for structured data and WebSockets for bidirectional, event-driven pub/sub (e.g., streaming PDO events or EMCY). This emulates pub/sub via gateways forwarding CAN events, supporting soft real-time for monitoring while keeping hard real-time on CAN.
- **Overall**: This phased rollout minimizes risk: Phase 1 delivers quick Ethernet integration; Phase 2 upgrades to CoE for TSN/production. It leverages Cando-RS's shared infrastructure (e.g., `cando-core` for parsing) and IDE workflows (e.g., async `tokio` for WebSockets).

## Phased Plan

### Phase 1: TCP/IP Foundation (309-3/5)

- Implement 309-3 ASCII-over-TCP for core CANopen tunneling (optional UDP for lighter payloads).
- Add 309-5 REST/WebSocket support for modern APIs and pub/sub emulation (e.g., JSON events over WebSockets).
- Migrate from ASCII to structured formats (JSON or Protocol Buffers) for efficiency.
- Use WebSockets to forward CAN events (e.g., PDO streams) as pub/sub, enabling remote clients to subscribe without polling.
- **Effort**: 2-4 weeks; user-space only, integrates with `monitor_can` and simulators via `tokio`.

### Phase 2: Real-Time Upgrade (CoE)

- Migrate time-critical paths (e.g., cyclic PDOs) to CoE over EtherCAT for native pub/sub via Sync Managers and TSN compatibility.
- Use gateways for hybrid CAN/EtherCAT bridging.
- **Effort**: 4-6 weeks; requires RT kernel (e.g., PREEMPT-RT on Debian), but builds on Phase 1 configs.

## Simplified Protocol Overviews

### Protocols with Defined Transports

- **Modbus**: Legacy client/server protocol (1970s) for PLC synchronization. Fixed offsets for I/O states; vendor-specific data. Obsolete for cross-vendor use; limited to serial (RS-422/485) or proprietary Ethernet. **Not recommended**—lacks flexibility for Cando-RS.
- **CAN Bus**: Robust multi-master serial protocol (Bosch, ISO-11898). Up to 8-byte payloads (64 on CAN-FD), 1 Mbps, 11/29-bit IDs with CRC. Royalty-free for 1.0; licenses needed for 2.0+ hardware (covered in transceivers). Basis for higher layers like CANopen. **Core for Cando-RS**—already implemented.
- **EtherCAT**: Real-time Industrial Ethernet (IEEE 802.3 base). "On-the-fly" processing in daisy-chain topology for <1 µs jitter, 100 Mbps. Uses EtherType 0x88A4; supports IEEE 802.1Q VLANs for segmentation. TSN-ready via prioritization. **High potential** but kernel-heavy on Linux.

### Application Layers on Other Transports

- **OpenCAN**: Proprietary (Maxxom) extension of CAN for motor control. Promises interoperability but not open/free. **Skip**—doesn't fit open-source Cando-RS.
- **CANopen**: Royalty-free CAN extension (CiA 301). Uses Object Dictionary for params; PDOs for cyclic real-time data (predictable, high-priority); SDOs for acyclic tasks (e.g., diagnostics). **Foundation**—map to Ethernet via 309-3/5 or CoE.

### Ethernet-Native CANopen Transports

- **CiA 309-3/5**: Tunnels CANopen over TCP/IP. 309-3: ASCII commands for SDO/PDO/NMT. 309-5: REST/JSON + WebSockets for pub/sub. Simple, web-friendly; soft real-time via event forwarding. **Phase 1 choice**—quick integration.
- **CoE**: Maps CANopen to EtherCAT. Reuses 80-90% CANopen code; large PDOs, <100 µs cycles via Sync Managers. TSN-native. **Phase 2 target**—for production determinism.

## Comparison: CoE vs. CiA 309-3/5

| Aspect              | CoE (EtherCAT) Strengths                     | CoE Weaknesses              | CiA 309-3/5 (TCP/IP) Strengths                 | CiA 309-3/5 Weaknesses              |
| ------------------- | -------------------------------------------- | --------------------------- | ---------------------------------------------- | ----------------------------------- |
| **Performance**     | <1 µs jitter; 100 Mbps; deterministic cycles | Kernel patches needed       | <100 ms latency; easy setup                    | Non-deterministic; polling overhead |
| **Scalability**     | 65k nodes; daisy-chain                       | Fixed topology              | Flexible Ethernet; remote access               | TCP bloat in high-volume data       |
| **Complexity/Cost** | Mature tools; TSN future-proof               | RT Linux expertise required | User-space; low-cost hardware                  | Limited real-time; needs add-ons    |
| **Pub/Sub**         | Native via PDOs/Sync Managers                | Less ad-hoc                 | Emulated via WebSockets/events                 | Soft real-time only                 |
| **Cando-RS Fit**  | Ultimate for TSN/control                     | Steeper Phase 2 curve       | Quick Phase 1 wins; builds on existing parsers | May need rewrite for hard real-time |

## Technical Details (Reference)

This section expands on key technical aspects for implementation reference, drawing from CiA specifications, EtherCAT standards, and Cando-RS integration notes. It includes detailed mappings, syntax, and deployment considerations to support Phase 1 (CiA 309-3/5) and Phase 2 (CoE).

### EtherCAT on Debian Linux

Implementing EtherCAT requires real-time capabilities not native to standard Linux kernels. Use the open-source IgH EtherCAT Master (EtherLab) stack:

- **Kernel Patching**: Apply PREEMPT-RT or Xenomai patches (from `linux-realtime` repo). Build process: `apt install build-essential bc bison flex libssl-dev` (Debian dependencies); `make menuconfig` to enable RT options; compile with `make -j$(nproc)`. Add boot params like `isolcpus=1-3` for CPU isolation to reduce jitter. Expect 1-2 days for setup; test latency with `cyclictest`.
- **Driver Integration**: Install EtherLab (`git clone git://git.etherlab.org/etherlabmaster.git`); build with `./configure --enable-generic`; `make install`. Load modules: `modprobe ec_generic`; bind to NIC via `ethercat master --device main`. Use Intel I210 or compatible NICs for low-latency.
- **TSN Integration**: Leverage IEEE 802.1Q for prioritization (e.g., PCP bits in VLAN tags). Route EtherCAT traffic (EtherType 0x88A4) via TSN switches; configure via `tc` tool for CBS (Credit-Based Shaper) or TAS (Time-Aware Shaper).
- **Effort and Tools**: 1-2 weeks for a basic master; optimize with cgroups for IRQ affinity. Alternatives: acontis EC-Master (pre-built Debian packages). Vs. 309-3/5: No patches needed—just standard sockets.
- **Cando-RS Notes**: Add a new crate (`ethercat_gateway`) depending on SOEM (Simple Open EtherCAT Master). Integrate with `cando-core/src/parser.rs` for CANopen mapping; simulate in Zed with `cargo run --features manpages` for docs.

### CiA 309-3 Command Syntax and Mapping

CiA 309-3 uses ASCII strings over TCP (persistent socket, e.g., port 1790) for gateway-mediated CANopen access. Commands: `[seq] [net] [node] <token> [args] \r\n`. Responses: `[seq] OK` or `[seq] <error>` (codes 100-600, e.g., 101 syntax, 103 timeout). Events (unsolicited): No seq, e.g., `EMCY 0x2310 0xFF ...`.

- **General Principles**: Non-case-sensitive; whitespace-separated tokens; hex prefixed with 0x; data types (u8, i8, u16, i16, u32, i32, etc.). Network ID ≥1 (multi-CAN support); node 1-127 (0 broadcast). Segmented transfers for large data (block mode, 500ms timeout).
- **SDO Mapping**:
  - Read: `[net] [node] read <index> <sub> <type>` (e.g., `[1] 4 read 0x1017 0 i16` → `[1] OK\n[1] 100` for heartbeat time).
  - Write: `[net] [node] write <index> <sub> <type> <value>`.
  - Multiplexer: Hex index:sub; supports expedited/segmented.
- **PDO Mapping**:
  - Config RPDO: `set rpdo <nr> <cob_id> <tx_type> <nr_data> <map1> ...` (e.g., `set rpdo 1 0x200 sync0 1 0x6040 0` for Controlword).
  - Config TPDO: `set tpdo <nr> <cob_id> <tx_type> <nr_data> <map1> ...` or extended `set tpdox` with inhibit/event timers (ms).
  - Read/Write: `read pdo <nr>`; `write pdo <nr> <nr_data> <values> ...` (triggers CAN frame).
  - Types: "event" (on change), "sync<N>" (cyclic), "event <inhibit> <event_time>".
  - Mapping: Multiplexer + bit length (e.g., 0x6000:0x01); up to 64 entries; dummies 0x0001-0x001F.
- **NMT Mapping**:
  - Commands: `start`, `stop`, `preoperational`, `reset node`, `reset communication`.
  - Monitoring: `enable heartbeat <time>`, `enable guarding <time> <factor>` (ms, 1-255); disable with 0. Violations trigger `ERROR 203`.
- **Event-Triggered Pub/Sub**: Unsolicited: `pdo <nr> <data>`, `EMCY <code> <reg> <data>`, `ERROR <code>`, `SYNC`, `USER`. Subscribe via TCP connect + enable commands.
- **Packet Encapsulation**: TCP payload = ASCII string; no extra headers. FIFO processing (~100ms inhibit).
- **Cando-RS Notes**: New crate `cia309_gateway`; use `tokio` for async TCP in `main.rs`. Parse with `cando-core/src/parser.rs`; output via `output.rs` (JSON for 309-5). Test in Zed: `cargo test --workspace`; man pages via `quick-gen-man.sh`.

### CiA 309-5 Extensions

Builds on 309-3 with REST/HTTP and WebSockets:

- **REST**: Acyclic ops, e.g., GET `/sdo/read?index=0x1017&sub=0&type=i16`; POST `/pdo/tpdo/1` with JSON mappings.
- **WebSockets**: Bidirectional pub/sub (ws://); subscribe via `/ws/canopen?net=1&node=2&events=emcy,pdo`. Publish JSON events, e.g., `{ "event": "EMCY", "code": 0x2310, "data": [123] }`. SSE fallback for HTTP.
- **PDO/Monitoring**: REST config triggers WebSocket streams; filter by event/type. JSON payloads (base64 binary).
- **Security**: WSS for encryption; token auth.
- **Cando-RS Notes**: Use `tokio-tungstenite` in `cia309_gateway`; integrate with `hvpc_simulator` for event publishing. Zed: Async snippets in `.zed/settings.json`; `cargo run` for WebSocket tests.

### CoE Mapping

CoE (EN 50325-4) maps CANopen (CiA 301) to EtherCAT datagrams in IEEE 802.3 frames (EtherType 0x88A4). Datagram header: 10 bytes (Cmd, Addr, Len, WC). Acyclic (mailbox SM0/1); cyclic (process SM2/3).

- **OD**: 16-bit index, 8-bit sub; types (u32, i16); access RO/RW. Stored in EEPROM (SII); examples: 0x1000 (Device Type), 0x1018 (Identity). Extensions: 0x1C00 (SM Types), 0x1C12/0x1C13 (PDO Assignments).
- **SDO**: Expedited/segmented via mailbox (Cmd 0x03 APRW). Header: 8-10 bytes (0x02 upload, 0x03 download + index:sub + data). Aborts: 0x06090030.
- **PDO**: Rx (SM2, master-to-slave) via 0x1400-0x15FF (comm), 0x1600-0x17FF (mapping: ARRAY u32 [index(16b):sub(8b):len(8b)]). Tx (SM3) via 0x1800-0x19FF, 0x1A00-0x1BFF. No 8-byte limit; concatenated in process datagrams (Cmd 0x01 LRW). Types: 1-240 (cyclic), 255 (async).
- **NMT**: Cmds (0x01 Start) via mailbox; states via Controlword (0x6040 bits 0-3,7). Heartbeat: 0x1017 (ms).
- **Pub/Sub Mechanisms**:
  1. **Sync Managers**: SM2 (RxPDO), SM3 (TxPDO) as buffers; assign via 0x1C12/0x1C13.
  2. **Logical Addressing**: 4GB process image; offsets for pub/sub (e.g., sensor publishes to 0x1000).
  3. **Distributed Clocks (DC)**: IEEE 1588 sync ( < 1 µs) via 0x1C32/0x1C33 align to SYNC0/1.
  4. **Event/Cyclic**: Event-driven (type 255) with inhibit (0x1800 Sub-5); cyclic on DC events.
  5. **Dynamic Mapping**: Runtime via SDOs; flexible sizes (kilobytes).
  6. **EMCY**: Publish via 0x1014 (default 0x80 + Node-ID) to mailbox/TxPDO.
- **Hybrid Gateways**: Tunnel CAN frames (message-based) or map payloads (process-based); supports 65k nodes, 30 µs cycles.
- **Benefits**: 80-90% CANopen reuse; TSN via 802.1Q.
