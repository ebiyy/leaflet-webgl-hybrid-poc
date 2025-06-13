use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct MapMarkerData {
    pub id: String,
    pub lat: f64,
    pub lng: f64,
    pub velocity: Velocity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct Velocity {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct PerformanceMetrics {
    pub fps: f64,
    pub memory_usage: Option<f64>,
    pub dropped_frames: u32,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct MapConfig {
    pub object_count: i32,
    pub render_mode: String,
    pub animation_speed: f32,
    pub auto_pan: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct ChaosEvent {
    pub event_type: String,
    pub timestamp: f64,
    pub description: String,
    pub payload: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(feature = "typescript")]
    fn export_typescript_bindings() {
        // TypeScript\u5b9a\u7fa9\u3092\u751f\u6210
        use std::fs;
        use std::path::Path;
        
        let bindings_dir = Path::new("bindings");
        fs::create_dir_all(bindings_dir).expect("Failed to create bindings directory");
        
        // \u5404\u578b\u306eTypeScript\u5b9a\u7fa9\u3092\u30a8\u30af\u30b9\u30dd\u30fc\u30c8
        let types = vec![
            MapMarkerData::export_to_string().expect("Failed to export MapMarkerData"),
            Velocity::export_to_string().expect("Failed to export Velocity"),
            PerformanceMetrics::export_to_string().expect("Failed to export PerformanceMetrics"),
            MapConfig::export_to_string().expect("Failed to export MapConfig"),
            ChaosEvent::export_to_string().expect("Failed to export ChaosEvent"),
        ];
        
        let combined = types.join("\n\n");
        fs::write(bindings_dir.join("types.d.ts"), combined)
            .expect("Failed to write TypeScript definitions");
    }
}