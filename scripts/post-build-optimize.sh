#!/bin/bash
# Post-build optimization script for Trunk
set -e

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

echo "🎯 Running post-build optimizations..."

# Find the WASM file in dist directory
WASM_FILE=$(find dist -name "*_bg.wasm" -type f | head -n 1)

if [ -z "$WASM_FILE" ]; then
    echo "❌ No WASM file found in dist directory"
    exit 1
fi

echo "📦 Found WASM file: $WASM_FILE"
ORIGINAL_SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')

# Run wasm-snip if available
if command_exists wasm-snip; then
    echo "✂️  Running wasm-snip to remove panic and fmt code..."
    wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code \
        "$WASM_FILE" -o "${WASM_FILE}.snipped" 2>/dev/null
    
    if [ -f "${WASM_FILE}.snipped" ]; then
        mv "${WASM_FILE}.snipped" "$WASM_FILE"
        echo "✅ wasm-snip completed"
    fi
fi

# Run wasm-opt if available (additional pass after Trunk's processing)
if command_exists wasm-opt; then
    echo "🔧 Running additional wasm-opt pass..."
    wasm-opt -Oz \
        --enable-bulk-memory \
        --enable-mutable-globals \
        --enable-nontrapping-float-to-int \
        --enable-sign-ext \
        --enable-simd \
        --converge \
        "$WASM_FILE" \
        -o "${WASM_FILE}.opt" 2>/dev/null
    
    if [ -f "${WASM_FILE}.opt" ]; then
        mv "${WASM_FILE}.opt" "$WASM_FILE"
        echo "✅ wasm-opt completed"
    fi
fi

# Show size reduction
FINAL_SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')
echo ""
echo "📊 Optimization results:"
echo "  Original: $ORIGINAL_SIZE"
echo "  Final: $FINAL_SIZE"

# Generate size report
echo ""
echo "📈 Full size report:"
find dist -name "*.wasm" -o -name "*.js" -o -name "*.css" | while read -r file; do
    SIZE=$(ls -lh "$file" | awk '{print $5}')
    echo "  $(basename "$file"): $SIZE"
done

echo ""
echo "✨ Post-build optimization complete!"