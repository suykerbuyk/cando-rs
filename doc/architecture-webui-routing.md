# WebUI Routing Architecture Guide

**Status**: ✅ Implementation Complete (Phases 1-3)
**Date**: 2026-02-10
**Branch**: `feature/enhanced-hvpc-support`

---

## Overview

Cando WebUI uses **path-based routing** to provide device-type-specific UIs with clean separation of concerns. Each device type (EMP, UDC, HVPC) has its own URL path and dedicated UI, while sharing common infrastructure (API, WebSocket, static assets).

### URL Structure

```
http://localhost:10752/           → Landing page (device type selector)
http://localhost:10752/emp        → EMP device monitoring UI
http://localhost:10752/udc        → UDC device monitoring UI (placeholder)
http://localhost:10752/hvpc       → HVPC device monitoring UI (placeholder)
http://localhost:10752/api/*      → Shared REST API endpoints
http://localhost:10752/ws         → Shared WebSocket endpoint
```

### Key Benefits

1. **Bookmarkable URLs**: Users can bookmark device-specific pages
2. **Browser Tabs**: Multiple device types can be open simultaneously
3. **Maintainability**: Clear separation between device-specific code
4. **Scalability**: Easy to add new device types
5. **No External Dependencies**: Pure Axum routing, no nginx/DNS required

---

## Architecture Overview

### Directory Structure

```
cando-webui/
├── src/
│   ├── server.rs           # Route handlers and router configuration
│   └── main.rs             # Entry point
├── templates/              # Embedded Tera templates
│   ├── landing.html.tera   # Device type selector
│   ├── emp/
│   │   └── index.html.tera # EMP device UI
│   ├── udc/
│   │   └── index.html.tera # UDC device UI (placeholder)
│   └── hvpc/
│       └── index.html.tera # HVPC device UI (placeholder)
└── static/                 # Embedded static assets
    ├── common/             # Shared assets
    │   ├── css/
    │   │   └── base.css    # CSS variables, reset styles
    │   └── js/
    │       ├── justgage.min.js
    │       └── raphael.min.js
    ├── emp/                # EMP-specific assets
    │   ├── css/
    │   │   └── emp-style.css
    │   └── js/
    │       └── emp-app.js
    ├── udc/                # UDC-specific assets (future)
    └── hvpc/               # HVPC-specific assets (future)
```

### Routing Implementation

The routing is implemented using **Axum's nested routing** feature (`.nest()`):

```rust
// Device-type specific routes
let emp_routes = Router::new()
    .route("/", get(emp_index_handler));

let udc_routes = Router::new()
    .route("/", get(udc_index_handler));

let hvpc_routes = Router::new()
    .route("/", get(hvpc_index_handler));

// Main router
Router::new()
    .route("/static/{*path}", get(serve_embedded_static))
    .nest("/emp", emp_routes)    // All /emp/* routes
    .nest("/udc", udc_routes)    // All /udc/* routes
    .nest("/hvpc", hvpc_routes)  // All /hvpc/* routes
    .nest("/api", api_routes)    // Shared API
    .route("/", get(landing_page_handler))  // Root
    .route("/ws", get(websocket_handler))   // WebSocket
    .with_state(app_state)
```

### Handler Pattern

Each device type has a dedicated handler that:
1. Filters devices by type from the environment
2. Renders the device-specific template
3. Returns HTML response

**Example** (EMP handler):

```rust
async fn emp_index_handler(State(state): State<AppState>) -> Html<String> {
    // Find environment
    let environment = state
        .cando_config
        .environments
        .iter()
        .find(|(name, _)| **name == state.environment_name);

    // Filter for EMP devices only
    let devices: Vec<_> = environment
        .map(|(env_name, env)| {
            env.devices
                .iter()
                .filter(|(_, device)| device.enabled)
                .filter(|(_, device)| {
                    device.device_type.to_string().to_lowercase() == "emp"
                })
                .filter_map(|(device_key, device)| {
                    // Parse device ID and create JSON structure
                    Some(serde_json::json!({
                        "device_id": device_id,
                        "name": format!("{}:{}", env_name, device_key),
                        // ... other fields
                    }))
                })
                .collect()
        })
        .unwrap_or_default();

    // Render template
    let mut context = tera::Context::new();
    context.insert("devices", &devices);

    match state.tera.render("emp/index.html.tera", &context) {
        Ok(html) => Html(html),
        Err(e) => Html(format!("<h1>Error</h1><p>{}</p>", e))
    }
}
```

---

## How to Add a New Device Type

Adding support for a new device type (e.g., J1939, custom protocol) follows a consistent 5-step pattern:

### Step 1: Create Template Directory

