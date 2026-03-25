# WITH_WEBUI: Optional Visual Monitoring During Integration Tests

**Feature Status:** ✅ Implemented  
**Version:** 1.0  
**Date:** 2025-01-19

---

## Overview

The `WITH_WEBUI` feature provides optional real-time visual monitoring during integration tests. When enabled, it automatically starts the Cando WebUI alongside test execution, allowing you to observe device state, CAN traffic, and system behavior in your browser while tests run.

## Purpose

**Problem:** Integration tests validate system behavior through automated assertions, but when debugging or demonstrating the system, you often want to **see** what's happening in real-time.

**Solution:** Set `WITH_WEBUI=1` to automatically start the WebUI alongside any integration test target. The WebUI provides:
- Real-time device state visualization
- Live gauge displays (RPM, voltage, current, power)
- Message statistics and protocol monitoring
- Human-verifiable system operation

## Quick Start

### Basic Usage

```bash
# Run tier2 tests with visual monitoring
WITH_WEBUI=1 make tier2

# Run physical CAN tests with visual monitoring
WITH_WEBUI=1 make tier2-physical
```

**Note:** `WITH_WEBUI` only works with `tier2` and `tier2-physical` targets, as they set up CAN interfaces. `tier1` is build-only validation without CAN, so WebUI cannot monitor any devices.

### What Happens

1. Tests start normally (build, setup, etc.)
2. CAN interface is configured (vcan0 or can0/can1)
3. **WebUI starts automatically** 🌐
4. **Browser URL displayed:** `http://localhost:10752`
5. Simulators start and begin broadcasting state
6. Tests run with full visual feedback
7. WebUI stops automatically when tests complete

### During Test Execution

Open `http://localhost:10752` in your browser to see:
- **Device Dashboard** - All active devices and their states
- **Real-time Gauges** - RPM, voltage, current, power
- **Status Indicators** - Ignition, direction, faults
- **Message Statistics** - Protocol activity and health

## Use Cases

### 1. Debugging Test Failures

**Scenario:** A test fails with "Expected RPM 1500, got 1450"

**Without WebUI:**
```bash
make tier2
# Test fails... now what? Check logs? Add more debug output?
```

**With WebUI:**
```bash
WITH_WEBUI=1 make tier2
# Open browser → See gauge showing 1450 RPM
# Observe: RPM oscillating around target
# Root cause: PID tuning or timing issue
```

### 2. Demonstrating System Functionality

**Scenario:** Show stakeholders that motor control works end-to-end

```bash
WITH_WEBUI=1 make test-webui-interactive
# Open browser on big screen
# Watch RPM ramp from 500 → 1500 → 2500 → 3500 RPM
# Gauges update in real-time
# Professional, visual demonstration
```

### 3. Developing New Features

**Scenario:** Adding support for a new device or message type

```bash
WITH_WEBUI=1 make tier2
# Open browser
# Send test messages
# Immediately see if WebUI decodes and displays correctly
# Iterate quickly without manual test scripts
```

### 4. Rapid Iteration During Development

**Scenario:** Testing simulator changes

```bash
# Make code changes to simulator
WITH_WEBUI=1 make tier2
# Browser shows live device state
# Verify changes work correctly
# No need for separate test harness
```

## Technical Details

### Architecture

```
┌─────────────────────────────────────────────────────┐
│  Integration Test Script (tier1/tier2/tier2-physical)│
│                                                       │
│  1. Setup CAN interface (vcan0 or can0/can1)        │
│  2. Check: should_start_webui()                      │
│  3. If WITH_WEBUI=1:                                 │
│     └─→ start_webui_for_test(interface, env)        │
│         ├─→ cando-webui --interface vcan0 \        │
│         │                 --environment tier2-virtual│
│         └─→ Wait for health check                    │
│  4. Start simulators                                 │
│  5. Run tests                                        │
│  6. Cleanup: stop_webui_if_running()                 │
└─────────────────────────────────────────────────────┘
```

### Configuration Flow

```
WITH_WEBUI=1 environment variable
    ↓
should_start_webui() returns true
    ↓
start_webui_for_test("vcan0", "tier2-virtual")
    ↓
cando-webui --interface vcan0 --environment tier2-virtual
    ↓
WebUI uses cando.toml (via --environment)
    ↓
Loads devices, ports, interfaces from single source of truth
    ↓
Displays real-time state at http://localhost:10752
```

