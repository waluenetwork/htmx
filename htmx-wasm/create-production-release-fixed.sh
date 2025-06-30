#!/bin/bash
set -e

echo "🚀 Creating Fixed HTMX WASM Production Release"

./build.sh

rm -rf releases/production-fixed
mkdir -p releases/production-fixed

cp pkg/htmx_wasm.js releases/production-fixed/
cp pkg/htmx_wasm_bg.wasm releases/production-fixed/
cp pkg/htmx_wasm.d.ts releases/production-fixed/

cp pkg/htmx_wasm_minimal.js releases/production-fixed/
cp pkg/htmx_wasm_minimal_bg.wasm releases/production-fixed/
cp pkg/htmx_wasm_minimal.d.ts releases/production-fixed/

cp single-bundle-fixed.html releases/production-fixed/
cp minimal-bundle-fixed.html releases/production-fixed/
cp production-server.py releases/production-fixed/

cp README.md releases/production-fixed/
cp package.json releases/production-fixed/

cd releases
tar -czf htmx-wasm-production-v0.1.1-fixed.tar.gz production-fixed/

echo "✅ Fixed production release created: releases/htmx-wasm-production-v0.1.1-fixed.tar.gz"
echo ""
echo "📋 Testing Instructions:"
echo "1. Extract: tar -xzf htmx-wasm-production-v0.1.1-fixed.tar.gz"
echo "2. Enter: cd production-fixed/"
echo "3. Start server: python3 production-server.py"
echo "4. Open: http://localhost:8080/single-bundle-fixed.html"
echo "5. Test: WebSocket chat, SSE events, API calls"
