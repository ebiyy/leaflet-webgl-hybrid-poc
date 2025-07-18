// wee_allocを使用してWASMバイナリサイズを削減
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use dioxus::prelude::*;
use dioxus_document::{Link, Script};

mod components;
mod utils;
mod routes;
mod hooks;
mod types;

use routes::{home::Home, map::MapRoute, chaos::ChaosRoute};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/map/:mode")]
    MapRoute { mode: String },
    #[route("/chaos/:intensity")]
    ChaosRoute { intensity: u8 },
}

fn main() {
    // WASM初期化時刻を記録
    #[cfg(target_arch = "wasm32")]
    {
        use crate::utils::performance_metrics::{LoadMetrics, saveLoadMetrics};
        
        // デバッグ情報を追加
        web_sys::console::log_1(&"Starting WASM application...".into());
        
        if let Some(window) = web_sys::window() {
            if let Ok(pathname) = window.location().pathname() {
                web_sys::console::log_1(&format!("Current pathname: {}", pathname).into());
            }
        }
        
        if let Some(metrics) = LoadMetrics::new() {
            web_sys::console::log_1(&format!("Initial load metrics captured: {:.0}ms", metrics.total_load_time).into());
            // メトリクスを保存
            let js_value = serde_wasm_bindgen::to_value(&metrics).unwrap_or(wasm_bindgen::JsValue::NULL);
            saveLoadMetrics(js_value);
        }
    }
    
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"Launching Dioxus app...".into());
    
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        // Tailwind CSS
        Link { rel: "stylesheet", href: "/assets/tailwind.css" }
        
        // 外部ライブラリのCSS/JSを読み込む
        Link { rel: "stylesheet", href: "https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" }
        Script { src: "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" }
        Script { src: "https://cdn.jsdelivr.net/npm/pixi.js@8.6.5/dist/pixi.min.js" }
        
        Router::<Route> {}
    }
}
