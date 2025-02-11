use crate::ui::components::prelude::*;

use crate::{Diagnosis, DB, db::model::{Module, Document, Blob}};



pub struct Documents {
    db: DB,
    selected_module: usize,
    /*
     * Using String as hashmap key for easier debugging
     */
    selected_docs: HashMap<String, HashMap<String, bool>>,
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

    pub fn new() -> Self {
        let mut s = Self {
            db: DB::new().unwrap(),
            selected_module: 0,
            selected_docs: HashMap::new(),
        };

        let modules: Vec<Module> = s.db.get_modules_all().unwrap();

        for module in modules {

            let docs: HashMap<String, bool> = s.db
                .get_documents_by_id(module.id)
                .unwrap()
                .into_iter()
                .map(|item| (item.descriptor, false))
                .collect();

            let None = s.selected_docs.insert(module.name, docs) else {
                panic!("Key should not already exist");
            };

        }

        s
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        ui.separator();

        self.ui_documentview(ui);

        // TODO: logging
        // TODO: implement mounting & download

        ui.separator();

        if ui.button("Download").clicked() {
            self.download();
        }

    }

    fn ui_documentview(&mut self, ui: &mut egui::Ui) {

        self.ui_moduleselect(ui);

        let module_id = self.selected_module as i64 + 1;
        let documents = self.db.get_documents_by_id(module_id).unwrap();

        for doc in documents {
            let module = self.db.get_module_by_id(module_id).unwrap();

            let checked = &mut self.selected_docs
                .get_mut(&module.name)
                .unwrap()
                .get_mut(&doc.descriptor)
                .unwrap();

            ui.checkbox(checked, doc.descriptor);
        }

    }

    fn ui_moduleselect(&mut self, ui: &mut egui::Ui) {
        let modules = self.db.get_modules_all().unwrap();

        egui::ComboBox::from_label("Selected Module")
            .selected_text(&modules[self.selected_module].name)
            .show_ui(ui, |ui| {

                for (index, module) in modules.iter().enumerate() {
                    ui.selectable_value(&mut self.selected_module, index, &module.name);
                }

            }
            );
    }

    fn download(&self) {
        let module = self.db.get_module_by_id(self.selected_module as i64 + 1).unwrap();
        let documents = self.db.get_documents_by_id(module.id).unwrap();
        let docs_state = &self.selected_docs[&module.name];

        /* only keep documents that are actually selected */
        let selected_docs: Vec<Document> = documents
            .into_iter()
            .filter(|item| docs_state[&item.descriptor])
            .collect();

        let blobs: Vec<Blob> = selected_docs.into_iter().map(|item| item.document).collect();
        self.mount(blobs);

    }

    fn mount(&self, blobs: Vec<Blob>) {

        if blobs.len() == 0 {
            return;
        }

        let mountdir = "/mnt";
        let device = "/dev/sda1";

        let status: Option<i32> = std::process::Command::new("mount")
            .args([device, mountdir])
            .status()
            .expect("failed to execute process")
            .code();
        dbg!(status);

        let filename = "datasheet.txt";
        // TODO: handle unwrap
        std::fs::File::create_new(format!("{mountdir}/{filename}")).unwrap();

        let status: Option<i32> = std::process::Command::new("umount")
            .arg(mountdir)
            .status()
            .expect("failed to execute process")
            .code();
        dbg!(status);

    }

}
