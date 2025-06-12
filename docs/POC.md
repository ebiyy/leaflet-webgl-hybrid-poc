# Leaflet WebGL Hybrid POC実施計画書
## 技術検証フェーズ（3週間スプリント）

---

## 1. POC概要

### 目的
技術設計レビューで指摘された**3つのクリティカルリスク**を早期検証し、本開発の技術スタック確定判断を行う。

### 検証期間
**3週間（15営業日）** - 2025年6月第3週〜7月第1週

### 成功基準
- ✅ 10,000マーカーで60FPS維持（またはフォールバック戦略確定）
- ✅ WASM初回ロード3秒以内（4G回線）
- ✅ カオスレベル3で入力遅延200ms以内

---

## 2. 週次スプリント計画

### Week 1: レンダリング性能検証
**「10,000オブジェクトを60FPSで表示できるか？」**

#### Day 1-2: ベンチマーク環境構築
```rust
// benchmark/src/main.rs
use dioxus::prelude::*;
use web_time::Instant;

#[derive(Clone)]
struct PerformanceMetrics {
    fps: f64,
    frame_time: f64,
    object_count: usize,
}

fn main() {
    launch(BenchmarkApp);
}

#[component]
fn BenchmarkApp() -> Element {
    let metrics = use_signal(|| PerformanceMetrics::default());
    let render_mode = use_signal(|| RenderMode::LeafletDOM);
    
    rsx! {
        div { class: "benchmark-container",
            RenderModeSelector { mode: render_mode }
            MetricsDisplay { metrics: metrics }
            
            match render_mode() {
                RenderMode::LeafletDOM => rsx! { LeafletDOMTest { metrics } },
                RenderMode::LeafletCanvas => rsx! { LeafletCanvasTest { metrics } },
                RenderMode::WebGL => rsx! { WebGLTest { metrics } },
                RenderMode::Hybrid => rsx! { HybridTest { metrics } },
            }
        }
    }
}
```

#### Day 3-4: 4つのレンダリング方式実装
```javascript
// 1. Leaflet DOM（標準）
function testLeafletDOM(count) {
    const markers = [];
    for (let i = 0; i < count; i++) {
        const marker = L.marker(randomPosition())
            .addTo(map);
        markers.push(marker);
    }
    measurePerformance('leaflet-dom', markers);
}

// 2. Leaflet Canvas
function testLeafletCanvas(count) {
    const canvasRenderer = L.canvas();
    for (let i = 0; i < count; i++) {
        L.circleMarker(randomPosition(), {
            renderer: canvasRenderer
        }).addTo(map);
    }
}

// 3. WebGL (Pixi.js overlay)
function testWebGL(count) {
    const pixiOverlay = L.pixiOverlay((utils) => {
        const container = utils.getContainer();
        for (let i = 0; i < count; i++) {
            const sprite = new PIXI.Sprite(markerTexture);
            container.addChild(sprite);
        }
    }).addTo(map);
}

// 4. ハイブリッド（静的=Canvas、動的=DOM）
function testHybrid(staticCount, dynamicCount) {
    // 背景の静的オブジェクトはCanvas
    testLeafletCanvas(staticCount);
    // インタラクティブなオブジェクトはDOM
    testLeafletDOM(dynamicCount);
}
```

#### Day 5: 性能測定と分析
```rust
// パフォーマンス測定ループ
async fn measure_performance() -> PerformanceReport {
    let test_counts = vec![100, 500, 1000, 5000, 10000, 20000];
    let mut results = Vec::new();
    
    for count in test_counts {
        let metrics = run_benchmark(count).await;
        results.push(BenchmarkResult {
            object_count: count,
            avg_fps: metrics.fps,
            p95_frame_time: metrics.frame_time_p95,
            memory_usage: metrics.memory_mb,
            first_paint: metrics.first_paint_ms,
        });
        
        // 60FPS（16.67ms）を下回ったら警告
        if metrics.frame_time_p95 > 16.67 {
            log::warn!("Performance degradation at {} objects", count);
        }
    }
    
    generate_report(results)
}
```

### Week 2: WASM最適化とカオスエンジン
**「初回ロード最適化＆カオスモード基本実装」**

#### Day 6-7: WASMサイズ削減
```toml
# Cargo.toml最適化
[dependencies]
dioxus = { version = "0.6", default-features = false, features = ["web", "router"] }
rapier2d = { version = "0.17", default-features = false, features = ["simd-stable"] }
serde = { version = "1.0", default-features = false, features = ["derive"] }

[profile.release]
opt-level = "z"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true

# wee_allocを使用してサイズ削減
[dependencies.wee_alloc]
version = "0.4"

[profile.release.package."*"]
opt-level = "z"
```

