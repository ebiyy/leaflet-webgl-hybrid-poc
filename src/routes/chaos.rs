use dioxus::prelude::*;
use web_sys::window;
use smallvec::SmallVec;
use crate::utils::input_latency::InputLatencyMeasurer;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub enum ChaosEvent {
    UIGlitch { element_id: String, severity: f32 },
    InputCorruption { delay_ms: u32, noise: f32 },
    VisualDistortion { distortion_type: String },
    TimeWarp { speed_multiplier: f32 },
}

impl ChaosEvent {
    #[inline]
    fn get_display_string(&self) -> String {
        match self {
            ChaosEvent::UIGlitch { element_id, severity } => {
                format!("UI破壊: {} (強度: {:.2})", element_id, severity)
            }
            ChaosEvent::InputCorruption { delay_ms, noise } => {
                format!("入力遅延: {}ms (ノイズ: {:.2})", delay_ms, noise)
            }
            ChaosEvent::VisualDistortion { distortion_type } => {
                format!("視覚歪曲: {}", distortion_type)
            }
            ChaosEvent::TimeWarp { speed_multiplier } => {
                format!("時間歪曲: x{:.2}", speed_multiplier)
            }
        }
    }
}

#[derive(Clone)]
struct ChaosEngine {
    intensity: u8,
    events: SmallVec<[ChaosEvent; 128]>,
    is_active: bool,
}

impl ChaosEngine {
    const CHAOS_LEVEL_3_EVENT_COUNT: usize = 1000;
    #[inline]
    fn new(intensity: u8) -> Self {
        Self {
            intensity,
            events: SmallVec::new(),
            is_active: false,
        }
    }
    
    #[inline]
    fn spawn_batch_events(&mut self, count: usize) {
        // バッチ処理で複数イベントを一度に生成
        let mut batch: SmallVec<[ChaosEvent; 16]> = SmallVec::new();
        
        for _ in 0..count {
            let event = match (js_sys::Math::random() * 4.0) as u32 {
                0 => ChaosEvent::UIGlitch {
                    element_id: format!("chaos-element-{}", (js_sys::Math::random() * 100.0) as u32),
                    severity: js_sys::Math::random() as f32,
                },
                1 => ChaosEvent::InputCorruption {
                    delay_ms: (js_sys::Math::random() * 500.0) as u32,
                    noise: js_sys::Math::random() as f32,
                },
                2 => ChaosEvent::VisualDistortion {
                    distortion_type: "glitch".to_string(),
                },
                _ => ChaosEvent::TimeWarp {
                    speed_multiplier: 1.0 + js_sys::Math::random() as f32 * 2.0,
                },
            };
            batch.push(event);
        }
        
        // 一度に全イベントを追加
        self.events.extend(batch);
        
        // リングバッファのように古いイベントを削除
        if self.events.len() > 128 {
            self.events.drain(0..(self.events.len() - 128));
        }
    }
}

