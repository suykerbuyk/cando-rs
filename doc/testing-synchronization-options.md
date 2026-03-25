```markdown
# Test Process Synchronization: Options Analysis

**Created**: 2026-02-07 (Session 92)
**Status**: Reference Document - Option 1 Currently Implemented
**Purpose**: Document alternative approaches for future test infrastructure upgrades

---

## Executive Summary

Integration tests need to synchronize with spawned simulator processes. We currently use **Option 1 (Retry with Exponential Backoff)**, which works well for our current scale (~10 integration tests). This document analyzes 5 alternative approaches for when we need to scale up.

**Current Implementation**: ✅ Option 1 - Retry logic with exponential backoff
**When to Upgrade**: Test count exceeds 20-30, or startup diagnostics become critical
**Recommended Next Step**: Option 2 - Health check endpoints

---

## Table of Contents

- [The Problem](#the-problem)
- [Option 1: Retry with Exponential Backoff](#option-1-retry-with-exponential-backoff) ✅ IMPLEMENTED
- [Option 2: Health Check Endpoint](#option-2-health-check-endpoint)
- [Option 3: Process-to-Test IPC Channel](#option-3-process-to-test-ipc-channel)
- [Option 4: Structured Test Framework](#option-4-structured-test-framework)
- [Option 5: Container-Based Orchestration](#option-5-container-based-orchestration)
- [Recommendation Matrix](#recommendation-matrix)
- [Migration Path](#migration-path)

---

## The Problem

### Background

Integration tests spawn simulator processes (hvpc-simulator, emp-simulator) and need to know when they're ready to accept connections. The challenge is **asynchronous process initialization**:

1. Test spawns simulator process
2. Simulator starts, loads config, binds WebSocket, connects to CAN
3. Test needs to connect to WebSocket
4. **Problem**: No synchronization mechanism - how does test know when to connect?

### Failed Approach: Fixed Sleep

**Previous implementation**:
```rust
let simulator = Command::new("hvpc-simulator").spawn()?;
thread::sleep(Duration::from_secs(2));  // Hope it's ready!
let (ws, _) = connect("ws://localhost:10761")?;  // May fail
```

**Why it failed**:
- ❌ After `make clean`, cold start takes 3-4 seconds (not 2)
- ❌ Non-deterministic (works sometimes, fails others)
- ❌ No feedback mechanism (blind guessing)
- ❌ Wastes time (sleep even when ready early)
- ❌ Race condition: WebSocket might not be bound yet

### Requirements for a Solution

1. **Deterministic**: Must work reliably after cold start
2. **Fast**: Don't waste time if process starts quickly
3. **Clear errors**: Diagnostic messages on failure
4. **Maintainable**: Simple to understand and modify
5. **Minimal changes**: Prefer test-side solutions over simulator modifications

---

## Option 1: Retry with Exponential Backoff

✅ **CURRENTLY IMPLEMENTED** (Session 92)

### Approach

Poll WebSocket connection with increasing delays until success or timeout.

### Implementation

**In test fixture** (`hvpc-simulator/tests/integration_test.rs`, lines 99-128):

```rust
fn new() -> Result<Self> {
    // Spawn simulator (no sleep!)
    let simulator_process = Command::new("cargo")
        .args(&["run", "--bin", "hvpc-simulator", "--", ...])
        .spawn()?;

    // Connect with retry + exponential backoff
    let ws_url = format!("ws://127.0.0.1:{}", ws_port);
    let max_retries = 30;  // 30 attempts
    let mut retry_delay = Duration::from_millis(100);  // Start at 100ms
    
    for attempt in 1..=max_retries {
        match connect(&ws_url) {
            Ok((client, _)) => {
                if attempt > 1 {
                    println!("WebSocket connected on attempt {}", attempt);
                }
                return Ok(Self { ws_client: client, ... });
            }
            Err(e) => {
                if attempt == max_retries {
                    panic!("Failed after {} attempts: {:?}", max_retries, e);
                }
                thread::sleep(retry_delay);
                // Exponential backoff: 100ms → 200ms → 400ms → 800ms → 1.6s → 2s (cap)
                retry_delay = std::cmp::min(retry_delay * 2, Duration::from_secs(2));
            }
        }
    }
}
```

### Characteristics

**Retry schedule** (30 attempts, exponential backoff with 2s cap):
```
Attempt  1:   0.0s (immediate)
Attempt  2: + 0.1s (total: 0.1s)
Attempt  3: + 0.2s (total: 0.3s)
Attempt  4: + 0.4s (total: 0.7s)
Attempt  5: + 0.8s (total: 1.5s)
Attempt  6: + 1.6s (total: 3.1s)
Attempt  7: + 2.0s (total: 5.1s) ← cap reached
...
Attempt 30: + 2.0s (total: 53.1s)
```

**Typical behavior**:
- Cold start (after `make clean`): Connects on attempt 3-5 (~0.5-1.5s)
- Warm start: Connects on attempt 1-2 (immediate or ~0.1s)
- Maximum wait: ~53 seconds before giving up

### Pros

✅ **Simple to implement**: 30 lines of code per test fixture
✅ **No simulator changes**: Works with existing code
✅ **Adapts to startup time**: Fast when possible, patient when needed
✅ **Clear diagnostics**: Shows which attempt succeeded, detailed error on failure
✅ **Works reliably**: Tested after `make clean`, all tests pass
✅ **Low maintenance**: Self-contained in test fixtures
✅ **Good enough for 10-20 tests**: Current scale doesn't justify more complexity

### Cons

⚠️ **Still polling**: Wastes CPU cycles checking repeatedly
⚠️ **Arbitrary timeout**: 30 attempts × 2s = 60s max is a guess
⚠️ **No crash detection**: Can't tell if process died during startup
⚠️ **Duplicated logic**: Each test fixture implements same retry pattern
⚠️ **Doesn't scale well**: With 100 tests, wasted polling adds up
⚠️ **No subsystem status**: Can't tell which part of startup is slow (WebSocket? CAN? Config?)

### When to Use

✅ Small number of integration tests (10-30)
✅ Process startup is generally reliable
✅ Need quick solution without modifying simulator code
✅ Development team is small
✅ Tests run serially (not parallel)

---

## Option 2: Health Check Endpoint / Readiness Probe

### Approach

Simulators expose a `/health` or `/ready` HTTP endpoint that returns readiness status. Tests poll this endpoint instead of blindly retrying connections.

### Architecture

```
┌─────────────────┐     1. Spawn Process      ┌──────────────────┐
│  Test Fixture   │─────────────────────────→  │  Simulator       │
│                 │                            │                  │
│  2. Poll Health │                            │  3. Initialize   │
│  GET /health    │←──────────────────────────│  - Load config   │
│                 │   "NOT_READY" (503)        │  - Bind WebSocket│
│                 │                            │  - Connect CAN   │
│  4. Poll Health │                            │                  │
│  GET /health    │←──────────────────────────│  5. Mark Ready   │
│                 │   "READY" (200 OK)         │  ✅ All systems  │
│                 │                            │     operational  │
│  6. Connect WS  │─────────────────────────→  │                  │
│  ws://...       │   Success (guaranteed)     │  7. Handle Test  │
└─────────────────┘                            └──────────────────┘
```

### Implementation Example

**Simulator side** (`cando-simulator-common/src/health.rs`):

```rust
pub struct SimulatorHealth {
    config_loaded: Arc<AtomicBool>,
    websocket_ready: Arc<AtomicBool>,
    can_ready: Arc<AtomicBool>,
}

