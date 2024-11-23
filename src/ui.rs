use eframe::egui;
use egui::{
    CentralPanel,
    ViewportBuilder,
    ViewportId,
    PopupCloseBehavior,
    containers::ComboBox
};

use crate::db::{
    DB,
    model::{Module, TargetValue}
};





const EXPERT_PASSWORD: &str = "foo";

const WINDOW_WIDTH:  f32 = 300.0;
const WINDOW_HEIGHT: f32 = 300.0;
const SCREEN_WIDTH:  f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;


// let id = egui::ViewportId::from_hash_of("db");
// let viewport = ViewportBuilder::default()
//     .with_title("DB Manager")
//     .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]);
//
// if self.show_db_manager {
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
// }






// let img = egui::Image::new(egui::ImageSource::Uri(
//     Cow::Borrowed("file://src/img_poweroff.png")));
// ui.add(egui::widgets::ImageButton::new(img));

// let popup_id = egui::Id::new("login-popup");
// let res = ui.button("login");
//
// if res.clicked() {
//     ui.memory_mut(|mem| mem.toggle_popup(popup_id));
// }
//
// egui::popup_below_widget(ui, popup_id, &res, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
//     ui.label("popup!");
// });
//
// // ui.menu_button("menu", |ui| {
// //     ui.button("poweroff");
// //     ui.button("reboot");
// //     ui.button("logout");
// // });











struct GuiState {
    db: DB,
    is_expert_mode:  bool,
    show_db_manager: bool,

    db_manager_selected: String,
}



impl GuiState {

    pub fn new(db: DB) -> Self {
        Self {
            db,
            is_expert_mode:  false,
            show_db_manager: false,
            db_manager_selected: "".to_owned(),
        }
    }

    fn db_manager_ui(&mut self, ui: &mut egui::Ui) {

        ui.heading("Database");
        ui.label("DB Verwaltung");
        ui.separator();

        let modules: Vec<Module> = self.db.get_modules_all().unwrap();

        ComboBox::from_label("Modul")
            .selected_text(&self.db_manager_selected)
            .show_ui(ui, |ui| {
                for module in modules {
                    ui.selectable_value(&mut self.db_manager_selected,
                        module.name.clone(),
                        format!("{}: {}", module.id.unwrap(), module.name)
                    );
                }
                // ui.selectable_value(&mut self.db_manager_selected, "foo".to_owned(), "foo");
            });

        ui.separator();

        egui_extras::TableBuilder::new(ui)
            .column(egui_extras::Column::auto().resizable(true))
            .column(egui_extras::Column::auto().resizable(true))
            .column(egui_extras::Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("id");
                });
                header.col(|ui| {
                    ui.heading("name");
                });
                header.col(|ui| {
                    ui.heading("serial");
                });
            })
            .body(|mut body| {
                body.row(10.0, |mut row| {
                    row.col(|ui| {
                        ui.label("1");
                    });
                    row.col(|ui| {
                        ui.label("OPV");
                    });
                    row.col(|ui| {
                        ui.label("123");
                    });
                });
            });

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


    fn ui_config(ctx: &egui::Context) {
        ctx.set_pixels_per_point(2.0);
        ctx.set_theme(egui::Theme::Dark);
    }


}





impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        Self::ui_config(ctx);


        CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui| {
                ui.heading("Home");
                ui.label(Self::get_time());
                ui.separator();

                if ui.button("Diagnose").clicked() {
                }

                if ui.button("DB-Manager").clicked() {
                    self.show_db_manager = true;
                }

                ui.separator();
                let _ = ui.button(egui_phosphor::regular::POWER); // Poweroff
            });





            let id = egui::ViewportId::from_hash_of("db");
            let viewport = ViewportBuilder::default()
                .with_title("DB Manager")
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]);

            if self.show_db_manager {
                ctx.show_viewport_immediate(id, viewport, |ctx, _class| {
                    CentralPanel::default().show(ctx, |ui| {
                        self.db_manager_ui(ui);
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
            .with_title("Home")
            .with_position([SCREEN_WIDTH/2.0 + WINDOW_WIDTH/2.0, SCREEN_HEIGHT/2.0])
            .with_resizable(false)
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };

    eframe::run_native(
        "Diagware",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Phosphor icons
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(GuiState::new(db)))

        }),
    )

}
