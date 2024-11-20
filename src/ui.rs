use eframe::egui;
use egui::{CentralPanel, ViewportBuilder};

use crate::db::{DB, Module};



const WIDTH:  f32 = 300.0;
const HEIGHT: f32 = 300.0;




struct GuiState {

    db: DB,

    show_db_manager: bool,

}



impl GuiState {
    pub fn new(db: DB) -> Self {
        Self {
            db,
            show_db_manager: false,
        }
    }


    // fn db_manager(
    //     &mut self
    // ) -> Box<dyn FnMut(&egui::Context, egui::ViewportClass)> {
    //
    //     Box::new(|ctx, _class| {
    //         CentralPanel::default().show(ctx, |ui| {
    //             ui.heading("Database");
    //             ui.label("Hier findet die DB Verwaltung statt");
    //         });
    //
    //         if ctx.input(|i| i.viewport().close_requested()) {
    //             self.show_db_manager = false;
    //         }
    //     }
    //     )
    //
    // }

}





impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.set_pixels_per_point(2.0);

        CentralPanel::default().show(ctx, |ui| {

            ui.heading("Diagnose");
            ui.label("Hier findet die Diagnose statt.");

            let id = egui::ViewportId::from_hash_of("db");
            let viewport = ViewportBuilder::default()
                .with_title("DB Manager")
                .with_inner_size([WIDTH, HEIGHT]);

            // egui_extras::TableBuilder::new(ui)
            //     .column(egui_extras::Column::auto().resizable(true))
            //     .header(20.0, |mut header| {
            //         header.col(|ui| {
            //             ui.heading("foo");
            //         });
            //         header.col(|ui| {
            //             ui.heading("foo");
            //         });
            //
            //     }).body(|mut body| {
            //         body.row(30.0, |mut row| {
            //             row.col(|ui| {
            //                 ui.label("bar");
            //             });
            //             row.col(|ui| {
            //                 ui.label("baz");
            //             });
            //         });
            //     });



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




pub fn run_gui(db: DB) -> eframe::Result {

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
            Ok(Box::new(GuiState::new(db)))
        }),

    )

}
