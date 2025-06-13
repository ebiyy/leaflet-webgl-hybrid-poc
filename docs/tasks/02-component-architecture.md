# タスク: コンポーネントアーキテクチャの再設計

## 背景
現在のコンポーネントは機能ごとに分離されているが、共通ロジックの重複やSignal/Memoの配線が複雑になりつつある。

## 現状の問題点
1. マップコンポーネント間でのコード重複
2. JSコード生成ロジックの散在
3. エラーハンドリングの不統一
4. コンポーネント間の境界が曖昧

## 目標
中粒度コンポーネント設計とトレイトベースの境界設計を導入し、ゲーム開発に耐えうるアーキテクチャを構築。

## 実装タスク

### 1. トレイトベースの抽象化
```rust
// src/traits/mod.rs
pub trait MapRenderer {
    fn initialize(&self, container_id: &str) -> Result<JsValue, MapError>;
    fn update_objects(&self, count: i32) -> Result<JsValue, MapError>;
    fn cleanup(&self) -> Result<(), MapError>;
    fn get_performance_hint(&self, count: i32) -> RenderMode;
}

pub trait MarkerAnimation {
    fn create_velocity(&self) -> Velocity;
    fn update_position(&mut self, bounds: &MapBounds);
    fn should_bounce(&self, bounds: &MapBounds) -> bool;
}
```

### 2. 共通コンポーネントの抽出
- [ ] `MapContainer` - 共通のコンテナロジック
- [ ] `MapRenderer` - レンダリング戦略パターン
- [ ] `PerformanceMonitor` - パフォーマンス計測の統一

### 3. ビルダーパターンでのコンポーネント構成
```rust
#[component]
pub fn GameMap<'a>(
    config: MapConfig,
    #[props(optional)] header: Option<Element<'a>>,
    #[props(optional)] overlay: Option<Element<'a>>,
    #[props(optional)] controls: Option<Element<'a>>,
) -> Element<'a> {
    let renderer = use_map_renderer(&config);
    
    rsx! {
        div { class: "game-map",
            if let Some(h) = header { div { class: "map-header", h } }
            MapContainer { 
                renderer: renderer,
                config: config,
            }
            if let Some(o) = overlay { div { class: "map-overlay", o } }
            if let Some(c) = controls { div { class: "map-controls", c } }
        }
    }
}
```

### 4. エラーバウンダリの実装
- [ ] `MapErrorBoundary` コンポーネント
- [ ] エラー状態の可視化
- [ ] リトライ機能

### 5. パフォーマンス最適化
- [ ] 仮想化レンダリング for 大量オブジェクト
- [ ] WebWorker対応の準備
- [ ] OffscreenCanvas統合の検討

## ゲーム開発への適用

### ユースケース例: RPGマップシステム
```rust
// 将来的な使用例
GameMap {
    config: MapConfig {
        render_mode: RenderMode::WebGL,
        object_count: 10000,  // NPCs + Items + Effects
        layers: vec![
            Layer::Terrain,
            Layer::Objects,
            Layer::Characters,
            Layer::Effects,
        ],
    },
    header: rsx!(QuestTracker { active_quests }),
    overlay: rsx!(MiniMap { player_position }),
    controls: rsx!(ActionBar { player_actions }),
}
```

## リファクタリング手順
1. [ ] トレイト定義
2. [ ] 既存コンポーネントをトレイト実装に移行
3. [ ] 共通コンポーネントの抽出
4. [ ] 統合テスト
5. [ ] パフォーマンス測定

## 完了基準
- [ ] 全マップコンポーネントがトレイトを実装
- [ ] コード重複が50%以上削減
- [ ] エラーハンドリングが統一
- [ ] ベンチマークで性能劣化なし