```bash
# ビルドと最適化スクリプト
#!/bin/bash
# build-optimized.sh

# 1. Rustビルド
cargo build --release --target wasm32-unknown-unknown

# 2. wasm-bindgen処理
wasm-bindgen target/wasm32-unknown-unknown/release/leaflet_webgl_hybrid_poc.wasm \
  --out-dir pkg \
  --target web

# 3. wasm-opt最適化
wasm-opt -Oz \
  -o pkg/leaflet_webgl_hybrid_poc_bg_optimized.wasm \
  pkg/leaflet_webgl_hybrid_poc_bg.wasm

# 4. サイズ確認
echo "Original size: $(wc -c < pkg/leaflet_webgl_hybrid_poc_bg.wasm) bytes"
echo "Optimized size: $(wc -c < pkg/leaflet_webgl_hybrid_poc_bg_optimized.wasm) bytes"

# 5. gzip/brotli圧縮後サイズ
gzip -9 -c pkg/leaflet_webgl_hybrid_poc_bg_optimized.wasm | wc -c
brotli -c pkg/leaflet_webgl_hybrid_poc_bg_optimized.wasm | wc -c
```

#### Day 8-9: ローディング戦略実装
```rust
// プログレッシブローディング
#[component]
fn App() -> Element {
    let loading_stage = use_signal(|| LoadingStage::Initial);
    
    use_effect(move || {
        spawn(async move {
            // Stage 1: 最小限のUI表示
            loading_stage.set(LoadingStage::CoreUI);
            
            // Stage 2: 地図初期化
            initialize_map().await;
            loading_stage.set(LoadingStage::MapReady);
            
            // Stage 3: ゲームロジック（WASM）
            load_game_engine().await;
            loading_stage.set(LoadingStage::GameReady);
            
            // Stage 4: アセット読み込み
            preload_assets().await;
            loading_stage.set(LoadingStage::Complete);
        });
    });
    
    rsx! {
        match loading_stage() {
            LoadingStage::Initial => rsx! { SplashScreen {} },
            LoadingStage::CoreUI => rsx! { SkeletonUI {} },
            LoadingStage::MapReady => rsx! { MapWithLoading {} },
            LoadingStage::GameReady => rsx! { GameUI { limited: true } },
            LoadingStage::Complete => rsx! { GameUI { limited: false } },
        }
    }
}
```

#### Day 10: カオスエンジン基本実装
```rust
// カオスレベル3の極限テスト
pub struct ChaosEngine {
    event_queue: VecDeque<ChaosEvent>,
    active_distortions: Vec<UIDistortion>,
    particle_system: ParticleSystem,
}

impl ChaosEngine {
    pub async fn run_chaos_level_3(&mut self) {
        let mut last_frame = Instant::now();
        
        loop {
            let frame_start = Instant::now();
            
            // 1000イベント同時発生
            for _ in 0..1000 {
                self.spawn_random_event();
            }
            
            // UI破壊処理
            self.corrupt_ui_elements(0.8);
            
            // パーティクル更新（10,000個）
            self.particle_system.update();
            
            // フレーム時間測定
            let frame_time = frame_start.elapsed();
            if frame_time.as_millis() > 16 {
                log::warn!("Frame took {}ms", frame_time.as_millis());
            }
            
            // 次フレームまで待機
            TimeoutFuture::new(16).await;
        }
    }
}
```

### Week 3: 統合テストと判断
**「実プレイ環境での検証と最終判断」**

#### Day 11-12: 統合ビルド
```rust
// 完全な統合テスト環境
#[component]
fn IntegrationTest() -> Element {
    let test_scenario = use_signal(|| TestScenario::default());
    let metrics = use_signal(|| IntegrationMetrics::default());
    
    rsx! {
        div { class: "integration-test",
            // コントロールパネル
            TestController { 
                scenario: test_scenario,
                on_start: move |s| run_integration_test(s, metrics)
            }
            
            // ゲーム画面
            GameContainer {
                scenario: test_scenario()
            }
            
            // メトリクス表示
            MetricsPanel { metrics: metrics() }
        }
    }
}

async fn run_integration_test(
    scenario: TestScenario,
    metrics: Signal<IntegrationMetrics>
) {
    // シナリオ実行
    match scenario {
        TestScenario::LoadingOptimization => {
            test_loading_performance(metrics).await;
        }
        TestScenario::ChaosStress => {
            test_chaos_performance(metrics).await;
        }
        TestScenario::EndToEnd => {
            test_full_gameplay(metrics).await;
        }
    }
}
```

