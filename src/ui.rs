use std::sync::{Arc, Mutex, mpsc};

use crate::diagnosis::{Diagnosis, DiagnosisState};

mod gui;
mod util;

mod config;
use config::{
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    PAGE_DIAGNOSIS,
    PAGE_DBMANAGEMENT,
    PAGE_SERIALMANAGER,
    COLOR_BACKGROUND,
    COLOR_ACTIVESTATE,
    COLOR_STATE,
};






struct GuiState {
    diagnosis:     Arc<Mutex<Diagnosis>>, // All HW/SW interfaces are owned by the diagnosis
    diag_sender:   mpsc::Sender<DiagnosisState>,
    diag_receiver: mpsc::Receiver<DiagnosisState>,
    diag_state:    DiagnosisState, // UI needs to keep track of current diagnosis state to: 1. show
                                   // the state in state machine diagram and 2. block off other ui elements

    is_expert_mode:     bool,

    show_windowlist:    bool,
    show_diagnosis:     bool,
    show_dbmanager:     bool,
    show_serialmanager: bool,
    show_pineditor:     bool,

}



impl GuiState {

    pub fn new(
        diagnosis: Diagnosis
    ) -> Self {

        let (tx, rx) = mpsc::channel();

        Self {
            diagnosis:     Arc::new(Mutex::new(diagnosis)),
            diag_sender:   tx,
            diag_receiver: rx,
            diag_state:    DiagnosisState::default(),

            is_expert_mode:  false,
            show_windowlist: true,

            show_diagnosis:     true,
            show_dbmanager:     false,
            show_serialmanager: false,
            show_pineditor:     false,

        }

    }

}





impl eframe::App for GuiState {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Config
        ctx.set_pixels_per_point(2.0);
        ctx.set_theme(egui::Theme::Dark);

        ctx.request_repaint(); // NOTE: egui only redraws UI when the position of the mouse cursor
                               // changes, therefore, to show the changing of states, we have to explicitly redraw the ui
                               // every frame

        // Receive new state from running diagnosis
        if let Ok(state) = self.diag_receiver.try_recv() {
            self.diag_state = state;
        }


        egui::TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            self.ui_topbar(&ctx, ui);
        });


        egui::SidePanel::left("WindowList")
            .show_animated(ctx, self.show_windowlist, |ui| {
                ui.toggle_value(&mut self.show_dbmanager,     PAGE_DBMANAGEMENT);
                ui.toggle_value(&mut self.show_diagnosis,     PAGE_DIAGNOSIS);
                ui.toggle_value(&mut self.show_serialmanager, PAGE_SERIALMANAGER);
                ui.toggle_value(&mut self.show_pineditor,     "Pin Editor");
            });


        self.show_dbmanager =
            util::new_window(ctx, self.show_dbmanager, PAGE_DBMANAGEMENT, |ui| {
                self.ui_dbmanager(ui);
            });

        self.show_serialmanager =
            util::new_window(ctx, self.show_serialmanager, PAGE_SERIALMANAGER, |ui| {
                self.ui_serialmanager(ui);
            });

        self.show_pineditor =
            util::new_window(ctx, self.show_pineditor, "Pin Editor", |ui| {
                self.ui_pineditor(&ctx, ui);
            });


        // TODO: refactor this into a macro!
        // TODO: min_width()
        let mut active = self.show_diagnosis;

        egui::Window::new(PAGE_DIAGNOSIS)
            .fade_in(true)
            .fade_out(true)
            .open(&mut active)
            .enabled(true)
            .show(ctx, |ui| {
                self.ui_diagnosis(ui);
            });

        self.show_diagnosis = active;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::containers::Frame::default().show(ui, |_ui| ());
        });


    }
}



fn frame_setup() -> eframe::NativeOptions {

    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Diagware")
            // .with_resizable(true)
            // .with_fullscreen(false)
            // .with_maximized(true)
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        centered: true,
        ..Default::default()
    }

}


pub fn run_gui(
    diagnosis: Diagnosis
) -> eframe::Result {

    let options: eframe::NativeOptions = frame_setup();

    eframe::run_native(
        "Diagware",
        options,
        Box::new(|cc| {

            // Image support
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Phosphor icons
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(GuiState::new(diagnosis)))

        }),
    )

}
