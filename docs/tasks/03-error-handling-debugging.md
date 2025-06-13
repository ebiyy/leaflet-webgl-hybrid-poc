# タスク: エラーハンドリングとデバッグ体験の向上

## 背景
現在の実装では`js_sys::eval`のエラーを握りつぶしており、デバッグが困難。ゲーム開発では複雑な状態遷移のデバッグが必須。

## 現状の問題点
```rust
// エラーを無視している箇所が多数
let _ = js_sys::eval(&update_code);
```

## 目標
プロダクションレベルのエラーハンドリングとデバッグツールの実装。

## 実装タスク

### 1. エラー型の定義
```rust
// src/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MapError {
    #[error("JavaScript evaluation failed: {0}")]
    JsEval(String),
    
    #[error("Map initialization failed: {0}")]
    Initialization(String),
    
    #[error("Renderer not supported: {0}")]
    UnsupportedRenderer(String),
    
    #[error("Performance threshold exceeded: {current} objects > {max} limit")]
    PerformanceLimit { current: i32, max: i32 },
}

pub type MapResult<T> = Result<T, MapError>;
```

### 2. エラー通知システム
```rust
// src/hooks/use_error_handler.rs
pub fn use_error_handler() -> (Signal<Vec<AppError>>, ErrorActions) {
    let errors = use_signal(|| Vec::new());
    
    let actions = ErrorActions {
        push: move |error| {
            errors.write().push(error);
            tracing::error!("{:?}", error);
        },
        clear: move || errors.set(Vec::new()),
        dismiss: move |id| {
            errors.write().retain(|e| e.id != id);
        },
    };
    
    (errors, actions)
}
```

### 3. デバッグオーバーレイ
```rust
#[component]
pub fn DebugOverlay() -> Element {
    let show = use_signal(|| cfg!(debug_assertions));
    let (errors, _) = use_error_handler();
    let metrics = use_performance_metrics();
    
    rsx! {
        if show() {
            div { class: "debug-overlay",
                // FPS表示
                div { class: "fps-counter",
                    "FPS: {metrics.read().fps}"
                }
                
                // Signal/Memo使用状況
                SignalDebugger {}
                
                // エラーログ
                ErrorConsole { errors: errors }
                
                // ステート履歴
                StateHistory {}
            }
        }
    }
}
```

### 4. トレーシング統合
```rust
// src/utils/tracing.rs
pub fn init_tracing() {
    use tracing_wasm::{WASMLayer, WASMLayerConfig};
    
    let config = WASMLayerConfig::default()
        .set_max_level(tracing::Level::DEBUG)
        .set_console_config(ConsoleConfig::ReportWithConsoleColor);
        
    tracing_subscriber::fmt()
        .with_writer(WASMLayer::new(config))
        .init();
}
```

### 5. 開発者ツール統合
- [ ] Chrome DevTools連携
- [ ] Redux DevTools風の状態インスペクター
- [ ] タイムトラベルデバッグ

## ゲーム開発での活用例

### バトルシステムのデバッグ
```rust
// ダメージ計算のトレース
#[instrument(skip(attacker, defender))]
pub fn calculate_damage(
    attacker: &Character,
    defender: &Character,
    skill: &Skill,
) -> DamageResult {
    tracing::debug!(
        attacker_id = %attacker.id,
        defender_id = %defender.id,
        skill_name = %skill.name,
        "Calculating damage"
    );
    
    // 計算ロジック...
    
    tracing::info!(
        damage = result.total_damage,
        critical = result.is_critical,
        "Damage calculated"
    );
}
```

### リプレイシステム
```rust
#[derive(Serialize, Deserialize)]
pub struct GameEvent {
    timestamp: f64,
    event_type: EventType,
    payload: serde_json::Value,
}

pub fn use_replay_system() -> ReplayActions {
    let events = use_signal(|| Vec::<GameEvent>::new());
    let replay_mode = use_signal(|| false);
    
    // イベント記録とリプレイ機能
}
```

## 実装手順
1. [ ] エラー型の定義
2. [ ] 既存のエラー握りつぶしを修正
3. [ ] エラー通知UIの実装
4. [ ] デバッグオーバーレイの実装
5. [ ] トレーシング統合
6. [ ] 開発者向けドキュメント作成

## 完了基準
- [ ] 全ての`Result`が適切に処理される
- [ ] エラーがUIに表示される
- [ ] デバッグモードで詳細情報が確認可能
- [ ] パフォーマンスへの影響が最小限（< 1% overhead）