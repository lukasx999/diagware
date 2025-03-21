use crate::ui::components::prelude::*;

use std::sync::mpsc;
use std::thread::JoinHandle;

use crate::diagnosis::{
    Diagnosis,
    DiagnosisResult,
    Report,
    State,
    STATE_COUNT,
};

const COLOR_ACCENT:     Color32 = Color32::from_rgb(41,  110, 214); // blue
const COLOR_REGULAR:    Color32 = Color32::from_rgb(178, 183, 191); // gray
const COLOR_BREAKPOINT: Color32 = Color32::from_rgb(176, 14,   33); // red

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
    logger: Rc<RefCell<Logger>>,

    diagnosis: Arc<Mutex<Diagnosis>>,
    // Handle to the diagnosis thread
    // None if diagnosis is not active
    diag_thread_handle: Option<JoinHandle<DiagnosisResult>>,
    // Receiving state from running diagnosis
    receiver:           mpsc::Receiver<State>,
    // current copy of state, updated by receiver
    diag_state:         State,

    breakpoint:     Option<State>,
    is_looping:     bool,
    diagnosis_gist: ModuleGist,
}

impl Component for DiagnosisUi {
    fn name(&self) -> &'static str {
        "Diagnosis"
    }
    fn show(&mut self, ctx: &egui::Context, active: &mut bool) {
        egui::Window::new(self.name())
            .fade_in(true)
            .fade_out(true)
            .open(active)
            .show(ctx, |ui| self.ui(ui));
    }
}



/* Core */
impl DiagnosisUi {

    pub fn new(logger: Rc<RefCell<Logger>>) -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            logger,

            diagnosis: Arc::new(Mutex::new(Diagnosis::new(tx).unwrap())),
            receiver: rx,
            diag_thread_handle: None,
            diag_state: State::default(),

            breakpoint:     None,
            is_looping:     false,
            diagnosis_gist: ModuleGist::default(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        /* Receive new state from running diagnosis */
        if let Ok(state) = self.receiver.try_recv() {
            self.diag_state = state;

            /* Reset breakpoint after reaching it */
            if self.breakpoint.is_some_and(|state| state == self.diag_state) {
                self.breakpoint = None;
            }

            let mut logger = self.logger.borrow_mut();
            logger.append(LogLevel::Info, format!("New State: {}", self.diag_state));
        }

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("State:").strong());
            ui.label(format!("{}", self.diag_state));
        });

        ui.separator();

        self.ui_controlpanel(ui);

        /* State Loop */
        let is_active = self.diag_thread_handle.is_some();
        if self.is_looping && !is_active {
            self.spawn_diag_thread(|diag| diag.run_state());
        }

        util::canvas_new(ui).show(ui, |ui| self.ui_statemachine(ui));

        ui.separator();
        ui.label(self.diagnosis_gist.get_richtext());
        ui.separator();
        self.ui_legend(ui);

        if let Some(h) = &self.diag_thread_handle {
            if h.is_finished() {
                let handle = Option::take(&mut self.diag_thread_handle).unwrap();
                let result: DiagnosisResult = handle.join().unwrap();
                self.handle_diagnosis_result(result);

            } else {
                self.diagnosis_gist = ModuleGist::Pending;
            }
        }

    }

}





/* Ui */
impl DiagnosisUi {
    fn ui_controlpanel(&mut self, ui: &mut egui::Ui) {
        use egui::Button;

        let is_running = self.diag_thread_handle.is_some();

        ui.horizontal(|ui| {

            if ui.add_enabled(!is_running, Button::new("Start")).clicked() {
                let bp = self.breakpoint;
                self.spawn_diag_thread(move |diag| diag.run_to_end(bp));
            }

            if ui.add_enabled(!is_running, Button::new("Next")).clicked() {
                self.spawn_diag_thread(|diag| diag.run_and_next());
            }

            if ui.add_enabled(!is_running, Button::new("Repeat")).clicked() {
                self.spawn_diag_thread(|diag| diag.run_state());
            }

            let (show_condition, label, new_loopstate) = if self.is_looping {
                (true, "Cancel", false)
            } else {
                (!is_running, "Loop", true)
            };

            if ui.add_enabled(show_condition, Button::new(label)).clicked() {
                self.is_looping = new_loopstate
            }

            if ui.add_enabled(!is_running, Button::new("Reset")).clicked() {
                self.diagnosis_gist = ModuleGist::NotYetMeasured;
                let mut diag = self.diagnosis.lock().unwrap();
                diag.reset_state();
                self.breakpoint = None;
            }

        });
    }