### Implementation Files

- **Helper Library:** `scripts/integration/lib/webui_helpers.sh`
  - `should_start_webui()` - Check if feature enabled
  - `start_webui_for_test()` - Start WebUI with config
  - `wait_for_webui_ready()` - Health check
  - `stop_webui_if_running()` - Graceful shutdown
  - `print_webui_status_header()` - User notification

- **Integration Points:**
  - `Makefile` - Pass WITH_WEBUI to scripts
  - `scripts/integration/integration_test_all_protocols.sh` (tier2)
  - `scripts/integration/integration_test_physical_can.sh` (tier2-physical)
  
**Not Supported:**
- `tier1` - Build validation only, no CAN interfaces, no simulators to monitor

### WebUI Configuration

**Default Values:**
- HTTP Port: 10752
- WebSocket Port: 10753
- Environment: Passed via `--environment` flag

**Environment-Specific Settings:**
- `tier2-virtual`: vcan0, test device IDs
- `tier2-physical`: can0/can1, physical device IDs
- All loaded from `cando.toml`

## Default Behavior (WITHOUT WITH_WEBUI)

**Fast automated testing** (optimal for CI/CD):

```bash
make tier2        # No WebUI, runs in ~20-30 minutes
make tier2-physical  # No WebUI, runs in ~10-12 minutes
```

- ✅ Maximum speed
- ✅ Minimal resource usage
- ✅ CI/CD friendly (no browser required)
- ✅ Clean output focused on test results

## Enabled Behavior (WITH WITH_WEBUI=1)

**Visual debugging and demonstration** (optimal for development):

```bash
WITH_WEBUI=1 make tier2           # +WebUI, visual feedback during tests
WITH_WEBUI=1 make tier2-physical  # +WebUI, visual feedback during tests
```

- ✅ Real-time visualization
- ✅ Human-verifiable operation
- ✅ Browser-based monitoring
- ⚠️ Slightly slower (WebUI startup overhead)
- ⚠️ Requires display/browser access

## Comparison: With vs Without

| Aspect | Default (Fast) | WITH_WEBUI=1 (Visual) |
|--------|----------------|----------------------|
| **Execution Time** | Optimal | +2-3s startup |
| **Resource Usage** | Minimal | +1 WebUI process |
| **CI/CD Friendly** | ✅ Perfect | ⚠️ Requires browser |
| **Visual Feedback** | ❌ None | ✅ Real-time |
| **Debugging** | Logs only | Browser + logs |
| **Demonstrations** | ❌ Not suitable | ✅ Excellent |

## Requirements

### Software

- ✅ `cando-webui` binary built (via `make build-release`)
- ✅ `curl` (for health checks)
- ✅ Browser access (for viewing)

### Network

- ✅ Port 10752 available (HTTP)
- ✅ Port 10753 available (WebSocket)

### Automatic Checks

The implementation automatically:
- Checks if `cando-webui` binary exists
- Waits for WebUI to be ready (health check)
- Falls back gracefully if WebUI fails to start
- Cleans up WebUI process on exit

## Troubleshooting

### WebUI Fails to Start

**Symptom:** Warning message "WebUI failed to start, continuing with tests..."

**Causes & Solutions:**

1. **Binary not built:**
   ```bash
   make build-release
   # Or: cargo build --release -p cando-webui
   ```

2. **Port already in use:**
   ```bash
   # Check what's using port 10752
   lsof -i :10752
   
   # Kill the process or use different port
   sudo kill <PID>
   ```

3. **Configuration error:**
   ```bash
   # Check WebUI log
   ls -lt logs/webui_test_*.log | head -1
   cat logs/webui_test_<timestamp>.log
   ```

### WebUI Starts But Browser Shows Nothing

**Symptom:** `http://localhost:10752` shows blank page or error

**Solutions:**

1. **Check WebUI health:**
   ```bash
   curl http://localhost:10752/health
   # Should return: {"status":"ok"}
   ```

2. **Check browser console for JavaScript errors**

3. **Verify simulators are running:**
   ```bash
   ps aux | grep simulator
   ```

4. **Check WebUI log:**
   ```bash
   tail -f logs/webui_test_*.log
   ```

