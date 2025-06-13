use dioxus::prelude::*;

// Canvas用のマーカー更新コードを生成するヘルパー関数
fn generate_canvas_marker_update_code(object_count: i32) -> String {
    format!(r#"
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
    "#, object_count, object_count, object_count, object_count)
}

#[component]
pub fn CanvasMap(object_count: i32) -> Element {
    // ユニークなコンテナ ID を生成（一度だけ計算）
    let container_id = use_memo(|| {
        format!("canvas-map-container-{}", js_sys::Math::random().to_string().replace(".", ""))
    });
    
    // クリーンアップコードをメモ化
    let cleanup_code = use_memo(|| {
        r#"
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
            function initializeCanvasMap() {{
                const container = document.getElementById('{}');
                if (!container) {{
                    setTimeout(initializeCanvasMap, 50);
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
                
                // 既存の Leaflet コンテナを削除
                const existingContainer = container.querySelector('.leaflet-container');
                if (existingContainer) {{
                    existingContainer.remove();
                }}
                
                // 新しいマップインスタンスを作成（Canvasレンダラー使用）
                try {{
                    const map = L.map('{}', {{
                        preferCanvas: true
                    }}).setView([35.6762, 139.6503], 13);
                    
                    L.tileLayer('https://{{s}}.tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
                        attribution: '© OpenStreetMap contributors'
                    }}).addTo(map);
                    
                    window.theMapInstance = map;
                    window.theMapMarkers = [];
                    console.log('Canvas Map initialized successfully');
                }} catch (e) {{
                    console.error('Canvas Map initialization error:', e);
                }}
            }}
            
            setTimeout(initializeCanvasMap, 100);
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
        web_sys::console::log_1(&format!("[CanvasMap] Marker update effect triggered with count: {}", current_count).into());
        
        // 地図の初期化を待つ
        let update_markers_with_delay = format!(r#"
            setTimeout(() => {{
                if (window.theMapInstance) {{
                    console.log('[CanvasMap] Effect: Updating markers with count: {}');
                    {}
                }} else {{
                    console.log('[CanvasMap] Effect: Map not yet initialized, retrying...');
                    setTimeout(arguments.callee, 200);
                }}
            }}, 300);
        "#, current_count, generate_canvas_marker_update_code(current_count));
        
        let _ = js_sys::eval(&update_markers_with_delay);
    });
    
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
                id: "{container_id}",
                class: "map-container"
            }
        }
    }
}