#!/bin/bash
set -e

echo "🧪 Testing Production Release"

./create-production-release.sh

rm -rf /tmp/htmx-wasm-test
mkdir -p /tmp/htmx-wasm-test
cd /tmp/htmx-wasm-test

tar -xzf /home/ubuntu/htmx/htmx-wasm/releases/htmx-wasm-production-v0.1.0-fixed.tar.gz

echo "📁 Production release extracted to /tmp/htmx-wasm-test/production/"
echo "📋 Contents:"
ls -la production/

echo ""
echo "🔗 To test manually:"
echo "   1. cd /tmp/htmx-wasm-test/production/"
echo "   2. python3 production-server.py"
echo "   3. Open: http://localhost:8080/single-bundle-fixed.html"
echo "   4. Open: http://localhost:8080/minimal-bundle-fixed.html"
