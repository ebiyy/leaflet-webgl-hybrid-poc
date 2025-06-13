# GitHub Pagesデプロイ問題レポート

**日付**: 2025年6月13日  
**URL**: https://ebiyy.github.io/leaflet-webgl-hybrid-poc/  
**問題**: デプロイは成功したが、サイトにアクセスしても何も表示されない（真っ黒な画面）

## 問題の詳細

GitHub Pagesへのデプロイは正常に完了し、すべてのアセットファイルは正しく配信されているにもかかわらず、ページが真っ黒な状態で表示され、アプリケーションが正しくレンダリングされない。

## 調査結果

### 1. ネットワークリクエストの状態

すべての静的アセットは正常に読み込まれている：

- `index.html` - 200 OK
- `/leaflet-webgl-hybrid-poc/assets/dioxus/leaflet-webgl-hybrid-poc.css` - 200 OK
- `/leaflet-webgl-hybrid-poc/assets/dioxus/leaflet-webgl-hybrid-poc.js` - 200 OK
- `/leaflet-webgl-hybrid-poc/assets/dioxus/leaflet-webgl-hybrid-poc_bg.wasm` - 200 OK
- `/leaflet-webgl-hybrid-poc/tailwind-generated.css` - 200 OK

パスの設定も正しく、base_path (`/leaflet-webgl-hybrid-poc/`) が適用されている。

### 2. コンソールログの分析

正常に動作している部分：
```
Starting WASM application...
Current pathname: /leaflet-webgl-hybrid-poc/
Initial load metrics captured: 634ms
Launching Dioxus app...
```

問題の兆候：
- 404エラーが1つ発生（詳細不明）
- Dioxusアプリケーションの起動後、レンダリングが行われていない

### 3. HTML/DOM構造の確認

- `<head>`タグ内にCSSファイルが正しく読み込まれている
- `<body>`タグ内に`<div id="main"></div>`が存在
- JavaScriptとWASMファイルのパスも正しく設定されている

### 4. 推測される原因

#### 4.1 Routerのbase_path問題
最も可能性が高い原因は、DioxusのRouterコンポーネントがbase_pathを正しく認識していないこと。

現在のコード：
```rust
Router::<Route> {}
```

この設定では、Routerは`/`をベースとして動作し、`/leaflet-webgl-hybrid-poc/`というパスを認識できない。

#### 4.2 初期ルートのマッチング失敗
`/leaflet-webgl-hybrid-poc/`というパスが、定義されたルート（`/`、`/about`など）にマッチしないため、何もレンダリングされない可能性がある。

## 解決策

### 1. 即時対応案

Routerコンポーネントにbase_path設定を追加：

```rust
#[component]
fn App() -> Element {
    let base_path = if cfg!(debug_assertions) {
        ""
    } else {
        "/leaflet-webgl-hybrid-poc"
    };
    
    rsx! {
        Router::<Route> {
            config: move || RouterConfig::default().base_path(base_path)
        }
    }
}
```

### 2. より堅牢な解決策

環境変数またはfeatureフラグを使用した切り替え：

```rust
#[cfg(feature = "github-pages")]
const BASE_PATH: &str = "/leaflet-webgl-hybrid-poc";

#[cfg(not(feature = "github-pages"))]
const BASE_PATH: &str = "";

// Cargo.tomlに追加
[features]
github-pages = []
```

### 3. デバッグ強化案

より詳細なデバッグ情報を追加：

```rust
web_sys::console::log_1(&format!("Router base_path: {}", BASE_PATH).into());
web_sys::console::log_1(&format!("Current location: {:?}", window().location().pathname()).into());
```

## 推奨アクションプラン

1. **即時修正**: Routerにbase_path設定を追加（解決策1）
2. **テスト**: ローカルでbase_pathを設定して動作確認
3. **CI/CD更新**: GitHub Actionsでfeatureフラグまたは環境変数を設定
4. **長期的改善**: 
   - エラーハンドリングの改善
   - 404ページの実装
   - より詳細なロギング

## 関連ファイル

- `/src/main.rs` - Routerコンポーネントの設定
- `/src/routes/mod.rs` - ルート定義
- `/Dioxus.toml` - base_path設定
- `/.github/workflows/deploy-demo.yml` - デプロイ設定

## 次のステップ

1. Routerのbase_path設定を実装
2. ローカルでの動作確認
3. 再デプロイとテスト
4. 問題が解決しない場合は、より詳細なデバッグログを追加

## 参考情報

- [Dioxus Router Documentation](https://dioxuslabs.com/learn/0.6/router)
- [GitHub Pages サブディレクトリデプロイ](https://docs.github.com/en/pages)