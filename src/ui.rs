use eframe::egui;
use egui::{CentralPanel, ViewportBuilder};

use crate::db::{DB, Module};



const WIDTH:  f32 = 320.0;
const HEIGHT: f32 = 240.0;




struct MyApp {

    db: DB,

    show_db_manager: bool,

}



impl MyApp {
    pub fn new(db: DB) -> Self {
        Self {
            db,
            show_db_manager: false,
        }
    }
}









impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        CentralPanel::default().show(ctx, |ui| {

            ui.heading("Diagnose");
            ui.label("Hier findet die Diagnose statt.");

            let id = egui::ViewportId::from_hash_of("db");
            let viewport = ViewportBuilder::default()
                .with_title("DB Manager")
                .with_inner_size([WIDTH, HEIGHT]);


            if ui.button("click me").clicked() {
                self.show_db_manager = true;
            }


            if self.show_db_manager {

                ctx.show_viewport_immediate(id, viewport, |ctx, _class| {

                    CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Database");
                        ui.label("Hier findet die DB Verwaltung statt");
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_db_manager = false;
                    }

                });
            }


        });

    }
}




pub fn app(db: DB) -> eframe::Result {

    let options = eframe::NativeOptions {

        viewport: ViewportBuilder::default()
            .with_title("Diagnose")
            .with_inner_size([WIDTH, HEIGHT]),
        ..Default::default()

    };

    eframe::run_native(

        "Diagware",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp::new(db)))
        }),

    )

}
