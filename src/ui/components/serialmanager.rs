use crate::ui::components::prelude::*;

use crate::Diagnosis;
use crate::ui::{Component, Logger, config};
use crate::io::eeprom::EEPROM_SERIAL_MAX_SIZE;



pub struct Serialmanager {
    diagnosis: Arc<Mutex<Diagnosis>>,
    logger:    Rc<RefCell<Logger>>,
    serial_textedit: String,
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
        logger:    Rc<RefCell<Logger>>
    ) -> Self {
        Self {
            diagnosis,
            logger,
            serial_textedit: String::new(),
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
        let response = egui::TextEdit::singleline(&mut self.serial_textedit)
            .char_limit(EEPROM_SERIAL_MAX_SIZE)
            .show(ui).response;

        response.request_focus();
        // TODO: confirm with enter

        let logger = &mut self.logger.borrow_mut();

        ui.horizontal(|ui| {

            if ui.button("Write").clicked() {

                if let Ok(diag) = self.diagnosis.try_lock() {
                    diag.eeprom.write_serial(&self.serial_textedit).unwrap();
                    logger.append(
                        LogLevel::Info,
                        format!("New Serial `{}` successfully written to EEPROM",
                            &self.serial_textedit)
                    );
                    self.serial_textedit.clear();

                } else {
                    logger.append(LogLevel::Error, "Writing Serial to EEPROM failed");
                }


            }

            if ui.button("Clear").clicked() {

                if let Ok(diag) = self.diagnosis.try_lock() {
                    diag.eeprom.clear().unwrap();
                    logger.append(LogLevel::Info, "EEPROM cleared");
                } else {
                    logger.append(LogLevel::Error, "Clearing Serial from EEPROM failed");
                }

            }

        });


    }

}
