# Leaflet WebGL Hybrid POC ドキュメント構造

このディレクトリには、Leaflet WebGL Hybrid POCプロジェクトのすべてのドキュメントが整理されています。

## 📁 ディレクトリ構造

```
docs/
├── README.md                      # このファイル
├── POC.md                        # POC概要と達成基準
│
├── articles/                     # 技術記事とナレッジベース
│   ├── index.md                  # 記事一覧
│   ├── wasm-optimization-techniques.md
│   ├── webgl-performance-optimization.md
│   ├── dioxus-spa-architecture.md
│   ├── e2e-testing-with-playwright.md
│   ├── development-tips.md       # 開発小ネタ集
│   └── source/                   # POC作業記録（アーカイブ）
│       ├── week1-rendering-benchmark.md
│       ├── week2-wasm-chaos.md
│       ├── week3-mini-integration.md
│       └── integration-test-report.md
│
├── design/                       # 技術設計ドキュメント
│   ├── アーキテクチャ.md
│   ├── レンダリング最適化.md
│   └── 技術設計書.md
│
├── tasks/                        # タスク管理
│   ├── poc-task-summary.md       # POCタスクサマリー
│   └── post-poc-tasks.md        # POC後の開発タスク
│
└── reports/                      # 技術レポート（参考資料）
    ├── poc-results-report.md     # POC最終結果
    ├── performance-comparison-report.md
    ├── wasm-optimization-report.md
    └── rust-dioxus-evaluation.md

```

## 📖 ドキュメントの種類

### 1. 技術記事 (`articles/`)
POCで得られた技術的知見を整理した記事群。再利用可能な技術パターンやベストプラクティスをまとめています。

### 2. 設計文書 (`design/`)
システム設計、技術アーキテクチャ、パフォーマンス最適化など、プロジェクトの基本設計に関するドキュメント。

### 3. タスク管理 (`tasks/`)
開発タスクの管理と進捗追跡のためのドキュメント。

### 4. 技術レポート (`reports/`)
POC実施時の詳細な技術評価レポートや測定結果。参考資料として保存。

## 🔍 主要ドキュメントへのクイックアクセス

- **POC概要**: [POC.md](./POC.md)
- **技術記事一覧**: [articles/index.md](./articles/index.md)
- **アーキテクチャ**: [design/アーキテクチャ.md](./design/アーキテクチャ.md)
- **POC最終結果**: [reports/poc-results-report.md](./reports/poc-results-report.md)
- **今後のタスク**: [tasks/post-poc-tasks.md](./tasks/post-poc-tasks.md)