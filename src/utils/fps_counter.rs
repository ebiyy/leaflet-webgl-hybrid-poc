use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
let frameCount = 0;
let lastTime = performance.now();
let fps = 0;
let animationId = null;

export function startFPSCounter(callback) {
    // 既存のカウンターを停止
    if (animationId !== null) {
        cancelAnimationFrame(animationId);
    }
    
    function updateFPS() {
        frameCount++;
        const currentTime = performance.now();
        const elapsed = currentTime - lastTime;
        
        if (elapsed >= 1000) {
            fps = (frameCount * 1000) / elapsed;
            try {
                callback(fps);
            } catch (e) {
                // コンポーネントがアンマウントされた場合のエラーを無視
                console.log('FPS callback error, stopping counter');
                stopFPSCounter();
                return;
            }
            frameCount = 0;
            lastTime = currentTime;
        }
        
        animationId = requestAnimationFrame(updateFPS);
    }
    
    animationId = requestAnimationFrame(updateFPS);
}

export function stopFPSCounter() {
    if (animationId !== null) {
        cancelAnimationFrame(animationId);
        animationId = null;
    }
}
"#)]
extern "C" {
    pub fn startFPSCounter(callback: &Closure<dyn FnMut(f64)>);
    pub fn stopFPSCounter();
}