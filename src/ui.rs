use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::logger::Logger;

mod components;
use components::topbar::Topbar;

pub const WINDOW_WIDTH:  f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

pub trait Component {
    fn name(&self) -> &'static str; // MUST be unique, as its a key for a hashtable
    fn show(&mut self, ctx: &egui::Context, active: &mut bool);
}

struct GuiApplication {
    // Shared data:
    show_windowlist: Rc<RefCell<bool>>,

    windows:       Vec<Box<dyn Component>>,
    windows_state: HashMap<&'static str, bool>,
    topbar:        Topbar,
}

impl GuiApplication {

    pub fn new() -> Self {

        use components::{
            serialmanager::Serialmanager,
            diagnosis::DiagnosisUi,
            logging::Logging,
            documents::Documents,
        };

        let show_windowlist = Rc::new(RefCell::new(true));
        let is_expertmode   = Rc::new(RefCell::new(false));
        let logger          = Rc::new(RefCell::new(Logger::new()));

        let windows: Vec<Box<dyn Component>> = vec![
            Box::new(DiagnosisUi  ::new(logger.clone())),
            Box::new(Serialmanager::new(logger.clone(), is_expertmode.clone())),
            Box::new(Logging      ::new(logger.clone())),
            Box::new(Documents    ::new(logger.clone())),
        ];

        Self {
            topbar: Topbar::new(
                logger.clone(),
                show_windowlist.clone(),
                is_expertmode.clone()
            ),
            windows_state: windows
                .iter()
                .map(|item| (item.name(), false))
                .collect(),
            windows,
            show_windowlist,
        }

    }

}



impl eframe::App for GuiApplication {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Visual options
        ctx.set_pixels_per_point(2.0);
        ctx.set_theme(egui::Theme::Dark);

        // egui only redraws UI when the position of the mouse cursor changes,
        // therefore, to show the changing of states, we have to explicitly
        // redraw the ui every frame
        ctx.request_repaint();

        let mut topbar_active = true;
        self.topbar.show(ctx, &mut topbar_active);

        egui::SidePanel::left("Windows")
            .show_animated(ctx, *self.show_windowlist.borrow(), |ui| {

                for window in &mut self.windows {
                    let active = self.windows_state
                        .get_mut(window.name())
                        .unwrap();
                    ui.toggle_value(active, window.name());
                }

            });

        for window in &mut self.windows {
            let active = self.windows_state
                .get_mut(window.name())
                .unwrap();
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
            .with_resizable(true)
            .with_fullscreen(false)
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        centered: true,
        ..Default::default()
    }

}

pub fn run_gui() -> eframe::Result {

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

            Ok(Box::new(GuiApplication::new()))

        }),
    )

}
