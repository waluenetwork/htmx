# HTMX WASM Production Testing Guide

## Quick Test Instructions

### 1. Download and Extract
```bash
# Download the production release
wget https://github.com/waluenetwork/htmx/releases/download/v0.1.0/htmx-wasm-production-v0.1.0.tar.gz

# Extract files
tar -xzf htmx-wasm-production-v0.1.0.tar.gz
cd htmx-wasm-production-v0.1.0/
```

### 2. Install Dependencies
```bash
# Install Python WebSocket support
pip install websockets
```

### 3. Start the Server
```bash
# Start the production server (handles both HTTP and WebSocket)
python3 production-server.py
```

You should see:
```
🚀 Starting HTMX WASM Production Server
============================================================
📍 HTTP Server: http://localhost:8080
📍 WebSocket Server: ws://localhost:8081
📍 SSE Endpoint: http://localhost:8080/events
📍 API Endpoints: http://localhost:8080/api/*
============================================================
🔗 Test URLs:
   Single Bundle: http://localhost:8080/single-bundle-fixed.html
   Minimal Bundle: http://localhost:8080/minimal-bundle-fixed.html
============================================================
```

### 4. Test Examples

#### Single Bundle Example (http://localhost:8080/single-bundle-fixed.html)

**Expected Functionality:**
1. **WASM Initialization**: Green status showing "✅ HTMX WASM initialized successfully"
2. **WebSocket Chat**: 
   - Type message and press Enter
   - Should see echo response: "📨 Echo: [your message] at [time]"
3. **Server-Sent Events**:
   - Automatic updates every 2 seconds: "📡 SSE update X at [time]"
   - Manual trigger button adds: "🎯 Manual event triggered at [time]"
4. **API Calls**:
   - "📊 Load Data" button loads sample data
   - "🧪 API Test" button shows success message
   - "🐌 Slow Request" button demonstrates loading states
5. **Performance Monitor**:
   - Load time, memory usage, active connections displayed
   - Updates every 5 seconds

#### Minimal Bundle Example (http://localhost:8080/minimal-bundle-fixed.html)

**Expected Functionality:**
1. **WASM Initialization**: Green status showing initialization time
2. **API Calls Only**:
   - "📊 Load Data" and "🧪 API Test" buttons work
   - No WebSocket or SSE functionality (not included in minimal build)
3. **Performance Stats**: Load time and memory usage displayed

### 5. Troubleshooting

#### Common Issues:

**"Failed to initialize HTMX WASM"**
- Check browser console for detailed error messages
- Ensure WASM files are served correctly (check Network tab)
- Verify server is running on correct ports

**WebSocket Connection Failed**
- Ensure WebSocket server is running on port 8081
- Check for firewall blocking WebSocket connections
- Verify `websockets` Python package is installed

**SSE Not Working**
- Check that HTTP server is responding to `/events` endpoint
- Verify browser supports Server-Sent Events
- Check Network tab for SSE connection status

**API Calls Failing**
- Verify HTTP server is running on port 8080
- Check CORS headers are being sent correctly
- Ensure API endpoints `/api/data`, `/api/test` are responding

#### Debug Commands:

```bash
# Test HTTP server
curl http://localhost:8080/api/test

# Test SSE endpoint
curl http://localhost:8080/events

# Check if WebSocket port is open
netstat -an | grep 8081
```

### 6. File Structure

The production release should contain:
```
production/
├── htmx_wasm.js                    # Single bundle WASM module
├── htmx_wasm_bg.wasm              # Single bundle WASM binary
├── htmx_wasm.d.ts                 # TypeScript definitions
├── htmx_wasm_minimal.js           # Minimal bundle module
├── htmx_wasm_minimal_bg.wasm      # Minimal bundle binary
├── htmx_wasm_minimal.d.ts         # Minimal TypeScript definitions
├── single-bundle-fixed.html       # Single bundle example
├── minimal-bundle-fixed.html      # Minimal bundle example
├── production-server.py           # Production server
├── js-extensions/                 # JavaScript extensions
├── htmx-wasm-wrapper.js          # Compatibility wrapper
├── README.md                      # Documentation
└── package.json                   # Package metadata
```

### 7. Success Criteria

✅ **All tests pass if:**
- Both example pages load without errors
- WASM modules initialize successfully
- WebSocket chat works in single bundle example
- SSE events stream automatically in single bundle example
- API calls return data in both examples
- Performance monitoring displays metrics
- No console errors related to WASM or HTMX functionality

❌ **Tests fail if:**
- WASM modules fail to load or initialize
- WebSocket connections don't establish
- SSE events don't stream
- API calls return errors or don't update DOM
- Console shows HTMX-related errors
- Examples don't match expected functionality described above