### Tests Pass But No Device Data

**Symptom:** WebUI shows empty dashboard or "No devices"

**Causes:**

1. **Wrong environment:**
   - Ensure script uses correct environment name
   - Check: `tier2-virtual` vs `tier2-physical`

2. **Simulators not broadcasting:**
   - WebUI only shows devices that send messages
   - Wait a few seconds for first broadcasts

3. **CAN interface mismatch:**
   - WebUI must monitor same interface as simulators
   - Check: vcan0 vs can0 vs can1

## Performance Impact

### Startup Overhead

- **WebUI startup:** ~2-3 seconds
- **Health check:** ~1 second
- **Total added time:** ~3 seconds per test run

### Runtime Overhead

- **Minimal:** WebUI runs independently
- **No impact on test execution**
- **Extra process:** ~50MB RAM

### When to Use vs Skip

**Use WITH_WEBUI=1 when:**
- 🐛 Debugging test failures
- 👀 Visual verification needed
- 📊 Demonstrating to stakeholders
- 🔧 Developing new features
- 📖 Learning how system works

**Skip WITH_WEBUI=1 when:**
- 🚀 Running in CI/CD
- ⚡ Need maximum speed
- 🤖 Automated regression testing
- 📦 Headless environment (no display)
- 🔁 Rapid iteration on tests
- 📋 Running tier1 (build validation only, no CAN)

## Examples

### Example 1: Quick Visual Check

```bash
# Make some changes to EMP simulator
vim emp-simulator/src/main.rs

# Rebuild and test with visual feedback
make build-release
WITH_WEBUI=1 make tier2

# Open http://localhost:10752 in browser
# Watch EMP devices update in real-time
# Verify changes work correctly
```

### Example 2: Debugging Timer Issues

```bash
# Test fails: "Expected message within 1 second, took 1.5s"
WITH_WEBUI=1 make tier2

# Open browser
# Watch message statistics
# See: Update rate is 0.67 Hz (should be 1 Hz)
# Root cause: Timer misconfigured in simulator
```

### Example 3: Stakeholder Demo

```bash
# Start interactive motor control demo
WITH_WEBUI=1 make test-webui-interactive

# Open browser on projector/big screen
# Show:
#   - RPM ramping smoothly
#   - Gauges responding in real-time
#   - Power calculations accurate
#   - Temperature tracking
# Professional, visual demonstration
```

### Example 4: Multi-Device Coordination

```bash
# Testing multiple devices working together
WITH_WEBUI=1 make tier2-physical

# Open browser
# See all devices on dashboard:
#   - EMP fan @ 2000 RPM
#   - EMP pump @ 1500 RPM
#   - HVPC charging @ 450V
#   - UDC converting 600V → 24V
# Verify coordination and no conflicts
```

## Future Enhancements

Potential improvements (not yet implemented):

- **Custom ports:** `WEBUI_PORT=8080 WITH_WEBUI=1 make tier2`
- **Auto-open browser:** Automatically open browser tab
- **Recording:** Capture WebUI session for playback
- **Remote access:** Access WebUI from another machine
- **Custom dashboards:** Test-specific layouts

## Related Documentation

- **WebUI Implementation:** `doc/REALTIME_MONITORING_WEBUI.md`
- **Configuration System:** `cando-cfg/README.md`
- **Integration Tests:** `scripts/integration/README.md` (if exists)
- **Make Targets:** Run `make help`

## Summary

The `WITH_WEBUI` feature provides **optional visual monitoring** during integration tests:

- ✅ **Easy to use:** Just add `WITH_WEBUI=1` before tier2/tier2-physical targets
- ✅ **No impact when disabled:** Default behavior unchanged
- ✅ **Automatic:** Starts/stops with tests
- ✅ **Configuration-driven:** Uses cando.toml
- ✅ **Graceful fallback:** Tests continue if WebUI fails
- ℹ️ **Requires CAN:** Only works with tier2/tier2-physical (tier1 is build-only)

**Best practice:**
- Use `WITH_WEBUI=1` during development and debugging
- Skip it for CI/CD and automated testing
- Enable it when demonstrating to stakeholders
- Remember: Only tier2 and tier2-physical support visual monitoring

**Philosophy:** Tests should be fast by default, but visual feedback should be one environment variable away when you need it.