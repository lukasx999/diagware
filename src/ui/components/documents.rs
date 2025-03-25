use std::io::Write;
use std::fs::{self, File};
use std::process::Command;

use crate::ui::components::prelude::*;
use crate::db::{DB, model::{Module, Document, Blob}};
use config::MOUNT_DIRNAME;


// dont modify.
const MOUNT_FAILURE: i32 = 32;


pub struct Documents {
    logger: Rc<RefCell<Logger>>,
    db: DB,
    selected_module: usize,
    /*
     * Using String as hashmap key for easier debugging
     */
    selected_docs: HashMap<String, HashMap<String, bool>>,
    export_log: bool,
    modal_open: bool,
}

impl Component for Documents {
    fn name(&self) -> &'static str {
        "Documents"
    }
    fn show(&mut self, ctx: &egui::Context, active: &mut bool) {
        egui::Window::new(self.name())
            .fade_in(true)
            .fade_out(true)
            .open(active)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl Documents {

    pub fn new(logger: Rc<RefCell<Logger>>) -> Self {
        let mut s = Self {
            logger,
            db: DB::new().unwrap(),
            selected_module: 0,
            selected_docs: HashMap::new(),
            export_log: false,
            modal_open: false,
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
        ui.separator();

        if self.modal_open {
            self.popup_error(ui);
        }

        if ui.button("Download").clicked() {
            self.download();
        }

    }

    fn ui_documentview(&mut self, ui: &mut egui::Ui) {

        ui.checkbox(&mut self.export_log, "Export Log");
        ui.separator();

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

    fn download(&mut self) {
        let module     = self.db.get_module_by_id(self.selected_module as i64 + 1).unwrap();
        let documents  = self.db.get_documents_by_id(module.id).unwrap();
        let docs_state = &self.selected_docs[&module.name];

        // only keep documents that are actually selected
        let selected_docs: Vec<Document> = documents
            .into_iter()
            .filter(|item| docs_state[&item.descriptor])
            .collect();

        self.download_docs(selected_docs);
    }

    fn mount(&self, device: &str, mountdir: &str) -> Option<()> {

        // Create mountpoint if not existant
        fs::create_dir_all(mountdir).unwrap();

        let status: i32 = Command::new("mount")
            .args([device, mountdir])
            .status()
            .expect("spawning process failed")
            .code()
            .unwrap();

        match status {
            MOUNT_FAILURE => None,
            0 => Some(()),
            _ => panic!("Failed to mount: Unknown reason"),
        }

    }

    fn unmount(&self, mountdir: &str) {
        let status = Command::new("umount")
            .arg(mountdir)
            .status()
            .expect("spawning process failed")
            .code()
            .unwrap();

        assert_eq!(status, 0);
    }

    fn download_docs(&mut self, documents: Vec<Document>) {
        let mut logger = self.logger.borrow_mut();

        if documents.is_empty() {
            logger.append(LogLevel::Warning, "No documents selected");
            return;
        }

        let mountdir = format!("{}/diag_mnt", env!("HOME"));
        let device = "/dev/sda1";

        if self.mount(device, &mountdir).is_none() {
            self.modal_open = true;
            println!("Mount failed");
            logger.append(LogLevel::Error, "Failed to mount USB Drive");
            return;
        }

        logger.append(LogLevel::Info, "Mounting USB Drive successful");

        // Create mount directory if not existant
        fs::create_dir_all(format!("{mountdir}/{MOUNT_DIRNAME}")).unwrap();

        let blobs: Vec<(String, Blob)> = documents
            .into_iter()
            .map(|item| (item.descriptor, item.document))
            .collect();

        let basepath = format!("{mountdir}/{MOUNT_DIRNAME}");

        if self.export_log {
            logger.export(&basepath);
        }

        for (name, blob) in blobs {
            let filename = format!("{basepath}/{name}");
            let mut file = File::create(filename).unwrap();
            file.write_all(&blob).unwrap();
            // cannot unmount open files, `file` is dropped at the end of this scope
        }

        logger.append(LogLevel::Info, "File transfer successful");

        self.unmount(&mountdir);
    }

    fn popup_error(&mut self, ui: &mut egui::Ui) {

        let modal = Modal::new(egui::Id::new("Error")).show(ui.ctx(), |ui| {

            ui.heading("Error");
            ui.separator();
            ui.label("Failed to mount USB device");

            egui::Sides::new().show( ui, |_ui| (), |ui| {
                if ui.button("Ok").clicked() {
                    self.modal_open = false;
                }
            },
            );

        });

        if modal.should_close() {
            self.modal_open = false;
        }
    }

}
