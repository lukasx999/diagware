use crate::ui::components::prelude::*;

use crate::io::eeprom::{EEPROM_SERIAL_MAX_SIZE, EEPROM};



pub struct Serialmanager {
    logger:          Rc<RefCell<Logger>>,
    is_expertmode:   Rc<RefCell<bool>>,
    eeprom:          EEPROM,
    serial_textedit: String,
}

impl Component for Serialmanager {
    fn name(&self) -> &'static str {
        "Serial Management"
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
    pub fn new(
        logger:        Rc<RefCell<Logger>>,
        is_expertmode: Rc<RefCell<bool>>,
    ) -> Self {
        Self {
            logger,
            is_expertmode,
            eeprom: EEPROM::new().unwrap(),
            serial_textedit: String::new(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        let serial: String = self.eeprom.get_serial().unwrap();

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

        let mut logger = self.logger.borrow_mut();
        let is_expert = *self.is_expertmode.borrow();

        ui.horizontal(|ui| {

            if ui.add_enabled(is_expert, egui::Button::new("Write")).clicked() {

                self.eeprom.write_serial(&self.serial_textedit).unwrap();
                self.serial_textedit.clear();

                logger.append(
                    LogLevel::Info,
                    format!(
                        "New Serial `{}` successfully written to EEPROM",
                        &self.serial_textedit
                    )
                );

            }

            if ui.add_enabled(is_expert, egui::Button::new("Clear")).clicked() {
                self.eeprom.clear().unwrap();
                logger.append(LogLevel::Info, "EEPROM cleared");
            }

        });
    }

}
