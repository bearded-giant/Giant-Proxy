# Giant Proxy Enhancement Roadmap

## Overview
This roadmap outlines the implementation plan for enhancing Giant Proxy to better compete with tools like Proxyman while maintaining its lightweight, CLI-first approach.

## Phase 1: Core Improvements (Current Sprint)

### 1. HTTPS Preservation for Local Redirects
**Goal**: Maintain HTTPS connections when redirecting to local services instead of downgrading to HTTP.

**Implementation Details**:
- Add SSL/TLS support for local redirects using mitmproxy's capabilities
- Add `--preserve-https` flag to maintain secure connections
- Generate and manage local certificates for development domains
- Update the generated Python script to handle HTTPS redirects properly

**Files to modify**:
- `giant-proxy`: Add new flag handling and certificate generation logic
- Generated Python scripts: Update request handler to preserve scheme

### 2. Proxyman Import/Export Support
**Goal**: Enable seamless migration from Proxyman by supporting its rule format.

**Implementation Details**:
- Research Proxyman's export format (.proxyman files)
- Create `import-proxyman` command to convert Proxyman rules to our format
- Create `export-proxyman` command for reverse compatibility
- Add support for Proxyman's advanced rule conditions where possible

**New commands**:
- `giant-proxy import-proxyman <file> --profile <name>`
- `giant-proxy export-proxyman --profile <name> --output <file>`

### 3. Structured Request/Response Logging
**Goal**: Provide detailed, parseable logs for debugging and analysis.

**Implementation Details**:
- Replace simple stdout redirection with structured logging
- Log requests/responses in JSON format with timestamps
- Include headers, method, status codes, and timing information
- Add `--log-level` flag (debug, info, warn, error)
- Create rotating log files to prevent disk space issues

**New features**:
- JSON-formatted logs in `logs/` directory
- Real-time log filtering with `giant-proxy logs --filter <pattern>`
- Request ID tracking for correlation

## Phase 2: Advanced Features (Future)

### 4. Header Manipulation
- Add ability to modify/add/remove headers in rules
- Support for dynamic header values (environment variables)

### 5. Web UI
- Simple web interface for rule management
- Real-time request monitoring
- Built with minimal dependencies (perhaps just Python's built-in HTTP server)

### 6. Request History & Replay
- Store request history in SQLite database
- Replay previous requests with modifications
- Export history as curl commands or HAR files

## Phase 3: Enterprise Features

### 7. WebSocket Support
- Proxy WebSocket connections
- Log WebSocket frames

### 8. Performance Monitoring
- Request/response timing metrics
- Bandwidth usage tracking
- Performance dashboards

## Technical Decisions

### Why These Three First?
1. **HTTPS Preservation**: Most critical for modern development - many APIs require HTTPS
2. **Proxyman Import/Export**: Enables easy adoption for existing Proxyman users
3. **Structured Logging**: Foundation for debugging and future features like web UI

### Design Principles
- Maintain backward compatibility
- Keep the tool lightweight and fast
- Minimize external dependencies
- CLI-first, GUI-optional approach
- Clear, helpful error messages

## Success Metrics
- HTTPS redirects work without certificate warnings
- Can import 90%+ of common Proxyman rules
- Logs provide enough detail to debug complex scenarios
- Performance overhead < 10ms per request

## Implementation Order
1. HTTPS preservation (highest user impact)
2. Proxyman import/export (user adoption)
3. Structured logging (developer experience)