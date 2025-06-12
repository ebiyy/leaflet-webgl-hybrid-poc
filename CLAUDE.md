# CLAUDE.md - Claude Code用コンテクストファイル

## 重要: このファイルの役割
これはClaude Code（claude.ai/code）専用のコンテクストファイルです。プロジェクト固有のルール、設定、技術的な決定事項を記録しています。

## プロジェクト概要

Leaflet.jsとWebGLを組み合わせたハイブリッドレンダリングの技術検証プロジェクトです。大量のオブジェクトを高速に描画する手法を検証し、WASMアプリケーションの最適化手法を探求します。

## 技術スタック

- **言語**: Rust (edition 2021)
- **UIフレームワーク**: Dioxus 0.6.3
- **スタイリング**: Tailwind CSS v4 (スタンドアロンCLI)
- **地図ライブラリ**: Leaflet.js (CDN)
- **WebGL**: Pixi.js (CDN)
- **ビルドツール**: Dioxus CLI 0.6.3
- **環境管理**: mise (Rust, Node.js等のランタイム管理)

## プロジェクト構造

```
/Users/ebiyy/ghq/github.com/ebiyy/leaflet-webgl-hybrid-poc/
├── README.md
├── CLAUDE.md (このファイル)
├── docs/
│   ├── reports/
│   └── articles/
├── src/
│   ├── components/    # レンダリングコンポーネント
│   ├── routes/        # ルーティング
│   └── utils/         # パフォーマンス計測ツール
├── public/           # 静的アセット
└── e2e/              # E2Eテスト
```

## 開発ルールとガイドライン

### 1. 環境管理の鉄則
- **必須**: すべての開発環境管理に`mise`を使用
- **禁止**: Homebrewの直接使用（GUI アプリケーションを除く）
- **推奨**: Cargoツールは`mise use cargo:<tool>@<version>`で管理

### 2. スタイリング規約 (Tailwind CSS)
- **バージョン**: Tailwind CSS v4 (スタンドアロンCLI)
- **設定ファイル**: `tailwind.config.js`
- **入力**: `src/tailwind.css`
- **出力**: `src/tailwind-generated.css` → `public/tailwind-generated.css` (ビルド時コピー)
- **ビルド**: Dioxus.tomlのpre_buildフックで自動実行
- **使用方針**: 
  - Tailwindユーティリティクラスを優先使用
  - カスタムCSSは最小限に留める
  - レスポンシブデザインを考慮

### 3. コーディング規約
- **Rustコード**: 
  - Dioxusのrsx!マクロ内でTailwindクラスを使用
  - SmallVec等でメモリ効率を重視
- **コメント**: 必要最小限、コードの意図が明確でない場合のみ
- **ファイル作成**: 既存ファイルの編集を優先、新規作成は最小限

## パフォーマンス目標と制約

### POC達成目標
- **WASMサイズ**: 446KB以内 ✅
- **初回ロード**: 3秒以内@4G ✅
- **レンダリング**: 10,000オブジェクト表示可能 ✅
- **入力遅延**: 200ms以内 ✅

### 最適化のポイント
1. **Tailwind CSS**: PurgeCSSで未使用クラスを削除（44KB）
2. **WASM最適化**: Dioxus CLIの内蔵最適化機能
3. **Service Worker**: オフライン対応とキャッシュ戦略
4. **コード分割**: 必要に応じて実装

## よく使うコマンド

```bash
# 開発サーバー起動 (ホットリロード対応)
# http://127.0.0.1:8080/ でアクセス可能
dx serve

# GitHub Pages用ビルド（base_path付き）
DX_BASE_PATH=leaflet-webgl-hybrid-poc dx build --platform web --release

# Tailwind CSS ビルド
npm run build-css  # 本番用 (minified)
npm run watch-css  # 開発用 (watch mode)

# リリースビルド
dx build --platform web --release

# テスト実行
cargo test

# ビルド出力は以下に生成
# target/dx/leaflet-webgl-hybrid-poc/release/web/public/
```

