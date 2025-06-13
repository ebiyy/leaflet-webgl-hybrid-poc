# タスク: ゲーム開発対応の統合実装

## 背景
これまでのリファクタリングタスクを統合し、実際のゲーム開発で使えるフレームワークとして完成させる。

## 目標
このPOCをベースに、実際のゲーム（RPG、ストラテジー、シミュレーション等）が開発できる状態にする。

## 統合タスク

### 1. ゲームループの実装
```rust
// src/game/game_loop.rs
pub struct GameLoop {
    target_fps: f32,
    accumulator: f32,
    fixed_timestep: f32,
}

impl GameLoop {
    pub fn run<F, U, R>(&mut self, 
        mut fixed_update: F,
        mut update: U,
        mut render: R,
    ) where
        F: FnMut(f32),
        U: FnMut(f32),
        R: FnMut(f32),
    {
        let delta = self.get_delta_time();
        self.accumulator += delta;
        
        // 固定タイムステップで物理更新
        while self.accumulator >= self.fixed_timestep {
            fixed_update(self.fixed_timestep);
            self.accumulator -= self.fixed_timestep;
        }
        
        // 可変タイムステップで描画更新
        update(delta);
        
        // 補間値を使ってレンダリング
        let interpolation = self.accumulator / self.fixed_timestep;
        render(interpolation);
    }
}
```

### 2. ECS (Entity Component System) 統合
```rust
// src/game/ecs.rs
use hecs::{World, Entity};

pub struct GameWorld {
    world: World,
    systems: Vec<Box<dyn System>>,
}

pub trait System {
    fn update(&mut self, world: &mut World, delta: f32);
}

// 使用例：移動システム
pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, delta: f32) {
        for (entity, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}
```

### 3. ゲーム固有のマップレイヤー
```rust
// src/game/layers.rs
pub enum GameLayer {
    Terrain,      // 地形レイヤー
    Objects,      // 建物・障害物
    Units,        // ユニット・キャラクター
    Effects,      // エフェクト・パーティクル
    UI,           // UI要素
    Debug,        // デバッグ表示
}

#[component]
pub fn GameMapView(world: Signal<GameWorld>) -> Element {
    let config = use_map_config();
    let layer_visibility = use_signal(|| HashMap::new());
    
    rsx! {
        div { class: "game-map-container",
            // ベースマップ
            GameMap {
                config: config.read().clone(),
                header: rsx!(GameHeader { world: world }),
                overlay: rsx!(GameOverlay { world: world }),
                controls: rsx!(GameControls { world: world }),
            }
            
            // レイヤーコントロール
            LayerControls { 
                visibility: layer_visibility,
                on_toggle: move |layer| {
                    layer_visibility.write()
                        .entry(layer)
                        .and_modify(|v| *v = !*v)
                        .or_insert(true);
                }
            }
        }
    }
}
```

### 4. イベントシステム
```rust
// src/game/events.rs
#[derive(Clone, Debug)]
pub enum GameEvent {
    UnitMoved { entity: Entity, from: Point, to: Point },
    UnitAttacked { attacker: Entity, target: Entity, damage: f32 },
    BuildingConstructed { entity: Entity, building_type: BuildingType },
    ResourceCollected { entity: Entity, resource: Resource, amount: u32 },
}

pub fn use_game_events() -> (Signal<Vec<GameEvent>>, EventDispatcher) {
    let events = use_signal(|| Vec::new());
    let listeners = use_signal(|| HashMap::<TypeId, Vec<Box<dyn Fn(&GameEvent)>>>::new());
    
    let dispatcher = EventDispatcher {
        emit: move |event| {
            events.write().push(event.clone());
            // リスナーに通知
        },
        on: move |event_type, handler| {
            // リスナー登録
        },
    };
    
    (events, dispatcher)
}
```

### 5. セーブ/ロードシステム
```rust
// src/game/save_system.rs
#[derive(Serialize, Deserialize)]
pub struct SaveGame {
    version: String,
    timestamp: f64,
    world_state: WorldState,
    player_data: PlayerData,
    map_data: MapData,
}

pub fn use_save_system() -> SaveSystem {
    SaveSystem {
        save: move |slot: &str| {
            let world = use_game_world();
            let save_data = SaveGame {
                version: env!("CARGO_PKG_VERSION").to_string(),
                timestamp: js_sys::Date::now(),
                world_state: world.serialize(),
                // ...
            };
            
            // IndexedDBに保存
            indexed_db::save(slot, &save_data).await
        },
        load: move |slot: &str| {
            indexed_db::load::<SaveGame>(slot).await
        },
    }
}
```

### 6. 実装例：簡単なRTS
```rust
// examples/rts_game.rs
fn main() {
    dioxus::launch(rts_app);
}

fn rts_app() -> Element {
    let world = use_game_world();
    let selected_units = use_signal(|| Vec::<Entity>::new());
    
    // ゲームループ
    use_game_loop(60.0, move |delta| {
        world.write().update(delta);
    });
    
    rsx! {
        div { class: "rts-game",
            // ゲームマップ
            GameMapView { world: world }
            
            // UI要素
            div { class: "game-ui",
                ResourceBar { world: world }
                MiniMap { world: world }
                UnitPanel { 
                    selected: selected_units,
                    world: world,
                }
                BuildMenu { world: world }
            }
        }
    }
}
```

## 実装優先順位
1. [ ] ゲームループ（必須）
2. [ ] ECS統合（ゲーム規模による）
3. [ ] イベントシステム（必須）
4. [ ] レイヤーシステム（推奨）
5. [ ] セーブ/ロード（ゲーム種別による）

## パフォーマンス目標
- 1,000ユニットのRTS @ 60FPS
- 10,000タイルのタイルマップ @ 60FPS  
- 100個の同時エフェクト @ 60FPS
- メモリ使用量 < 200MB

## 成果物
- [ ] ゲームフレームワークのドキュメント
- [ ] RTSデモ
- [ ] RPGデモ
- [ ] パフォーマンスベンチマーク
- [ ] 開発者向けチュートリアル

## 完了基準
- [ ] 2つ以上のゲームジャンルでデモ作成
- [ ] 全プラットフォームで60FPS達成
- [ ] 開発者が1日でプロトタイプ作成可能