# WASM最適化テクニック - Leaflet WebGL Hybrid POCプロジェクトから学ぶ

## 概要

Leaflet WebGL Hybrid POCプロジェクトのPOCで、Rust/WASMアプリケーションのサイズを5MBから446KB（約91%削減）まで削減することに成功しました。本記事では、実際に効果があった最適化テクニックを体系的にまとめます。

## 1. 軽量アロケータの採用

### wee_allocの導入

```toml
[dependencies]
wee_alloc = "0.4"
```

```rust
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

**効果**: 約20-30KBのサイズ削減

### 理由
- デフォルトのアロケータ（dlmalloc）は汎用的だが、WASMには過剰
- wee_allocはWASM専用に設計された軽量アロケータ
- メモリフラグメンテーションには弱いが、小規模アプリには十分

## 2. 依存関係の最小化

### default-featuresの無効化

```toml
[dependencies]
dioxus = { version = "0.6", default-features = false, features = ["web", "router"] }
serde = { version = "1.0", default-features = false, features = ["derive"] }
```

**効果**: 依存関係により20-100KBの削減

### 不要なクレートの削除
- glooクレートの削除で90KB削減
- web-sysへの直接依存に切り替え

## 3. ビルド最適化設定

### Cargo.tomlの最適化設定

```toml
[profile.release]
opt-level = "z"        # サイズ最適化優先
lto = "fat"           # Link Time Optimization
codegen-units = 1     # 単一コード生成ユニット
strip = true          # シンボル情報削除
panic = "abort"       # パニック時の巻き戻し無効化
```

**効果**: 約30-40%のサイズ削減

## 4. 多段階最適化スクリプト

### build-optimized.sh

```bash
#!/bin/bash
# 1. リリースビルド（Dioxus CLIで自動最適化）
dx build --platform web --release

# 2. wasm-optによる最適化
wasm-opt -Oz \
  --enable-simd \
  --enable-bulk-memory \
  dist/*.wasm \
  -o dist/optimized.wasm

# 3. wasm-snipによる不要コード削除
wasm-snip dist/optimized.wasm \
  -o dist/final.wasm \
  --snip-rust-fmt-code \
  --snip-rust-panicking-code

# 4. 圧縮
brotli -9 dist/final.wasm
```

**効果**: 追加で10-15%のサイズ削減

## 5. メモリ効率的なデータ構造

### SmallVecの活用

```rust
use smallvec::SmallVec;

// 128要素まではスタック上に配置
type EventList = SmallVec<[ChaosEvent; 128]>;

pub fn generate_chaos_events(count: usize) -> EventList {
    let mut events = SmallVec::new();
    // ヒープアロケーションを避ける
    for _ in 0..count.min(128) {
        events.push(generate_event());
    }
    events
}
```

**効果**: ランタイムメモリ使用量の削減

## 6. インライン展開の戦略的使用

```rust
#[inline(always)]
fn critical_path_function() {
    // 頻繁に呼ばれる小さな関数
}

#[inline]
fn medium_function() {
    // 適度なサイズの関数
}

#[inline(never)]
fn large_function() {
    // 大きな関数（コードサイズ優先）
}
```

## 7. 未使用コードの積極的削除

### 条件付きコンパイル

```rust
#[cfg(target_arch = "wasm32")]
mod wasm_specific {
    // WASM専用コード
}

#[cfg(not(target_arch = "wasm32"))]
mod native_specific {
    // ネイティブ専用コード
}
```

## 実測結果

| 最適化段階 | サイズ | 削減率 |
|-----------|--------|--------|
| 開発ビルド | 5MB | - |
| リリースビルド | 1.2MB | 76% |
| wasm-opt適用 | 520KB | 89.6% |
| wasm-snip適用 | 446KB | 91.1% |
| Brotli圧縮 | 146KB | 97.1% |

## まとめ

WASM最適化は複数の技術を組み合わせることで大きな効果を発揮します。特に重要なのは：

1. **早期の最適化方針決定**: プロジェクト初期から最適化を意識
2. **段階的な最適化**: 各ステップの効果を測定しながら進める
3. **トレードオフの理解**: サイズ vs パフォーマンス vs 開発効率

これらの技術により、Leaflet WebGL Hybrid POCプロジェクトは当初目標の2MB以内を大幅に下回る446KBを達成し、高速なロード時間を実現しました。