Create a new directory for your device type's templates:

```bash
mkdir -p cando-webui/templates/j1939
```

### Step 2: Create Template File

Create `cando-webui/templates/j1939/index.html.tera`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Cando WebUI - J1939 Devices</title>
    <link rel="stylesheet" href="/static/common/css/base.css">
    <link rel="stylesheet" href="/static/j1939/css/j1939-style.css">
</head>
<body>
    <header class="header">
        <h1>J1939 Device Monitoring</h1>
    </header>

    <main class="main-content">
        <div class="container">
            {% for device in devices %}
            <div class="device-card">
                <h2>{{ device.name }}</h2>
                <p>ID: {{ device.formatted_id }}</p>
            </div>
            {% endfor %}
        </div>
    </main>
</body>
</html>
```

### Step 3: Create Route Handler

Add handler to `cando-webui/src/server.rs`:

```rust
async fn j1939_index_handler(State(state): State<AppState>) -> Html<String> {
    debug!("GET /j1939 - J1939 device page requested");

    let environment = state
        .cando_config
        .environments
        .iter()
        .find(|(name, _)| **name == state.environment_name);

    let devices: Vec<_> = environment
        .map(|(env_name, env)| {
            env.devices
                .iter()
                .filter(|(_, device)| device.enabled)
                .filter(|(_, device)| {
                    device.device_type.to_string().to_lowercase() == "j1939"
                })
                .filter_map(|(device_key, device)| {
                    let id_str = device.device_id.trim_start_matches("0x");
                    let device_id = u8::from_str_radix(id_str, 16).ok()?;

                    Some(serde_json::json!({
                        "device_id": device_id,
                        "formatted_id": format!("0x{:02X}", device_id),
                        "name": format!("{}:{}", env_name, device_key),
                        "interface": device.interface.as_deref().unwrap_or("(inherited)"),
                    }))
                })
                .collect()
        })
        .unwrap_or_default();

    debug!("J1939: {} devices ready for rendering", devices.len());

    let mut context = tera::Context::new();
    context.insert("devices", &devices);

    match state.tera.render("j1939/index.html.tera", &context) {
        Ok(html) => Html(html),
        Err(e) => {
            warn!("J1939: template render failed: {}", e);
            Html(format!(
                r#"<!DOCTYPE html>
<html><head><title>Error</title></head>
<body><h1>Template Error</h1><p>{}</p></body></html>"#,
                e
            ))
        }
    }
}
```

### Step 4: Register Routes

Add route definition and nest it in the main router:

```rust
// In build_app() method:

// J1939 device routes
let j1939_routes = Router::new()
    .route("/", get(j1939_index_handler));

// Main router
Router::new()
    // ... existing routes ...
    .nest("/emp", emp_routes)
    .nest("/udc", udc_routes)
    .nest("/hvpc", hvpc_routes)
    .nest("/j1939", j1939_routes)  // ← Add this
    // ... rest of router ...
```

### Step 5: Update Tera Initialization

Load the new template in `AppState::new()`:

```rust
// In AppState::new() method:

// Load J1939 template
let j1939_template = Templates::get("j1939/index.html.tera")
    .expect("j1939/index.html.tera must exist at compile time");

let j1939_template_str = std::str::from_utf8(&j1939_template.data)
    .expect("Template must be valid UTF-8");

tera.add_raw_template("j1939/index.html.tera", j1939_template_str)
    .expect("Embedded J1939 template must be valid Tera syntax");

// Add to logging
tracing::info!("  - j1939/index.html.tera: {} bytes", j1939_template_str.len());
```

### Step 6: Add Unit Tests (Recommended)

Add test for the new handler in the `#[cfg(test)]` module:

```rust
#[tokio::test]
async fn test_j1939_index_handler_renders() {
    let (config, env_name) = create_test_config();
    let (state_manager, _rx) = StateManager::new(&config, &env_name).unwrap();
    let broadcast_tx = state_manager.broadcast_tx.clone();
    let command_service = Arc::new(tokio::sync::Mutex::new(DirectCanCommandService::new()));
    let state = AppState::new(
        config,
        env_name,
        8080,
        state_manager,
        broadcast_tx,
        command_service,
    );

    let html = j1939_index_handler(State(state)).await;

    assert!(html.0.contains("J1939"));
    assert!(html.0.contains("<!DOCTYPE html>"));
}
```

### Build and Test

```bash
# Build with new template
cargo build -p cando-webui

# Run tests
cargo test -p cando-webui

# Start server and visit http://localhost:10752/j1939
cargo run -p cando-webui -- --cando-config cando.yaml --environment your-env
```

---

## Landing Page Auto-Discovery

