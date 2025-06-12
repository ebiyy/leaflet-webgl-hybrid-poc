use dioxus::prelude::*;
use crate::utils::performance_metrics::getLoadMetrics;

#[component]
pub fn Home() -> Element {
    let mut show_metrics = use_signal(|| false);
    let mut load_metrics_text = use_signal(|| String::new());
    
    use_effect(move || {
        // ロードメトリクスを取得
        let metrics_js = getLoadMetrics();
        if !metrics_js.is_null() && !metrics_js.is_undefined() {
            // JavaScriptオブジェクトから情報を抽出
            let total_time = js_sys::Reflect::get(&metrics_js, &"total_load_time".into())
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            
            if total_time > 0.0 {
                load_metrics_text.set(format!(
                    "Initial Load Time: {:.0}ms (Target: <3000ms) {}",
                    total_time,
                    if total_time < 3000.0 { "✅" } else { "❌" }
                ));
            }
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
                if !load_metrics_text().is_empty() {
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
                                p {
                                    class: "text-green-400 font-mono",
                                    {load_metrics_text()}
                                }
                                p {
                                    class: "text-gray-500 mt-2",
                                    "Note: Actual 4G network will add communication latency on top of this time"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}