use crate::ui::components::prelude::*;

use crate::Diagnosis;
use crate::ui::{Component, Logger, config};




pub struct Documents {
    diagnosis:       Arc<Mutex<Diagnosis>>,
    logger:          Rc<RefCell<Logger>>,
    download_mode:   bool,
    selected_module: usize,
    // TODO:
    // checked_documents: HashMap<String, bool>,
    // selected_module: i64,
}

impl Component for Documents {
    fn name(&self) -> &'static str {
        config::PAGE_DOCUMENTMANAGER
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

impl Documents {

    pub fn new(
        diagnosis: Arc<Mutex<Diagnosis>>,
        logger:    Rc<RefCell<Logger>>
    ) -> Self {
        Self {
            diagnosis,
            logger,
            download_mode: false,
            selected_module: 0,
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Download Mode:").strong());

            let (label, color) = if self.download_mode {
                ("On", Color32::GREEN)
            } else {
                ("Off", Color32::RED)
            };

            ui.label(egui::RichText::new(label).color(color).strong());

        });
        ui.separator();

        ui.toggle_value(&mut self.download_mode, "Toggle Download Mode");
        if self.download_mode {
        }

        let diag = self.diagnosis.clone();
        let db = &diag.lock().unwrap().db;
        let modules = db.get_modules_all().unwrap();


        egui::ComboBox::from_label("Selected Module")
            .selected_text(&modules[self.selected_module].name)
            .show_ui(ui, |ui| {
                for (index, module) in modules.iter().enumerate() {
                    ui.selectable_value(&mut self.selected_module, index, &module.name);
                }
            }
            );


        let documents = db.get_documents_by_id(self.selected_module as i64).unwrap();

        for doc in documents {
            ui.label(doc.descriptor);
        }

        // TODO: logging
        // TODO: document selector

        if ui.button("Mount").clicked() {

            let result = drives::get_devices();
            for device in result.unwrap() {
                dbg!(device);
            }


        }

        // let mut s = String::from("foo");
        // egui::ComboBox::from_label("Select one!")
        //     .selected_text(format!("{:?}", s))
        //     .show_ui(ui, |ui| {
        //         ui.selectable_value(&mut s, 1, "First".to_owned());
        //         ui.selectable_value(&mut s, 2, "Second");
        //         ui.selectable_value(&mut s, 3, "Third");
        //     }
            // );

    }

}
