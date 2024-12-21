use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use std::thread::JoinHandle;
use std::sync::mpsc;
use egui::Color32;

use crate::diagnosis::{Diagnosis, DiagnosisState, DiagnosisResult};
use crate::ui::{Show, Logger};
use crate::ui::{config, util};
use crate::diagnosis::{STATE_COUNT, DIAGNOSIS_STATE_REPRS};
use crate::ui::logger::LogLevel;






pub struct DiagnosisUi {
    diagnosis: Arc<Mutex<Diagnosis>>,
    logger:    Rc<RefCell<Logger>>,

    // the state in state machine diagram and 2. block off other ui elements
    // Handle to the diagnosis thread
    // None if diagnosis is not active
    diag_thread_handle: Option<JoinHandle<DiagnosisResult>>,
    receiver:           mpsc::Receiver<DiagnosisState>,
    diag_state:         DiagnosisState, // UI needs to keep track of current diagnosis state to: 1. show

}

impl Show for DiagnosisUi {
    fn name(&self) -> &'static str {
        config::PAGE_DIAGNOSIS
    }
    fn show(&mut self, ctx: &egui::Context, active: &mut bool) {
        egui::Window::new(self.name())
            .fade_in(true)
            .fade_out(true)
            .open(active)
            .enabled(true)
            .vscroll(true)
            .hscroll(true)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}


impl DiagnosisUi {

    pub fn new(
        diagnosis: Arc<Mutex<Diagnosis>>,
        logger:    Rc<RefCell<Logger>>,
        receiver:  mpsc::Receiver<DiagnosisState>,
    ) -> Self {
        Self {
            diagnosis,
            logger,
            receiver,
            diag_thread_handle: None,
            diag_state: DiagnosisState::default(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        // Receive new state from running diagnosis
        if let Ok(state) = self.receiver.try_recv() {
            self.diag_state = state;
        }


        self.ui_legend(ui);


        util::canvas_new(ui).show(ui, |ui| {
            self.canvas_statemachine(ui);
        });


        ui.label(format!("Status: {}", self.diag_state));

        // TODO: this
        /*
        let selected = &mut self.diagnosis.lock().unwrap().mode;

        egui::ComboBox::from_label("Modus")
        .selected_text(format!("{:?}", selected))
        .show_ui(ui, |ui| {
        ui.selectable_value(selected, DiagnosisMode::Automatic, "Automatisch");
        ui.selectable_value(selected, DiagnosisMode::Manual,    "Manuell");
        });
        */

        let is_running = self.diag_thread_handle.is_some();




        let btn_start: egui::Response = ui.add_enabled(
            !is_running,
            egui::Button::new("Start")
        );

        if btn_start.clicked() {
            assert!(self.diag_thread_handle.is_none(), "Diagnosis is already running");

            let diag = self.diagnosis.clone();

            let handle = std::thread::Builder::new()
                .name("diagnosis".to_owned())
                .spawn(move || {
                    diag.lock().unwrap().run_to_end()
                }).unwrap();

            self.diag_thread_handle = Some(handle);
        }




        /*
        // BUG: blocks UI when running diagnosis
        let mode = self.diagnosis.lock().unwrap().mode;

        use DiagnosisMode as M;
        match mode {

        M::Automatic => {
        }

        M::Manual => {

        ui.horizontal(|ui| {
        if ui.button("Next").clicked() {
        todo!();
        }

        if ui.button("Repeat").clicked() {
        todo!();
        }

        if ui.button("Loop").clicked() {
        todo!();
        }
        });
        }
        }
        */



        let logger = &mut self.logger.borrow_mut();

        if let Some(h) = &self.diag_thread_handle {

            if h.is_finished() {
                let handle = Option::take(&mut self.diag_thread_handle).unwrap();
                let result: DiagnosisResult = handle.join().unwrap();

                match result {
                    Ok(value) => {
                        println!("Diagnosis was successful!");
                        logger.append(LogLevel::Info, "Diagnosis successful");
                    }
                    Err(error) => {
                        println!("Diagnosis failed!");
                        logger.append(LogLevel::Error, "Diagnosis failed");
                    }
                }

            }

        }


    }

    fn ui_legend(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Legend", |ui| {
            ui.horizontal_wrapped(|ui| {

                let state = self.diag_state as usize;

                for i in 0..STATE_COUNT {

                    let color = if i == state {
                        config::COLOR_ACCENT
                    } else {
                        Color32::GRAY
                    };

                    ui.colored_label(color, format!("{i}"));
                    ui.colored_label(Color32::DARK_GRAY, "->");
                    ui.colored_label(color, DIAGNOSIS_STATE_REPRS[i]);
                    ui.end_row();
                }

            });
        });
    }


    fn canvas_statemachine(&mut self, ui: &mut egui::Ui) {
        use egui::{vec2, Stroke};

        let width:  f32 = ui.available_width();
        let height: f32 = 150.0;
        let (response, painter, center) = util::canvas_setup(ui, width, height);

        let gap               = 30.0;                         // space between circles
        let segment_size      = width / (STATE_COUNT as f32); // +1 for extra space at the sides
        let radius            = (segment_size - gap) / 2.0;
        let offset            = radius * 2.0 + gap;               // distance to next circle center from current circle center
        let offset_to_origin  = width / 2.0 - segment_size / 2.0; // offset at the very left for the starting circle
        let outline_thickness = 1.5;

        let mut font = egui::FontId::default();
        font.size = 15.0;
        // font.size = radius * 1.3; // NOTE: resizing will cause lag at first, because new font size is not cached yet

        // TODO: dont show anything of available_width is smaller than min size
        // TODO: increase font step-wise
        // TODO: hover popup for descriptions

        let state = self.diag_state as usize;

        for i in 0..STATE_COUNT {

            let mut color_circle = if i == state {
                config::COLOR_ACCENT
            } else {
                config::COLOR_CIRCLE
            };


            let circle_center = center
            - vec2(offset_to_origin, 0.0)
            + vec2(i as f32 * offset, 0.0);


            if let Some(pos) = response.hover_pos() {
                if pos.distance(circle_center) < radius {
                    color_circle = color_circle.gamma_multiply(0.75);
                }
            }


            painter.circle_filled(
                circle_center,
                radius + outline_thickness,
                Color32::BLACK
            );

            painter.circle_filled(
                circle_center,
                radius,
                color_circle
            );

            painter.text(
                circle_center,
                egui::Align2::CENTER_CENTER,
                format!("{i}"),
                font.clone(),
                Color32::BLACK
            );

            // Dont render arrow after the last state
            if i == STATE_COUNT - 1 {
                break;
            }

            painter.arrow(
                center - vec2(offset_to_origin - radius, 0.0)
                + vec2(i as f32 * offset, 0.0)
                + vec2(outline_thickness, 0.0),
                vec2(gap, 0.0)
                - vec2(outline_thickness*2.0, 0.0),
                Stroke::new(1.0, Color32::GRAY)
            );

        }

    }

}
