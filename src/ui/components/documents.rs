use crate::ui::components::prelude::*;

use crate::{Diagnosis, DB, db::model::{Module, Document}};
use crate::ui::{Component, Logger, config};




pub struct Documents {
    diagnosis:       Arc<Mutex<Diagnosis>>,
    logger:          Rc<RefCell<Logger>>,
    download_mode:   bool,
    selected_module: usize,
    selected_docs:   HashMap<String, HashMap<String, bool>>,
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
        let mut s = Self {
            diagnosis,
            logger,
            download_mode: false,
            selected_module: 0,
            selected_docs: HashMap::new(),
        };

        let diag = s.diagnosis.clone();
        let db = &diag.lock().unwrap().db;
        let modules: Vec<Module> = db.get_modules_all().unwrap();

        for module in modules {
            let docs: HashMap<String, bool> = db
                .get_documents_by_id(module.id)
                .unwrap()
                .into_iter()
                .map(|item| (item.descriptor, false))
                .collect();

            s.selected_docs.insert(module.name, docs).unwrap();
        }

        s
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        self.ui_downloadmode(ui);
        ui.separator();

        ui.toggle_value(&mut self.download_mode, "Toggle Download Mode");
        if self.download_mode {
        }

        let diag = self.diagnosis.clone();
        if let Ok(d) = diag.try_lock() {
            let db: &DB = &d.db;

            self.ui_moduleselect(ui, &db);

            let documents = db.get_documents_by_id(self.selected_module as i64).unwrap();
            for doc in documents {

                let module = &db.get_modules_all().unwrap()[self.selected_module];

                let checked: &mut bool = &mut self.selected_docs[&module.name][&doc.descriptor];
                ui.checkbox(&mut false, doc.descriptor);
            }

        } else {
            ui.label("Unavailable");
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

    fn ui_moduleselect(&mut self, ui: &mut egui::Ui, db: &DB) {
        let modules = db.get_modules_all().unwrap();

        egui::ComboBox::from_label("Selected Module")
            .selected_text(&modules[self.selected_module].name)
            .show_ui(ui, |ui| {
                for (index, module) in modules.iter().enumerate() {
                    ui.selectable_value(&mut self.selected_module, index, &module.name);
                }
            }
            );
    }

    fn ui_downloadmode(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Download Mode:").strong());

            let (label, color) = if self.download_mode {
                ("On", Color32::GREEN)
            } else {
                ("Off", Color32::RED)
            };

            ui.label(egui::RichText::new(label).color(color).strong());

        });
    }



}
