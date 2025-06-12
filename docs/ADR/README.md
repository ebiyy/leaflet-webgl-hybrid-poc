# Architecture Decision Records (ADR)

このディレクトリには、Leaflet WebGL Hybrid POCプロジェクトにおける重要な技術的決定事項を記録しています。

## ADRとは

Architecture Decision Record（ADR）は、プロジェクトにおける重要な技術的決定とその理由を文書化したものです。将来の開発者（自分自身を含む）が、なぜその決定がなされたのかを理解できるようにすることが目的です。

## ADR一覧

| 番号 | タイトル | ステータス | 日付 |
|------|---------|-----------|------|
| [001](001-map-rendering-technology.md) | 地図レンダリング技術の選定 | 承認済み | 2025-06-12 |
| [002](002-wasm-optimization-strategy.md) | WASM最適化戦略 | 承認済み | 2025-06-12 |
| [003](003-state-management-signals.md) | 状態管理とシグナルの使用 | 承認済み | 2025-06-12 |

## ADRのフォーマット

各ADRは以下の構成で記述されています：

- **タイトル**: ADR-XXX: 決定内容の簡潔な説明
- **日付**: 決定日
- **ステータス**: 提案中/承認済み/廃止/置換
- **コンテキスト**: 決定が必要になった背景
- **検討した選択肢**: 評価した各オプション
- **決定**: 採用した選択肢
- **理由**: なぜその選択肢を選んだか
- **結果**: 実装後の成果や影響

## 新しいADRの追加

新しい技術的決定を行った場合は、次の番号でADRを作成してください：

```bash
# 例: 004-authentication-strategy.md
```

## 参考リンク

- [POCタスクサマリー](../tasks/poc-task-summary.md)
- [POC結果レポート](../reports/poc-results-report.md)