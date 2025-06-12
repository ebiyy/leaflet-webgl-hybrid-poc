use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use crate::utils::fps_counter::startFPSCounter;

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
    
    // FPS計測を開始
    use_effect(move || {
        let callback = Closure::new(move |current_fps: f64| {
            fps.set(current_fps);
            
            // メトリクスを更新
            if is_recording() {
                metrics.with_mut(|m| m.update_fps(current_fps));
            }
        });
        
        startFPSCounter(&callback);
        
        // クロージャを保持
        std::mem::forget(callback);
    });
    
    // パフォーマンス評価を計算
    let performance_color = metrics().get_performance_color();
    let performance_text = metrics().get_performance_text();
    
    let button_color = if is_recording() {
        "#f44336"
    } else {
        "#4CAF50"
    };
    
    let button_text = if is_recording() {
        "記録停止"
    } else {
        "記録開始"
    };
    
    rsx! {
        div {
            class: "benchmark-panel",
            h2 { "ベンチマーク設定" }
            
            div {
                class: "control-group",
                label { "オブジェクト数: {object_count}" }
                input {
                    r#type: "range",
                    min: "100",
                    max: "10000",
                    step: "100",
                    value: "{object_count}",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<i32>() {
                            object_count.set(val);
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
                p { "現在のFPS: {fps:.1}" }
                
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
                        style: "background: {button_color};",
                        "{button_text}"
                    }
                }
                
                if metrics().frame_count > 0 {
                    div {
                        class: "metrics-results",
                        h4 { "記録結果" }
                        p { "最小FPS: {metrics().min_fps:.1}" }
                        p { "最大FPS: {metrics().max_fps:.1}" }
                        p { "平均FPS: {metrics().avg_fps:.1}" }
                        p { "フレーム数: {metrics().frame_count}" }
                        p {
                            class: "performance-score",
                            style: "font-weight: bold; color: {performance_color};",
                            "パフォーマンス: {performance_text}"
                        }
                    }
                }
            }
        }
    }
}