use dioxus::prelude::*;

#[component]
pub fn WebGLMap(object_count: i32) -> Element {
    // ユニークなコンテナ ID を生成
    let container_id = use_signal(|| {
        format!("webgl-map-container-{}", js_sys::Math::random().to_string().replace(".", ""))
    });
    // コンポーネントのマウント時にクリーンアップ
    use_effect(move || {
        // 既存のマップインスタンスをクリーンアップ
        let cleanup_js = r#"
            if (window.theMapInstance) {
                try {
                    window.theMapInstance.remove();
                } catch (e) {
                    console.log('Previous WebGL map cleanup error:', e);
                }
                window.theMapInstance = null;
                window.theMapMarkers = null;
                if (window.animationId) {
                    cancelAnimationFrame(window.animationId);
                    window.animationId = null;
                }
            }
            // Pixi.jsのクリーンアップ
            if (window.pixiApp) {
                window.pixiApp.destroy(true);
                window.pixiApp = null;
                window.pixiMarkers = null;
            }
            if (window.pixiContainer) {
                window.pixiContainer.remove();
                window.pixiContainer = null;
            }
        "#;
        let _ = js_sys::eval(cleanup_js);
    });
    
    // 地図の初期化
    use_effect(move || {
        let js_code = r#"
            // マップコンテナがDOMに存在するまで待機
            function initializeWebGLMap() {
                const container = document.getElementById('{container_id}');
                if (!container) {
                    setTimeout(initializeWebGLMap, 50);
                    return;
                }
                
                // 既存のマップインスタンスを破棄
                if (window.theMapInstance) {
                    try {
                        window.theMapInstance.remove();
                    } catch (e) {
                        console.log('Map already removed');
                    }
                    window.theMapInstance = null;
                    window.theMapMarkers = null;
                    if (window.animationId) {
                        cancelAnimationFrame(window.animationId);
                        window.animationId = null;
                    }
                }
                
                // Pixi.jsのクリーンアップ
                if (window.pixiApp) {
                    window.pixiApp.destroy(true);
                    window.pixiApp = null;
                    window.pixiMarkers = null;
                }
                if (window.pixiContainer) {
                    window.pixiContainer.remove();
                    window.pixiContainer = null;
                }
                
                // 既存の Leaflet コンテナを削除
                const existingContainer = container.querySelector('.leaflet-container');
                if (existingContainer) {
                    existingContainer.remove();
                }
                
                // 新しいマップインスタンスを作成
                try {
                    const map = L.map('{container_id}').setView([35.6762, 139.6503], 13);
                    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
                        attribution: '© OpenStreetMap contributors'
                    }).addTo(map);
                    
                    window.theMapInstance = map;
                    window.theMapMarkers = [];
                    console.log('WebGL Map initialized successfully');
                } catch (e) {
                    console.error('WebGL Map initialization error:', e);
                }
            }
            
            setTimeout(initializeWebGLMap, 100);
        "#
            .replace("{container_id}", &container_id())
            .replace("{container_id}", &container_id());
        
        let _ = js_sys::eval(&js_code);
    });
    
    // WebGLマーカーの更新
    use_effect(use_reactive!(|object_count| {
        let count = object_count;
        web_sys::console::log_1(&format!("[WebGLMap] Updating markers to count: {}", count).into());
        
        let update_code = format!(r#"
            setTimeout(() => {{
                if (window.theMapInstance) {{
                    console.log('[WebGL JS] Starting marker update. Current markers:', window.theMapMarkers ? window.theMapMarkers.length : 0);
                    
                    const map = window.theMapInstance;
                    const bounds = map.getBounds();
                    const sw = bounds.getSouthWest();
                    const ne = bounds.getNorthEast();
                    
                    const targetCount = {count};
                    console.log('[WebGL JS] Adding ' + targetCount + ' WebGL markers');
                    
                    // WebGLモードの初期化
                    if (!window.pixiApp) {{
                        // Pixi.jsアプリケーションを作成
                        const mapContainer = document.getElementById('{container_id}');
                        const mapSize = map.getSize();
                        
                        window.pixiApp = new PIXI.Application({{
                            width: mapSize.x,
                            height: mapSize.y,
                            backgroundAlpha: 0,
                            resolution: window.devicePixelRatio || 1,
                            autoDensity: true,
                            antialias: true
                        }});
                        
                        // Leafletの上にPixiキャンバスを配置
                        const pixiContainer = L.DomUtil.create('div', 'pixi-overlay');
                        pixiContainer.style.position = 'absolute';
                        pixiContainer.style.top = '0';
                        pixiContainer.style.left = '0';
                        pixiContainer.style.pointerEvents = 'none';
                        pixiContainer.style.zIndex = '1000';
                        mapContainer.appendChild(pixiContainer);
                        pixiContainer.appendChild(window.pixiApp.view);
                        
                        // Pixiキャンバスを透明に設定
                        window.pixiApp.view.style.background = 'transparent';
                        window.pixiApp.view.style.backgroundColor = 'transparent';
                        
                        window.pixiContainer = pixiContainer;
                        
                        // マップの移動/ズームに合わせてPixiレイヤーを更新
                        map.on('move zoom', () => {{
                            if (window.pixiMarkers) {{
                                window.pixiMarkers.forEach((marker, index) => {{
                                    const originalMarker = window.theMapMarkers[index];
                                    if (originalMarker) {{
                                        const point = map.latLngToContainerPoint(originalMarker._latlng);
                                        marker.x = point.x;
                                        marker.y = point.y;
                                    }}
                                }});
                            }}
                        }});
                    }}
                    
                    // 既存のマーカーとPixiマーカーをクリア
                    if (window.theMapMarkers) {{
                        console.log('[WebGL JS] Removing existing markers...');
                        window.theMapMarkers = [];
                        console.log('[WebGL JS] All markers removed');
                    }}
                    
                    if (window.pixiMarkers) {{
                        window.pixiMarkers.forEach(marker => {{
                            window.pixiApp.stage.removeChild(marker);
                        }});
                    }}
                    window.pixiMarkers = [];
                    
                    // 新しいWebGLマーカーを追加
                    window.theMapMarkers = [];
                    for (let i = 0; i < targetCount; i++) {{
                        const lat = sw.lat + Math.random() * (ne.lat - sw.lat);
                        const lng = sw.lng + Math.random() * (ne.lng - sw.lng);
                        
                        // WebGLマーカー用の位置情報を保存
                        const latlng = {{ lat, lng }};
                        const marker = {{ _latlng: latlng }};
                        
                        // Pixiスプライトを作成
                        const graphics = new PIXI.Graphics();
                        graphics.beginFill(0xff7800, 0.8);
                        graphics.lineStyle(1, 0x000000, 1);
                        graphics.drawCircle(0, 0, 8);
                        graphics.endFill();
                        
                        const point = map.latLngToContainerPoint(latlng);
                        graphics.x = point.x;
                        graphics.y = point.y;
                        
                        window.pixiApp.stage.addChild(graphics);
                        window.pixiMarkers.push(graphics);
                        
                        // アニメーション用の速度を設定（より大きな値に）
                        graphics._velocity = {{
                            lat: (Math.random() - 0.5) * 0.0002,
                            lng: (Math.random() - 0.5) * 0.0002
                        }};
                        
                        // アニメーション用の速度を設定
                        marker._velocity = {{
                            lat: (Math.random() - 0.5) * 0.00005,
                            lng: (Math.random() - 0.5) * 0.00005
                        }};
                        
                        window.theMapMarkers.push(marker);
                    }}
                    
                    console.log('[WebGL JS] Actually created markers:', window.theMapMarkers.length);
                    
                    // アニメーションを開始
                    if (window.animationId) {{
                        cancelAnimationFrame(window.animationId);
                    }}
                    
                    function animate() {{
                        // WebGLモードのアニメーション
                        window.theMapMarkers.forEach((marker, index) => {{
                            const pixiMarker = window.pixiMarkers[index];
                            if (pixiMarker && pixiMarker._velocity) {{
                                let newLat = marker._latlng.lat + pixiMarker._velocity.lat;
                                let newLng = marker._latlng.lng + pixiMarker._velocity.lng;
                                
                                // 境界でバウンス
                                if (newLat <= sw.lat || newLat >= ne.lat) {{
                                    pixiMarker._velocity.lat *= -1;
                                    newLat = marker._latlng.lat + pixiMarker._velocity.lat;
                                }}
                                if (newLng <= sw.lng || newLng >= ne.lng) {{
                                    pixiMarker._velocity.lng *= -1;
                                    newLng = marker._latlng.lng + pixiMarker._velocity.lng;
                                }}
                                
                                marker._latlng.lat = newLat;
                                marker._latlng.lng = newLng;
                                
                                const point = map.latLngToContainerPoint(marker._latlng);
                                pixiMarker.x = point.x;
                                pixiMarker.y = point.y;
                            }}
                        }});
                        
                        window.animationId = requestAnimationFrame(animate);
                    }}
                    
                    if (targetCount > 0) {{
                        animate();
                    }}
                }}
            }}, 200);
        "#, count = count)
            .replace("{container_id}", &container_id());
        
        let _ = js_sys::eval(&update_code);
    }));
    
    rsx! {
        div {
            class: "map-wrapper",
            div {
                class: "map-info",
                h2 { "WebGL モード (Pixi.js)" }
                p { "オブジェクト数: {object_count}" }
                p {
                    style: "font-size: 0.8rem; color: #999;",
                    "WebGL: 超高速レンダリング（Pixi.js使用）"
                }
            }
            div {
                id: "{container_id()}",
                class: "map-container"
            }
        }
    }
}