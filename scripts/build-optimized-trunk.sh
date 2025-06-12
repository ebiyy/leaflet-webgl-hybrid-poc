#!/bin/bash

# WASM最適化ビルドスクリプト（Trunkベース）
# HTMLファイルでwasm-optのフラグを指定済み

set -e

echo "🚀 Starting optimized WASM build with Trunk..."

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

# Trunkでビルド（HTMLで指定したwasm-optフラグが使用される）
echo "🔨 Building with Trunk..."
trunk build --release

# index-optimized.htmlがある場合の処理は不要（Trunkが処理するため）

# Service Workerをコピー
if [ -f "service-worker.js" ]; then
    cp service-worker.js dist/
fi

# 最終レポート
echo ""
echo "📊 Build complete!"
echo "Files in dist:"
ls -lh dist/

echo ""
echo "🎉 Optimized build complete!"