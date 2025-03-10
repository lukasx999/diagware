use std::time::Duration;
use egui::Color32;

// TODO: refactor all variables to config.rs
// problem: all variables must be const

// Modify here!
pub const EXPERT_PASSWORD: &str = "Kennwort0";

pub const WINDOW_WIDTH:  f32 = 2300.0;
pub const WINDOW_HEIGHT: f32 = 1200.0;

// canvas elements
pub const COLOR_BACKGROUND:        Color32 = Color32::from_rgb(35,  35,   35);
pub const COLOR_ACCENT:            Color32 = Color32::from_rgb(41,  110, 214);
pub const COLOR_CIRCLE:            Color32 = Color32::from_rgb(178, 183, 191);
pub const COLOR_CIRCLE_BREAKPOINT: Color32 = Color32::from_rgb(176, 14,   33);

pub const COLOR_LOG_INFO:    Color32 = Color32::BLUE;
pub const COLOR_LOG_WARNING: Color32 = Color32::ORANGE;
pub const COLOR_LOG_ERROR:   Color32 = Color32::RED;


pub const DIAGNOSIS_LOOP_CYLCE: Duration = Duration::from_millis(250);
pub const DIAGNOSIS_MODULE_TOLERANCE: f32 = 5.0;
