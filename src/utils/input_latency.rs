use wasm_bindgen::prelude::*;
use web_sys::{window, Performance};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// 入力遅延測定用の構造体
pub struct InputLatencyMeasurer {
    performance: Performance,
    measurements: Rc<RefCell<Vec<f64>>>,
    last_input_time: Rc<RefCell<f64>>,
}

impl InputLatencyMeasurer {
    pub fn new() -> Self {
        let window = window().expect("Window should exist");
        let performance = window.performance().expect("Performance API should be available");
        
        Self {
            performance,
            measurements: Rc::new(RefCell::new(Vec::new())),
            last_input_time: Rc::new(RefCell::new(0.0)),
        }
    }
    
    /// 入力イベントの開始時刻を記録
    pub fn mark_input_start(&self) {
        let now = self.performance.now();
        *self.last_input_time.borrow_mut() = now;
    }
    
    /// 入力処理の完了時刻を記録し、遅延を計算
    pub fn mark_input_end(&self) {
        let now = self.performance.now();
        let start = *self.last_input_time.borrow();
        
        if start > 0.0 {
            let latency = now - start;
            self.measurements.borrow_mut().push(latency);
            
            // 最大1000件まで保持
            if self.measurements.borrow().len() > 1000 {
                self.measurements.borrow_mut().remove(0);
            }
        }
    }
    
    /// 統計情報を取得
    pub fn get_stats(&self) -> LatencyStats {
        let measurements = self.measurements.borrow();
        
        if measurements.is_empty() {
            return LatencyStats::default();
        }
        
        let mut sorted = measurements.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let avg = measurements.iter().sum::<f64>() / measurements.len() as f64;
        let min = sorted[0];
        let max = sorted[sorted.len() - 1];
        let p50 = percentile(&sorted, 0.5);
        let p95 = percentile(&sorted, 0.95);
        let p99 = percentile(&sorted, 0.99);
        
        LatencyStats {
            count: measurements.len(),
            avg,
            min,
            max,
            p50,
            p95,
            p99,
        }
    }
    
    /// 測定をリセット
    pub fn reset(&self) {
        self.measurements.borrow_mut().clear();
        *self.last_input_time.borrow_mut() = 0.0;
    }
    
    /// 直接測定値を追加
    pub fn add_measurement(&self, latency: f64) {
        self.measurements.borrow_mut().push(latency);
        
        // 最大1000件まで保持
        if self.measurements.borrow().len() > 1000 {
            self.measurements.borrow_mut().remove(0);
        }
    }
    
    /// クリック開始時刻を記録し、次のフレームで遅延を測定するクロージャを返す
    pub fn measure_with_raf<F>(&self, callback: F) -> impl FnOnce()
    where
        F: FnOnce(f64) + 'static,
    {
        let start_time = self.performance.now();
        let performance = self.performance.clone();
        let measurements = self.measurements.clone();
        
        move || {
            let window = window().expect("Window should exist");
            let closure = Closure::once(move || {
                let end_time = performance.now();
                let latency = end_time - start_time;
                
                // 測定値を記録
                measurements.borrow_mut().push(latency);
                if measurements.borrow().len() > 1000 {
                    measurements.borrow_mut().remove(0);
                }
                
                // コールバックを実行
                callback(latency);
            });
            
            window.request_animation_frame(closure.as_ref().unchecked_ref())
                .expect("Failed to request animation frame");
            closure.forget();
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LatencyStats {
    pub count: usize,
    pub avg: f64,
    pub min: f64,
    pub max: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
}

impl LatencyStats {
    /// 95パーセンタイルが目標値以下かチェック
    pub fn meets_target(&self, target_ms: f64) -> bool {
        self.p95 <= target_ms
    }
    
    /// 統計情報を文字列として整形
    pub fn format_report(&self) -> String {
        format!(
            "Input Latency Report (n={})\n\
             Average: {:.1}ms\n\
             Min: {:.1}ms\n\
             Max: {:.1}ms\n\
             P50: {:.1}ms\n\
             P95: {:.1}ms\n\
             P99: {:.1}ms",
            self.count,
            self.avg,
            self.min,
            self.max,
            self.p50,
            self.p95,
            self.p99
        )
    }
}

/// パーセンタイル計算
fn percentile(sorted_data: &[f64], p: f64) -> f64 {
    let idx = (sorted_data.len() as f64 * p) as usize;
    let idx = idx.min(sorted_data.len() - 1);
    sorted_data[idx]
}

/// JavaScriptからアクセス可能な測定インターフェース
#[wasm_bindgen]
pub struct LatencyMeasurer {
    inner: InputLatencyMeasurer,
}

#[wasm_bindgen]
impl LatencyMeasurer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: InputLatencyMeasurer::new(),
        }
    }
    
    #[wasm_bindgen(js_name = markInputStart)]
    pub fn mark_input_start(&self) {
        self.inner.mark_input_start();
    }
    
    #[wasm_bindgen(js_name = markInputEnd)]
    pub fn mark_input_end(&self) {
        self.inner.mark_input_end();
    }
    
    #[wasm_bindgen(js_name = getP95)]
    pub fn get_p95(&self) -> f64 {
        self.inner.get_stats().p95
    }
    
    #[wasm_bindgen(js_name = getReport)]
    pub fn get_report(&self) -> String {
        self.inner.get_stats().format_report()
    }
    
    #[wasm_bindgen(js_name = meetsTarget)]
    pub fn meets_target(&self, target_ms: f64) -> bool {
        self.inner.get_stats().meets_target(target_ms)
    }
    
    pub fn reset(&self) {
        self.inner.reset();
    }
}