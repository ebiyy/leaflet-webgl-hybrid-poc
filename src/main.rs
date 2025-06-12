// wee_allocを使用してWASMバイナリサイズを削減
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use dioxus::prelude::*;

mod components;
mod utils;
mod routes;

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
        if let Some(metrics) = LoadMetrics::new() {
            web_sys::console::log_1(&format!("Initial load metrics captured: {:.0}ms", metrics.total_load_time).into());
            // メトリクスを保存
            let js_value = serde_wasm_bindgen::to_value(&metrics).unwrap_or(wasm_bindgen::JsValue::NULL);
            saveLoadMetrics(js_value);
        }
    }
    
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