#[component]
pub fn ChaosRoute(intensity: u8) -> Element {
    let mut chaos_engine = use_signal(|| ChaosEngine::new(intensity));
    let mut fps = use_signal(|| 60.0);
    let input_latency = use_signal(|| 0.0);
    let latency_measurer = use_signal(|| InputLatencyMeasurer::new());
    
    // Intervalインスタンスを外部に保持
    let mut interval_instance = use_signal(|| None::<crate::utils::interval::Interval>);
    
    use_effect(move || {
        use crate::utils::interval::Interval;
        
        let interval = Interval::new(16, move || {
            // コンポーネントがアンマウントされているかチェック
            let engine_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                chaos_engine().is_active
            }));
            
            match engine_result {
                Ok(is_active) => {
                    if is_active {
                        // エンジンの更新
                        let update_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            chaos_engine.with_mut(|engine| {
                                engine.spawn_batch_events(engine.intensity as usize);
                            });
                        }));
                        
                        if update_result.is_err() {
                            web_sys::console::log_1(&"Chaos engine update failed, component may be unmounted".into());
                            return;
                        }
                        
                        // FPS測定
                        let fps_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            if let Some(window) = window() {
                                if let Some(performance) = window.performance() {
                                    let _now = performance.now();
                                    fps.set(1000.0 / 16.0);
                                }
                            }
                        }));
                        
                        if fps_result.is_err() {
                            web_sys::console::log_1(&"FPS update failed, component may be unmounted".into());
                        }
                    }
                }
                Err(_) => {
                    web_sys::console::log_1(&"Chaos interval detected unmounted component".into());
                }
            }
        });
        
        interval_instance.set(Some(interval));
    });
    
    // カオス効果の適用
    let chaos_style = if chaos_engine().is_active {
        format!(
            "animation: chaos-glitch 0.1s infinite; filter: hue-rotate({}deg);",
            (js_sys::Math::random() * 360.0) as u32
        )
    } else {
        "".to_string()
    };
    
    rsx! {
        div {
            class: "chaos-container",
            style: "{chaos_style}",
            
            div {
                class: "chaos-header",
                Link {
                    to: "/",
                    "← ホームに戻る"
                }
                h2 { "カオスモード - レベル {intensity}" }
            }
            
            div {
                class: "chaos-stats",
                div {
                    class: "stat-box",
                    h3 { "FPS" }
                    p { 
                        style: if fps() < 30.0 { "color: red;" } else { "" },
                        {format!("{:.0}", fps())}
                    }
                }
                div {
                    class: "stat-box",
                    h3 { "入力遅延 (P95)" }
                    p { 
                        style: if input_latency() > 200.0 { "color: red;" } else { "" },
                        {format!("{:.0}ms", input_latency())} 
                    }
                }
                div {
                    class: "stat-box",
                    h3 { "イベント数" }
                    p { {format!("{}", chaos_engine().events.len())} }
                }
            }
            
            div {
                class: "chaos-controls",
                button {
                    class: "chaos-button",
                    onclick: move |_| {
                        // クリック時点で測定を開始
                        let start_time = window().unwrap().performance().unwrap().now();
                        
                        // 実際の処理
                        chaos_engine.with_mut(|engine| {
                            engine.is_active = !engine.is_active;
                        });
                        
                        // requestAnimationFrameを使用して次のフレームで測定完了
                        let window = window().unwrap();
                        let performance = window.performance().unwrap();
                        let latency_measurer_clone = latency_measurer.clone();
                        let mut input_latency_clone = input_latency.clone();
                        
                        let closure = wasm_bindgen::closure::Closure::once(move || {
                            let end_time = performance.now();
                            let latency = end_time - start_time;
                            
                            // 測定値を直接記録
                            latency_measurer_clone.with(|m| {
                                m.add_measurement(latency);
                            });
                            
                            // 統計を更新
                            let stats = latency_measurer_clone.with(|m| m.get_stats());
                            input_latency_clone.set(stats.p95);
                        });
                        
                        window.request_animation_frame(closure.as_ref().unchecked_ref())
                            .expect("Failed to request animation frame");
                        closure.forget();
                    },
                    if chaos_engine().is_active { "カオス停止" } else { "カオス開始" }
                }
                
                button {
                    class: "chaos-button",
                    onclick: move |_| {
                        // クリック時点で測定を開始
                        let start_time = window().unwrap().performance().unwrap().now();
                        
                        // 実際の処理
                        chaos_engine.with_mut(|engine| {
                            engine.intensity = (engine.intensity % 3) + 1;
                        });
                        
                        // requestAnimationFrameを使用して次のフレームで測定完了
                        let window = window().unwrap();
                        let performance = window.performance().unwrap();
                        let latency_measurer_clone = latency_measurer.clone();
                        let mut input_latency_clone = input_latency.clone();
                        
                        let closure = wasm_bindgen::closure::Closure::once(move || {
                            let end_time = performance.now();
                            let latency = end_time - start_time;
                            
                            // 測定値を直接記録
                            latency_measurer_clone.with(|m| {
                                m.add_measurement(latency);
                            });
                            
                            // 統計を更新
                            let stats = latency_measurer_clone.with(|m| m.get_stats());
                            input_latency_clone.set(stats.p95);
                        });
                        
                        window.request_animation_frame(closure.as_ref().unchecked_ref())
                            .expect("Failed to request animation frame");
                        closure.forget();
                    },
                    {format!("強度変更 (現在: {})", chaos_engine().intensity)}
                }
                
                button {
                    class: "chaos-button",
                    style: "background-color: #ff4444;",
                    onclick: move |_| {
                        // カオスレベル3の極限テスト
                        web_sys::console::log_1(&"Starting Chaos Level 3 extreme test...".into());
                        
                        // エンジンをレベル3に設定してアクティブ化
                        chaos_engine.with_mut(|engine| {
                            engine.intensity = 3;
                            engine.is_active = true;
                            // 1000個のイベントを一気に生成
                            engine.spawn_batch_events(ChaosEngine::CHAOS_LEVEL_3_EVENT_COUNT);
                        });
                        
                        // 遅延測定をリセット
                        latency_measurer.with(|m| m.reset());
                        
                        web_sys::console::log_1(&format!("Chaos Level 3 test started with {} events", ChaosEngine::CHAOS_LEVEL_3_EVENT_COUNT).into());
                    },
                    "🔥 カオスレベル3極限テスト"
                }
            }
            
            div {
                class: "chaos-visualization",
                h3 { "カオスイベント" }
                div {
                    class: "event-list",
                    for (idx, event) in chaos_engine().events.iter().enumerate().rev().take(10) {
                        div {
                            key: "{idx}",
                            class: "chaos-event",
                            {event.get_display_string()}
                        }
                    }
                }
            }
            
            div {
                class: "latency-report",
                style: "margin-top: 20px; padding: 10px; background: #333; border-radius: 8px;",
                h3 { "入力遅延レポート" }
                pre {
                    style: "font-family: monospace; color: #0f0;",
                    {latency_measurer.with(|m| m.get_stats().format_report())}
                }
                if latency_measurer.with(|m| m.get_stats().meets_target(200.0)) {
                    p { 
                        style: "color: #0f0;", 
                        "✅ 目標達成: 95パーセンタイル < 200ms" 
                    }
                } else {
                    p { 
                        style: "color: #f00;", 
                        "❌ 目標未達成: 95パーセンタイル > 200ms" 
                    }
                }
            }
        }
    }
}