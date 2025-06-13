use dioxus::prelude::*;
use crate::components::{map::Map, webgl_map::WebGLMap, canvas_map::CanvasMap, benchmark::BenchmarkPanel};
use crate::hooks::{use_map_config, RenderMode};

#[component]
pub fn MapRoute(mode: String) -> Element {
    // カスタムフックを使用してマップ設定を管理
    let (config, actions) = use_map_config();
    
    // オブジェクト数用の派生シグナルを作成
    let mut object_count_signal = use_signal(|| config.read().object_count);
    
    // configのobject_countが変更されたら派生シグナルも更新
    use_effect(move || {
        object_count_signal.set(config.read().object_count);
    });
    
    // object_count_signalが変更されたらconfigも更新
    use_effect(move || {
        let signal_val = object_count_signal();
        let config_val = config.read().object_count;
        web_sys::console::log_1(&format!("[MapRoute] object_count_signal: {}, config.object_count: {}", signal_val, config_val).into());
        if config_val != signal_val {
            web_sys::console::log_1(&format!("[MapRoute] Updating config.object_count to: {}", signal_val).into());
            (actions.set_object_count)(signal_val);
        }
    });
    
    // URLパラメータからレンダーモードを設定
    let mode_clone = mode.clone();
    use_effect(move || {
        let new_mode = match mode_clone.as_str() {
            "webgl" => RenderMode::WebGL,
            "canvas" => RenderMode::Canvas,
            _ => RenderMode::DOM,
        };
        (actions.set_render_mode)(new_mode);
    });
    
    // 推奨モードが現在のモードと異なる場合の警告
    let show_mode_suggestion = use_memo(move || {
        config.read().render_mode != *actions.recommended_mode.read()
    });
    
    rsx! {
        div {
            class: "map-container",
            div {
                class: "map-header",
                Link {
                    to: "/",
                    "← ホームに戻る"
                }
                h2 { "マップモード: {mode}" }
                
                // パフォーマンスモード推奨表示
                if show_mode_suggestion() {
                    div {
                        class: "suggestion-banner",
                        p {
                            {format!("💡 現在のオブジェクト数({})には", config.read().object_count)},
                            strong { {actions.recommended_mode.read().as_str()} },
                            "モードが推奨されます"
                        }
                    }
                }
            }
            
            div {
                class: "map-content",
                style: "display: flex; height: 100vh;",
                
                div {
                    style: "flex: 1;",
                    match mode.as_str() {
                        "webgl" => rsx! {
                            WebGLMap {
                                object_count: object_count_signal()
                            }
                        },
                        "canvas" => rsx! {
                            CanvasMap {
                                object_count: object_count_signal()
                            }
                        },
                        _ => {
                            let count = object_count_signal();
                            web_sys::console::log_1(&format!("[MapRoute] Rendering Map with object_count: {}", count).into());
                            rsx! {
                                Map {
                                    object_count: count
                                }
                            }
                        }
                    }
                }
                
                div {
                    style: "width: 300px; padding: 20px; background: #f5f5f5;",
                    BenchmarkPanel {
                        object_count: object_count_signal,
                        render_mode: mode.clone(),
                    }
                }
            }
        }
    }
}