# Leaflet WebGL Hybrid POC 開発小ネタ集

POC開発中に得られた細かなTipsやトラブルシューティング情報をまとめています。

## 🔧 開発環境系の小ネタ

### mise経由でのツール管理

```bash
# Rust/WASMツールチェーンの統一管理
mise use rust@latest
mise use cargo:dioxus-cli@0.6.3
mise use cargo:wasm-pack@latest
mise use cargo:wasm-bindgen-cli@latest
```

### wasm-packとDioxus CLIの競合回避策
- wasm-packの代わりにwasm-bindgen直接使用で競合を回避
- Dioxus CLIは内部でwasm-optを自動実行

### トークン消費削減のビルドログ制御

```bash
# エラーのみ表示
cargo build --release 2>&1 | grep -E "(error|warning|FAILED)"

# ログレベル調整
RUST_LOG=error cargo build --release

# 進捗バー無効化
CARGO_TERM_PROGRESS_WHEN=never cargo build --release
```

## 💡 具体的なエラーメッセージと対処法

### Dioxusのuse_effect依存性追跡問題
- **症状**: marker_countの変更を検知できない
- **対処**: 明示的な依存配列の指定が必要
```rust
use_effect(
    move || { /* 処理 */ },
    [marker_count], // 明示的に依存を指定
);
```

### Tailwind CSSビルドエラー
- **症状**: `dx serveでtailwind-generated.cssが見つからない`
- **原因**: pre_buildフックのタイミング問題
- **対処**: `npm install`実行後に`dx serve`

### マップモードのDOM要素待機タイムアウト
- **症状**: #map要素の表示に10秒以上かかる場合がある
- **原因**: Leaflet.jsのCDN読み込み遅延
- **対処**: ローカルバンドルまたはリトライ機構の実装

## 🧪 実験的な試みと失敗事例

### wasm-snipの効果なし
- **期待**: パニック経路とfmt系削除で3-8%削減
- **結果**: 効果なし（すでにパニック経路が最適化済み）
- **教訓**: `panic="abort"`設定で十分な場合が多い

### twiggyでの分析失敗
- Bloat分析ツールとしてtwiggyを試みたが互換性問題あり
- 代替として`wasm-opt --metrics`で基本的な分析は可能

### 高度な最適化版カオスエンジンの断念
- Dioxusの型推論問題で断念、基本実装で安定性を優先
- 複雑な型は明示的なアノテーションが必要

## 📊 詳細な測定スクリプト

### メモリ測定用JavaScriptコード

```javascript
const checkMemory = () => {
  if (performance.memory) {
    console.log({
      total: `${(performance.memory.totalJSHeapSize / 1048576).toFixed(2)} MB`,
      used: `${(performance.memory.usedJSHeapSize / 1048576).toFixed(2)} MB`,
      limit: `${(performance.memory.jsHeapSizeLimit / 1048576).toFixed(2)} MB`
    });
  }
};
setInterval(checkMemory, 30000); // 30秒ごと
```

### ブラウザ別の初回ロード時間測定結果
- Chrome: 648ms
- Firefox: 941ms  
- Safari: 882ms
- 測定方法: Performance.timingを使用、キャッシュクリア後の測定

## 🎯 その他の小ネタ

### Service Worker登録のタイミング
```javascript
// DOMContentLoadedではなくloadイベントで登録
window.addEventListener('load', () => {
  if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/sw.js');
  }
});
```

### Rust最適化ビルドの隠しフラグ
```toml
# Cargo.tomlには書けないがRUSTFLAGSで指定可能
# -C link-arg=-zstack-size=65536  # スタックサイズ調整
# -C target-cpu=generic           # 汎用CPU向け最適化
```

### デバッグ時のWASMサイズ最小化
開発中でもサイズを抑えたい場合：
```toml
[profile.dev]
opt-level = "s"  # サイズ最適化しつつデバッグ情報保持
```

## 🗺️ 地図ナビゲーション問題の修正

### 問題の症状
- 初回訪問時は地図が正常に表示される
- home→map→home→mapの順で遷移すると2回目以降地図が表示されない

### 原因
1. Leafletマップインスタンスが適切にクリーンアップされていない
2. 既存のマップインスタンスが残ったまま新しいインスタンスを作成しようとして失敗
3. window.theMapInstanceが古いDOM要素を参照し続ける

### 解決策
```rust
// 地図の初期化時に既存インスタンスを破棄
use_effect(move || {
    let js_code = r#"
        // 既存のマップインスタンスを破棄
        if (window.theMapInstance) {
            try {
                window.theMapInstance.remove();
            } catch (e) {
                console.log('Map already removed');
            }
            window.theMapInstance = null;
        }
        
        // 既存のLeafletコンテナを削除
        const container = document.getElementById('map-container');
        const existingContainer = container?.querySelector('.leaflet-container');
        if (existingContainer) {
            existingContainer.remove();
        }
        
        // 新しいマップを作成
        // ...
    "#;
});
```

### 実装のポイント
- コンポーネントマウント時に必ず既存のマップインスタンスをクリーンアップ
- DOM要素の存在確認とリトライ機構を実装
- WebGLモードではPixi.jsのリソースも適切にクリーンアップ

これらの小ネタは、実際の開発で遭遇した問題や工夫の記録です。同様のプロジェクトで参考になれば幸いです。