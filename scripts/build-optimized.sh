#!/bin/bash

# WASM最適化ビルドスクリプト
# 複数の最適化手法を適用してWASMサイズを削減

set -e

echo "🚀 Starting optimized WASM build..."

# 環境変数設定 - 積極的な最適化フラグ
export RUSTFLAGS="-C link-arg=-s -C opt-level=z -C embed-bitcode=yes"

# ビルドディレクトリの準備
echo "📁 Preparing build directory..."
rm -rf dist
mkdir -p dist

# Tailwind CSSのビルド
echo "🎨 Building Tailwind CSS..."
if [ -f "./tailwindcss" ]; then
    ./tailwindcss -i ./src/tailwind.css -o ./src/tailwind-generated.css --minify
elif command -v npx &> /dev/null; then
    npx tailwindcss -i ./src/tailwind.css -o ./src/tailwind-generated.css --minify
else
    echo "⚠️  Tailwind CSS CLI not found. Skipping CSS build."
fi

# 1. wee_allocを使用したリリースビルド
echo "🔨 Building with wee_alloc..."
cargo build --release --target wasm32-unknown-unknown --features wee_alloc

# Trunkを使ってビルド（HTMLの処理も含む、最適化は無効）
echo "🔨 Building with Trunk (minification disabled)..."
trunk build --release --minify=false

# index-optimized.htmlがある場合は、それをベースに再処理
if [ -f "index-optimized.html" ]; then
    # Trunk出力から必要なスクリプトタグを抽出
    SCRIPT_TAGS=$(grep -E '<script.*leaflet-webgl-hybrid-poc.*</script>|<link.*modulepreload.*leaflet-webgl-hybrid-poc' dist/index.html | sed 's/^[[:space:]]*//')
    
    # index-optimized.htmlのdata-trunk部分を置換
    cp index-optimized.html dist/index.html.tmp
    # <link data-trunk rel="rust" />の行を見つけて置換
    awk -v scripts="$SCRIPT_TAGS" '/<link data-trunk rel="rust"/ {print scripts; next} 1' dist/index.html.tmp > dist/index.html
    rm dist/index.html.tmp
fi
cp src/style.css dist/
cp src/tailwind-generated.css dist/
cp src/leaflet_helpers.js dist/
# Service Workerをコピー
if [ -f "service-worker.js" ]; then
    cp service-worker.js dist/
fi

# wasm-bindgenでバインディング生成 (追加の最適化フラグ付き)
echo "🔄 Generating WASM bindings..."
wasm-bindgen \
    --out-dir dist \
    --target web \
    --no-typescript \
    --remove-name-section \
    --remove-producers-section \
    target/wasm32-unknown-unknown/release/leaflet-webgl-hybrid-poc.wasm

# 2. wasm-optによる最適化（サイズ最優先）
echo "🔧 Optimizing with wasm-opt..."
if command -v wasm-opt &> /dev/null; then
    # 元のWASMファイルサイズを記録
    ORIGINAL_SIZE=$(stat -f%z dist/leaflet-webgl-hybrid-poc_bg.wasm 2>/dev/null || stat -c%s dist/leaflet-webgl-hybrid-poc_bg.wasm)
    
    # wasm-optでサイズ最適化（互換性のあるフラグのみ使用）
    wasm-opt -Oz \
        --enable-bulk-memory \
        --enable-nontrapping-float-to-int \
        --strip-debug \
        --strip-producers \
        dist/leaflet-webgl-hybrid-poc_bg.wasm \
        -o dist/leaflet-webgl-hybrid-poc_bg_opt.wasm
    
    # 最適化後のサイズ
    OPTIMIZED_SIZE=$(stat -f%z dist/leaflet-webgl-hybrid-poc_bg_opt.wasm 2>/dev/null || stat -c%s dist/leaflet-webgl-hybrid-poc_bg_opt.wasm)
    
    # 最適化したファイルで元のファイルを置き換え
    mv dist/leaflet-webgl-hybrid-poc_bg_opt.wasm dist/leaflet-webgl-hybrid-poc_bg.wasm
    
    echo "✅ wasm-opt optimization complete"
    echo "   Original size: $((ORIGINAL_SIZE / 1024))KB"
    echo "   Optimized size: $((OPTIMIZED_SIZE / 1024))KB"
    echo "   Reduction: $(( (ORIGINAL_SIZE - OPTIMIZED_SIZE) * 100 / ORIGINAL_SIZE ))%"
