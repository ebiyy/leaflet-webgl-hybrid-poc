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
- **ビルドツール**: Trunk 0.21.14
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
- **出力**: `dist/tailwind.css` (自動生成、git ignore推奨)
- **ビルド**: Trunk pre_buildフックで自動実行
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
2. **WASM最適化**: wasm-opt、wasm-snip使用
3. **Service Worker**: オフライン対応とキャッシュ戦略
4. **コード分割**: 必要に応じて実装

## よく使うコマンド

```bash
# 開発サーバー起動 (ホットリロード対応)
trunk serve

# Tailwind CSS ビルド
npm run build-css  # 本番用 (minified)
npm run watch-css  # 開発用 (watch mode)

# リリースビルド
trunk build --release

# WASMサイズ最適化ビルド
./scripts/build-optimized.sh

# テスト実行
cargo test
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
- `Trunk.toml` - ビルド設定（Tailwind pre_buildフック含む）
- `tailwind.config.js` - Tailwind設定
- `package.json` - Node.js依存関係（Tailwind CLI）
- `mise.toml` - 開発環境設定

## クイックリファレンス

### エラー対処
- `tailwindcss: command not found` → `npm install`実行
- Trunk serve失敗 → `trunk clean && trunk build`
- WASM関連エラー → `mise use rust@latest`確認

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
- `trunk serve`でローカルサーバーを起動してから実行
- `http://localhost:8080`でアクセス
- スナップショットで要素のref属性を確認してから操作

### 本番デプロイ準備
- GitHub Actions設定済み（`.github/workflows/`）
- Cloudflare Pages対応
- Service Worker実装済み