## レンダリング手法の比較

### DOM方式
- Leaflet標準のマーカー
- 1,000オブジェクトで性能限界

### Canvas方式
- Leaflet.CanvasLayerプラグイン使用
- 10,000オブジェクトまで安定

### WebGL方式
- Pixi.js統合
- 50,000オブジェクト以上も可能

## 重要な設定ファイル
- `Cargo.toml` - Rust依存関係
- `Dioxus.toml` - ビルド設定（base_path、pre_buildフック、リソース設定）
- `tailwind.config.js` - Tailwind設定
- `package.json` - Node.js依存関係（Tailwind CLI）
- `mise.toml` - 開発環境設定（cargo:dioxus-cli含む）

## クイックリファレンス

### エラー対処
- `tailwindcss: command not found` → `npm install`実行
- dx serve失敗 → `dx clean && dx build`
- WASM関連エラー → `mise use rust@latest`確認
- `dx: command not found` → `mise use cargo:dioxus-cli@0.6.3`実行

### デバッグツール
- Chrome DevTools Performance タブ
- Memory Profiler (15分連続テスト用)
- Lighthouse (パフォーマンス測定)

### MCP Playwright統合テスト
Claude Codeには`mcp__playwright__`プレフィックスでPlaywrightツールが統合されています。

#### 基本的な使い方
```bash
# ブラウザ操作の基本フロー
1. mcp__playwright__browser_navigate - URLへ移動
2. mcp__playwright__browser_snapshot - ページの状態を取得
3. mcp__playwright__browser_click - 要素をクリック
4. mcp__playwright__browser_type - テキスト入力
5. mcp__playwright__browser_take_screenshot - スクリーンショット撮影
```

#### 統合テストシナリオ例
- ホーム画面の表示確認
- ナビゲーションリンクの動作確認
- マップモードへの遷移テスト
- WebGLエフェクトの動作確認
- 15分連続動作テスト（メモリリーク検証）

#### テスト時の注意点
- `dx serve`でローカルサーバーを起動してから実行
- `http://localhost:8080`でアクセス
- GitHub Pagesでは`https://ebiyy.github.io/leaflet-webgl-hybrid-poc/`
- スナップショットで要素のref属性を確認してから操作

### 本番デプロイ準備
- GitHub Actions設定済み（`.github/workflows/`）
  - `deploy-demo.yml` - GitHub Pagesデプロイ
  - `size-budget.yml` - WASMサイズチェック
- Cloudflare Pages対応
- Service Worker実装済み

## Dioxus.toml設定のポイント

### GitHub Pagesデプロイ用設定
```toml
[web.app]
base_path = "leaflet-webgl-hybrid-poc"  # サブディレクトリデプロイ対応
```

### リソース読み込み設定
```toml
[web.resource]
dev = [
    { rel = "script", src = "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" },
    { rel = "script", src = "https://cdn.jsdelivr.net/npm/pixi.js@8.6.5/dist/pixi.min.js" },
    # ...
]
```

### ビルドフック
```toml
[[hooks.build]]
stage = "pre"
command = "npm"
args = ["run", "build-css"]
```

## マイグレーションヒストリー

### TrunkからDioxus CLIへの移行（2025/6/13）
- ビルドツールをTrunkからDioxus CLIに変更
- ビルド最適化スクリプトを削除（Dioxusが内部で実施）
- GitHub Pagesデプロイ問題をbase_path設定で解決
- CI/CDをtaiki-e/install-actionで高速化（5分→2分）

### 環境変数によるbase_path切り替え（2025/6/13）
- Dioxus.tomlからbase_path固定値を削除
- DX_BASE_PATH環境変数で動的に設定可能に
- 開発時: http://127.0.0.1:8080/ でアクセス
- 本番時: GitHub ActionsでDX_BASE_PATHを設定