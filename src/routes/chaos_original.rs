use dioxus::prelude::*;
use web_sys::window;

#[derive(Clone)]
pub enum ChaosEvent {
    UIGlitch { element_id: String, severity: f32 },
    InputCorruption { delay_ms: u32, noise: f32 },
    VisualDistortion { distortion_type: String },
    TimeWarp { speed_multiplier: f32 },
}

#[derive(Clone)]
struct ChaosEngine {
    intensity: u8,
    events: Vec<ChaosEvent>,
    is_active: bool,
}

impl ChaosEngine {
    fn new(intensity: u8) -> Self {
        Self {
            intensity,
            events: Vec::new(),
            is_active: false,
        }
    }
    
    fn spawn_random_event(&mut self) {
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
        
        self.events.push(event);
        if self.events.len() > 100 {
            self.events.remove(0);
        }
    }
}

#[component]
pub fn ChaosRoute(intensity: u8) -> Element {
    let mut chaos_engine = use_signal(|| ChaosEngine::new(intensity));
    let mut fps = use_signal(|| 60.0);
    let input_latency = use_signal(|| 0.0);
    
    use_effect(move || {
        use crate::utils::interval::Interval;
        
        let interval = Interval::new(16, move || {
            if chaos_engine().is_active {
                chaos_engine.with_mut(|engine| {
                    for _ in 0..engine.intensity {
                        engine.spawn_random_event();
                    }
                });
                
                // FPS測定
                if let Some(window) = window() {
                    if let Some(performance) = window.performance() {
                        let _now = performance.now();
                        fps.set(1000.0 / 16.0); // 簡易FPS計算
                    }
                }
            }
        });
        
        // Keep interval alive
        std::mem::forget(interval);
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
                    h3 { "入力遅延" }
                    p { {format!("{:.0}ms", input_latency())} }
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
                        chaos_engine.with_mut(|engine| {
                            engine.is_active = !engine.is_active;
                        });
                    },
                    if chaos_engine().is_active { "カオス停止" } else { "カオス開始" }
                }
                
                button {
                    class: "chaos-button",
                    onclick: move |_| {
                        chaos_engine.with_mut(|engine| {
                            engine.intensity = (engine.intensity % 3) + 1;
                        });
                    },
                    {format!("強度変更 (現在: {})", chaos_engine().intensity)}
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
                            match event {
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
                }
            }
        }
    }
}