impl SimulatorHealth {
    pub fn new() -> Self { /* ... */ }
    pub fn set_config_loaded(&self) { /* ... */ }
    pub fn set_websocket_ready(&self) { /* ... */ }
    pub fn set_can_ready(&self) { /* ... */ }
    
    pub fn is_ready(&self) -> bool {
        self.config_loaded.load(Ordering::Relaxed)
            && self.websocket_ready.load(Ordering::Relaxed)
            && self.can_ready.load(Ordering::Relaxed)
    }
    
    pub fn health_endpoint(self: Arc<Self>) -> impl Filter { /* ... */ }
}
```

**Test side**:

```rust
fn new() -> Result<Self> {
    let simulator_process = spawn_simulator()?;
    
    // Wait for health check to report ready
    let health_url = "http://127.0.0.1:10762/health";
    let client = reqwest::blocking::Client::new();
    
    loop {
        match client.get(health_url).send() {
            Ok(resp) if resp.status().is_success() => {
                let status: HealthStatus = resp.json()?;
                println!("Simulator ready: {:?}", status);
                break;
            }
            _ => thread::sleep(Duration::from_millis(100)),
        }
    }
    
    // Now connect (guaranteed ready)
    let (ws_client, _) = connect(&ws_url)?;
    Ok(Self { simulator_process, ws_client, ... })
}
```

### Pros

✅ **Explicit readiness signal**: Not guessing, simulator tells us
✅ **Subsystem granularity**: Know which part is slow (config, WebSocket, CAN)
✅ **Better diagnostics**: Health status JSON shows what's pending
✅ **Standard pattern**: Used in Kubernetes (readiness probes), Docker (healthchecks)
✅ **Reusable**: All simulators use same health check pattern
✅ **Production-ready**: Health endpoint useful beyond testing

### Cons

⚠️ **Simulator modifications required**: Must change all 3 simulators
⚠️ **Additional port**: Health endpoint needs separate port
⚠️ **HTTP dependency**: Need HTTP client in tests (`reqwest` crate)
⚠️ **More complex**: Health state tracking in simulator code
⚠️ **Still polling**: Just polling a different endpoint

### Complexity

**Implementation Effort**: Medium (2-3 days)
**Maintenance**: Medium (health logic in each simulator)

### When to Use

✅ Growing test suite (20-50 tests)
✅ Complex startup sequences (multiple dependencies)
✅ Need detailed startup diagnostics
✅ Planning to add more simulators

---

## Option 3: Process-to-Test IPC Channel

### Approach

Simulator writes a "READY" signal to a file, pipe, or socket when initialization completes. Test watches for this signal.

### Architecture

```
┌─────────────────┐     1. Create Ready File   ┌──────────────────┐
│  Test Fixture   │     /tmp/sim-ready-XXXXX    │  Simulator       │
│                 │                              │                  │
│  2. Set Env Var │     TEST_READY_FILE         │  3. Read Env Var │
│  Spawn Process  │─────────────────────────→   │  Initialize      │
│                 │                              │  - Load config   │
│  4. Watch File  │                              │  - Bind WebSocket│
│  (inotify/poll) │                              │  - Connect CAN   │
│                 │                              │                  │
│                 │   5. Write "READY"           │  6. Signal Ready │
│  File Changed!  │←─────────────────────────────│  Write to file   │
│                 │                              │                  │
│  7. Connect WS  │─────────────────────────────→│  8. Handle Test  │
└─────────────────┘                              └──────────────────┘
```

### Implementation Example

**Simulator side**:

```rust
async fn signal_test_ready() -> Result<()> {
    if let Ok(ready_file) = env::var("TEST_READY_FILE") {
        fs::write(&ready_file, b"READY")?;
        println!("✓ Signaled ready to test via {}", ready_file);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize everything
    let config = load_config()?;
    let ws_server = start_websocket(port).await?;
    let can = CanSocket::open(interface)?;
    
    // Signal we're ready (if in test mode)
    signal_test_ready().await?;
    
    println!("✅ Simulator running");
    run_simulator(ws_server, can).await
}
```

**Test side** (with polling):

```rust
fn new() -> Result<Self> {
    let ready_file = NamedTempFile::new()?;
    let ready_path = ready_file.path().to_string_lossy().to_string();
    
    let simulator_process = Command::new("cargo")
        .env("TEST_READY_FILE", &ready_path)
        .spawn()?;
    
    // Poll for ready signal
    while !ready_file.path().exists() || 
          fs::read_to_string(ready_file.path())? != "READY" {
        thread::sleep(Duration::from_millis(50));
    }
    
    let (ws_client, _) = connect(&ws_url)?;
    Ok(Self { simulator_process, ws_client, ... })
}
```

### Pros

✅ **No network overhead**: File system (faster than HTTP)
✅ **Simple protocol**: Just write "READY" string
✅ **Zero polling with inotify**: On Linux, can use kernel events
✅ **Minimal dependencies**: Standard library mostly sufficient

### Cons

⚠️ **File system dependency**: Need to clean up temp files
⚠️ **Platform-specific optimizations**: inotify (Linux), kqueue (BSD)
⚠️ **Still polling without inotify**: Cross-platform solution polls
⚠️ **No subsystem granularity**: Just "ready" or "not ready"

### Complexity

**Implementation Effort**: Medium (1 day)
**Maintenance**: Low (simple protocol)

### When to Use

✅ Want zero network overhead
✅ Process startup is complex/long
✅ Don't want to add HTTP server to simulator
✅ Willing to use platform-specific optimizations (inotify)

---

## Option 4: Structured Test Framework (pytest-style)

### Approach

Create a dedicated test framework crate (`cando-test-harness`) that provides managed fixtures, automatic cleanup, and consistent test patterns.

### Architecture

```
cando-test-harness (new crate)
├── TestContext (manages lifecycle)
├── ManagedSimulator (auto cleanup)
└── Macros (integration_test!, etc.)
        ↓
    Test Files
    ├── hvpc_integration_test.rs
    ├── emp_integration_test.rs
    └── multi_simulator_test.rs
```

### Implementation Example

**Framework crate** (`cando-test-harness/src/lib.rs`):

```rust
pub struct TestContext {
    simulators: HashMap<String, ManagedSimulator>,
    cleanup_handlers: Vec<Box<dyn FnOnce() + Send>>,
}

impl TestContext {
    pub async fn spawn_hvpc(&mut self, device_name: &str) 
        -> Result<&mut ManagedSimulator> {
        let config = SimulatorConfig { /* ... */ };
        let simulator = ManagedSimulator::spawn(config).await?;
        self.simulators.insert(device_name.to_string(), simulator);
        Ok(self.simulators.get_mut(device_name).unwrap())
    }
}

pub struct ManagedSimulator {
    process: Option<Child>,
    websocket: Arc<Mutex<WebSocket>>,
    can_socket: CanSocket,
}

impl ManagedSimulator {
    pub async fn spawn(config: SimulatorConfig) -> Result<Self> {
        // Spawn, wait for health, connect
    }
    
    pub async fn send_command(&mut self, cmd: Value) -> Result<()> { /* ... */ }
    pub async fn get_state(&mut self) -> Result<Value> { /* ... */ }
}

impl Drop for ManagedSimulator {
    fn drop(&mut self) {
        // Graceful shutdown with timeout
    }
}

#[macro_export]
macro_rules! integration_test {
    ($name:ident, $body:expr) => {
        #[tokio::test]
        async fn $name() {
            let mut ctx = TestContext::new();
            let result = $body(&mut ctx).await;
            ctx.cleanup().await;
            result.unwrap();
        }
    };
}
```

**Test usage**:

```rust
use cando_test_harness::{TestContext, integration_test};

integration_test!(test_hvpc_valve_control, |ctx: &mut TestContext| async move {
    let hvpc = ctx.spawn_hvpc("hvpc_test_device").await?;
    
    hvpc.send_command(json!({"type": "SetValve", "position": 50})).await?;
    let state = hvpc.get_state().await?;
    assert_eq!(state["valve_position"], 50);
    
    Ok(())
    // Automatic cleanup!
});
```

### Pros

✅ **Consistent pattern**: All tests use same structure
✅ **Automatic cleanup**: Even on panic
✅ **Composable**: Easy to test multi-simulator scenarios
✅ **Parallel execution**: Can run independent tests in parallel
✅ **Reduces boilerplate**: 80% less code per test

### Cons

⚠️ **Significant upfront investment**: ~1000-2000 lines of code
⚠️ **Learning curve**: Team needs to learn framework API
⚠️ **Overkill for current scale**: Not justified for 10 tests
⚠️ **Framework maintenance**: New code to maintain

### Complexity

**Implementation Effort**: High (1-2 weeks)
**Maintenance**: Medium (framework bugs affect all tests)

### When to Use

✅ 50+ integration tests
✅ Multiple developers writing tests
✅ Need parallel test execution
✅ Long-term project with growing test suite

---

## Option 5: Container-Based Test Orchestration

### Approach

Use Docker/Docker Compose or `testcontainers-rs` to manage simulator lifecycles. Containers provide complete isolation and built-in health check mechanisms.

### Architecture

```
Docker Compose / Testcontainers
├── Container: hvpc-simulator (health check)
├── Container: emp-simulator (health check)
└── Container: test-runner (depends_on: healthy)
```

### Implementation Example

**Docker Compose** (`docker-compose.test.yml`):

```yaml
version: '3.8'
services:
  hvpc-simulator:
    build: .
    command: hvpc-simulator --config cando.yaml ...
    ports:
      - "10761:10761"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:10762/health"]
      interval: 1s
      timeout: 3s
      retries: 30
      
  test-runner:
    build: .
    command: cargo test --test hvpc_integration
    depends_on:
      hvpc-simulator:
        condition: service_healthy
```

**Testcontainers-rs**:

```rust
use testcontainers::*;

#[tokio::test]
async fn test_hvpc_with_container() {
    let docker = clients::Cli::default();
    
    let hvpc = docker.run(HvpcSimulator::default());
    // Health check waited for automatically
    
    let ws_port = hvpc.get_host_port_ipv4(10761);
    let (ws, _) = connect(&format!("ws://localhost:{}", ws_port))?;
    
    // Test...
    
    // Container auto-stops when `hvpc` drops
}
```

### Pros

✅ **Industry standard**: Docker/K8s patterns
✅ **Complete isolation**: Each test gets fresh environment
✅ **Reproducible**: Container image = consistent environment
✅ **CI/CD friendly**: Most CI systems support Docker
✅ **Port conflict resolution**: Docker assigns random ports

### Cons

⚠️ **Requires Docker**: Overhead on developer machines
⚠️ **Slower**: Container startup adds 5-10 seconds per test
⚠️ **More complex setup**: Dockerfiles, compose files
⚠️ **Can't test real CAN hardware**: Containers isolate from host

### Complexity

**Implementation Effort**: High (1-2 weeks)
**Maintenance**: Medium (Docker images to maintain)

### When to Use

✅ Deploying to production via containers
✅ Need complete environment isolation
✅ CI/CD pipeline uses Docker
✅ 100+ tests with parallelization needs

⚠️ **Not suitable for**: Hardware-in-the-loop tests (real CAN devices)

---

## Recommendation Matrix

| Current State | Test Count | Recommended Option | Rationale |
|--------------|------------|-------------------|-----------|
| **Now** | 10 tests | ✅ **Option 1** (Retry) | Already implemented, good enough |
| **Near future** | 20-30 tests | **Option 2** (Health checks) | Explicit readiness, better diagnostics |
| **Medium future** | 50-100 tests | **Option 4** (Test framework) | Consistency, reduced boilerplate |
| **Long-term** | 100+ tests | **Option 4 or 5** | Professional infrastructure |
| **Hardware-in-loop** | Any | **Option 2 or 3** | Can't use containers with real CAN |
| **Multi-simulator** | Any | **Option 4 or 5** | Orchestration needed |

### Decision Tree

```
Are you testing with real CAN hardware?
├─ YES → Option 2 (Health) or Option 3 (IPC)
│         (Can't use containers)
│
└─ NO → How many tests do you have?
        ├─ <20 tests → Option 1 (Retry) ✅ CURRENT
        ├─ 20-50 tests → Option 2 (Health checks)
        ├─ 50-100 tests → Option 4 (Test framework)
        └─ 100+ tests → Option 4 or 5 (Framework or Containers)
```

---

## Migration Path

If you decide to upgrade from Option 1, here's the recommended incremental path:

### Stage 1: Add Health Endpoints (Option 1 → Option 2)

**Effort**: 2-3 days
**Value**: Explicit readiness, better diagnostics

**Steps**:
1. Add `SimulatorHealth` to `cando-simulator-common` (4 hours)
2. Modify HVPC simulator to expose `/health` (2 hours)
3. Modify EMP simulator to expose `/health` (2 hours)
4. Update HVPC test fixture to poll health endpoint (1 hour)
5. Update EMP test fixture to poll health endpoint (1 hour)
6. Keep retry logic as fallback (1 hour)
7. Test and validate (2 hours)

**Result**: Tests use health endpoint when available, fall back to retry if not.

### Stage 2: Create Shared Test Utilities

**Effort**: 1-2 days
**Value**: Reduce code duplication

**Steps**:
1. Create `cando-simulator-common/src/testing.rs` (3 hours)
2. Extract common spawn/connect logic (2 hours)
3. Add helper functions for health checks (2 hours)
4. Refactor test fixtures to use helpers (3 hours)

**Result**: ~50% less code in each test fixture.

### Stage 3: Extract Test Framework (Option 2 → Option 4)

**Effort**: 1-2 weeks
**Value**: Professional-grade test infrastructure

**Steps**:
1. Create `cando-test-harness` crate (2 days)
2. Implement `TestContext` and `ManagedSimulator` (3 days)
3. Port 5 tests as proof-of-concept (2 days)
4. Port remaining tests (2 days)
5. Documentation and examples (1 day)

**When**: Only when test count exceeds 50.

---

## Metrics Comparison

### Startup Time (Cold Start)

| Option | Time to Connection | Overhead |
|--------|-------------------|----------|
| Option 1 (Retry) | ~1.5s | None (polling only) |
| Option 2 (Health) | ~1.0s | +HTTP server |
| Option 3 (IPC) | ~0.8s | +File I/O |
| Option 4 (Framework) | ~1.0s | +Framework abstraction |
| Option 5 (Containers) | ~8-12s | +Container startup |

### Code Complexity (Per Test)

| Option | Test Code Lines | Infrastructure Lines |
|--------|----------------|---------------------|
| Option 1 (Retry) | 80 lines | 0 |
| Option 2 (Health) | 60 lines | +200 (shared) |
| Option 3 (IPC) | 70 lines | +100 (shared) |
| Option 4 (Framework) | 15 lines | +1000-2000 (framework) |
| Option 5 (Containers) | 25 lines | +500 (Docker) |

### Maintainability Score (1-10)

| Option | Score | Reasoning |
|--------|-------|-----------|
| Option 1 (Retry) | 7/10 | Simple, but duplicated logic |
| Option 2 (Health) | 8/10 | Clear semantics, minimal duplication |
| Option 3 (IPC) | 7/10 | Simple protocol, platform differences |
| Option 4 (Framework) | 9/10 | Consistent, but framework to maintain |
| Option 5 (Containers) | 6/10 | Powerful, but Docker complexity |

---

## Summary

**Current Status**: ✅ Option 1 (Retry with Exponential Backoff) implemented and working

**Recommendation**: Stay with Option 1 until test count exceeds 20-30

**Next Upgrade**: Option 2 (Health Check Endpoints) when you add UDC integration tests

**Long-term**: Consider Option 4 (Test Framework) if test count exceeds 50

---

## References

**Implementation Examples**:
- Current implementation: `hvpc-simulator/tests/integration_test.rs:99-128`
- Current implementation: `emp-simulator/tests/integration_test.rs:111-140`

**Related Documents**:
- `doc/TIER1-OPTIMIZATION-ANALYSIS-SESSION-92.md` - Tier1 optimization rationale
- `doc/CONFIG-BASED-INTEGRATION-TESTS-SESSION-91.md` - Config-based test migration

**External Resources**:
- Kubernetes Readiness Probes: https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/
- Testcontainers: https://github.com/testcontainers/testcontainers-rs

---

**Document Version**: 1.0
**Last Updated**: 2026-02-07 (Session 92)
**Status**: Reference - Approved for future consideration
```

