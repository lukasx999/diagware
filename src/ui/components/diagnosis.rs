use std::sync::{Arc, Mutex, mpsc};
use std::rc::Rc;
use std::cell::RefCell;
use std::thread::JoinHandle;

use egui::Color32;

use crate::{
    diagnosis::{
        self as diag,
        Diagnosis,
        DiagnosisResult,
        State,
        STATE_COUNT,
    },
    ui::{
        Component,
        config,
        util,
        Logger,
        logger::LogLevel
    },
};




#[derive(Debug, Clone, Copy, Default)]
enum ModuleGist {
    #[default] NotYetMeasured,
    Pending,
    Defective,
    Functional,
}

impl ModuleGist {
    pub fn get_richtext(&self) -> egui::RichText {
        use ModuleGist as M;
        let (text, color) = match self {
            M::NotYetMeasured => ("No Measurements yet",  Color32::GRAY),
            M::Pending        => ("Gist is pending...",   Color32::WHITE),
            M::Defective      => ("Module is defective",  Color32::RED),
            M::Functional     => ("Module is functional", Color32::GREEN),
        };
        egui::RichText::new(text).strong().color(color)
    }
}




pub struct DiagnosisUi {
    diagnosis: Arc<Mutex<Diagnosis>>,
    logger:    Rc<RefCell<Logger>>,

    // the state in state machine diagram and 2. block off other ui elements
    // Handle to the diagnosis thread
    // None if diagnosis is not active
    diag_thread_handle: Option<JoinHandle<DiagnosisResult>>,
    receiver:           mpsc::Receiver<State>,
    diag_state:         State, // UI needs to keep track of current diagnosis state to: 1. show

    // is_looping: Arc<Mutex<bool>>,

    diagnosis_report: ModuleGist,
}

impl Component for DiagnosisUi {
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
        receiver:  mpsc::Receiver<State>,
    ) -> Self {
        Self {
            diagnosis,
            logger,
            receiver,
            diag_thread_handle: None,
            diag_state: State::default(),

            // is_looping: Arc::new(Mutex::new(false)),

            diagnosis_report: ModuleGist::default(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::Button;

        // Receive new state from running diagnosis
        if let Ok(state) = self.receiver.try_recv() {
            self.diag_state = state;
        }

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("State:").strong());
            ui.label(format!("{}", self.diag_state));
        });
        ui.separator();


        let is_running = self.diag_thread_handle.is_some();

        ui.horizontal(|ui| {

            if ui.add_enabled(!is_running, Button::new("Start")).clicked() {
                self.spawn_diag_thread(|diag| diag.run_to_end());
            }

            if ui.add_enabled(!is_running, Button::new("Next")).clicked() {
                self.spawn_diag_thread(|diag| diag.run_and_next());
            }

            if ui.add_enabled(!is_running, Button::new("Repeat")).clicked() {
                self.spawn_diag_thread(|diag| diag.run_state());
            }

            if ui.add_enabled(!is_running, Button::new("Loop")).clicked() {
                todo!();
                /*
                *self.is_looping.lock().unwrap() = true;
                let is_looping = self.is_looping.clone();
                self.spawn_diag_thread(move |diag| {
                    loop {
                        println!("looping");
                        let is_looping = is_looping.lock().unwrap();
                        let ret = diag.run_state();
                        if !*is_looping {
                            break ret;
                        }
                    }
                });
                */
            }

            if ui.add_enabled(!is_running, Button::new("Reset")).clicked() {
                self.diagnosis_report = ModuleGist::NotYetMeasured;
                self.diagnosis.lock().unwrap().reset_state();
            }

            /*
            if ui.add_enabled(*self.is_looping.lock().unwrap(), Button::new("Cancel")).clicked() {
                *self.is_looping.lock().unwrap() = false;
            }
            */

        });

        util::canvas_new(ui).show(ui, |ui| {
            self.render_statemachine(ui);
        });

        ui.separator();
        ui.label(self.diagnosis_report.get_richtext());
        ui.separator();
        self.ui_legend(ui);


        let logger = &mut self.logger.borrow_mut();

        if let Some(h) = &self.diag_thread_handle {

            if h.is_finished() {
                let handle = Option::take(&mut self.diag_thread_handle).unwrap();
                let result: DiagnosisResult = handle.join().unwrap();

                match result {
                    Ok(report) => {
                        logger.append(LogLevel::Info, "Diagnosis successful");

                        use diag::Report as R;
                        self.diagnosis_report = match report {
                            R::Pending => ModuleGist::Pending,
                            R::Completed { is_functional } => {
                                if is_functional {
                                    ModuleGist::Functional
                                } else {
                                    ModuleGist::Defective
                                }
                            }
                        };

                        dbg!(report);

                    }
                    Err(error) => {
                        logger.append(LogLevel::Error, "Diagnosis failed");
                        dbg!(error);
                    }
                }

            }

        }


    }


    // Launch a new thread, save the handle, and let the caller provide a callback receiving a
    // mutable reference to the diagnosis
    fn spawn_diag_thread<T>(&mut self, callback: T)
where T: Fn(&mut Diagnosis) -> DiagnosisResult + std::marker::Send + 'static
    {
        assert!(self.diag_thread_handle.is_none(), "Diagnosis is already running");

        let diag = self.diagnosis.clone();

        let handle = std::thread::Builder::new()
            .name("diagnosis".to_owned())
            .spawn(move || {
                callback(&mut diag.lock().unwrap())
            }).unwrap();

        self.diag_thread_handle = Some(handle);
    }


    fn ui_legend(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Legend", |ui| {
            ui.horizontal_wrapped(|ui| {

                let state = self.diag_state as u32;

                for i in 0..STATE_COUNT {

                    let color = if i == state {
                        config::COLOR_ACCENT
                    } else {
                        Color32::GRAY
                    };

                    ui.colored_label(color, format!("{i}"));
                    ui.colored_label(Color32::DARK_GRAY, "->");
                    let state = diag::State::from_u32(i);
                    ui.colored_label(color, state.to_string());
                    ui.end_row();
                }

            });
        });
    }


    fn render_statemachine(&mut self, ui: &mut egui::Ui) {
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


        for i in 0..STATE_COUNT {

            let state = self.diag_state as u32;
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
