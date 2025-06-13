# 06. WASMサイズ最適化タスク ✅

## 目標
618KB → 140KB台 (Brotli圧縮後) の実現

## 現状分析

### サイズ構成 (twiggy分析待ち)
- 非圧縮: 618KB
- Brotli圧縮: ~200KB (推定)
- 目標: Brotli圧縮で140KB以下

### 主要な膨張要因
1. core::fmt (Debug trait)
2. panic infrastructure
3. default allocator (dlmalloc)
4. 未使用generics monomorphization
5. TypeScript型生成の残骸

## 実装計画

### Phase 1: 即効性のある最適化 (想定削減: 100-150KB) ✅

#### 1.1 wasm-opt強化 [CI統合] ✅
```bash
wasm-opt -Oz --strip-dwarf --strip-producers --vacuum \
         --strip-target-features --remove-unused-module-elements \
         -o dist/app.opt.wasm dist/app.wasm
```
- 削減効果: 30-70KB
- 実装: deploy-demo.yml修正済み

#### 1.2 パニック基盤削除 [Cargo.toml] ✅
```toml
[profile.release]
panic = "abort"
strip = true
opt-level = "z"
lto = "fat"
codegen-units = 1
```
- 削減効果: 15-25KB
- 副作用: エラーハンドリング要見直し

#### 1.3 wasm-snip/strip [CI統合] ✅
```bash
wasm-snip -o app.snip.wasm --snip-rust-fmt-code \
  --snip-rust-panicking-code dist/app.wasm
wasm-tools strip app.snip.wasm -o dist/app.final.wasm
```
- 削減効果: 5-10KB
- 対象: panic_fmt, core::fmt::*

### Phase 2: アロケータ最適化 (想定削減: 5-10KB) ✅

#### 2.1 wee_alloc評価 ✅
```toml
[features]
default = ["wee_alloc"]

[dependencies]
wee_alloc = { version = "0.4", optional = true }
```
```rust
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

#### 2.2 ベンチマーク基準 ✅
- メモリ使用量: 10,000オブジェクト時
- フレームレート: 60FPS維持確認
- 起動時間: 3秒以内
- 実装: wee_allocをデフォルト有効化済み

### Phase 3: コード最適化 (想定削減: 20-50KB)

#### 3.1 format!マクロ削除
```rust
// Before
format!("Point: {:?}", point)

// After
log::info!("Point: {},{}", point.x, point.y)
```

#### 3.2 Genericsモノモーフ化抑制
```rust
// Before
fn process<T: Into<f64>>(val: T) { ... }

// After
fn process_f64(val: f64) { ... }
fn process_i32(val: i32) { process_f64(val as f64) }
```

#### 3.3 未使用依存削除
- serde → serde-lite検討
- regex不使用確認
- getrandom → JS側委譲

### Phase 4: Nightly最適化 (想定削減: 15-30KB) ✅

#### 4.1 build-std ✅
```bash
RUSTFLAGS="-C link-arg=-zseparate-code" \
cargo +nightly build \
  -Z build-std=panic_abort,core,alloc \
  --target wasm32-unknown-unknown --release
```

#### 4.2 CI/CD対応 ✅
- GitHub Actions matrix追加
- stable/nightlyの並列ビルド
- サイズ比較自動化
- 実装: nightly-optimization.yml作成済み

### Phase 5: JavaScript側分離 (最終手段)

#### 5.1 分離候補
- 乱数生成 (chaos.rs)
- ベクトル演算の一部
- フォーマット処理

#### 5.2 実装基準
- 呼び出し頻度 < 10Hz
- 処理時間 > 1ms
- WASMバウンダリコスト考慮

## 検証項目

### サイズ測定
```bash
# 各段階でのサイズ確認
ls -lh dist/*.wasm
brotli -q 11 -k dist/app.wasm
ls -lh dist/*.wasm.br
```

### パフォーマンス測定
- [ ] 10,000オブジェクト@60FPS維持
- [ ] 初回ロード3秒以内
- [ ] メモリリーク15分テスト合格

### 互換性確認
- [ ] Chrome/Firefox/Safari動作
- [ ] モバイルブラウザ対応
- [ ] オフライン動作 (Service Worker)

## リスクと対策

### リスク
1. wee_allocでのメモリ断片化
2. panic=abortでのデバッグ困難
3. Nightly依存によるCI不安定

### 対策
1. 長時間動作テスト必須
2. development profile分離
3. stable/nightlyの自動切替

## スケジュール

1. Week 1: Phase 1-2実装 (即効性重視) ✅
2. Week 2: Phase 3-4実装 (コード最適化) - Phase 4完了✅、Phase 3進行中
3. Week 3: Phase 5検討 (必要に応じて)

## 成功基準

### 必須
- [ ] Brotli圧縮後 < 150KB
- [ ] パフォーマンス劣化なし
- [ ] 既存機能維持

### 理想
- [ ] Brotli圧縮後 < 140KB
- [ ] 起動時間改善
- [ ] メモリ使用量削減

## CI/CD統合 ✅

```yaml
# deploy-demo.yml追加分
- name: Install WASM optimization tools
  run: |
    npm install -g wasm-opt@latest
    cargo install wasm-snip wasm-tools

- name: Optimize WASM
  run: |
    # 最適化チェーン実行
    wasm-opt -Oz --strip-dwarf ... 
    wasm-snip ...
    wasm-tools strip ...
    
    # サイズレポート
    echo "### WASM Size Report" >> $GITHUB_STEP_SUMMARY
    ls -lh dist/*.wasm* | awk '{print "- "$9": "$5}' >> $GITHUB_STEP_SUMMARY
```

実装完了:
- deploy-demo.yml: 最適化ツールチェーン統合済み
- nightly-optimization.yml: 週次比較ワークフロー作成済み

## トラブルシューティング

### wee_alloc動作不良
→ feature flag無効化、dlmalloc継続

### Nightlyビルド失敗
→ MSRV固定、stable fallback

### 最適化後の動作不良
→ 段階的ロールバック、bisect