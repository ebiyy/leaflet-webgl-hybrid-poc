export function initMap(elementId) {
    console.log('Initializing map for element:', elementId);
    const element = document.getElementById(elementId);
    if (!element) {
        console.error('Map container not found:', elementId);
        return null;
    }
    
    // Leafletの地図を初期化する前に、要素のサイズを確認
    if (element.offsetWidth === 0 || element.offsetHeight === 0) {
        console.error('Map container has no size');
        return null;
    }
    
    try {
        const map = L.map(elementId).setView([35.6762, 139.6503], 13);
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '© OpenStreetMap contributors'
        }).addTo(map);
        
        // 地図のサイズを強制的に更新
        setTimeout(() => {
            map.invalidateSize();
        }, 100);
        
        console.log('Map initialized successfully');
        return map;
    } catch (error) {
        console.error('Error initializing map:', error);
        return null;
    }
}

export function addMarkers(map, count, renderMode) {
    const markers = [];
    const bounds = map.getBounds();
    const sw = bounds.getSouthWest();
    const ne = bounds.getNorthEast();
    
    for (let i = 0; i < count; i++) {
        const lat = sw.lat + Math.random() * (ne.lat - sw.lat);
        const lng = sw.lng + Math.random() * (ne.lng - sw.lng);
        
        let marker;
        if (renderMode === 'Canvas') {
            marker = L.circleMarker([lat, lng], {
                radius: 8,
                fillColor: '#ff7800',
                color: '#000',
                weight: 1,
                opacity: 1,
                fillOpacity: 0.8,
                renderer: L.canvas()
            }).addTo(map);
        } else {
            marker = L.marker([lat, lng]).addTo(map);
        }
        
        marker._velocity = {
            lat: (Math.random() - 0.5) * 0.0001,
            lng: (Math.random() - 0.5) * 0.0001
        };
        
        markers.push(marker);
    }
    
    return markers;
}

export function clearMarkers(markers) {
    if (markers) {
        markers.forEach(marker => marker.remove());
    }
}

let animationId = null;

export function startAnimation(map, markers) {
    const bounds = map.getBounds();
    
    function animate() {
        const sw = bounds.getSouthWest();
        const ne = bounds.getNorthEast();
        
        markers.forEach(marker => {
            const currentPos = marker.getLatLng();
            let newLat = currentPos.lat + marker._velocity.lat;
            let newLng = currentPos.lng + marker._velocity.lng;
            
            if (newLat <= sw.lat || newLat >= ne.lat) {
                marker._velocity.lat *= -1;
                newLat = currentPos.lat + marker._velocity.lat;
            }
            if (newLng <= sw.lng || newLng >= ne.lng) {
                marker._velocity.lng *= -1;
                newLng = currentPos.lng + marker._velocity.lng;
            }
            
            marker.setLatLng([newLat, newLng]);
        });
        
        animationId = requestAnimationFrame(animate);
    }
    
    animate();
    return animationId;
}

export function stopAnimation(id) {
    if (id || animationId) {
        cancelAnimationFrame(id || animationId);
        animationId = null;
    }
}