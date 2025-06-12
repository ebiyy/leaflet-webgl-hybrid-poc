use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// 軽量なInterval実装（glooの代替）
pub struct Interval {
    _closure: Closure<dyn FnMut()>,
    interval_id: i32,
}

impl Interval {
    #[inline]
    pub fn new(millis: u32, mut callback: impl FnMut() + 'static) -> Self {
        let window = web_sys::window().expect("window should exist");
        let closure = Closure::wrap(Box::new(move || {
            callback();
        }) as Box<dyn FnMut()>);
        
        let interval_id = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                millis as i32,
            )
            .expect("failed to set interval");
        
        Self {
            _closure: closure,
            interval_id,
        }
    }
    
    // Intervalを明示的に停止するメソッドを追加
    pub fn stop(&mut self) {
        if let Some(window) = web_sys::window() {
            window.clear_interval_with_handle(self.interval_id);
            self.interval_id = -1; // 無効化
        }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        if self.interval_id != -1 {
            if let Some(window) = web_sys::window() {
                window.clear_interval_with_handle(self.interval_id);
                web_sys::console::log_1(&format!("Interval {} cleared on drop", self.interval_id).into());
            }
        }
    }
}