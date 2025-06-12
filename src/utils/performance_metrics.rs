use wasm_bindgen::prelude::*;
use web_sys::Performance;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadMetrics {
    pub navigation_start: f64,
    pub dom_content_loaded: f64,
    pub load_complete: f64,
    pub wasm_init_start: f64,
    pub wasm_init_complete: f64,
    pub first_contentful_paint: f64,
    pub total_load_time: f64,
}

impl LoadMetrics {
    pub fn new() -> Option<Self> {
        let window = web_sys::window()?;
        let performance = window.performance()?;
        let navigation_timing = performance.timing();
        
        // Navigation Timing API values
        let navigation_start = navigation_timing.navigation_start() as f64;
        let dom_content_loaded = navigation_timing.dom_content_loaded_event_end() as f64;
        let load_complete = navigation_timing.load_event_end() as f64;
        
        // First Contentful Paint from Paint Timing API
        let first_contentful_paint = get_first_contentful_paint(&performance).unwrap_or(0.0);
        
        // Calculate total load time
        let total_load_time = if load_complete > 0.0 {
            load_complete - navigation_start
        } else {
            performance.now()
        };
        
        Some(LoadMetrics {
            navigation_start,
            dom_content_loaded,
            load_complete,
            wasm_init_start: 0.0, // Will be set when WASM starts
            wasm_init_complete: performance.now(),
            first_contentful_paint,
            total_load_time,
        })
    }
    
    pub fn get_formatted_report(&self) -> String {
        format!(
            r#"=== 初回ロード性能レポート ===
DOM Content Loaded: {:.0}ms
First Contentful Paint: {:.0}ms
Page Load Complete: {:.0}ms
WASM Init Time: {:.0}ms
Total Load Time: {:.0}ms

4G回線での目標: < 3000ms
現在のステータス: {}"#,
            self.dom_content_loaded - self.navigation_start,
            self.first_contentful_paint,
            self.load_complete - self.navigation_start,
            self.wasm_init_complete - self.wasm_init_start,
            self.total_load_time,
            if self.total_load_time < 3000.0 {
                "✅ 目標達成"
            } else {
                "❌ 目標未達成"
            }
        )
    }
}

fn get_first_contentful_paint(_performance: &Performance) -> Option<f64> {
    // JavaScript経由でPerformance Paint Timing APIにアクセス
    let js_code = r#"
        const entries = performance.getEntriesByType('paint');
        const fcp = entries.find(entry => entry.name === 'first-contentful-paint');
        return fcp ? fcp.startTime : 0;
    "#;
    
    let result = js_sys::eval(js_code).ok()?;
    result.as_f64()
}

// グローバルなロード時間を保存
#[wasm_bindgen(inline_js = r#"
let globalLoadMetrics = null;

export function saveLoadMetrics(metrics) {
    globalLoadMetrics = metrics;
}

export function getLoadMetrics() {
    return globalLoadMetrics;
}
"#)]
extern "C" {
    pub fn saveLoadMetrics(metrics: JsValue);
    pub fn getLoadMetrics() -> JsValue;
}