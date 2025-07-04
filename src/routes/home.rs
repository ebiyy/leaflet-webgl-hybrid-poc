use dioxus::prelude::*;
use crate::utils::performance_metrics::getLoadMetrics;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoadMetrics {
    total_load_time: f64,
    dom_load_time: Option<f64>,
    wasm_init_time: Option<f64>,
}

// 非同期でロードメトリクスを取得
async fn fetch_load_metrics() -> Result<LoadMetrics, String> {
    // JavaScriptの結果を非同期で取得
    let metrics_js = getLoadMetrics();
    
    if metrics_js.is_null() || metrics_js.is_undefined() {
        return Err("No metrics available yet".to_string());
    }
    
    // JavaScriptオブジェクトから情報を抽出
    let total_time = js_sys::Reflect::get(&metrics_js, &"total_load_time".into())
        .ok()
        .and_then(|v| v.as_f64())
        .ok_or("Failed to get total_load_time")?;
    
    let dom_time = js_sys::Reflect::get(&metrics_js, &"dom_load_time".into())
        .ok()
        .and_then(|v| v.as_f64());
        
    let wasm_time = js_sys::Reflect::get(&metrics_js, &"wasm_init_time".into())
        .ok()
        .and_then(|v| v.as_f64());
    
    Ok(LoadMetrics {
        total_load_time: total_time,
        dom_load_time: dom_time,
        wasm_init_time: wasm_time,
    })
}

#[component]
pub fn Home() -> Element {
    let mut show_metrics = use_signal(|| false);
    
    // use_resourceを使用して非同期でメトリクスを取得（値を返すため）
    let metrics_resource = use_resource(move || async move {
        fetch_load_metrics().await
    });
    
    // メトリクス表示テキストをメモ化
    let metrics_display = use_memo(move || {
        match &*metrics_resource.read() {
            Some(Ok(metrics)) => {
                let mut display = format!(
                    "Initial Load Time: {:.0}ms (Target: <3000ms) {}",
                    metrics.total_load_time,
                    if metrics.total_load_time < 3000.0 { "✅" } else { "❌" }
                );
                
                if let Some(dom_time) = metrics.dom_load_time {
                    display.push_str(&format!("\nDOM Load: {:.0}ms", dom_time));
                }
                
                if let Some(wasm_time) = metrics.wasm_init_time {
                    display.push_str(&format!("\nWASM Init: {:.0}ms", wasm_time));
                }
                
                display
            }
            Some(Err(_)) => String::new(),
            None => "Loading metrics...".to_string(),
        }
    });
    
    rsx! {
        div {
            class: "min-h-screen bg-gray-900 text-white flex items-center justify-center",
            div {
                class: "max-w-4xl mx-auto px-4 py-8 text-center",
                h1 { 
                    class: "text-6xl font-bold mb-4 bg-gradient-to-r from-blue-400 to-purple-600 bg-clip-text text-transparent",
                    "Leaflet WebGL Hybrid POC" 
                }
                p { 
                    class: "text-xl mb-12 text-gray-300",
                    "高性能レンダリング技術検証" 
                }
                
                nav {
                    class: "grid grid-cols-2 gap-4 max-w-2xl mx-auto",
                    Link {
                        to: "/map/dom",
                        class: "bg-blue-600 hover:bg-blue-700 px-6 py-4 rounded-lg transition-colors duration-200 text-lg font-medium",
                        "マップモード (DOM)"
                    }
                    Link {
                        to: "/map/canvas",
                        class: "bg-indigo-600 hover:bg-indigo-700 px-6 py-4 rounded-lg transition-colors duration-200 text-lg font-medium",
                        "マップモード (Canvas)"
                    }
                    Link {
                        to: "/map/webgl",
                        class: "bg-purple-600 hover:bg-purple-700 px-6 py-4 rounded-lg transition-colors duration-200 text-lg font-medium",
                        "マップモード (WebGL)"
                    }
                    // ベンチマークルートは後で実装
                    // Link {
                    //     to: "/benchmark/canvas/10000",
                    //     class: "bg-green-600 hover:bg-green-700 px-6 py-4 rounded-lg transition-colors duration-200 text-lg font-medium",
                    //     "ベンチマーク"
                    // }
                    Link {
                        to: "/chaos/1",
                        class: "bg-red-600 hover:bg-red-700 px-6 py-4 rounded-lg transition-colors duration-200 text-lg font-medium",
                        "カオスモード"
                    }
                }
                
                // パフォーマンスメトリクス表示
                match &*metrics_resource.read() {
                    Some(Ok(_)) => rsx! {
                        div {
                            class: "mt-8 p-4 bg-gray-800 rounded-lg",
                            button {
                                class: "text-sm text-gray-400 hover:text-white transition-colors",
                                onclick: move |_| show_metrics.set(!show_metrics()),
                                {format!("Performance Metrics {}", if show_metrics() { "[-]" } else { "[+]" })}
                            }
                            
                            if show_metrics() {
                                div {
                                    class: "mt-4 text-left text-sm",
                                    pre {
                                        class: "text-green-400 font-mono whitespace-pre-wrap",
                                        {metrics_display()}
                                    }
                                    p {
                                        class: "text-gray-500 mt-2",
                                        "Note: Actual 4G network will add communication latency on top of this time"
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! {
                        div {
                            class: "mt-8 p-4 bg-red-900 rounded-lg text-red-300",
                            "Failed to load metrics: {e}"
                        }
                    },
                    None => rsx! {
                        div {
                            class: "mt-8 p-4 bg-gray-800 rounded-lg text-gray-400",
                            "Loading performance metrics..."
                        }
                    }
                }
            }
        }
    }
}