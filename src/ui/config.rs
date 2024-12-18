use egui::Color32;


// Modify here!
pub const EXPERT_PASSWORD: &str = "foo";

pub const WINDOW_WIDTH:  f32 = 2300.0;
pub const WINDOW_HEIGHT: f32 = 1200.0;

// page titles and ids (must be unique)
pub const PAGE_DIAGNOSIS:     &str = "Diagnosis";
pub const PAGE_DBMANAGEMENT:  &str = "DB-Manager";
pub const PAGE_SERIALMANAGER: &str = "Serial Management";
pub const PAGE_PINEDITOR:     &str = "Pin Editor";
pub const PAGE_LOGGING:       &str = "Logging";

// canvas elements
pub const COLOR_BACKGROUND:  Color32 = Color32::from_rgb(35,  35 , 35 );
pub const COLOR_ACCENT:      Color32 = Color32::from_rgb(41,  110, 214);
pub const COLOR_CIRCLE:      Color32 = Color32::from_rgb(178, 183, 191);
pub const CANVAS_ROUNDING:   f32     = 10.0;

pub const COLOR_LOG_INFO:    Color32 = Color32::BLUE;
pub const COLOR_LOG_WARNING: Color32 = Color32::ORANGE;
pub const COLOR_LOG_ERROR:   Color32 = Color32::RED;
