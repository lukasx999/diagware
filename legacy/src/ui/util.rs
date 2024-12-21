use std::sync::{Arc, Mutex, mpsc};

use crate::ui::GuiState;
use crate::diagnosis::{Diagnosis, DiagnosisState};
use crate::ui::config;



// Utility and helper functions

pub fn get_time() -> String {
    chrono::Local::now()
        .time()
        .format("%H:%M:%S")
        .to_string()
}

pub fn get_date() -> String {
    chrono::Local::now()
        .date_naive()
        .format("%d.%m.%Y")
        .to_string()
}

// Returns new state of `enabled`
pub fn new_window(
    ctx:     &egui::Context,
    enabled: bool,
    title:   &str,
    mut ui_callback: impl FnMut(&mut egui::Ui),
) -> bool {

    let mut active: bool = enabled;

    egui::Window::new(title)
        .fade_in(true)
        .fade_out(true)
        .open(&mut active)
        .show(ctx, |ui| {
            ui_callback(ui);
        });

    active

}

pub fn canvas_setup(
    ui: &mut egui::Ui,
    width: f32,
    height: f32
) -> (egui::Response, egui::Painter, egui::Pos2) {

    use egui::{vec2, Sense, Painter, Rect, Response};

    let (response, painter): (Response, Painter) = ui.allocate_painter(
        vec2(width, height),
        Sense::click_and_drag()
    );

    let rect: Rect = ui.allocate_at_least(
        vec2(0.0, 0.0),
        Sense::click()
    ).0;

    let center = rect.center()
    - vec2(0.0, height/2.0)
    + vec2(width/2.0, 0.0);

    (response, painter, center)

}

pub fn canvas_new(ui: &egui::Ui) -> egui::containers::Frame {
    egui::containers::Frame::canvas(ui.style())
        .rounding(config::CANVAS_ROUNDING)
        .outer_margin(10.0)
        // .stroke(egui::Stroke::new(1.0, config::COLOR_BACKGROUND))
        .fill(config::COLOR_BACKGROUND)
}
