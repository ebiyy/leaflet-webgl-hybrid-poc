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
        // head要素に入るタグ群
        // Leaflet.js
        Script { src: "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" }
        Link { rel: "stylesheet", href: "https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" }
        
        // Pixi.js
        Script { src: "https://cdnjs.cloudflare.com/ajax/libs/pixi.js/7.2.4/pixi.min.js" }
        
        // ローカルスタイル
        Link { rel: "stylesheet", href: "/assets/style.css" }
        Link { rel: "stylesheet", href: "/assets/tailwind-generated.css" }
        
        // 開発環境専用
        if cfg!(debug_assertions) {
            // 将来的にライブリロード用スクリプトを追加可能
            // Script { src: "http://localhost:8098/livereload.js" }
        }
        
        // アプリ本体
        // 注: Dioxus CLIは自動的にbase_pathを処理するため、
        // ここでは明示的な設定は不要
        Router::<Route> {}
    }
}
