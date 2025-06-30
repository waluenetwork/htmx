#!/bin/bash
set -e

echo "🚀 Building HTMX WASM Multi-Extension Architecture..."

echo "📦 Building single WASM bundle with built-in extensions..."
wasm-pack build --target web --features all-extensions

echo "🔌 Building modular WebSocket extension..."
cd extensions/websocket-module
wasm-pack build --target web
cd ../..

echo "📡 Building modular SSE extension..."
cd extensions/sse-module
wasm-pack build --target web
cd ../..

echo "📋 Copying JavaScript extensions..."
mkdir -p pkg/js-extensions
cp js-extensions/*.js pkg/js-extensions/

echo "📄 Copying examples..."
cp examples/*.html pkg/

echo "✅ Build complete! All extension architectures ready:"
echo "   - Single WASM bundle: pkg/htmx_wasm.js"
echo "   - Modular WebSocket: extensions/websocket-module/pkg/"
echo "   - Modular SSE: extensions/sse-module/pkg/"
echo "   - JavaScript extensions: pkg/js-extensions/"
echo "   - Examples: pkg/*.html"
