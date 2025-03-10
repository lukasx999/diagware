use crate::ui::components::prelude::*;

use crate::ui::Component;
use crate::logger::{LogMessage, LogLevel, Logger};
use crate::ui::config;
use crate::Diagnosis;

use egui::Color32;



pub struct Logging {
    logger: Rc<RefCell<Logger>>,
}

impl Component for Logging {
    fn name(&self) -> &'static str {
        "Logging"
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


impl Logging {

    pub fn new(logger: Rc<RefCell<Logger>>) -> Self {
        Self {
            logger,
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        let mut logger = self.logger.borrow_mut();

        ui.separator();

        ui.horizontal(|ui| {

            if ui.button("Clear").clicked() {
                logger.clear();
            }

            if ui.button("Export").clicked() {
                logger.export();
            }

        });

        ui.separator();


        for msg in &logger.log {

            use LogLevel as L;
            let color = match msg.level {
                L::Info    => config::COLOR_LOG_INFO,
                L::Warning => config::COLOR_LOG_WARNING,
                L::Error   => config::COLOR_LOG_ERROR,
            };

            ui.horizontal(|ui| {
                ui.colored_label(Color32::DARK_GRAY, msg.timestamp.as_str());

                ui.label(
                    egui::RichText::new(msg.level.to_string())
                        .background_color(color)
                        .strong()
                );

                ui.label(msg.message.as_str());
                ui.end_row();
            });

        }

    }

}
