#!/bin/bash
# Quiet build script to reduce token consumption
set -e

echo "🔨 Building Leaflet WebGL Hybrid POC (quiet mode)..."

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Build with minimal output
BUILD_OUTPUT=$(cargo build --release --target wasm32-unknown-unknown 2>&1)
if echo "$BUILD_OUTPUT" | grep -E "(error)" | head -n 50; then
    echo "❌ Build failed with errors"
    exit 1
fi

# Show warnings if any
if echo "$BUILD_OUTPUT" | grep -E "(warning)" | head -n 20; then
    echo "⚠️  Build completed with warnings"
fi

echo "✅ Build completed successfully"

# Show only build size summary
if [ -f "target/wasm32-unknown-unknown/release/leaflet_webgl_hybrid_poc.wasm" ]; then
    SIZE=$(ls -lh target/wasm32-unknown-unknown/release/leaflet_webgl_hybrid_poc.wasm | awk '{print $5}')
    echo "📦 WASM size: $SIZE"
fi

# Run wasm-bindgen with quiet output
echo "🔧 Running wasm-bindgen..."
if ! wasm-bindgen target/wasm32-unknown-unknown/release/leaflet_webgl_hybrid_poc.wasm \
    --out-dir dist \
    --web \
    --no-typescript \
    --remove-name-section \
    --remove-producers-section 2>&1 | grep -E "(error|warning)" | head -n 20; then
    echo "✅ wasm-bindgen completed"
fi

# Run optimization if wasm-opt exists
if command_exists wasm-opt; then
    echo "🎯 Running wasm-opt..."
    wasm-opt -Oz \
        --enable-bulk-memory \
        --enable-mutable-globals \
        --enable-nontrapping-float-to-int \
        --enable-sign-ext \
        --enable-simd \
        dist/leaflet_webgl_hybrid_poc_bg.wasm \
        -o dist/leaflet_webgl_hybrid_poc_bg.wasm 2>&1 | grep -E "(error|warning|Optimizing)" | head -n 10
    echo "✅ Optimization complete"
fi

# Final size report
echo ""
echo "📊 Final build sizes:"
ls -lh dist/*.wasm 2>/dev/null | awk '{print "  " $9 ": " $5}' | head -n 5

echo ""
echo "✨ Build process completed!"