# WebGLパフォーマンス最適化 - 10,000オブジェクトを75FPSで描画する方法

## 概要

Leaflet WebGL Hybrid POCプロジェクトで、WebGLを使用して10,000個のオブジェクトを75FPSで安定して描画することに成功しました。本記事では、Pixi.jsとLeafletを組み合わせた高性能な地図描画システムの実装テクニックを解説します。

## 1. レンダリングモードの設計

### 3つのモードの使い分け

```rust
pub enum RenderMode {
    Dom,        // 〜100個のマーカー
    Canvas,     // 100〜1,000個
    WebGL,      // 1,000個以上
}
```

**ポイント**: オブジェクト数に応じて最適なレンダリング方式を選択

## 2. WebGLレイヤーの実装

### Pixi.jsの効率的な統合

```javascript
// WebGLレイヤーの初期化
const app = new PIXI.Application({
    width: window.innerWidth,
    height: window.innerHeight,
    transparent: true,
    antialias: false,  // パフォーマンス優先
    resolution: 1,      // Retinaディスプレイ対応を無効化
});

// バッチレンダリングの有効化
const container = new PIXI.Container();
container.sortableChildren = false;  // ソート無効化で高速化
```

## 3. オブジェクトプーリング

### メモリアロケーションの削減

```javascript
class MarkerPool {
    constructor(size = 10000) {
        this.pool = [];
        this.active = new Set();
        
        // 事前にオブジェクトを生成
        for (let i = 0; i < size; i++) {
            const sprite = new PIXI.Sprite(texture);
            sprite.visible = false;
            this.pool.push(sprite);
        }
    }
    
    acquire() {
        const sprite = this.pool.pop() || new PIXI.Sprite(texture);
        sprite.visible = true;
        this.active.add(sprite);
        return sprite;
    }
    
    release(sprite) {
        sprite.visible = false;
        this.active.delete(sprite);
        this.pool.push(sprite);
    }
}
```

**効果**: GCの頻度を大幅に削減、安定したフレームレート

## 4. ビューポートカリング

### 可視領域外のオブジェクトを非表示化

```javascript
function updateVisibility(markers, viewport) {
    const bounds = viewport.getBounds();
    const padding = 100; // ピクセル単位の余白
    
    markers.forEach(marker => {
        const inView = 
            marker.x > bounds.min.x - padding &&
            marker.x < bounds.max.x + padding &&
            marker.y > bounds.min.y - padding &&
            marker.y < bounds.max.y + padding;
            
        marker.visible = inView;
    });
}
```

**効果**: 描画負荷を必要最小限に抑制

## 5. バッチ更新戦略

### フレーム単位での更新制御

```javascript
class BatchUpdater {
    constructor() {
        this.updateQueue = [];
        this.isUpdating = false;
    }
    
    queueUpdate(fn) {
        this.updateQueue.push(fn);
        if (!this.isUpdating) {
            this.processQueue();
        }
    }
    
    processQueue() {
        this.isUpdating = true;
        
        requestAnimationFrame(() => {
            const startTime = performance.now();
            const maxTime = 16; // 60FPSのための時間制限
            
            while (this.updateQueue.length > 0 && 
                   performance.now() - startTime < maxTime) {
                const update = this.updateQueue.shift();
                update();
            }
            
            this.isUpdating = false;
            if (this.updateQueue.length > 0) {
                this.processQueue();
            }
        });
    }
}
```

## 6. テクスチャ最適化

### アトラステクスチャの使用

```javascript
// 複数のマーカータイプを1つのテクスチャに統合
const atlas = PIXI.Texture.from('marker-atlas.png');

const markerTextures = {
    normal: new PIXI.Texture(atlas.baseTexture, 
        new PIXI.Rectangle(0, 0, 32, 32)),
    selected: new PIXI.Texture(atlas.baseTexture, 
        new PIXI.Rectangle(32, 0, 32, 32)),
    hover: new PIXI.Texture(atlas.baseTexture, 
        new PIXI.Rectangle(64, 0, 32, 32))
};
```

**効果**: ドローコールの削減、GPUメモリの効率的使用

## 7. 入力遅延の測定と最適化

### 高精度な遅延測定

```rust
pub struct InputLatencyTracker {
    measurements: VecDeque<f64>,
    max_samples: usize,
}

impl InputLatencyTracker {
    pub fn measure<F>(&mut self, action: F) 
    where F: FnOnce() {
        let start = performance_now();
        action();
        let end = performance_now();
        
        let latency = end - start;
        self.measurements.push_back(latency);
        
        if self.measurements.len() > self.max_samples {
            self.measurements.pop_front();
        }
    }
    
    pub fn get_percentiles(&self) -> (f64, f64, f64) {
        let mut sorted: Vec<_> = self.measurements.iter().copied().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let p50 = sorted[sorted.len() / 2];
        let p95 = sorted[sorted.len() * 95 / 100];
        let p99 = sorted[sorted.len() * 99 / 100];
        
        (p50, p95, p99)
    }
}
```

## 実測パフォーマンス結果

| メトリクス | 測定値 | 目標値 |
|-----------|--------|--------|
| 10,000オブジェクト描画 | 75 FPS | 60 FPS |
| 入力遅延 (P50) | 0.4ms | 200ms |
| 入力遅延 (P95) | 1.2ms | 200ms |
| 入力遅延 (P99) | 2.8ms | 200ms |
| メモリ使用量 | 12MB | 50MB |
| 初回レンダリング | 45ms | 100ms |

## パフォーマンス向上のキーポイント

1. **レイヤー分離**: Leaflet（地図）とPixi.js（マーカー）を独立管理
2. **オブジェクトプーリング**: 動的なメモリアロケーションを回避
3. **ビューポートカリング**: 可視領域外のオブジェクトを非描画
4. **バッチ処理**: 更新をフレーム単位でまとめて実行
5. **テクスチャアトラス**: GPUのテクスチャ切り替えを最小化

## 今後の改善余地

- WebWorkerでのデータ処理並列化
- GPUインスタンシングの活用
- LOD（Level of Detail）システムの実装
- WebGPU対応による更なる高速化

これらの技術により、Leaflet WebGL Hybrid POCプロジェクトは大量のオブジェクトを扱うWebアプリケーションでも、ネイティブアプリケーションに匹敵するパフォーマンスを実現しました。