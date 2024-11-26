use std::sync::{Arc, Mutex, MutexGuard};

use eframe::egui::{
    self,
    Color32,
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

use crate::diagnosis::{Diagnosis, DiagnosisState, STATE_COUNT};






// Modify here!
const EXPERT_PASSWORD: &str = "foo";

// const WINDOW_WIDTH:  f32 = 1280.0;
// const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_WIDTH:  f32 = 2300.0;
const WINDOW_HEIGHT: f32 = 1200.0;
const SCREEN_WIDTH:  f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;

const PAGE_DIAGNOSIS:    &str = "Diagnosis";
const PAGE_DBMANAGEMENT: &str = "DB-Management";




struct GuiState {
    db: DB,
    diagnosis: Arc<Mutex<Diagnosis>>,

    is_expert_mode:  bool,
    show_windowlist: bool,

    show_diagnosis: bool,
    show_dbmanager: bool,
}



impl GuiState {

    pub fn new(db: DB, diagnosis: Diagnosis) -> Self {
        Self {
            db,
            diagnosis: Arc::new(Mutex::new(diagnosis)),


            is_expert_mode:  false,
            show_windowlist: true,

            show_diagnosis: true,
            show_dbmanager: false,
        }
    }

    fn get_time() -> String {
        chrono::Local::now()
            .time()
            .format("%H:%M:%S")
            .to_string()
    }



    // Returns new state of `enabled`
    fn new_window(
        ctx:     &egui::Context,
        enabled: bool,
        title:   &str,
        mut ui_callback: impl FnMut(&mut egui::Ui),
    ) -> bool {

        let mut active: bool = enabled;

        egui::Window::new(title)
            .fade_in(true)
            .fade_out(true)
            .open(&mut active)
            .show(ctx, |ui| {
                ui_callback(ui);
            });

        active

    }



    fn ui_config(ctx: &egui::Context) {
        ctx.set_pixels_per_point(2.0);
        ctx.set_theme(egui::Theme::Dark);
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






    fn ui_painting_setup(ui: &mut egui::Ui, width: f32, height: f32) -> (egui::Painter, egui::Pos2) {
        use egui::{vec2, Sense, Painter, Rect, };

        let painter: Painter = ui.allocate_painter(
            vec2(width, height),
            Sense::hover()
        ).1;

        let rect: Rect = ui.allocate_at_least(
            vec2(0.0, 0.0),
            Sense::hover()
        ).0;

        let center = rect.center()
        - vec2(0.0, height/2.0)
        + vec2(width/2.0, 0.0);

        (painter, center)

    }



    fn ui_statemachine(&mut self, ui: &mut egui::Ui) {
        use egui::{vec2, Vec2, Pos2, pos2, Sense, Painter, Rect, Rounding};

        let width:  f32 = ui.available_width();
        let height: f32 = 200.0;
        let (painter, center): (Painter, Pos2) =
        Self::ui_painting_setup(ui, width, height);

        let gap            = 10.0; // space between circles
        let segment_size   = width / (STATE_COUNT as f32 + 1.0); // +1 for extra space at the sides
        let radius         = (segment_size - gap) / 2.0;
        let offset         = (radius * 2.0) + gap; // distance to next circle center from current circle center
        let initial_offset = width/2.0 - segment_size; // offset at the very left for the starting circle

        for i in 0..STATE_COUNT {

            painter.circle_filled(
                center
                - vec2(initial_offset, 0.0)
                + vec2(i as f32 * offset, 0.0),
                radius,
                Color32::DARK_BLUE
            );

        }


        // painter.circle_filled(
        //     center,
        //     30.0,
        //     Color32::BLUE
        // );


        // let mut font = egui::FontId::default();
        // font.size = 30.0;
        //
        // painter.text(
        //     center,
        //     egui::Align2::CENTER_CENTER,
        //     "State",
        //     font,
        //     Color32::ORANGE
        // );



    }


    fn ui_diagnosis(&mut self, ui: &mut egui::Ui) {

        // TODO: ui.collapsing

        egui::containers::Frame::canvas(ui.style()).show(ui, |ui| {
            self.ui_statemachine(ui);
        });

        ui.heading("Diagnose");

        if ui.button("Start").clicked() {

            let diag: Arc<_> = self.diagnosis.clone();

            std::thread::spawn(move || {

                let mut diag: MutexGuard<Diagnosis> = diag.lock().unwrap();

                let Ok(_) = diag.diagnosis() else {
                    todo!("Show error popup");
                };

            });
        }

    }


    fn ui_dbmanager(&mut self, ui: &mut egui::Ui) {

        ui.heading("Database");
        ui.label("DB Verwaltung");

        if ui.button("add").clicked() {
            todo!("add");
        }

        ui.separator();



        // TODO: remove .unwrap() -> Error Popup
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
                ui.toggle_value(&mut self.show_dbmanager, PAGE_DBMANAGEMENT);
                ui.toggle_value(&mut self.show_diagnosis, PAGE_DIAGNOSIS);
            });


        self.show_dbmanager =
            Self::new_window(ctx, self.show_dbmanager, PAGE_DBMANAGEMENT, |ui| {
                self.ui_dbmanager(ui);
            });

        self.show_diagnosis =
            Self::new_window(ctx, self.show_diagnosis, PAGE_DIAGNOSIS, |ui| {
                self.ui_diagnosis(ui);
            });



        egui::CentralPanel::default().show(ctx, |ui| {
            egui::containers::Frame::default().show(ui, |_ui| ());
        });


    }
}



fn setup_options() -> eframe::NativeOptions {

    eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Diagware")
            // .with_resizable(true)
            // .with_fullscreen(false)
            // .with_maximized(true)
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        centered: true,
        ..Default::default()
    }

}


pub fn run_gui(
    db: DB,
    diagnosis: Diagnosis
) -> eframe::Result {

    let options: eframe::NativeOptions = setup_options();

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

            Ok(Box::new(GuiState::new(db, diagnosis)))

        }),
    )

}
