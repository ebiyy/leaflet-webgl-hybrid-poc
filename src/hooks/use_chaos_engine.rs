use dioxus::prelude::*;
use crate::utils::interval::Interval;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChaosEventType {
    ObjectSpawn,
    ObjectRemove,
    ViewportChange,
    ModeSwitch,
    MemoryPressure,
}

#[derive(Clone, Debug)]
pub struct ChaosEvent {
    pub event_type: ChaosEventType,
    pub timestamp: Instant,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct ChaosEngine {
    pub intensity: u8,
    pub events: Vec<ChaosEvent>,
    pub is_running: bool,
    pub total_events: usize,
    pub start_time: Option<Instant>,
}

impl ChaosEngine {
    pub fn new(intensity: u8) -> Self {
        Self {
            intensity: intensity.min(10),
            events: Vec::new(),
            is_running: false,
            total_events: 0,
            start_time: None,
        }
    }
    
    pub fn add_event(&mut self, event_type: ChaosEventType, description: String) {
        self.events.push(ChaosEvent {
            event_type,
            timestamp: Instant::now(),
            description,
        });
        self.total_events += 1;
        
        // 最新100イベントのみ保持（メモリ効率のため）
        if self.events.len() > 100 {
            self.events.remove(0);
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct EventStats {
    pub total: usize,
    pub by_type: Vec<(ChaosEventType, usize)>,
    pub events_per_second: f64,
}

pub struct ChaosEngineHandle {
    pub engine: Signal<ChaosEngine>,
    pub interval: Signal<Option<Interval>>,
    pub event_stats: Memo<EventStats>,
    pub performance_monitor: Resource<Result<PerformanceReport, String>>,
}

#[derive(Clone, Debug)]
pub struct PerformanceReport {
    pub memory_usage: f64,
    pub fps_average: f64,
    pub dropped_frames: usize,
}

/// カオスパフォーマンスを監視する非同期関数
async fn monitor_chaos_performance(_engine: Signal<ChaosEngine>) -> Result<PerformanceReport, String> {
    // 実際の実装では、JavaScript側からパフォーマンスメトリクスを取得
    Ok(PerformanceReport {
        memory_usage: 0.0,
        fps_average: 60.0,
        dropped_frames: 0,
    })
}

/// イベント統計を計算するヘルパー関数
fn categorize_events(events: &[ChaosEvent]) -> Vec<(ChaosEventType, usize)> {
    use std::collections::HashMap;
    
    let mut counts = HashMap::new();
    for event in events {
        *counts.entry(event.event_type.clone()).or_insert(0) += 1;
    }
    
    let mut result: Vec<_> = counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1)); // 降順ソート
    result
}

/// カスタムフック: カオスエンジンの統一管理
pub fn use_chaos_engine(initial_intensity: u8) -> ChaosEngineHandle {
    let engine = use_signal(|| ChaosEngine::new(initial_intensity));
    let mut interval = use_signal(|| None::<Interval>);
    
    // イベント統計をメモ化
    let event_stats = use_memo(move || {
        let e = engine.read();
        let elapsed = e.start_time
            .map(|start| start.elapsed().as_secs_f64())
            .unwrap_or(1.0);
        
        EventStats {
            total: e.events.len(),
            by_type: categorize_events(&e.events),
            events_per_second: e.total_events as f64 / elapsed,
        }
    });
    
    // パフォーマンス監視をリソースとして管理
    let performance_monitor = use_resource(move || async move {
        monitor_chaos_performance(engine).await
    });
    
    // クリーンアップ: コンポーネントのアンマウント時にインターバルを停止
    use_drop(move || {
        if let Some(mut int) = interval.write().take() {
            int.stop();
        }
    });
    
    ChaosEngineHandle {
        engine,
        interval,
        event_stats,
        performance_monitor,
    }
}

impl ChaosEngineHandle {
    /// カオスエンジンを開始
    pub fn start(&mut self) {
        self.engine.write().is_running = true;
        self.engine.write().start_time = Some(Instant::now());
    }
    
    /// カオスエンジンを停止
    pub fn stop(&mut self) {
        self.engine.write().is_running = false;
        if let Some(mut int) = self.interval.write().take() {
            int.stop();
        }
    }
    
    /// 強度を変更
    pub fn set_intensity(&mut self, intensity: u8) {
        self.engine.write().intensity = intensity.min(10);
    }
    
    /// イベントを追加
    pub fn add_event(&mut self, event_type: ChaosEventType, description: String) {
        self.engine.write().add_event(event_type, description);
    }
}