# Dioxus SPAアーキテクチャ設計 - MPA的アプローチによる課題解決

## 概要

Leaflet WebGL Hybrid POCプロジェクトで、Dioxus v0.6を使用したSPA開発において遭遇した課題と、それを解決するために採用した「MPA的アプローチ」について解説します。この設計パターンは、フレームワークの制約を回避しながら高性能なWebアプリケーションを構築する実践的な手法です。

## 1. Dioxusの課題と発見

### use_effectの依存性追跡問題

```rust
// 期待通りに動作しないパターン
use_effect(move || {
    // marker_countの変更を検知できない
    let count = marker_count();
    update_markers(count);
});

// 回避策：明示的な依存配列
use_effect(
    move || {
        update_markers(marker_count());
    },
    [marker_count], // 明示的な依存性
);
```

### 状態管理の複雑性

- グローバル状態とローカル状態の同期が困難
- 複雑なコンポーネント間通信でバグが発生しやすい
- WebGLとDOMの状態同期で一貫性を保つのが難しい

## 2. MPA的アプローチの採用

### ルートベースの機能分離

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home,
    
    #[route("/map")]
    MapMode,      // Leaflet地図モード
    
    #[route("/game")]
    GameMode,     // ゲームプレイモード
    
    #[route("/chaos")]
    ChaosMode,    // WebGLカオスモード
}
```

**メリット**:
- 各ルートが独立した状態管理
- 機能ごとのコード分割が自然に実現
- 複雑な依存関係を回避

## 3. コンポーネント設計パターン

### 機能単位でのコンポーネント分割

```rust
// ルートコンポーネント
#[component]
fn MapMode() -> Element {
    // 地図専用の状態管理
    let map_state = use_signal(|| MapState::default());
    
    rsx! {
        div { class: "map-container",
            LeafletMap { state: map_state }
            MapControls { state: map_state }
        }
    }
}

// 独立したWebGLコンポーネント
#[component]
fn ChaosMode() -> Element {
    // WebGL専用の状態管理
    let chaos_state = use_signal(|| ChaosState::default());
    
    rsx! {
        div { class: "chaos-container",
            WebGLCanvas { state: chaos_state }
            ChaosControls { state: chaos_state }
        }
    }
}
```

## 4. 状態管理の最適化

### ローカル状態優先の設計

```rust
// グローバル状態は最小限に
#[derive(Clone)]
struct GlobalState {
    user_id: String,
    auth_token: String,
}

// 機能ごとにローカル状態を管理
#[derive(Default)]
struct MapState {
    markers: Vec<Marker>,
    viewport: Viewport,
    render_mode: RenderMode,
}

#[derive(Default)]
struct ChaosState {
    events: SmallVec<[ChaosEvent; 128]>,
    intensity: f32,
    active: bool,
}
```

## 5. 外部ライブラリとの統合

### JavaScript相互運用の設計

```rust
// Leaflet統合
#[wasm_bindgen]
extern "C" {
    fn initializeLeafletMap(container_id: &str) -> JsValue;
    fn updateMarkers(map: &JsValue, markers: &JsValue);
}

// Pixi.js統合
#[wasm_bindgen]
extern "C" {
    fn createPixiApp(config: &JsValue) -> JsValue;
    fn renderWebGLMarkers(app: &JsValue, data: &JsValue);
}
```

### ライフサイクル管理

```rust
#[component]
fn LeafletMap(state: Signal<MapState>) -> Element {
    let map_ref = use_signal(|| None::<JsValue>);
    
    // マウント時の初期化
    use_effect(move || {
        let map = initializeLeafletMap("map-container");
        map_ref.set(Some(map));
        
        // クリーンアップ
        move || {
            if let Some(map) = map_ref() {
                destroyMap(&map);
            }
        }
    });
    
    // 状態変更時の更新
    use_effect(
        move || {
            if let Some(map) = map_ref() {
                updateMarkers(&map, &state().markers);
            }
        },
        [state],
    );
    
    rsx! {
        div { id: "map-container", class: "w-full h-full" }
    }
}
```

## 6. パフォーマンス考慮事項

### メモ化と最適化

```rust
// 重い計算のメモ化
let filtered_markers = use_memo(
    move || {
        state().markers
            .iter()
            .filter(|m| m.in_viewport(&viewport))
            .cloned()
            .collect::<Vec<_>>()
    },
    [state, viewport],
);

// コンポーネントの条件付きレンダリング
rsx! {
    if state().render_mode == RenderMode::WebGL {
        WebGLRenderer { markers: filtered_markers }
    } else {
        DOMRenderer { markers: filtered_markers }
    }
}
```

## 7. 実装上の教訓

### Do's（推奨事項）

1. **機能単位でのルート分割**: 複雑な状態管理を避ける
2. **ローカル状態の活用**: グローバル状態は認証情報など最小限に
3. **明示的な依存性管理**: use_effectには必ず依存配列を指定
4. **早期のアーキテクチャ決定**: 後からの変更は困難

### Don'ts（避けるべき事項）

1. **過度な状態共有**: コンポーネント間の密結合を避ける
2. **暗黙的な副作用**: 予測可能な動作を心がける
3. **巨大なコンポーネント**: 責務を明確に分離
4. **フレームワークとの戦い**: 制約は受け入れて回避策を探る

## まとめ

Dioxus v0.6でのSPA開発において、MPA的アプローチを採用することで：

- フレームワークの制約を回避
- 保守性の高いコード構造を実現
- パフォーマンスの最適化が容易
- 外部ライブラリとの統合がスムーズ

このアーキテクチャパターンは、他のSPAフレームワークでも応用可能な普遍的な設計手法です。特に、複雑な状態管理や外部ライブラリとの統合が必要なプロジェクトにおいて有効です。