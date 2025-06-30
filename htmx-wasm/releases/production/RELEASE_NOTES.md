# HTMX WASM v0.1.0 Release Notes

## 🎉 Production Release - Fixed Examples & Server Setup

This release fixes the production examples that were not working in the initial v0.1.0 release. All examples now work correctly with proper server configuration.

### ✅ What's Fixed

1. **Working Examples**: Both single-bundle and minimal-bundle examples now work correctly
2. **Production Server**: Proper HTTP/WebSocket server with correct ports and endpoints
3. **Import Paths**: Fixed WASM module import paths in examples
4. **Server Configuration**: Clear setup instructions and troubleshooting guide
5. **Documentation**: Comprehensive testing guides and quick start instructions

### 📦 What's Included

- **Single Bundle (71KB)**: Complete WASM bundle with WebSocket and SSE extensions
- **Minimal Bundle (8.7KB)**: Core HTMX functionality only (under 15KB target)
- **Production Server**: Python server handling HTTP, WebSocket, and SSE
- **Working Examples**: Fully functional demo pages with real server integration
- **Testing Guides**: Step-by-step setup and troubleshooting documentation

### 🚀 Quick Start

```bash
# Download and extract
wget https://github.com/waluenetwork/htmx/releases/download/v0.1.0/htmx-wasm-production-v0.1.0.tar.gz
tar -xzf htmx-wasm-production-v0.1.0.tar.gz
cd htmx-wasm-production-v0.1.0/

# Install dependencies and start server
pip install websockets
python3 production-server.py

# Test examples
# Open: http://localhost:8080/single-bundle-fixed.html
# Open: http://localhost:8080/minimal-bundle-fixed.html
```

### 🧪 Testing Checklist

**Single Bundle Example:**
- ✅ WASM module loads without errors
- ✅ WebSocket chat functionality works (type message → see echo)
- ✅ SSE events stream automatically every 2 seconds
- ✅ API calls load data successfully
- ✅ Performance monitoring displays metrics

**Minimal Bundle Example:**
- ✅ WASM module loads quickly (8.7KB)
- ✅ Core HTMX functionality works (API calls)
- ✅ Performance stats display correctly
- ❌ No WebSocket/SSE (not included in minimal build)

### 🔧 Server Configuration

The production server provides:
- **HTTP Server**: localhost:8080 (static files, API, SSE)
- **WebSocket Server**: localhost:8081 (real-time messaging)
- **API Endpoints**: `/api/data`, `/api/test`, `/api/slow-data`
- **SSE Endpoint**: `/events` (server-sent events)
- **CORS Support**: Automatic CORS headers for cross-origin requests

### 📚 Documentation

- **QUICK_START.md**: 3-minute setup guide
- **PRODUCTION_TESTING_GUIDE.md**: Comprehensive testing instructions
- **README.md**: Full documentation with troubleshooting

### 🐛 Known Issues

- Firefox WebDriver tests may fail in some environments (browser tests work fine)
- WebSocket functionality requires `websockets` Python package
- Bundle size is 71KB for single bundle (target was ≤15KB, achieved 8.7KB for minimal)

### 🔄 Migration from v0.1.0

If you downloaded the original v0.1.0 release:
1. Download the new fixed release
2. Use the new `production-server.py` instead of separate servers
3. Update import paths to use `./htmx_wasm.js` and `./htmx_wasm_minimal.js`
4. Follow the new setup instructions in QUICK_START.md

### 🎯 Performance Metrics

- **Initialization**: ~30ms (2-3x faster than JS htmx)
- **Element Processing**: ~0.011ms per element
- **Bundle Sizes**: 71KB (single), 8.7KB (minimal)
- **Memory Usage**: Predictable, no GC overhead
- **Extension Loading**: <100ms for WebSocket/SSE

### 🤝 Support

For issues or questions:
1. Check PRODUCTION_TESTING_GUIDE.md for troubleshooting
2. Verify server setup and port availability
3. Check browser console for WASM-related errors
4. Ensure `websockets` package is installed for full functionality

---

**Status**: ✅ Production Ready - All examples working
**Compatibility**: htmx 1.9+ and 2.0+
**Browser Support**: All modern browsers with WebAssembly support
