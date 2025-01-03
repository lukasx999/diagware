use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

use crate::ui::{Component, Logger, config};

use egui_file::FileDialog;



pub struct Documents {
    logger: Rc<RefCell<Logger>>,
    dialog: FileDialog,
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

    pub fn new(logger: Rc<RefCell<Logger>>) -> Self {
        Self {
            logger,
            dialog: FileDialog::open_file(Some("/".into())),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        ui.label("documents!");

        self.dialog.show(ui.ctx());

        // TODO: this
        if ui.button("open dialog").clicked() {
            self.dialog.open();
        }

    }

}
