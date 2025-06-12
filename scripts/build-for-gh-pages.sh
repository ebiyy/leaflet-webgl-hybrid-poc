#!/bin/bash

# GitHub Pages用の簡易ビルドスクリプト
# Trunkのwasm-optエラーを回避するため、手動でビルドプロセスを制御

set -e

echo "🚀 Starting GitHub Pages build..."

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

# 1. Rustのビルド
echo "🔨 Building Rust/WASM..."
cargo build --release --target wasm32-unknown-unknown --features wee_alloc

# 2. wasm-bindgenでバインディング生成
echo "🔄 Generating WASM bindings..."
wasm-bindgen \
    --out-dir dist \
    --target web \
    --no-typescript \
    target/wasm32-unknown-unknown/release/leaflet-webgl-hybrid-poc.wasm

# 3. HTMLファイルの処理
echo "📄 Processing HTML..."
if [ -f "index-optimized.html" ]; then
    cp index-optimized.html dist/index.html
else
    cp index.html dist/index.html
fi

# data-trunk rel="rust"を実際のスクリプトタグで置換
WASM_NAME="leaflet-webgl-hybrid-poc"

# Perlを使用して複数行の置換を行う（sedよりも信頼性が高い）
perl -i -pe 's|<link data-trunk rel="rust" />|<link rel="modulepreload" href="./'"${WASM_NAME}"'.js">\n<script type="module">\nimport init from "./'"${WASM_NAME}"'.js";\ninit("./'"${WASM_NAME}"'_bg.wasm");\n</script>|' dist/index.html

# CSSパスの修正
sed -i.bak 's|href="./src/style.css"|href="./style.css"|g' dist/index.html
sed -i.bak 's|href="./src/tailwind-generated.css"|href="./tailwind-generated.css"|g' dist/index.html
rm dist/index.html.bak

# 4. アセットのコピー
echo "📦 Copying assets..."
cp src/style.css dist/
cp src/tailwind-generated.css dist/
cp src/leaflet_helpers.js dist/
if [ -f "service-worker.js" ]; then
    cp service-worker.js dist/
fi

# 5. wasm-optによる最適化（オプション）
echo "🔧 Optimizing with wasm-opt..."
if command -v wasm-opt &> /dev/null; then
    wasm-opt \
        --enable-bulk-memory \
        --enable-simd \
        --enable-nontrapping-float-to-int \
        --enable-sign-ext \
        --enable-mutable-globals \
        -O3 \
        dist/${WASM_NAME}_bg.wasm \
        -o dist/${WASM_NAME}_bg_opt.wasm
    
    if [ -f "dist/${WASM_NAME}_bg_opt.wasm" ]; then
        mv dist/${WASM_NAME}_bg_opt.wasm dist/${WASM_NAME}_bg.wasm
        echo "✅ wasm-opt optimization complete"
    fi
else
    echo "⚠️  wasm-opt not found. Skipping optimization."
fi

# 6. 最終レポート
echo ""
echo "📊 Build complete!"
echo "Files in dist:"
ls -lh dist/

echo ""
echo "🎉 GitHub Pages build complete!"