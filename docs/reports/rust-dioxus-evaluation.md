# Rust/Dioxus 評価レポート

## 概要

Leaflet WebGL Hybrid POCを通じて得られたRust/Dioxusの実践的な評価をまとめる。パフォーマンス重視のWebアプリケーション開発における利点と課題を整理。

## パフォーマンス評価

### 実測値
- **WASMバンドルサイズ**: 最適化前でも実用的なサイズ
- **実行時性能**: 10,000オブジェクト@75FPS（Canvas経由）
- **メモリ効率**: JavaScriptと比較して予測可能なメモリ使用
- **起動時間**: 初回ロードは若干重いが、その後は高速

### ベンチマーク結果
```
レンダリングモード比較（10,000オブジェクト）:
- React + JavaScript: 実装困難
- Dioxus + WASM: 75 FPS達成
```

## 開発体験

### 良い点

1. **型安全性**
   - コンパイル時にエラーを検出
   - JavaScriptとの境界でも型情報を維持
   - リファクタリングが安全

2. **開発ツール**
   - Trunkの自動リロードが高速（通常1-2秒）
   - コンパイルエラーが明確で解決しやすい
   - VS Code + rust-analyzerの組み合わせが優秀

3. **構文**
   ```rust
   rsx! {
       div { class: "container",
           h1 { "Hello, World!" }
           button { onclick: move |_| count.set(count() + 1),
               "Count: {count}"
           }
       }
   }
   ```
   React JSXに慣れていれば直感的

### 課題点

1. **JavaScript連携**
   ```rust
   // 現状: 原始的なeval使用
   let _ = js_sys::eval(&format!("console.log('{}')", message));
   
   // 理想: 型安全なブリッジ
   js_bridge::console::log(&message);
   ```

2. **既存ライブラリ統合**
   - Leaflet.jsのような大規模ライブラリとの統合が複雑
   - npm エコシステムの恩恵を受けにくい

3. **リアクティビティの理解**
   ```rust
   // 間違いやすい例
   use_effect(move || {
       // propsの変更を検知しない
   });
   
   // 正しい使い方
   use_reactive(&props, move |props| {
       // propsの変更を検知
   });
   ```

## プロダクション導入への提案

### 1. カスタムJSブリッジの構築

```rust
// js-bridge/src/lib.rs
pub mod leaflet {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen]
    extern "C" {
        pub type Map;
        
        #[wasm_bindgen(method)]
        pub fn setView(this: &Map, lat: f64, lng: f64, zoom: i32);
        
        #[wasm_bindgen(method)]
        pub fn addMarker(this: &Map, lat: f64, lng: f64) -> Marker;
    }
}

// 使用例
let map = leaflet::Map::new("map-container");
map.setView(35.6762, 139.6503, 13);
```

### 2. ハイブリッドアーキテクチャ

```
┌─────────────────────────────────────┐
│         UI Layer (React/Vue)         │
├─────────────────────────────────────┤
│    Bridge Layer (TypeScript)        │
├─────────────────────────────────────┤
│  Core Logic (Rust/WASM)             │
│  - Heavy computation                │
│  - Game state management            │
│  - Performance critical parts       │
└─────────────────────────────────────┘
```

### 3. 段階的導入戦略

**Phase 1: パフォーマンスクリティカルな部分のみ**
- ゲームエンジンのコア部分
- 物理演算
- 大量データ処理

**Phase 2: 状態管理の統合**
- Rust側でゲームステート管理
- JSへの効率的なステート同期

**Phase 3: UI層の段階的移行**
- 重要なUIコンポーネントから順次移行
- 既存JSコードとの共存

### 4. 開発環境の整備

```toml
# Cargo.toml
[workspace]
members = [
    "core",        # Rustコアロジック
    "js-bridge",   # JS連携層
    "web-app",     # Dioxusアプリ
]

[profile.release]
opt-level = "z"     # サイズ最適化
lto = true          # Link Time Optimization
codegen-units = 1   # 単一コンパイルユニット
```

### 5. CI/CDパイプライン

```yaml
# .github/workflows/deploy.yml
- name: Build WASM
  run: |
    wasm-pack build --target web
    wasm-opt -Oz -o output.wasm input.wasm
    
- name: Test Integration
  run: |
    npm run test:integration
    cargo test --target wasm32-unknown-unknown
```

## エコシステムの成熟度

### 現状（2025年6月）
- **Dioxus**: v0.5系で基本機能は安定
- **wasm-bindgen**: 成熟、プロダクション実績多数
- **web-sys**: ほぼ全Web APIをカバー

### 不足している部分
- UIコンポーネントライブラリ
- フォームバリデーションライブラリ
- 国際化（i18n）サポート
- アニメーションライブラリ

## 推奨事項

### 採用すべきケース
1. パフォーマンスが最重要な場合
2. 複雑な計算処理が必要な場合
3. 型安全性を重視する場合
4. チームにRust経験者がいる場合

### 避けるべきケース
1. 迅速なプロトタイピングが必要
2. 大量のサードパーティJSライブラリに依存
3. SEOが重要（SSRサポートが限定的）
4. チームのRust習熟度が低い

## まとめ

Rust/Dioxusは、パフォーマンスクリティカルなWebアプリケーションには優れた選択肢。ただし、プロダクション投入には以下が必要：

1. **カスタムJSブリッジ層の構築**
2. **ハイブリッドアーキテクチャの採用**
3. **段階的な導入アプローチ**
4. **チームのRustスキル向上**

Leaflet WebGL Hybrid POCのような高性能レンダリングアプリケーションでは、コアエンジンをRust/WASMで、UI層を既存のJSフレームワークで構築するハイブリッドアプローチが現実的。