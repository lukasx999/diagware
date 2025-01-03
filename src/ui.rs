use std::sync::{Arc, Mutex, mpsc};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::diagnosis::{Diagnosis, State};

mod util;
mod config;

mod logger;
use logger::Logger;

mod components;
use components::topbar::Topbar;





pub trait Component {
    fn name(&self) -> &'static str; // MUST be unique
    fn show(&mut self, ctx: &egui::Context, active: &mut bool);
}



struct GuiApplication {
    // Shared data:
    diagnosis:       Arc<Mutex<Diagnosis>>, // All HW interfaces are owned by the diagnosis
    logger:          Rc<RefCell<Logger>>,
    show_windowlist: Rc<RefCell<bool>>,
    is_expertmode:   Rc<RefCell<bool>>,

    windows:       Vec<Box<dyn Component>>,
    windows_state: HashMap<&'static str, bool>,

    topbar: Topbar,
}




impl GuiApplication {

    pub fn new(
        diagnosis: Diagnosis,
        receiver:  mpsc::Receiver<State>,
    ) -> Self {
        use components::{
            serialmanager::Serialmanager,
            pineditor    ::Pineditor,
            diagnosis    ::DiagnosisUi,
            dbmanager    ::DBManager,
            logging      ::Logging,
            documents    ::Documents,
        };

        let diagnosis       = Arc::new(Mutex::new(diagnosis));
        let logger          = Rc::new(RefCell::new(Logger::new()));
        let show_windowlist = Rc::new(RefCell::new(true));
        let is_expertmode   = Rc::new(RefCell::new(false));

        let windows: Vec<Box<dyn Component>> = vec![
            Box::new(Serialmanager::new(diagnosis.clone(), logger.clone())),
            Box::new(Pineditor    ::new()),
            Box::new(DiagnosisUi  ::new(diagnosis.clone(), logger.clone(), receiver)),
            Box::new(DBManager    ::new(diagnosis.clone(), logger.clone())),
            Box::new(Logging      ::new(logger.clone())),
            Box::new(Documents    ::new(logger.clone())),
        ];

        let mut windows_state = HashMap::new();
        for window in &windows {
            let state = window.name() == "Diagnosis"; // TODO:
            windows_state.insert(window.name(), state);
        }

        Self {
            diagnosis,

            topbar: Topbar::new(
                show_windowlist.clone(),
                is_expertmode.clone(),
                logger.clone()
            ),

            logger,
            show_windowlist,
            is_expertmode,
            windows,
            windows_state,
        }

    }

}



impl eframe::App for GuiApplication {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        /* Config */
        ctx.set_pixels_per_point(2.0);
        ctx.set_theme(egui::Theme::Dark);

        ctx.request_repaint(); // NOTE: egui only redraws UI when the position of the mouse cursor
        // changes, therefore, to show the changing of states, we have to explicitly redraw the ui
        // every frame

        let mut topbar_active = true;
        self.topbar.show(ctx, &mut topbar_active);

        egui::SidePanel::left("Windows")
            .show_animated(ctx, *self.show_windowlist.borrow(), |ui| {
                for window in &mut self.windows {
                    let active = self.windows_state.get_mut(window.name()).unwrap();
                    ui.toggle_value(active, window.name());
                }
            });

        for window in &mut self.windows {
            let active = self.windows_state.get_mut(window.name()).unwrap();
            window.show(ctx, active);
        }

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
            .with_inner_size([config::WINDOW_WIDTH, config::WINDOW_HEIGHT]),
        centered: true,
        ..Default::default()
    }

}


pub fn run_gui(
    diagnosis: Diagnosis,
    receiver:  mpsc::Receiver<State>
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

            Ok(Box::new(GuiApplication::new(diagnosis, receiver)))

        }),
    )

}
