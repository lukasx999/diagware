use crate::ui::components::prelude::*;

use crate::Diagnosis;
use crate::io::eeprom::EEPROM_SERIAL_MAX_SIZE;



pub struct Serialmanager {
    diagnosis:       Arc<Mutex<Diagnosis>>,
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
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}

impl Serialmanager {
    pub fn new(diagnosis: Arc<Mutex<Diagnosis>>) -> Self {
        Self {
            diagnosis,
            serial_textedit: String::new(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        let serial: String = if let Ok(diag) = self.diagnosis.try_lock() {
            diag.eeprom.get_serial().unwrap()
        } else {
            "<Not Available>".to_owned()
        };

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Serial: ").strong());
            ui.label(egui::RichText::new(serial).color(Color32::GRAY).strong());
        });

        ui.separator();

        let response = egui::TextEdit::singleline(&mut self.serial_textedit)
            .char_limit(EEPROM_SERIAL_MAX_SIZE)
            .show(ui).response;

        response.request_focus();
        // TODO: confirm with enter


        self.ui_buttons(ui);

    }

    fn ui_buttons(&mut self, ui: &mut egui::Ui) {

        ui.horizontal(|ui| {

            if ui.button("Write").clicked() {
                let logger = &mut self.diagnosis.lock().unwrap().logger;

                if let Ok(diag) = self.diagnosis.try_lock() {
                    diag.eeprom.write_serial(&self.serial_textedit).unwrap();

                    logger.append(
                        LogLevel::Info,
                        format!(
                            "New Serial `{}` successfully written to EEPROM",
                            &self.serial_textedit
                        )
                    );

                    self.serial_textedit.clear();

                } else {
                    logger.append(LogLevel::Error, "Writing Serial to EEPROM failed");
                }
            }

            if ui.button("Clear").clicked() {
                let logger = &mut self.diagnosis.lock().unwrap().logger;

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
