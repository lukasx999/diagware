use std::borrow::Cow;

use eframe::egui;
use egui::{CentralPanel, ViewportBuilder, ViewportId};

use crate::db::{DB, Module};



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





const WINDOW_WIDTH:  f32 = 300.0;
const WINDOW_HEIGHT: f32 = 300.0;
const SCREEN_WIDTH:  f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;




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

    fn db_manager_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Database");
        ui.label("Hier findet die DB Verwaltung statt");
    }


    fn get_time() -> String {
        chrono::Local::now()
            .time()
            .format("%H:%M:%S")
            .to_string()
    }


    // Returns true if window should be closed
    fn new_window(
        name: &str,
        ctx: &egui::Context,
        mut callback: impl FnMut(&egui::Context) -> ()
    ) -> bool {

        let viewport_id = ViewportId::from_hash_of(name);
        let viewport    = ViewportBuilder::default()
            .with_title(name)
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]);

        ctx.show_viewport_immediate(viewport_id, viewport, |ctx, _class| {

            callback(ctx);

            if ctx.input(|i| i.viewport().close_requested()) {
                true
            }
            else {
                false
            }

        })

    }

}





impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.set_pixels_per_point(2.0);
        ctx.set_theme(egui::Theme::Dark);

        CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui| {
                ui.heading("Home");
                ui.label(Self::get_time());

                let _ = ui.button("Diagnose");

                if ui.button("DB-Manager").clicked() {
                    self.show_db_manager = true;
                }

                // let img_src = egui::include_image!("./src/ferris.png");
                // let img_src = egui::ImageSource::Uri(Cow::Borrowed("file://src/img_poweroff.png"));
                // let img = egui::Image::new(img_src);
                // ui.add(egui::widgets::ImageButton::new(img));

                let popup_id = egui::Id::new("login-popup");
                let res = ui.button("login");

                if res.clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                }

                egui::popup_below_widget(ui, popup_id, &res, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
                    ui.label("popup!");
                });

                // ui.menu_button("menu", |ui| {
                //     ui.button("poweroff");
                //     ui.button("reboot");
                //     ui.button("logout");
                // });

            });


            let mut should_close: bool = false;

            if self.show_db_manager {

                should_close =
                    Self::new_window("db", ctx, |ctx| {

                        CentralPanel::default().show(ctx, |ui| {
                            ui.heading("greetings");
                        });

                    });

            }

            if should_close {
                self.show_db_manager = false;
            }


            // let id = egui::ViewportId::from_hash_of("db");
            // let viewport = ViewportBuilder::default()
            //     .with_title("DB Manager")
            //     .with_inner_size([WIDTH, HEIGHT]);
            //
            // if self.show_db_manager {
            //
            //     ctx.show_viewport_immediate(id, viewport, |ctx, _class| {
            //
            //         CentralPanel::default().show(ctx, |ui| {
            //             self.db_manager_ui(ui);
            //         });
            //
            //         if ctx.input(|i| i.viewport().close_requested()) {
            //             self.show_db_manager = false;
            //         }
            //
            //     });
            //
            // }


        });

    }
    }




pub fn run_gui(db: DB) -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Home")
            .with_position([SCREEN_WIDTH/2.0 + WINDOW_WIDTH/2.0, SCREEN_HEIGHT/2.0])
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };

    eframe::run_native(
        "Diagware",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(GuiState::new(db)))
        }),
    )

}
