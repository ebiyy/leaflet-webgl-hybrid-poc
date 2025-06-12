// Service Worker for Leaflet WebGL Hybrid POC
const CACHE_NAME = 'leaflet-webgl-hybrid-poc-v1';
const CRITICAL_ASSETS = [
  '/leaflet-webgl-hybrid-poc/',
  '/leaflet-webgl-hybrid-poc/index.html',
  '/leaflet-webgl-hybrid-poc/leaflet-webgl-hybrid-poc_bg.wasm',
  '/leaflet-webgl-hybrid-poc/leaflet-webgl-hybrid-poc.js',
  '/leaflet-webgl-hybrid-poc/style.css'
];

// Install event - キャッシュの初期化
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => {
        console.log('[SW] Caching critical assets');
        return cache.addAll(CRITICAL_ASSETS);
      })
      .then(() => self.skipWaiting())
  );
});

// Activate event - 古いキャッシュのクリーンアップ
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then(cacheNames => {
      return Promise.all(
        cacheNames.map(cacheName => {
          if (cacheName !== CACHE_NAME) {
            console.log('[SW] Removing old cache:', cacheName);
            return caches.delete(cacheName);
          }
        })
      );
    }).then(() => self.clients.claim())
  );
});

// Fetch event - キャッシュ戦略
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // 同じオリジンのリクエストのみ処理
  if (url.origin !== location.origin) {
    return;
  }

  // WASMとJSファイルは優先的にキャッシュから提供
  if (request.url.includes('.wasm') || request.url.includes('.js')) {
    event.respondWith(
      caches.match(request)
        .then(response => {
          if (response) {
            console.log('[SW] Serving from cache:', request.url);
            return response;
          }
          return fetch(request).then(response => {
            // 成功したレスポンスをキャッシュに追加
            if (response.status === 200) {
              const responseToCache = response.clone();
              caches.open(CACHE_NAME)
                .then(cache => cache.put(request, responseToCache));
            }
            return response;
          });
        })
    );
    return;
  }

  // その他のリクエストはネットワーク優先
  event.respondWith(
    fetch(request)
      .then(response => {
        // 成功したレスポンスをキャッシュに追加
        if (response.status === 200) {
          const responseToCache = response.clone();
          caches.open(CACHE_NAME)
            .then(cache => cache.put(request, responseToCache));
        }
        return response;
      })
      .catch(() => {
        // ネットワークエラー時はキャッシュから提供
        return caches.match(request);
      })
  );
});

// メッセージイベント - キャッシュクリアなどの制御
self.addEventListener('message', (event) => {
  if (event.data === 'skipWaiting') {
    self.skipWaiting();
  }
  if (event.data === 'clearCache') {
    caches.delete(CACHE_NAME).then(() => {
      console.log('[SW] Cache cleared');
    });
  }
});