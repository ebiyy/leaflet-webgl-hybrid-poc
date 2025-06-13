use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum RenderMode {
    DOM,
    Canvas,
    WebGL,
}

impl RenderMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            RenderMode::DOM => "DOM",
            RenderMode::Canvas => "Canvas",
            RenderMode::WebGL => "WebGL",
        }
    }
}

#[derive(Clone, Debug)]
pub struct MapConfig {
    pub object_count: i32,
    pub render_mode: RenderMode,
    pub animation_speed: f32,
    pub auto_pan: bool,
    pub show_fps: bool,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            object_count: 1000,
            render_mode: RenderMode::DOM,
            animation_speed: 1.0,
            auto_pan: false,
            show_fps: true,
        }
    }
}

pub struct MapConfigActions {
    pub set_object_count: Box<dyn Fn(i32)>,
    pub set_render_mode: Box<dyn Fn(RenderMode)>,
    pub set_animation_speed: Box<dyn Fn(f32)>,
    pub toggle_auto_pan: Box<dyn Fn()>,
    pub toggle_fps_display: Box<dyn Fn()>,
    pub is_high_performance: Memo<bool>,
    pub recommended_mode: Memo<RenderMode>,
}

/// カスタムフック: マップ設定の統一管理
pub fn use_map_config() -> (Signal<MapConfig>, MapConfigActions) {
    let config = use_signal(MapConfig::default);
    
    // 高パフォーマンスモードの判定をメモ化
    let is_high_performance = use_memo(move || {
        config.read().object_count > 5000
    });
    
    // 推奨レンダリングモードをメモ化
    let recommended_mode = use_memo(move || {
        let count = config.read().object_count;
        match count {
            0..=1000 => RenderMode::DOM,
            1001..=10000 => RenderMode::Canvas,
            _ => RenderMode::WebGL,
        }
    });
    
    let actions = MapConfigActions {
        set_object_count: Box::new(move |count| {
            let mut c = config;
            c.write().object_count = count.max(0).min(100000);
        }),
        set_render_mode: Box::new(move |mode| {
            let mut c = config;
            c.write().render_mode = mode;
        }),
        set_animation_speed: Box::new(move |speed| {
            let mut c = config;
            c.write().animation_speed = speed.max(0.1).min(10.0);
        }),
        toggle_auto_pan: Box::new(move || {
            let mut c = config;
            let current = c.read().auto_pan;
            c.write().auto_pan = !current;
        }),
        toggle_fps_display: Box::new(move || {
            let mut c = config;
            let current = c.read().show_fps;
            c.write().show_fps = !current;
        }),
        is_high_performance,
        recommended_mode,
    };
    
    (config, actions)
}