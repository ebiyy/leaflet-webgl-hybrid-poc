use dioxus::prelude::*;

#[component]
pub fn WebGLMap(object_count: i32) -> Element {
    // ユニークなコンテナ ID を生成（一度だけ計算）
    let container_id = use_memo(|| {
        format!("webgl-map-container-{}", js_sys::Math::random().to_string().replace(".", ""))
    });
    // クリーンアップコードをメモ化
    let cleanup_code = use_memo(|| {
        r#"
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
        "#.to_string()
    });
    
    // コンポーネントのマウント時にクリーンアップ
    use_effect(move || {
        let _ = js_sys::eval(&cleanup_code());
    });
    
    // アンマウント時のクリーンアップ
    use_drop(move || {
        let _ = js_sys::eval(&cleanup_code());
    });
    
    // 地図初期化コードをメモ化（container_idの変更時のみ再生成）
    let init_code = use_memo(move || {
        let id = container_id();
        format!(r#"
            // マップコンテナがDOMに存在するまで待機
            function initializeWebGLMap() {{
                const container = document.getElementById('{}');
                if (!container) {{
                    setTimeout(initializeWebGLMap, 50);
                    return;
                }}
                
                // 既存のマップインスタンスを破棄
                if (window.theMapInstance) {{
                    try {{
                        window.theMapInstance.remove();
                    }} catch (e) {{
                        console.log('Map already removed');
                    }}
                    window.theMapInstance = null;
                    window.theMapMarkers = null;
                    if (window.animationId) {{
                        cancelAnimationFrame(window.animationId);
                        window.animationId = null;
                    }}
                }}
                
                // Pixi.jsのクリーンアップ
                if (window.pixiApp) {{
                    window.pixiApp.destroy(true);
                    window.pixiApp = null;
                    window.pixiMarkers = null;
                }}
                if (window.pixiContainer) {{
                    window.pixiContainer.remove();
                    window.pixiContainer = null;
                }}
                
                // 既存の Leaflet コンテナを削除
                const existingContainer = container.querySelector('.leaflet-container');
                if (existingContainer) {{
                    existingContainer.remove();
                }}
                
                // 新しいマップインスタンスを作成
                try {{
                    const map = L.map('{}').setView([35.6762, 139.6503], 13);
                    L.tileLayer('https://{{s}}.tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
                        attribution: '© OpenStreetMap contributors'
                    }}).addTo(map);
                    
                    window.theMapInstance = map;
                    window.theMapMarkers = [];
                    console.log('WebGL Map initialized successfully');
                }} catch (e) {{
                    console.error('WebGL Map initialization error:', e);
                }}
            }}
            
            setTimeout(initializeWebGLMap, 100);
        "#, id, id)
    });
    
    // 地図の初期化（一度だけ実行）
    use_effect(move || {
        let _ = js_sys::eval(&init_code());
    });
    
    // object_countの変更を検知するためのシグナル
    let mut count_signal = use_signal(|| object_count);
    
    // propsのobject_countが変更されたらシグナルを更新
    if count_signal() != object_count {
        count_signal.set(object_count);
    }
    
    // シグナルの変更時にマーカーを更新
    use_effect(move || {
        let current_count = count_signal();
        web_sys::console::log_1(&format!("[WebGLMap] Marker update effect triggered with count: {}", current_count).into());
        
        let container_id_str = container_id();
        let update_code = format!(r#"
            setTimeout(() => {{
                if (window.theMapInstance) {{
                    console.log('[WebGL JS] Starting marker update. Current markers:', window.theMapMarkers ? window.theMapMarkers.length : 0);
                    
                    const map = window.theMapInstance;
                    const bounds = map.getBounds();
                    const sw = bounds.getSouthWest();
                    const ne = bounds.getNorthEast();
                    
                    const targetCount = {};
                    console.log('[WebGL JS] Adding ' + targetCount + ' WebGL markers');
                    
                    // WebGLモードの初期化
                    if (!window.pixiApp) {{
                        // Pixi.jsアプリケーションを作成
                        const mapContainer = document.getElementById('{}');
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
        "#, current_count, container_id_str);
        
        let _ = js_sys::eval(&update_code);
    });
    
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
                id: "{container_id}",
                class: "map-container"
            }
        }
    }
}