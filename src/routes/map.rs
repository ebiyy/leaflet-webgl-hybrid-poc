use dioxus::prelude::*;
use crate::components::{map::Map, webgl_map::WebGLMap, canvas_map::CanvasMap, benchmark::BenchmarkPanel};

#[component]
pub fn MapRoute(mode: String) -> Element {
    let object_count = use_signal(|| 1000);
    
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
            }
            
            div {
                class: "map-content",
                style: "display: flex; height: 100vh;",
                
                div {
                    style: "flex: 1;",
                    match mode.as_str() {
                        "webgl" => rsx! {
                            WebGLMap {
                                object_count: object_count()
                            }
                        },
                        "canvas" => rsx! {
                            CanvasMap {
                                object_count: object_count()
                            }
                        },
                        _ => rsx! {
                            Map {
                                object_count: object_count()
                            }
                        }
                    }
                }
                
                div {
                    style: "width: 300px; padding: 20px; background: #f5f5f5;",
                    BenchmarkPanel {
                        object_count: object_count,
                        render_mode: mode.clone(),
                    }
                }
            }
        }
    }
}