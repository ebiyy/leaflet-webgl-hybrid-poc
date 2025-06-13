use dioxus::prelude::*;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct BenchmarkMetrics {
    pub fps: f64,
    pub min_fps: f64,
    pub max_fps: f64,
    pub avg_fps: f64,
    pub frame_count: u32,
    pub start_time: Option<Instant>,
    pub memory_snapshots: Vec<f64>,
}

impl Default for BenchmarkMetrics {
    fn default() -> Self {
        Self {
            fps: 0.0,
            min_fps: 0.0,
            max_fps: 0.0,
            avg_fps: 0.0,
            frame_count: 0,
            start_time: None,
            memory_snapshots: Vec::new(),
        }
    }
}

impl BenchmarkMetrics {
    pub fn update_fps(&mut self, current_fps: f64) {
        self.frame_count += 1;
        self.fps = current_fps;
        
        if self.min_fps == 0.0 || current_fps < self.min_fps {
            self.min_fps = current_fps;
        }
        if current_fps > self.max_fps {
            self.max_fps = current_fps;
        }
        
        // 移動平均でavg_fpsを計算
        self.avg_fps = (self.avg_fps * (self.frame_count - 1) as f64 + current_fps) / self.frame_count as f64;
    }
    
    pub fn add_memory_snapshot(&mut self, memory_mb: f64) {
        self.memory_snapshots.push(memory_mb);
        // 最新100件のみ保持
        if self.memory_snapshots.len() > 100 {
            self.memory_snapshots.remove(0);
        }
    }
}

#[derive(Clone)]
pub struct BenchmarkHandle {
    pub metrics: Signal<BenchmarkMetrics>,
    pub is_recording: Signal<bool>,
    pub performance_score: Memo<PerformanceScore>,
    pub recommendations: Memo<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PerformanceScore {
    Excellent,
    Good,
    Fair,
    Poor,
}

impl PerformanceScore {
    pub fn color(&self) -> &'static str {
        match self {
            Self::Excellent => "#4CAF50",
            Self::Good => "#8BC34A",
            Self::Fair => "#FF9800",
            Self::Poor => "#f44336",
        }
    }
    
    pub fn text(&self) -> &'static str {
        match self {
            Self::Excellent => "優秀",
            Self::Good => "良好",
            Self::Fair => "可",
            Self::Poor => "要改善",
        }
    }
}

/// ベンチマーク管理用のカスタムフック
pub fn use_benchmark() -> BenchmarkHandle {
    let metrics = use_signal(BenchmarkMetrics::default);
    let is_recording = use_signal(|| false);
    
    // パフォーマンススコアを計算（メモ化）
    let performance_score = use_memo(move || {
        let avg_fps = metrics.read().avg_fps;
        match avg_fps {
            f if f >= 55.0 => PerformanceScore::Excellent,
            f if f >= 45.0 => PerformanceScore::Good,
            f if f >= 30.0 => PerformanceScore::Fair,
            _ => PerformanceScore::Poor,
        }
    });
    
    // 推奨事項を生成（メモ化）
    let recommendations = use_memo(move || {
        let m = metrics.read();
        let mut recs = Vec::new();
        
        if m.avg_fps < 30.0 {
            recs.push("オブジェクト数を減らすことを検討してください".to_string());
        }
        
        if m.max_fps - m.min_fps > 20.0 {
            recs.push("FPSの変動が大きいです。処理の最適化を検討してください".to_string());
        }
        
        let sum: f64 = m.memory_snapshots.iter().sum();
        if !m.memory_snapshots.is_empty() {
            let avg_memory = sum / m.memory_snapshots.len() as f64;
            if avg_memory > 100.0 {
                recs.push("メモリ使用量が高いです。不要なオブジェクトの削除を検討してください".to_string());
            }
        }
        
        if recs.is_empty() {
            recs.push("パフォーマンスは良好です！".to_string());
        }
        
        recs
    });
    
    BenchmarkHandle {
        metrics,
        is_recording,
        performance_score,
        recommendations,
    }
}

impl BenchmarkHandle {
    pub fn start_recording(&mut self) {
        self.metrics.write().start_time = Some(Instant::now());
        self.metrics.write().frame_count = 0;
        self.metrics.write().memory_snapshots.clear();
        self.is_recording.set(true);
    }
    
    pub fn stop_recording(&mut self) {
        self.is_recording.set(false);
    }
    
    pub fn update_fps(&mut self, fps: f64) {
        if *self.is_recording.read() {
            self.metrics.write().update_fps(fps);
        }
    }
    
    pub fn record_memory(&mut self, memory_mb: f64) {
        if *self.is_recording.read() {
            self.metrics.write().add_memory_snapshot(memory_mb);
        }
    }
}