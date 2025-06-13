use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use crate::utils::fps_counter::{startFPSCounter, stopFPSCounter};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug, Default)]
pub struct BenchmarkMetrics {
    pub fps: f64,
    pub min_fps: f64,
    pub max_fps: f64,
    pub avg_fps: f64,
    pub frame_count: u32,
}

impl BenchmarkMetrics {
    #[inline]
    fn update_fps(&mut self, current_fps: f64) {
        self.frame_count += 1;
        self.fps = current_fps;
        
        if self.min_fps == 0.0 || current_fps < self.min_fps {
            self.min_fps = current_fps;
        }
        if current_fps > self.max_fps {
            self.max_fps = current_fps;
        }
        
        // 移動平均でavg_fpsを計算
        self.avg_fps = (self.avg_fps * (self.frame_count - 1) as f64 + current_fps) / self.frame_count as f64;
    }
    
    #[inline]
    fn get_performance_color(&self) -> &'static str {
        if self.avg_fps >= 55.0 {
            "#4CAF50"
        } else if self.avg_fps >= 30.0 {
            "#FF9800"
        } else {
            "#f44336"
        }
    }
    
    #[inline]
    fn get_performance_text(&self) -> &'static str {
        if self.avg_fps >= 55.0 {
            "良好"
        } else if self.avg_fps >= 30.0 {
            "可"
        } else {
            "要改善"
        }
    }
}

#[component]
pub fn BenchmarkPanel(
    mut object_count: Signal<i32>,
    render_mode: String,
) -> Element {
    let mut fps = use_signal(|| 0.0);
    let mut metrics = use_signal(BenchmarkMetrics::default);
    let mut is_recording = use_signal(|| false);
    
    // FPSコールバックを保持するためのSignal
    let mut callback_holder = use_signal(|| None::<Rc<RefCell<Closure<dyn FnMut(f64)>>>>);
    
    // FPS計測を開始（初回のみ実行）
    use_effect(move || {
        // 既にコールバックが設定されている場合は何もしない
        if callback_holder.read().is_some() {
            return;
        }
        
        let callback = Closure::new(move |current_fps: f64| {
            fps.set(current_fps);
            
            // メトリクスを更新
            if is_recording() {
                metrics.with_mut(|m| m.update_fps(current_fps));
            }
        });
        
        startFPSCounter(&callback);
        
        // コールバックを保持
        callback_holder.set(Some(Rc::new(RefCell::new(callback))));
    });
    
    // コンポーネントがアンマウントされる時にFPSカウンターを停止
    use_drop(move || {
        stopFPSCounter();
    });
    
    // パフォーマンス評価をメモ化（metricsが変更されたときのみ再計算）
    let performance_evaluation = use_memo(move || {
        let m = metrics.read();
        (m.get_performance_color(), m.get_performance_text())
    });
    
    // ボタンの状態をメモ化
    let button_state = use_memo(move || {
        if is_recording() {
            ("#f44336", "記録停止")
        } else {
            ("#4CAF50", "記録開始")
        }
    });
    
    // FPSカテゴリをメモ化
    let fps_category = use_memo(move || {
        match fps() {
            f if f >= 55.0 => "🔥 優秀",
            f if f >= 30.0 => "✅ 良好",
            f if f >= 15.0 => "⚠️ 警告",
            _ => "🆘 危険",
        }
    });
    
    rsx! {
        div {
            class: "benchmark-panel",
            h2 { "ベンチマーク設定" }
            
            div {
                class: "control-group",
                label { "オブジェクト数: {object_count()}" }
                input {
                    r#type: "range",
                    min: "100",
                    max: "10000",
                    step: "100",
                    value: "{object_count()}",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<i32>() {
                            web_sys::console::log_1(&format!("[BenchmarkPanel] Slider changed to: {}", val).into());
                            object_count.set(val);
                            web_sys::console::log_1(&format!("[BenchmarkPanel] object_count Signal updated to: {}", object_count()).into());
                        }
                    }
                }
            }
            
            div {
                class: "control-group",
                label { "レンダリングモード" }
                div {
                    "レンダリングモード: {render_mode}"
                }
            }
            
            div {
                class: "metrics",
                h3 { "パフォーマンスメトリクス" }
                p { 
                    {format!("現在のFPS: {:.1} ", fps())},
                    span {
                        class: "fps-category",
                        {fps_category()}
                    }
                }
                
                div {
                    class: "recording-controls",
                    button {
                        onclick: move |_| {
                            if is_recording() {
                                is_recording.set(false);
                            } else {
                                // 記録開始時にメトリクスをリセット
                                metrics.set(BenchmarkMetrics::default());
                                is_recording.set(true);
                            }
                        },
                        style: format!("background: {};", button_state.read().0),
                        {button_state.read().1}
                    }
                }
                
                if metrics().frame_count > 0 {
                    div {
                        class: "metrics-results",
                        h4 { "記録結果" }
                        p { {format!("最小FPS: {:.1}", metrics().min_fps)} }
                        p { {format!("最大FPS: {:.1}", metrics().max_fps)} }
                        p { {format!("平均FPS: {:.1}", metrics().avg_fps)} }
                        p { "フレーム数: {metrics().frame_count}" }
                        p {
                            class: "performance-score",
                            style: format!("font-weight: bold; color: {};", performance_evaluation.read().0),
                            {"パフォーマンス: "},
                            {performance_evaluation.read().1}
                        }
                    }
                }
            }
        }
    }
}