#### Day 13-14: 自動化テストスイート
```javascript
// Playwright E2Eテスト
import { test, expect } from '@playwright/test';

test.describe('Leaflet WebGL Hybrid POC Performance Tests', () => {
  test('初回ロード3秒以内', async ({ page }) => {
    const startTime = Date.now();
    
    await page.goto('http://localhost:8080');
    await page.waitForSelector('.game-ready', { timeout: 8000 });
    
    const loadTime = Date.now() - startTime;
    expect(loadTime).toBeLessThan(3000);
  });
  
  test('カオスモード入力遅延', async ({ page }) => {
    await page.goto('http://localhost:8080/#/chaos/3');
    
    // パフォーマンス測定開始
    const metrics = await page.evaluate(() => {
      return new Promise((resolve) => {
        const results = [];
        let clickCount = 0;
        
        document.addEventListener('click', (e) => {
          const responseTime = performance.now() - e.timeStamp;
          results.push(responseTime);
          clickCount++;
          
          if (clickCount >= 10) {
            resolve({
              avg: results.reduce((a, b) => a + b) / results.length,
              max: Math.max(...results)
            });
          }
        });
      });
    });
    
    expect(metrics.max).toBeLessThan(200);
  });
});
```

#### Day 15: 最終判断会議
```markdown
## 技術スタック判断チェックリスト

### ✅ 必須要件
- [ ] 10,000オブジェクト表示
  - [ ] Leaflet DOM: _____ FPS
  - [ ] Canvas: _____ FPS  
  - [ ] WebGL: _____ FPS
  - [ ] Hybrid: _____ FPS

- [ ] 初回ロード時間
  - [ ] 3G: _____ 秒
  - [ ] 4G: _____ 秒
  - [ ] WiFi: _____ 秒

- [ ] カオスモード安定性
  - [ ] Level 1: 入力遅延 _____ ms
  - [ ] Level 2: 入力遅延 _____ ms
  - [ ] Level 3: 入力遅延 _____ ms

### 📊 判断基準
1. **GO判断**: 全項目で目標値クリア
2. **条件付きGO**: 一部未達だが対策案あり
3. **PIVOT**: 代替技術スタックへ移行

### 🔄 代替案
Plan A失敗時:
- Pixi.js + TypeScriptへ全面移行
- Unity WebGLビルド
- Godot HTML5エクスポート
```

---

## 3. リスク対応マトリクス

| リスク | 発生確率 | 影響度 | 対応策 |
|------|---------|-------|-------|
| Leaflet DOM限界 | 高 | 高 | Week1で判明→即Canvas/WebGL検証 |
| WASMサイズ超過 | 中 | 中 | 機能分割＋遅延ロード |
| Safari互換性 | 高 | 低 | フォールバックUI準備 |
| 開発遅延 | 中 | 高 | 2名体制で並行検証 |

---

## 4. 成果物定義

### Week 1成果物
- `benchmark-report.html` - インタラクティブな性能比較レポート
- `rendering-recommendation.md` - 技術選定推奨
- `fallback-strategy.md` - プランB詳細

### Week 2成果物
- `wasm-size-report.txt` - 最適化前後のサイズ比較
- `loading-timeline.json` - 段階ロードのタイミングデータ
- `chaos-demo.mp4` - カオスレベル3の動作録画

### Week 3成果物
- `integration-test-results.pdf` - 統合テスト結果
- `go-nogo-decision.md` - 最終判断書
- `next-steps.md` - 本開発移行計画

---

## 5. 成功指標サマリー

```typescript
interface POCSuccessCriteria {
  rendering: {
    targetFPS: 60,
    targetObjects: 10000,
    acceptableMinFPS: 45
  },
  loading: {
    targetFCP: 3000, // ms
    targetWASMReady: 8000, // ms
    maxBundleSize: 5 * 1024 * 1024 // 5MB
  },
  chaos: {
    maxInputLatency: 200, // ms
    minPlayableFPS: 30,
    uiCorruptionTolerance: 0.8
  }
}
```

---

## 6. POC実施体制

### チーム構成
- **技術リード**: Rustアーキテクト（フルタイム）
- **フロントエンド**: Dioxus実装者（フルタイム）
- **QAエンジニア**: パフォーマンステスト専任（Week3）

### 日次進捗共有
- 毎日17:00にSlackで進捗報告
- 金曜日に週次デモ（15分）
- Blockerは即座にエスカレーション

---

これで「**3週間で技術的実現可能性を確定**」できます。特にWeek1の**レンダリング検証**で早期に限界が見えれば、すぐにPivotできる構成です。