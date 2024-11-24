use std::collections::HashMap;

use eframe::egui::{
    self,
    CentralPanel,
    ViewportBuilder,
    ViewportId,
    PopupCloseBehavior,
    containers::ComboBox,
    widget_text::WidgetText
};

use crate::db::{
    DB,
    model::{Module, TargetValue}
};






// Modify here!
const EXPERT_PASSWORD: &str = "foo";

const WINDOW_WIDTH:  f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH:  f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;

const PAGE_DIAGNOSIS:    &str = "Diagnosis";
const PAGE_DBMANAGEMENT: &str = "DB-Management";



// ComboBox::from_label("Modul")
//     .selected_text(self.ui_dbmanager_currentmod.clone())
//     .show_ui(ui, |ui| {
//         for module in &modules {
//             ui.selectable_value(
//                 &mut self.ui_dbmanager_currentmod,
//                 module.clone(),
//                 module.clone(),
//             );
//         }
//     });



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









impl Into<WidgetText> for Module {
    fn into(self) -> WidgetText {
        WidgetText::RichText(egui::RichText::new(self.name))
    }
}



struct GuiState {
    db: DB,
    is_expert_mode:  bool,
    show_windowlist: bool,
    state_windows: HashMap<String, bool>,
}



impl GuiState {

    pub fn new(db: DB) -> Self {
        Self {
            db,
            is_expert_mode:  false,
            show_windowlist: true,
            state_windows: HashMap::from([
                ("Diagnosis"    .to_owned(), false),
                ("DB-Management".to_owned(), false),
            ]),
        }
    }



    fn get_time() -> String {
        chrono::Local::now()
            .time()
            .format("%H:%M:%S")
            .to_string()
    }

    fn ui_config(ctx: &egui::Context) {
        ctx.set_pixels_per_point(2.0);
        ctx.set_theme(egui::Theme::Dark);

        // let mut v = egui::Visuals::default();
        // v.window_fill = egui::Color32::from_rgb(50, 0, 0);
        // ctx.set_visuals(v);
    }


    fn ui_topbar(&mut self, ui: &mut egui::Ui) {

        ui.horizontal(|ui| {

            ui.toggle_value(&mut self.show_windowlist, "ToggleWindowList");

            if ui.button(egui_phosphor::regular::POWER).clicked() {
                todo!("Poweroff");
            }

            ui.label(Self::get_time());

            let username = whoami::username();
            let ip = local_ip_address::local_ip().unwrap();
            ui.label(format!("{}@{}", username, ip));

        });

    }


    fn ui_diagnosis(&mut self, ui: &mut egui::Ui) {
        ui.heading("Diag!");
        ui.label("Diagnose!");
    }


    fn ui_dbmanager(&mut self, ui: &mut egui::Ui) {

        ui.heading("Database");
        ui.label("DB Verwaltung");

        if ui.button("add").clicked() {
            todo!("add");
        }
        ui.separator();



        // TODO: remove .unwrap()
        let modules: Vec<Module> = self.db.get_modules_all().unwrap();

        egui_extras::TableBuilder::new(ui)
            .column(egui_extras::Column::auto().resizable(true))
            .column(egui_extras::Column::auto().resizable(true))
            .column(egui_extras::Column::auto().resizable(true))
            .column(egui_extras::Column::remainder())
            .header(30.0, |mut header| {
                header.col(|ui| { ui.heading("rm");     });
                header.col(|ui| { ui.heading("id");     });
                header.col(|ui| { ui.heading("name");   });
                header.col(|ui| { ui.heading("serial"); });
            })
            .body(|mut body| {

                for module in modules {
                    body.row(10.0, |mut row| {
                        row.col(|ui| {
                            if ui.button(egui_phosphor::regular::X).clicked() {
                                todo!("Removing entries");
                            }
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", module.id.unwrap()));
                        });
                        row.col(|ui| {
                            ui.label(module.name);
                        });
                        row.col(|ui| {
                            ui.label(module.serial);
                        });
                    });
                }

            });

    }



}





impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        Self::ui_config(ctx);


        egui::TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            self.ui_topbar(ui);
        });


        egui::SidePanel::left("WindowList")
            .show_animated(ctx, self.show_windowlist, |ui| {

                // TODO: change hashmap hasher (.with_hasher)
                // right now iteration is randomized every run
                for page in &mut self.state_windows {
                    ui.toggle_value(page.1, page.0);
                }

            });






        // TODO: wrapper for this
        let mut active: bool = *self.state_windows.get_mut(PAGE_DBMANAGEMENT).unwrap();
        egui::Window::new(PAGE_DBMANAGEMENT)
            .fade_in(true)
            .fade_out(true)
            .open(&mut active)
            .show(ctx, |ui| {
                self.ui_dbmanager(ui);
            });
        *self.state_windows.get_mut(PAGE_DBMANAGEMENT).unwrap() = active;






        let mut active: bool = *self.state_windows.get_mut(PAGE_DIAGNOSIS).unwrap();
        egui::Window::new(PAGE_DIAGNOSIS)
            .fade_in(true)
            .fade_out(true)
            .open(&mut active)
            .show(ctx, |ui| {
                self.ui_diagnosis(ui);
            });
        *self.state_windows.get_mut(PAGE_DIAGNOSIS).unwrap() = active;







        CentralPanel::default().show(ctx, |ui| {
            egui::containers::Frame::default().show(ui, |_ui| ());
        });


    }
}




pub fn run_gui(db: DB) -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Home")
            .with_position([SCREEN_WIDTH/2.0 + WINDOW_WIDTH/2.0, SCREEN_HEIGHT/2.0])
            .with_resizable(true)
            .with_fullscreen(false)
            .with_maximized(true)
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Diagware",
        options,
        Box::new(|cc| {

            // Image support
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Phosphor icons
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(GuiState::new(db)))

        }),
    )

}
