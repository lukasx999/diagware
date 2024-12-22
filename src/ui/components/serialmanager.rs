use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

use crate::Diagnosis;
use crate::ui::{Component, Logger};
use crate::ui::config;


pub struct Serialmanager {
    diagnosis: Arc<Mutex<Diagnosis>>,
    logger: Rc<RefCell<Logger>>,
}

impl Component for Serialmanager {
    fn name(&self) -> &'static str {
        config::PAGE_SERIALMANAGER
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


impl Serialmanager {

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
        use crate::ui::logger::LogLevel;

        let serial: String = if let Ok(diag) = self.diagnosis.try_lock() {
            diag.eeprom.get_serial().unwrap()
        } else {
            "<Not Available>".to_owned()
        };

        ui.label(format!("Serial: {}", serial));

        let mut serial_edit = "123";
        ui.text_edit_singleline(&mut serial_edit);




        let logger = &mut self.logger.borrow_mut();


        ui.horizontal(|ui| {


            if ui.button("Write").clicked() {

                if let Ok(diag) = self.diagnosis.try_lock() {
                    diag.eeprom.write_serial(&serial_edit).unwrap();
                } else {
                    logger.append(LogLevel::Error, "Writing Serial to EEPROM failed");
                }

                logger.append(
                    LogLevel::Info,
                    format!("New Serial (`{serial_edit}`) successfully written to EEPROM")
                );

            }

            if ui.button("Clear").clicked() {

                if let Ok(diag) = self.diagnosis.try_lock() {
                    diag.eeprom.clear().unwrap();
                } else {
                    logger.append(LogLevel::Error, "Clearing Serial from EEPROM failed");
                }

                logger.append(
                    LogLevel::Info,
                    "EEPROM cleared",
                );

            }

        });


    }

}
