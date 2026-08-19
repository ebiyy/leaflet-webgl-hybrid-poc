# WASMアロケータ比較レポート

> **注記 (2026-08-19)**: `wee_alloc` はメンテナンス終了（GHSA-rc23-xxgq-x27g）のため削除しました。
> 現在はRust標準のデフォルトアロケータ（dlmalloc）を使用しています。
> 経緯は [ADR-004](../ADR/004-drop-wee-alloc.md) を参照。

## 概要
wee_alloc vs dlmalloc (デフォルト) の比較結果

## 測定方法

### ビルドコマンド
```bash
# wee_alloc有効
cargo build --release --target wasm32-unknown-unknown --features wee_alloc

# dlmalloc（デフォルト）
cargo build --release --target wasm32-unknown-unknown --no-default-features
```

### 測定項目
1. WASMファイルサイズ
2. 初回ロード時間
3. メモリ使用量（10,000オブジェクト描画時）
4. フレームレート（10,000オブジェクト描画時）

## 予想される結果

### wee_alloc
- **利点**:
  - WASMサイズ: 約4-8KB削減
  - シンプルな実装
- **欠点**:
  - やや遅いメモリ割り当て
  - マルチスレッド非対応
  - メモリ断片化の可能性

### dlmalloc（デフォルト）
- **利点**:
  - 高速なメモリ割り当て
  - 成熟した実装
  - メモリ効率が良い
- **欠点**:
  - WASMサイズが大きい

## 推奨事項

### wee_allocを選択すべきケース
- WASMサイズが最優先（140KB目標）
- メモリ割り当て頻度が低い
- シングルスレッドアプリケーション

### dlmallocを維持すべきケース
- パフォーマンスが最優先
- 頻繁なメモリ割り当て/解放
- 長時間動作するアプリケーション

## 現在の設定
wee_allocは削除済み。Rust標準のデフォルトアロケータ（dlmalloc）を使用：
```toml
[features]
default = []
```

## ベンチマーク実行方法
```bash
# サイズ比較
dx build --platform web --release
ls -lh target/dx/*/release/web/public/assets/*.wasm

# パフォーマンステスト
# 1. dx serveで起動
# 2. /map/webglへアクセス
# 3. Chrome DevToolsでメモリとパフォーマンスを測定
```

## 結論
POC期間中は140KB目標のためwee_allocを採用していたが、
メンテナンス終了に伴い削除しdlmallocへ回帰した
（生バイナリで +7,010 bytes / +0.51%、`dx bundle` 後で +6,576 bytes / +1.09%）。
サイズバジェット640KBに対して余裕があり、実用上の影響はない。