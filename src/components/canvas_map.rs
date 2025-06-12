use dioxus::prelude::*;

#[component]
pub fn CanvasMap(object_count: i32) -> Element {
    // ユニークなコンテナ ID を生成
    let container_id = use_signal(|| {
        format!("canvas-map-container-{}", js_sys::Math::random().to_string().replace(".", ""))
    });
    
    // コンポーネントのマウント時にクリーンアップ
    use_effect(move || {
        // 既存のマップインスタンスをクリーンアップ
        let cleanup_js = r#"
            if (window.theMapInstance) {
                try {
                    window.theMapInstance.remove();
                } catch (e) {
                    console.log('Previous canvas map cleanup error:', e);
                }
                window.theMapInstance = null;
                window.theMapMarkers = null;
                if (window.animationId) {
                    cancelAnimationFrame(window.animationId);
                    window.animationId = null;
                }
            }
        "#;
        let _ = js_sys::eval(cleanup_js);
    });
    
    // 地図の初期化
    use_effect(move || {
        // 初期化コード
        let js_code = r#"
            // マップコンテナがDOMに存在するまで待機
            function initializeCanvasMap() {
                const container = document.getElementById('{container_id}');
                if (!container) {
                    setTimeout(initializeCanvasMap, 50);
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
                
                // 既存の Leaflet コンテナを削除
                const existingContainer = container.querySelector('.leaflet-container');
                if (existingContainer) {
                    existingContainer.remove();
                }
                
                // 新しいマップインスタンスを作成（Canvasレンダラー使用）
                try {
                    const map = L.map('{container_id}', {
                        preferCanvas: true
                    }).setView([35.6762, 139.6503], 13);
                    
                    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
                        attribution: '© OpenStreetMap contributors'
                    }).addTo(map);
                    
                    window.theMapInstance = map;
                    window.theMapMarkers = [];
                    console.log('Canvas Map initialized successfully');
                } catch (e) {
                    console.error('Canvas Map initialization error:', e);
                }
            }
            
            setTimeout(initializeCanvasMap, 100);
        "#
            .replace("{container_id}", &container_id())
            .replace("{container_id}", &container_id());
        
        let _ = js_sys::eval(&js_code);
    });
    
    // マーカーの更新（object_countが変更されたとき）
    use_effect(use_reactive!(|object_count| {
        let update_code = format!(r#"
            setTimeout(() => {{
                if (window.theMapInstance) {{
                    // 既存のマーカーをクリア
                    if (window.theMapMarkers) {{
                        window.theMapMarkers.forEach(marker => marker.remove());
                    }}
                    window.theMapMarkers = [];
                    
                    // 新しいマーカーを追加（Canvas CircleMarker使用）
                    const map = window.theMapInstance;
                    const bounds = map.getBounds();
                    const sw = bounds.getSouthWest();
                    const ne = bounds.getNorthEast();
                    
                    console.log('Adding {} Canvas CircleMarkers', {});
                    
                    // Canvasレンダラーを作成
                    const canvasRenderer = L.canvas();
                    
                    for (let i = 0; i < {}; i++) {{
                        const lat = sw.lat + Math.random() * (ne.lat - sw.lat);
                        const lng = sw.lng + Math.random() * (ne.lng - sw.lng);
                        
                        // CircleMarkerはCanvasで効率的にレンダリング
                        let marker = L.circleMarker([lat, lng], {{
                            renderer: canvasRenderer,
                            radius: 8,
                            fillColor: '#ff7800',
                            color: '#000',
                            weight: 1,
                            opacity: 1,
                            fillOpacity: 0.8
                        }}).addTo(map);
                        
                        // アニメーション用の速度を設定
                        marker._velocity = {{
                            lat: (Math.random() - 0.5) * 0.00005,
                            lng: (Math.random() - 0.5) * 0.00005
                        }};
                        
                        window.theMapMarkers.push(marker);
                    }}
                    
                    // アニメーションを開始
                    if (window.animationId) {{
                        cancelAnimationFrame(window.animationId);
                    }}
                    
                    function animate() {{
                        window.theMapMarkers.forEach(marker => {{
                            const pos = marker.getLatLng();
                            let newLat = pos.lat + marker._velocity.lat;
                            let newLng = pos.lng + marker._velocity.lng;
                            
                            // 境界でバウンス
                            if (newLat <= sw.lat || newLat >= ne.lat) {{
                                marker._velocity.lat *= -1;
                                newLat = pos.lat + marker._velocity.lat;
                            }}
                            if (newLng <= sw.lng || newLng >= ne.lng) {{
                                marker._velocity.lng *= -1;
                                newLng = pos.lng + marker._velocity.lng;
                            }}
                            
                            marker.setLatLng([newLat, newLng]);
                        }});
                        
                        window.animationId = requestAnimationFrame(animate);
                    }}
                    
                    if ({} > 0) {{
                        animate();
                    }}
                }}
            }}, 200);
        "#, object_count, object_count, object_count, object_count);
        
        let _ = js_sys::eval(&update_code);
    }));
    
    rsx! {
        div {
            class: "map-wrapper",
            div {
                class: "map-info",
                h2 { "レンダリングモード: Canvas" }
                p { "オブジェクト数: {object_count}" }
                p {
                    style: "font-size: 0.8rem; color: #999;",
                    "Canvas: 高速レンダリング（CircleMarker使用）"
                }
            }
            div {
                id: "{container_id()}",
                class: "map-container"
            }
        }
    }
}