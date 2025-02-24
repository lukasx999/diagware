use std::io::Write;
use std::fs::File;

use crate::ui::components::prelude::*;
use crate::{Diagnosis, DB, db::model::{Module, Document, Blob}};

use egui::containers::Modal;

const MOUNT_FAILURE: i32 = 32;


pub struct Documents {
    logger: Rc<RefCell<Logger>>,
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

    pub fn new(logger: Rc<RefCell<Logger>>) -> Self {
        let mut s = Self {
            logger,
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

        self.mount(selected_docs);

    }

    fn mount(&self, documents: Vec<Document>) {
        let mut logger = self.logger.borrow_mut();

        if documents.len() == 0 {
            logger.append(LogLevel::Warning, "No documents selected");
            return;
        }

        let mountdir = format!("{}/diag_mnt", env!("HOME"));
        //std::fs::create_dir().unwrap();
        let device = "/dev/sda1";


        let status: i32 = std::process::Command::new("mount")
            .args([device, &mountdir])
            .status()
            .expect("failed to execute process")
            .code()
            .unwrap();

        if status == MOUNT_FAILURE {
            // TODO: show error popup
            println!("Mount failed");
            logger.append(LogLevel::Error, "Failed to mount USB Drive");
            return;
        }
        assert_eq!(status, 0);

        logger.append(LogLevel::Info, "Mounting USB Drive successful");



        unsafe {
            let err = libc::seteuid(0);
            assert_eq!(err, 0);
        }


        let dirname = "diagnosis_documents";
        /* Only create dir if not existant */
        std::fs::create_dir_all(format!("{mountdir}/{dirname}")).unwrap();

        let blobs: Vec<(String, Blob)> = documents
            .into_iter()
            .map(|item| (item.descriptor, item.document))
            .collect();

        for (name, blob) in blobs {
            let f = format!("{mountdir}/{dirname}/{name}");
            let mut file = File::create(f).unwrap();
            file.write_all(&blob).unwrap();
            drop(file); // cannot unmount open files
        }

        logger.append(LogLevel::Info, "File transfer successful");


        unsafe {
            let err = libc::seteuid(1000);
            assert_eq!(err, 0);
        }


        let status: i32 = std::process::Command::new("umount")
            .arg(mountdir)
            .status()
            .expect("failed to execute process")
            .code()
            .unwrap();
        assert_eq!(status, 0);

    }

    /*
    fn popup_error(&self, ui: &mut egui::Ui) {


        let modal = Modal::new(egui::Id::new("Login")).show(ui.ctx(), |ui| {

            ui.heading("Error");

            //response.request_focus();

            ui.separator();

            egui::Sides::new().show( ui, |_ui| (), |ui| {
                if ui.button("Cancel").clicked() {
                    self.modal_current_password.clear();
                    self.modal_open = false;
                }
                if ui.button("Login").clicked() {
                    self.login();
                }
            },
            );

        });

        if modal.should_close() {
            self.modal_open = false;
        }
}
    */

}
