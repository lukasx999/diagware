use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

use crate::Diagnosis;
use crate::ui::{Component, Logger};
use crate::ui::config;


pub struct DBManager {
    diagnosis: Arc<Mutex<Diagnosis>>,
    logger: Rc<RefCell<Logger>>,
}

impl Component for DBManager {
    fn name(&self) -> &'static str {
        config::PAGE_DBMANAGEMENT
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


impl DBManager {

    pub fn new(
        diagnosis: Arc<Mutex<Diagnosis>>,
        logger: Rc<RefCell<Logger>>
    ) -> Self {
        Self {
            diagnosis,
            logger,
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {


        if ui.button("add").clicked() {
            todo!("add");
        }

        ui.separator();

        // TODO: Modal for adding users

        // TODO: use try_lock() for making changes and show error popup
        // TODO: remove .unwrap() -> Error Popup

        /*
        let db = self.db.lock().unwrap();
        let modules: Vec<Module> = db.get_modules_all().unwrap();

        egui_extras::TableBuilder::new(ui)
        .column(egui_extras::Column::auto().resizable(true))
        .column(egui_extras::Column::auto().resizable(true))
        .column(egui_extras::Column::auto().resizable(true))
        .column(egui_extras::Column::remainder())
        .header(30.0, |mut header| {
        header.col(|ui| { ui.heading("rm");     });
        header.col(|ui| { ui.heading("id");     });
        header.col(|ui| { ui.heading("name");   });
        header.col(|ui| { ui.heading("serial"); });
        })
        .body(|mut body| {

        for module in modules {
        body.row(10.0, |mut row| {
        row.col(|ui| {
        if ui.button(egui_phosphor::regular::X).clicked() {
        todo!("Removing entries");
        }
        });
        row.col(|ui| {
        ui.label(format!("{}", module.id.unwrap()));
        });
        row.col(|ui| {
        ui.label(module.name);
        });
        row.col(|ui| {
        ui.label(module.serial);
        });
        });
        }

        });
        */

    }

}
