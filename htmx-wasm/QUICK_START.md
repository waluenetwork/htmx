# HTMX WASM Quick Start Guide

## 🚀 Get Started in 3 Minutes

### Step 1: Download & Extract
```bash
wget https://github.com/waluenetwork/htmx/releases/download/v0.1.0/htmx-wasm-production-v0.1.0.tar.gz
tar -xzf htmx-wasm-production-v0.1.0.tar.gz
cd htmx-wasm-production-v0.1.0/
```

### Step 2: Install Dependencies
```bash
pip install websockets
```

### Step 3: Start Server
```bash
python3 production-server.py
```

### Step 4: Test Examples
Open in browser:
- **Single Bundle**: http://localhost:8080/single-bundle-fixed.html
- **Minimal Bundle**: http://localhost:8080/minimal-bundle-fixed.html

## ✅ Expected Results

### Single Bundle Example
- ✅ Green "HTMX WASM initialized successfully" message
- ✅ WebSocket chat: Type message → see echo response
- ✅ SSE updates: Automatic messages every 2 seconds
- ✅ API calls: All buttons load data successfully
- ✅ Performance stats: Load time, memory usage displayed

### Minimal Bundle Example  
- ✅ Green initialization message with load time
- ✅ API calls: Load Data and API Test buttons work
- ✅ Performance stats: Load time and memory displayed
- ❌ No WebSocket/SSE (not included in minimal build)

## 🔧 Integration in Your Project

### Single Bundle (Recommended)
```html
<script type="module">
  import init, { HtmxWasm } from './htmx_wasm.js';
  
  await init();
  const htmx = new HtmxWasm();
  window.htmx = htmx;
  
  // Extensions are built-in
  htmx.enable_extension('ws');
  htmx.enable_extension('sse');
</script>
```

### Minimal Bundle (8.7KB)
```html
<script type="module">
  import init, { HtmxWasm } from './htmx_wasm_minimal.js';
  
  await init();
  const htmx = new HtmxWasm();
  window.htmx = htmx;
  
  // No extensions in minimal build
</script>
```

## 🐛 Troubleshooting

| Issue | Solution |
|-------|----------|
| WebSocket connection fails | `pip install websockets` |
| WASM module fails to load | Check browser console, ensure files served correctly |
| Server won't start | Check ports 8080/8081 are available |
| API calls fail | Verify server is running on localhost:8080 |
| SSE not working | Check `/events` endpoint responds |

## 📁 File Structure
```
production/
├── htmx_wasm.js                    # Single bundle (71KB)
├── htmx_wasm_bg.wasm              # Single bundle binary
├── htmx_wasm_minimal.js           # Minimal bundle (8.7KB)
├── htmx_wasm_minimal_bg.wasm      # Minimal bundle binary
├── single-bundle-fixed.html       # Full example
├── minimal-bundle-fixed.html      # Minimal example
├── production-server.py           # Backend server
└── README.md                      # Full documentation
```

## 🎯 Success Criteria
- Both examples load without console errors
- WASM modules initialize successfully  
- WebSocket chat works (single bundle only)
- SSE events stream (single bundle only)
- API calls return data and update DOM
- Performance stats display correctly

**Need help?** Check `PRODUCTION_TESTING_GUIDE.md` for detailed troubleshooting.
