# タスク: リアクティブフックのリファクタリング

## 背景
現在の実装では、Dioxus 0.6.2のフックAPIの制約により、依存配列パターンが使えず冗長な実装になっている。

## 現状の問題点
```rust
// 現在の冗長な実装
let mut count_signal = use_signal(|| object_count);
if count_signal() != object_count {
    count_signal.set(object_count);
}
use_effect(move || {
    let current_count = count_signal();
    // ...
});
```

## 目標
React風の依存配列パターンを実現するカスタムフックを実装し、コード全体で統一する。

## 実装タスク

### 1. ベースフックの実装
- [ ] `use_effect_with`フックの実装
- [ ] `use_future_with`フックの実装
- [ ] `use_memo_with`フックの実装

### 2. 実装例
```rust
// hooks/use_reactive.rs
pub fn use_effect_with<T: PartialEq + Clone + 'static>(
    dep: T, 
    mut f: impl FnMut(&T) + 'static
) {
    let sig = use_signal(|| dep.clone());
    if sig() != dep {
        sig.set(dep.clone());
    }
    use_effect(move || f(&sig()));
}

pub fn use_effect_with2<T1, T2>(
    deps: (T1, T2),
    mut f: impl FnMut(&T1, &T2) + 'static
) where
    T1: PartialEq + Clone + 'static,
    T2: PartialEq + Clone + 'static,
{
    let sig = use_signal(|| deps.clone());
    if sig() != deps {
        sig.set(deps.clone());
    }
    use_effect(move || {
        let (d1, d2) = &sig();
        f(d1, d2)
    });
}
```

### 3. 既存コードの移行
- [ ] `src/components/map.rs`
- [ ] `src/components/canvas_map.rs`
- [ ] `src/components/webgl_map.rs`
- [ ] `src/routes/map.rs`

### 4. テストの追加
- [ ] 依存変更時の再実行を確認
- [ ] 依存が同じ場合は再実行されないことを確認
- [ ] メモリリークがないことを確認

## 期待される効果
- コードの可読性向上
- React開発者にとって親しみやすいAPI
- 将来的なDioxus APIの変更への対応が容易

## ゲーム開発への影響
- 複雑な依存関係を持つゲームステートの管理が簡潔に
- パフォーマンスクリティカルな更新制御が可能
- 60FPSを維持しながらの状態管理パターンの確立

## 完了基準
- [ ] 全てのmap系コンポーネントで新フックを使用
- [ ] ドキュメントの作成
- [ ] パフォーマンス測定（旧実装との比較）