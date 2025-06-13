# タスク: パフォーマンス最適化とゲーム向け最適化

## 背景
現在のPOCは基本的な描画性能は達成しているが、ゲーム開発で必要となる60FPS維持や大規模オブジェクト管理には課題がある。

## 現状の性能指標
- DOM: 1,000オブジェクトで限界
- Canvas: 10,000オブジェクトまで
- WebGL: 50,000オブジェクト可能（ただし最適化の余地あり）

## 目標
ゲーム品質の描画性能とメモリ効率を実現。

## 実装タスク

### 1. Signal/Memoの最適化
```rust
// src/hooks/use_batch_updates.rs
pub fn use_batch_updates<T: 'static>() -> BatchUpdater<T> {
    let pending = use_signal(|| Vec::new());
    let frame_id = use_signal(|| None);
    
    BatchUpdater {
        queue: move |update| {
            pending.write().push(update);
            
            if frame_id().is_none() {
                let id = request_animation_frame(move || {
                    let updates = pending.take();
                    // バッチ処理
                    frame_id.set(None);
                });
                frame_id.set(Some(id));
            }
        }
    }
}
```

### 2. WebGLレンダリングの最適化
```rust
// インスタンスレンダリング対応
const INSTANCED_VERTEX_SHADER: &str = r#"
    attribute vec2 a_position;
    attribute vec2 a_instance_position;
    attribute vec2 a_instance_velocity;
    
    uniform mat3 u_projection;
    
    void main() {
        vec2 position = a_position + a_instance_position;
        gl_Position = vec4((u_projection * vec3(position, 1)).xy, 0, 1);
    }
"#;
```

### 3. オブジェクトプールの実装
```rust
// src/utils/object_pool.rs
pub struct ObjectPool<T> {
    available: Vec<T>,
    in_use: HashMap<usize, T>,
    factory: Box<dyn Fn() -> T>,
}

impl<T> ObjectPool<T> {
    pub fn acquire(&mut self) -> PoolHandle<T> {
        let obj = self.available.pop()
            .unwrap_or_else(|| (self.factory)());
        // ...
    }
}
```

### 4. 空間インデックスの導入
```rust
// src/utils/spatial_index.rs
pub struct QuadTree<T> {
    bounds: Bounds,
    max_objects: usize,
    max_levels: usize,
    level: usize,
    objects: Vec<(Point, T)>,
    nodes: Option<Box<[QuadTree<T>; 4]>>,
}

impl<T> QuadTree<T> {
    pub fn query(&self, range: &Bounds) -> Vec<&T> {
        // 効率的な範囲検索
    }
}
```

### 5. WebWorker統合
```rust
// src/workers/physics_worker.rs
#[wasm_bindgen]
pub struct PhysicsWorker {
    objects: Vec<PhysicsObject>,
    quad_tree: QuadTree<usize>,
}

#[wasm_bindgen]
impl PhysicsWorker {
    pub fn update(&mut self, delta: f32) -> Vec<ObjectUpdate> {
        // 物理演算をワーカーで処理
    }
}
```

### 6. メモリ最適化
```rust
// SmallVecの活用
use smallvec::SmallVec;

pub struct OptimizedMarker {
    // スタック上に8要素まで保持
    path_points: SmallVec<[Point; 8]>,
    // ビットフラグでメモリ節約
    flags: MarkerFlags,
}
```

## ゲーム向け最適化例

### LOD (Level of Detail) システム
```rust
pub trait LevelOfDetail {
    fn get_detail_level(&self, distance: f32) -> DetailLevel;
    fn render_at_level(&self, level: DetailLevel);
}

pub struct CharacterRenderer {
    high_detail: MeshData,
    medium_detail: MeshData,
    low_detail: MeshData,
    billboard: TextureData,
}
```

### カリングシステム
```rust
pub fn use_frustum_culling() -> CullingSystem {
    let visible_objects = use_signal(|| Vec::new());
    
    use_effect(move || {
        let camera = use_camera();
        let all_objects = use_game_objects();
        
        let frustum = camera.get_frustum();
        let visible = all_objects.read()
            .iter()
            .filter(|obj| frustum.contains(&obj.bounds))
            .collect();
            
        visible_objects.set(visible);
    });
    
    CullingSystem { visible_objects }
}
```

## ベンチマーク目標
- [ ] 10,000オブジェクト @ 60FPS (Canvas)
- [ ] 100,000オブジェクト @ 60FPS (WebGL)
- [ ] メモリ使用量 < 100MB (10,000オブジェクト時)
- [ ] 初期化時間 < 500ms

## 実装手順
1. [ ] 現状のボトルネック分析
2. [ ] Signal/Memoの最適化
3. [ ] レンダリング最適化
4. [ ] メモリプール実装
5. [ ] WebWorker統合
6. [ ] ベンチマーク作成と継続的測定

## 完了基準
- [ ] 全ベンチマーク目標の達成
- [ ] メモリリークなし（15分連続実行）
- [ ] Chrome DevToolsでのプロファイリング結果
- [ ] モバイルデバイスでの動作確認