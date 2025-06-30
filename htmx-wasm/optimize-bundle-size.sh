#!/bin/bash
echo "=== HTMX WASM Bundle Size Optimization ==="

echo "Current bundle sizes:"
ls -lh pkg/*.wasm pkg/*.js 2>/dev/null || echo "No current build"

echo "=== Step 1: Ultra-minimal build ==="
cp Cargo.ultra-minimal.toml Cargo.toml
wasm-pack build --target web --release --no-default-features
echo "Ultra-minimal build size:"
ls -lh pkg/*.wasm pkg/*.js

echo "=== Step 2: Aggressive optimization build ==="
cat > Cargo.toml << 'EOF'
[package]
name = "htmx-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = { version = "0.2", default-features = false }
web-sys = { version = "0.3", default-features = false, features = [
  "console", "Document", "Element", "Event", "EventTarget", "HtmlElement"
]}
js-sys = { version = "0.3", default-features = false }

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = "symbols"

[profile.release.package."*"]
opt-level = "z"

[dependencies.wasm-bindgen]
version = "0.2"
default-features = false
EOF

echo "=== Building with maximum optimization ==="
wasm-pack build --target web --release --no-default-features
echo "Optimized build size:"
ls -lh pkg/*.wasm pkg/*.js

echo "=== Step 3: wee_alloc optimization ==="
cat >> Cargo.toml << 'EOF'

wee_alloc = { version = "0.4", optional = true }

[features]
default = []
wee_alloc = ["dep:wee_alloc"]
EOF

echo "=== Final optimized build ==="
wasm-pack build --target web --release --features wee_alloc
echo "Final optimized size:"
ls -lh pkg/*.wasm pkg/*.js

echo "=== Size comparison ==="
WASM_SIZE=$(stat -c%s pkg/*.wasm 2>/dev/null || echo "0")
JS_SIZE=$(stat -c%s pkg/*.js 2>/dev/null || echo "0")
TOTAL_SIZE=$((WASM_SIZE + JS_SIZE))
TARGET_SIZE=15360  # 15KB in bytes

echo "WASM: ${WASM_SIZE} bytes"
echo "JS: ${JS_SIZE} bytes" 
echo "Total: ${TOTAL_SIZE} bytes"
echo "Target: ${TARGET_SIZE} bytes"
echo "Ratio: $(echo "scale=1; $TOTAL_SIZE / $TARGET_SIZE" | bc)x of target"

if [ $TOTAL_SIZE -le $TARGET_SIZE ]; then
    echo "✅ Bundle size optimization SUCCESS!"
else
    echo "⚠️  Still over target by $((TOTAL_SIZE - TARGET_SIZE)) bytes"
fi
