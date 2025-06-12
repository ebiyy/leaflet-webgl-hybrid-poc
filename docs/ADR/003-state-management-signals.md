# ADR-003: 状態管理とシグナルの使用

**日付**: 2025-06-12  
**ステータス**: 承認済み  
**決定者**: Leaflet WebGL Hybrid POC開発チーム  

## コンテキスト

Dioxusアプリケーションにおいて、コンポーネント間の状態共有と動的な値の更新を効率的に行う必要がある。特にオブジェクト数の動的変更など、リアクティブな更新が求められる。

## 問題

初期実装では、`use_effect`の依存性追跡が期待通りに動作せず、プロパティ変更時の再レンダリングが発生しない問題があった。

```rust
// 問題のあるコード
use_effect(move || {
    // object_countの変更が検知されない
    if object_count > 0 {
        update_markers(object_count);
    }
});
```

## 検討した選択肢

### 1. 手動での依存性管理
- **内容**: 明示的に変更を追跡
- **評価**: ❌ エラーが発生しやすい

### 2. use_reactive!マクロの使用
- **内容**: Dioxusの`use_reactive!`マクロで依存性を自動追跡
- **評価**: ✅ 採用

### 3. グローバル状態管理
- **内容**: Reduxパターンの実装
- **評価**: ❌ オーバーエンジニアリング

## 決定

`use_reactive!`マクロを使用して依存性を明示的に宣言する。

```rust
// 修正後のコード
use_effect(use_reactive!(|object_count| {
    // object_countの変更が正しく検知される
    if object_count > 0 {
        update_markers(object_count);
    }
}));
```

## 理由

1. **確実な動作**
   - 依存性が明示的に追跡される
   - プロパティ変更時の再実行が保証される

2. **Dioxus標準機能**
   - フレームワークが提供する標準的な解決策
   - 追加の依存関係不要

3. **シンプルな実装**
   - マクロを追加するだけ
   - 既存のコード構造を維持

## 結果

- スライダー操作によるオブジェクト数変更が即座に反映
- すべてのレンダリングモードで正常動作
- コードの可読性も維持

## ガイドライン

1. **プロパティを監視する`use_effect`では必ず`use_reactive!`を使用**
2. **シグナルは`mut`で宣言**（`let mut signal = use_signal(...)`）
3. **コンポーネントのアンマウント時のクリーンアップに注意**

## 参考資料

- [Dioxus Hooks Documentation](https://dioxuslabs.com/docs/0.4/guide/en/interactivity/hooks.html)
- 実装例: [map.rs](../../src/components/map.rs)
- 実装例: [webgl_map.rs](../../src/components/webgl_map.rs)