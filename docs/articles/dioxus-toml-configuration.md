# Dioxus.toml設定ガイド

## 概要

Dioxus.tomlは、Dioxus CLIプロジェクトの中心的な設定ファイルです。ビルド設定、リソース管理、デプロイメント設定などを一元管理します。

## 基本構造

```toml
[project]          # プロジェクト情報
[web.app]          # Webアプリ設定
[web.watcher]      # 開発サーバー設定
[web.resource]     # 外部リソース設定
[[hooks.build]]    # ビルドフック
[serve.proxy]      # プロキシ設定
```

## 詳細設定

### [project] セクション

プロジェクトの基本情報を定義します。

```toml
[project]
name = "leaflet-webgl-hybrid-poc"
authors = ["ebiyy"]
```

### [web.app] セクション

Webアプリケーション固有の設定を行います。

```toml
[web.app]
base_path = "leaflet-webgl-hybrid-poc"  # サブディレクトリデプロイ用
title = "Leaflet WebGL Hybrid POC"      # HTMLタイトル
```

#### base_pathの重要性
- GitHub Pagesのサブディレクトリデプロイ時に必須
- `/username.github.io/repo-name/`形式のURLに対応
- ローカル開発でも同じパスが適用される

### [web.watcher] セクション

開発サーバーの動作を制御します。

```toml
[web.watcher]
watch_path = ["src", "public"]  # 監視対象ディレクトリ
reload_html = true              # HTMLの自動リロード
index_on_404 = true             # SPA用404フォールバック
```

### [web.resource] セクション

外部リソースの読み込みを管理します。開発と本番で異なる設定が可能です。

```toml
[web.resource]
# 開発環境用
dev = [
    { rel = "script", src = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" },
    { rel = "script", src = "https://cdn.jsdelivr.net/npm/pixi.js@8.6.5/dist/pixi.min.js" },
    { rel = "stylesheet", href = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" },
    { rel = "stylesheet", href = "./assets/tailwind-generated.css" }
]

# 本番環境用（省略時はdevと同じ）
release = [
    # 本番用に最適化されたCDNやバンドルを指定可能
]
```

#### リソースタイプ
- `rel = "script"`: JavaScriptファイル
- `rel = "stylesheet"`: CSSファイル
- `rel = "modulepreload"`: ESモジュールのプリロード
- `rel = "manifest"`: PWAマニフェスト

### [[hooks.build]] セクション

ビルドプロセスにカスタムコマンドを追加します。

```toml
[[hooks.build]]
stage = "pre"           # 実行タイミング（pre/post）
command = "npm"         # 実行コマンド
args = ["run", "build-css"]  # コマンド引数

# 複数のフックを定義可能
[[hooks.build]]
stage = "post"
command = "cp"
args = ["public/service-worker.js", "target/dx/leaflet-webgl-hybrid-poc/release/web/public/"]
```

#### ステージの種類
- `pre`: ビルド開始前に実行
- `post`: ビルド完了後に実行

### [serve.proxy] セクション

開発サーバーのプロキシ設定を行います。

```toml
[[serve.proxy]]
backend = "http://localhost:3000/api"  # プロキシ先
path = "/api"                          # プロキシパス
```

## 実際の設定例

本プロジェクトのDioxus.toml全体：

```toml
[project]
name = "leaflet-webgl-hybrid-poc"
authors = ["ebiyy"]

[web.app]
base_path = "leaflet-webgl-hybrid-poc"
title = "Leaflet WebGL Hybrid POC"

[web.watcher]
watch_path = ["src", "public"]
reload_html = true
index_on_404 = true

[[hooks.build]]
stage = "pre"
command = "npm"
args = ["run", "build-css"]

[[hooks.build]]
stage = "pre"
command = "sh"
args = ["-c", "cp src/tailwind-generated.css public/tailwind-generated.css 2>/dev/null || echo 'tailwind-generated.css not found yet'"]

[web.resource]
dev = [
    { rel = "script", src = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" },
    { rel = "script", src = "https://cdn.jsdelivr.net/npm/pixi.js@8.6.5/dist/pixi.min.js" },
    { rel = "script", src = "https://unpkg.com/leaflet@1.9.4/dist/leaflet-canvas-layer.js" },
    { rel = "stylesheet", href = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" },
    { rel = "stylesheet", href = "./assets/style.css" },
    { rel = "stylesheet", href = "./assets/tailwind-generated.css" }
]
release = [
    { rel = "script", src = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" },
    { rel = "script", src = "https://cdn.jsdelivr.net/npm/pixi.js@8.6.5/dist/pixi.min.js" },
    { rel = "script", src = "https://unpkg.com/leaflet@1.9.4/dist/leaflet-canvas-layer.js" },
    { rel = "stylesheet", href = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" },
    { rel = "stylesheet", href = "./assets/style.css" },
    { rel = "stylesheet", href = "./assets/tailwind-generated.css" }
]
```

## Tips & ベストプラクティス

### 1. パス指定の注意点
- 相対パスは`./`で始める
- assetsは`public/`ディレクトリ内に配置
- ビルド出力は`target/dx/{project-name}/release/web/public/`

### 2. CDNリソースの管理
- 開発時は高速なCDNを使用
- 本番では信頼性の高いCDNまたはセルフホスティング
- バージョンは固定して予期しない変更を防ぐ

### 3. ビルドフックの活用
- Tailwind CSSのビルドなど、前処理が必要なタスクに使用
- エラーハンドリングを含めると安定性が向上
- 並列実行可能なタスクは分割して高速化

### 4. 環境別設定
```toml
# 開発環境専用の設定
[web.resource.dev]
# ソースマップ付きライブラリなど

# 本番環境専用の設定  
[web.resource.release]
# 最小化されたライブラリなど
```

## トラブルシューティング

### よくある問題

1. **リソースが読み込まれない**
   - パスが正しいか確認（`./assets/`プレフィックス）
   - ファイルが`public/`ディレクトリに存在するか確認

2. **ビルドフックが実行されない**
   - コマンドがPATHに存在するか確認
   - 引数の形式が正しいか確認

3. **base_pathが反映されない**
   - ブラウザキャッシュをクリア
   - `dx clean`でビルドキャッシュをクリア

## まとめ

Dioxus.tomlは、Trunkと比較して以下の利点があります：

- ✅ より直感的な設定構造
- ✅ base_pathによるサブディレクトリデプロイの簡易化
- ✅ リソース管理の一元化
- ✅ 環境別設定の柔軟性
- ✅ ビルドフックの統合

これらの機能により、複雑なWebアプリケーションの設定管理が大幅に簡素化されます。