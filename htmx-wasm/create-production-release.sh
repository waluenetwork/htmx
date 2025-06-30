#!/bin/bash
set -e

echo "🚀 Creating HTMX WASM Production Release"

echo "📦 Building single WASM bundle..."
wasm-pack build --target web --out-dir pkg --release

echo "📦 Building minimal WASM bundle..."
wasm-pack build --target web --out-dir pkg-minimal --release --features ultra-minimal

rm -rf releases/production
mkdir -p releases/production

cp pkg/htmx_wasm.js releases/production/
cp pkg/htmx_wasm_bg.wasm releases/production/
cp pkg/htmx_wasm.d.ts releases/production/

cp pkg-minimal/htmx_wasm.js releases/production/htmx_wasm_minimal.js
cp pkg-minimal/htmx_wasm_bg.wasm releases/production/htmx_wasm_minimal_bg.wasm
cp pkg-minimal/htmx_wasm.d.ts releases/production/htmx_wasm_minimal.d.ts

cp -r js-extensions releases/production/

cp single-bundle-fixed.html releases/production/
cp minimal-bundle-fixed.html releases/production/
cp production-server.py releases/production/
cp PRODUCTION_TESTING_GUIDE.md releases/production/
cp QUICK_START.md releases/production/
cp RELEASE_NOTES.md releases/production/

cp htmx-wasm-wrapper.js releases/production/

cp README.md releases/production/
cp package.json releases/production/

cd releases
tar -czf htmx-wasm-production-v0.1.0-fixed.tar.gz production/

echo "✅ Production release created: releases/htmx-wasm-production-v0.1.0-fixed.tar.gz"
echo "📦 Contents:"
echo "   - Single bundle: htmx_wasm.js"
echo "   - Minimal bundle: htmx_wasm_minimal.js"
echo "   - Working examples: single-bundle-fixed.html, minimal-bundle-fixed.html"
echo "   - Production server: production-server.py"
echo "   - Testing guides: PRODUCTION_TESTING_GUIDE.md, QUICK_START.md, RELEASE_NOTES.md"
echo "   - JavaScript extensions: js-extensions/"
echo ""
echo "🔗 To test:"
echo "   1. Extract: tar -xzf htmx-wasm-production-v0.1.0-fixed.tar.gz"
echo "   2. cd production/"
echo "   3. pip install websockets"
echo "   4. python3 production-server.py"
echo "   5. Open: http://localhost:8080/single-bundle-fixed.html"
