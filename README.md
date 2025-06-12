# Leaflet WebGL Hybrid POC

[![WASM Size](https://img.shields.io/badge/WASM%20Size-140KB-brightgreen)](https://github.com/ebiyy/leaflet-webgl-hybrid-poc)
[![Performance](https://img.shields.io/badge/10k%20Objects-75FPS-brightgreen)](https://github.com/ebiyy/leaflet-webgl-hybrid-poc)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🎯 概要

Leaflet.jsとWebGLを組み合わせたハイブリッドレンダリングの技術検証プロジェクトです。大量のオブジェクトを高速に描画する手法を検証します。

### 主な検証項目

- 🚀 **超軽量**: 140KB (Brotli圧縮) のWASMサイズ
- ⚡ **高性能**: 10,000オブジェクトを75FPSで描画
- 🦀 **Rust製**: DioxusフレームワークによるWebアプリケーション
- 🎨 **ハイブリッド描画**: Leaflet.js + WebGL/Canvas切り替え

## 🛠️ 技術スタック

- **言語**: Rust
- **UIフレームワーク**: Dioxus 0.6
- **地図描画**: Leaflet.js (Canvas/WebGL)
- **ビルドツール**: Dioxus CLI 0.6.3
- **最適化**: Dioxus内蔵の最適化機能

## 🚀 クイックスタート

### 必要環境

- Rust 1.75以上
- Node.js 18以上
- mise（推奨）またはcargo

### セットアップ

```bash
# リポジトリをクローン
git clone https://github.com/ebiyy/leaflet-webgl-hybrid-poc.git
cd leaflet-webgl-hybrid-poc

# 依存関係をインストール（mise使用推奨）
mise install
mise use rust@latest
mise use cargo:dioxus-cli@0.6.3

# 開発サーバーを起動
dx serve

# ブラウザで http://localhost:8080 を開く
```

### ビルド

```bash
# 開発ビルド
dx build --platform web

# 最適化ビルド（本番用）
dx build --platform web --release

# ビルド出力は target/dx/leaflet-webgl-hybrid-poc/release/web/public/ に生成されます
```

## 📊 パフォーマンスベンチマーク

### レンダリング性能比較

| オブジェクト数 | DOM | Canvas | WebGL |
|------------|-----|--------|-------|
| 1,000 | 56 FPS | 75 FPS | 75 FPS |
| 5,000 | <20 FPS | 75 FPS | 75 FPS |
| 10,000 | フリーズ | 75 FPS | 75 FPS |

### 最適化成果

- 初期サイズ: 556KB
- 最終サイズ: 430KB（非圧縮）
- **削減率: 23%**
- Brotli圧縮後: 140KB

詳細は[最適化レポート](docs/reports/wasm-optimization-report.md)を参照。

## 🔧 開発

### プロジェクト構成

```
leaflet-webgl-hybrid-poc/
├── src/
│   ├── main.rs              # エントリーポイント
│   ├── components/          # UIコンポーネント
│   ├── routes/              # ルート定義
│   └── utils/               # ユーティリティ
├── scripts/                 # ビルドスクリプト
├── docs/                    # ドキュメント
└── e2e/                     # E2Eテスト
```

### テスト

```bash
# ユニットテスト
cargo test

# E2Eテスト（Playwright）
npm test

# ベンチマーク
dx serve
# ブラウザで http://localhost:8080/benchmark/canvas/10000 を開く
```

### デモページ

- `/` - ホーム
- `/map` - 地図表示デモ
- `/benchmark/:type/:count` - ベンチマーク（type: dom/canvas/webgl, count: オブジェクト数）
- `/chaos` - WebGLエフェクトデモ

## 📈 技術検証項目

- [ ] WebWorker化によるレンダリング最適化
- [ ] OffscreenCanvasの活用
- [ ] WebGPU対応の検討
- [ ] モバイルパフォーマンスの検証

## 🤝 コントリビューション

プルリクエスト歓迎です！以下のガイドラインに従ってください：

1. フォークしてfeatureブランチを作成
2. コミットメッセージは[Conventional Commits](https://www.conventionalcommits.org/)形式で
3. テストを追加・更新
4. サイズバジェット（200KB）を超えないこと

## 📄 ライセンス

MIT License - 詳細は[LICENSE](LICENSE)を参照。

---

<p align="center">
  <strong>🚀 高性能WebGLレンダリングの技術検証</strong>
</p>