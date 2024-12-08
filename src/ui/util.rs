use std::sync::{Arc, Mutex, mpsc};

use crate::ui::GuiState;
use crate::diagnosis::{Diagnosis, DiagnosisState};



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

pub fn ui_painting_setup(
    ui: &mut egui::Ui,
    width: f32,
    height: f32
) -> (egui::Painter, egui::Pos2) {

    use egui::{vec2, Sense, Painter, Rect, };

    let painter: Painter = ui.allocate_painter(
        vec2(width, height),
        Sense::hover()
    ).1;

    let rect: Rect = ui.allocate_at_least(
        vec2(0.0, 0.0),
        Sense::hover()
    ).0;

    let center = rect.center()
    - vec2(0.0, height/2.0)
    + vec2(width/2.0, 0.0);

    (painter, center)

}






impl GuiState {

    pub fn start_diagnosis(&self) {

        let diag = self.diagnosis.clone();
        let sender = self.diag_sender.clone();

        std::thread::Builder::new()
            .name("diagnosis".to_string())
            .spawn(move || {
                diag.lock()
                    .unwrap()
                    .diagnosis(sender)
                    .unwrap();
            }).unwrap();

    }

}
