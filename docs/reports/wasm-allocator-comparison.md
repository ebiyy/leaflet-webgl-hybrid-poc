# WASMアロケータ比較レポート

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
Cargo.tomlでwee_allocをデフォルトで有効化済み：
```toml
[features]
default = ["wee_alloc"]
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
POCの目標（140KB）達成のため、現在はwee_allocを採用。
本番環境では要再評価。