The landing page automatically discovers and displays all device types present in the current environment. No manual updates needed!

### How It Works

The `get_device_types_in_environment()` helper function:
1. Scans all enabled devices in the environment
2. Groups them by device type
3. Counts devices per type
4. Returns `DeviceTypeInfo` structs with display info

```rust
struct DeviceTypeInfo {
    display_name: String,  // "EMP Devices"
    path: String,          // "emp"
    icon: String,          // "🔌"
    count: usize,          // 3
}
```

The landing page template renders cards for each device type automatically.

---

## Static Asset Organization

### Common Assets (Shared)

Location: `static/common/`

**Purpose**: Assets used by multiple device types

- **CSS Variables**: `base.css` defines color palette, spacing, typography
- **JavaScript Libraries**: `justgage.min.js`, `raphael.min.js` (gauges)
- **Placeholders**: `websocket.js`, `api-client.js` (future extraction)

### Device-Specific Assets

Location: `static/{device_type}/`

**Purpose**: Assets specific to one device type

- **EMP**: `static/emp/css/emp-style.css`, `static/emp/js/emp-app.js`
- **UDC**: `static/udc/` (future)
- **HVPC**: `static/hvpc/` (future)

### Asset Embedding

All static assets are **embedded at compile time** using `rust-embed`:

```rust
#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAssets;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;
```

**Benefits**:
- Single-binary deployment
- No runtime filesystem dependencies
- Automatic MIME type detection
- Built-in compression (gzip)

---

## API and WebSocket (Shared)

Both API and WebSocket endpoints are **shared across all device types**:

### REST API Structure

```
GET  /api/config                    # Server configuration
GET  /api/environments              # List all environments
GET  /api/environment/{name}        # Get environment details
POST /api/devices/{name}/rpm        # Set RPM (EMP)
POST /api/devices/{name}/direction  # Set direction (EMP)
POST /api/devices/{name}/hvpc/valve # HVPC valve control
```

### WebSocket Protocol

**Endpoint**: `ws://localhost:10752/ws`

**Message Types**:
- Device state updates (broadcasted to all clients)
- Command acknowledgments
- Real-time telemetry

All device types share the same WebSocket connection.

---

## Testing the Routing

### Manual Testing

```bash
# Start WebUI
cargo run -p cando-webui -- \
    --cando-config cando.yaml \
    --environment your-env

# Visit URLs
open http://localhost:10752/         # Landing page
open http://localhost:10752/emp      # EMP UI
open http://localhost:10752/udc      # UDC UI
open http://localhost:10752/hvpc     # HVPC UI
```

### Automated Testing

Unit tests verify handler behavior:

```bash
# Run all WebUI tests
cargo test -p cando-webui

# Run specific handler test
cargo test -p cando-webui test_emp_index_handler_renders
```

---

## Future Enhancements

### Phase 4 (Future): Full UDC/HVPC UIs

Replace placeholder templates with full monitoring UIs:
- Real-time telemetry displays
- Interactive controls
- Device-specific visualizations
- Command interfaces

### Phase 5 (Future): Shared Component Extraction

Extract common UI components:
- WebSocket client → `static/common/js/websocket.js`
- API client → `static/common/js/api-client.js`
- Gauge widgets → Reusable components

### Phase 6 (Future): Advanced Features

- Multi-device comparison views
- Historical data charts
- Alert/notification system
- Mobile-responsive layouts

---

## Troubleshooting

### Template Not Found Error

**Error**: `landing.html.tera must exist at compile time`

**Solution**: Ensure templates are in correct directory and rebuild:
```bash
ls cando-webui/templates/  # Verify files exist
cargo clean -p cando-webui
cargo build -p cando-webui
```

### Route Not Working

**Symptom**: 404 Not Found for `/mydevice`

**Solution**:
1. Verify handler function exists and is `async`
2. Verify route is added to router with `.nest()`
3. Check route path matches URL (case-sensitive)

### Template Rendering Error

**Error**: `Failed to render template: ...`

**Solution**:
1. Check Tera syntax (variables, filters, control flow)
2. Ensure context variables match template expectations
3. Test template with minimal data first

---

## References

- **Implementation**: `cando-webui/src/server.rs`
- **Templates**: `cando-webui/templates/`
- **Investigation**: `doc/WEBUI-MULTI-DEVICE-ARCHITECTURE-INVESTIGATION.md`
- **Implementation Guide**: `doc/WEBUI-PATH-BASED-ROUTING-IMPLEMENTATION.md`
- **Session Notes**: `RESUME.md` (Sessions 94-95)

---

**Document Status**: ✅ Complete
**Last Updated**: 2026-02-10 (Phase 4 Documentation)
