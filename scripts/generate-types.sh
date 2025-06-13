#!/bin/bash

# TypeScript型定義を生成するスクリプト

echo "Generating TypeScript type definitions..."

# TypeScript機能を有効にしてテストを実行（型定義生成）
cargo test --features typescript export_typescript_bindings

if [ $? -eq 0 ]; then
    echo "✅ TypeScript definitions generated successfully in bindings/"
else
    echo "❌ Failed to generate TypeScript definitions"
    exit 1
fi