else
    echo "⚠️  wasm-opt not found. Install with: cargo install wasm-opt"
fi

# 2.5. wasm-snipによるパニック・fmt系コードの削除
echo "✂️  Running wasm-snip to remove panic and fmt code..."
if command -v wasm-snip &> /dev/null; then
    # 元のサイズを記録
    BEFORE_SNIP=$(stat -f%z dist/leaflet-webgl-hybrid-poc_bg.wasm 2>/dev/null || stat -c%s dist/leaflet-webgl-hybrid-poc_bg.wasm)
    
    # wasm-snipでパニックとfmtコードを削除
    wasm-snip \
        --snip-rust-fmt-code \
        --snip-rust-panicking-code \
        dist/leaflet-webgl-hybrid-poc_bg.wasm \
        -o dist/leaflet-webgl-hybrid-poc_bg_snipped.wasm
    
    # スニップ後のサイズ
    AFTER_SNIP=$(stat -f%z dist/leaflet-webgl-hybrid-poc_bg_snipped.wasm 2>/dev/null || stat -c%s dist/leaflet-webgl-hybrid-poc_bg_snipped.wasm)
    
    # 最適化したファイルで元のファイルを置き換え
    mv dist/leaflet-webgl-hybrid-poc_bg_snipped.wasm dist/leaflet-webgl-hybrid-poc_bg.wasm
    
    echo "✅ wasm-snip complete"
    echo "   Before: $((BEFORE_SNIP / 1024))KB"
    echo "   After: $((AFTER_SNIP / 1024))KB"
    echo "   Reduction: $(( (BEFORE_SNIP - AFTER_SNIP) * 100 / BEFORE_SNIP ))%"
else
    echo "⚠️  wasm-snip not found. Install with: mise use cargo:wasm-snip@latest"
fi

# 3. Brotli圧縮の準備
echo "🗜️  Preparing compressed versions..."
if command -v brotli &> /dev/null; then
    # Brotli圧縮（最高圧縮レベル）
    brotli -Z -k dist/leaflet-webgl-hybrid-poc_bg.wasm
    BR_SIZE=$(stat -f%z dist/leaflet-webgl-hybrid-poc_bg.wasm.br 2>/dev/null || stat -c%s dist/leaflet-webgl-hybrid-poc_bg.wasm.br)
    echo "✅ Brotli compressed size: $((BR_SIZE / 1024))KB"
else
    echo "⚠️  brotli not found. Install with: brew install brotli"
fi

# gzip圧縮（互換性のため）
gzip -9 -c dist/leaflet-webgl-hybrid-poc_bg.wasm > dist/leaflet-webgl-hybrid-poc_bg.wasm.gz
GZ_SIZE=$(stat -f%z dist/leaflet-webgl-hybrid-poc_bg.wasm.gz 2>/dev/null || stat -c%s dist/leaflet-webgl-hybrid-poc_bg.wasm.gz)
echo "✅ Gzip compressed size: $((GZ_SIZE / 1024))KB"

# 4. 最終レポート
echo ""
echo "📊 Build Optimization Report:"
echo "================================"
FINAL_SIZE=$(stat -f%z dist/leaflet-webgl-hybrid-poc_bg.wasm 2>/dev/null || stat -c%s dist/leaflet-webgl-hybrid-poc_bg.wasm)
echo "Final WASM size: $((FINAL_SIZE / 1024))KB"
echo "Brotli size: $((BR_SIZE / 1024))KB"
echo "Gzip size: $((GZ_SIZE / 1024))KB"
echo ""

# サイズ目標チェック
if [ $FINAL_SIZE -lt 2097152 ]; then
    echo "✅ Success: WASM size < 2MB target"
else
    echo "❌ Warning: WASM size exceeds 2MB target"
fi

if [ $BR_SIZE -lt 409600 ]; then
    echo "✅ Success: Brotli compressed size < 400KB target"
else
    echo "❌ Warning: Brotli compressed size exceeds 400KB target"
fi

echo ""
echo "🎉 Build optimization complete!"