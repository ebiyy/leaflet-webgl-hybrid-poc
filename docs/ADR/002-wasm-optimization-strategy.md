# ADR-002: WASM最適化戦略

**日付**: 2025-06-12  
**ステータス**: 承認済み  
**決定者**: Leaflet WebGL Hybrid POC開発チーム  

## コンテキスト

Webアプリケーションにおいて初回ロード時間は重要なUX指標。特にモバイル環境（4G回線）では3秒以内のロードが求められる。

## 検討した選択肢

### 1. 標準ビルド設定
- **結果**: 556KB（非圧縮）
- **評価**: ❌ 最適化の余地あり

### 2. wee_alloc + 最適化設定
- **適用内容**:
  - `wee_alloc`（軽量アロケータ）
  - `opt-level = "z"`（サイズ最適化）
  - `lto = "fat"`（Link Time Optimization）
  - `codegen-units = 1`
  - `panic = "abort"`
  - `strip = true`
- **結果**: 430KB（非圧縮）、172KB（Gzip）
- **評価**: ✅ 採用

### 3. 依存関係の見直し
- **適用内容**:
  - 不要なfeatureの削除
  - `default-features = false`の活用
- **結果**: 90KB削減
- **評価**: ✅ 採用

### 4. コード分割（Code Splitting）
- **検討内容**: ルートごとに動的ロード
- **評価**: ❌ 現時点では不要（既に十分軽量）

## 決定

以下の最適化戦略を採用：

```toml
[profile.release]
opt-level = "z"     # サイズ最適化
lto = "fat"         # Link Time Optimization
codegen-units = 1   # 並列性を犠牲にして最適化
panic = "abort"     # パニックハンドラ削除
strip = true        # シンボル削除

[dependencies]
wee_alloc = { version = "0.4.5", optional = true }
```

## 理由

1. **目標を大幅に達成**
   - 初回ロード146ms（目標3000msの5%）
   - Gzipサイズ172KB（一般的なWebアプリの1/10）

2. **シンプルな実装**
   - ビルド設定の調整のみ
   - 追加の複雑性なし

3. **十分な軽量性**
   - コード分割は現時点で不要
   - 将来必要になれば検討

## 結果

- **非圧縮**: 430KB（23%削減）
- **Gzip**: 172KB
- **Brotli**: 140KB
- **初回ロード**: 146ms（ローカル環境）

## 今後の検討事項

- SharedArrayBufferによる並列処理（将来）
- WebAssembly Streamsによる段階的ロード（必要時）

## 参考資料

- [Cargo.toml](../../Cargo.toml)
- [POC Week2 レポート](../tasks/week2-wasm-chaos.md)