    fn ui_legend(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Legend", |ui| {
            ui.horizontal_wrapped(|ui| {
                let state = self.diag_state as u32;

                for i in 0..STATE_COUNT {

                    let is_breakpoint = self.breakpoint.is_some_and(|state| state as u32 == i);

                    let color = if i == state {
                        COLOR_ACCENT
                    } else if is_breakpoint {
                        COLOR_BREAKPOINT
                    } else {
                        Color32::GRAY
                    };

                    ui.colored_label(color, format!("{i}"));
                    ui.colored_label(Color32::DARK_GRAY, "->");
                    let state = State::from_u32(i);
                    ui.colored_label(color, state.to_string());
                    ui.end_row();
                }

            });
        });
    }


    fn ui_statemachine(&mut self, ui: &mut egui::Ui) {
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
        // font.size = radius * 1.3; // resizing will cause lag at first, because new font size is not cached yet

        // TODO: dont show anything if available_width is smaller than min size
        // TODO: increase font step-wise
        // TODO: hover popup for descriptions


        for i in 0..STATE_COUNT {

            let is_breakpoint = self.breakpoint.is_some_and(|state| state as u32 == i);
            let state_current = self.diag_state as u32;

            let mut color_circle = if i == state_current {
                COLOR_ACCENT
            } else if is_breakpoint {
                COLOR_BREAKPOINT
            } else {
                COLOR_REGULAR
            };

            let circle_center = center
            - vec2(offset_to_origin, 0.0)
            + vec2(i as f32 * offset, 0.0);

            let hovered = if let Some(pos) = response.hover_pos() {
                pos.distance(circle_center) < radius
            } else { false };

            let clicked = hovered && response.clicked();

            if hovered {
                color_circle = color_circle.gamma_multiply(0.75);
            }

            if clicked {
                /* dont set breakpoint at first, last or current state */
                if i != 0
                && i != STATE_COUNT - 1
                && i != state_current {

                    self.breakpoint =
                        if self.breakpoint.is_some_and(|j| j as u32 == i) {
                            None // reset active breakpoint by clicking on it again
                        } else {
                            Some(State::from_u32(i))
                        };

                }
            }

            // TODO: dynamically render the popup offset
            let popup_offset = 35.0;
            if hovered {
                painter.text(
                    circle_center - vec2(0.0, popup_offset),
                    egui::Align2::CENTER_CENTER,
                    State::from_u32(i).to_string(),
                    font.clone(),
                    Color32::WHITE
                );
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



/* Utility */
impl DiagnosisUi {

    fn handle_diagnosis_result(&mut self, result: DiagnosisResult) {
        let mut logger = self.logger.borrow_mut();

        match result {
            Ok(report) => {

                use Report as R;
                self.diagnosis_gist = match report {

                    R::Pending => ModuleGist::Pending,

                    R::Completed { is_functional } => {
                        logger.append(LogLevel::Info, "Diagnosis successful");
                        println!("Diagnosis successful");

                        if is_functional {
                            ModuleGist::Functional
                        } else {
                            ModuleGist::Defective
                        }
                    }

                };

            }

            Err(error) => {
                logger.append(LogLevel::Error, format!("Diagnosis failed: {error}"));
                println!("Diagnosis failed");
                dbg!(&error);
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
            .spawn(move ||
                callback(&mut diag.lock().unwrap())
            ).unwrap();

        self.diag_thread_handle = Some(handle);
    }

}
