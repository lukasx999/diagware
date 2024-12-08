use std::{
    sync::{Arc, Mutex, mpsc},
    rc::Rc,
};

use eframe::egui::{
    self,
    Color32,
    RichText,
    containers::ComboBox,
    widget_text::WidgetText
};

use crate::db::{
    DB,
    model::{Module, TargetValue},
};
use crate::diagnosis::{Diagnosis, DiagnosisState, STATE_COUNT, STATE_LABELS};
use crate::eeprom::EEPROM;







// Modify here!
const EXPERT_PASSWORD: &str = "foo";

const WINDOW_WIDTH:  f32 = 2300.0;
const WINDOW_HEIGHT: f32 = 1200.0;

const PAGE_DIAGNOSIS:     &str = "Diagnosis";
const PAGE_DBMANAGEMENT:  &str = "DB-Management";
const PAGE_SERIALMANAGER: &str = "Serial Management";

const COLOR_BACKGROUND:  Color32 = Color32::from_rgb(27,  27 , 27 );
const COLOR_ACTIVESTATE: Color32 = Color32::from_rgb(41,  110, 214);
const COLOR_STATE:       Color32 = Color32::from_rgb(178, 183, 191);


struct GuiState {
    diagnosis:     Arc<Mutex<Diagnosis>>, // All HW/SW interfaces are owned by the diagnosis
    diag_sender:   mpsc::Sender<DiagnosisState>,
    diag_receiver: mpsc::Receiver<DiagnosisState>,
    diag_state:    DiagnosisState, // UI needs to keep track of current diagnosis state to: 1. show
                                   // the state in state machine diagram and 2. block off other ui elements

    is_expert_mode:  bool,

    show_windowlist: bool,
    show_diagnosis:  bool,
    show_dbmanager:  bool,
    show_serialmanager: bool,

    show_foo: bool,

}



/* UTILITY */
impl GuiState {

    pub fn new(
        diagnosis: Diagnosis
    ) -> Self {

        let (tx, rx) = mpsc::channel();

        Self {
            diagnosis:     Arc::new(Mutex::new(diagnosis)),
            diag_sender:   tx,
            diag_receiver: rx,
            diag_state:    DiagnosisState::default(),

            is_expert_mode:  false,
            show_windowlist: true,

            show_diagnosis:     true,
            show_dbmanager:     false,
            show_serialmanager: false,

            show_foo: false,

        }

    }


    fn get_time() -> String {
        chrono::Local::now()
            .time()
            .format("%H:%M:%S")
            .to_string()
    }


    fn get_date() -> String {
        chrono::Local::now()
            .date_naive()
            .format("%d.%m.%Y")
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

    fn ui_painting_setup(
        ui: &mut egui::Ui,
        width: f32,
        height: f32
    ) -> (egui::Painter, egui::Pos2) {

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


    fn start_diagnosis(&self) {

        let diag = self.diagnosis.clone();
        let sender = self.diag_sender.clone();

        std::thread::Builder::new()
            .name("diagnosis".to_string())
            .spawn(move || {
                diag.lock()
                    .unwrap()
                    .diagnosis(sender)
                    .unwrap();
            }).unwrap();

    }



}



/* UI */
impl GuiState {

    fn ui_topbar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

        let modal = egui_modal::Modal::new(ctx, "Login");

        modal.show(|ui| {

            modal.title(ui, "Login");

            modal.frame(ui, |ui| {
                modal.body(ui, "Passworteingabe");
                // TODO: this
            });

            modal.buttons(ui, |ui| {
                if modal.button(ui, "Abbruch").clicked() {}
                if modal.button(ui, "Ok").clicked() {}
            });

        });


        ui.horizontal(|ui| {

            ui.toggle_value(&mut self.show_windowlist, "Windows");

            if ui.button("Login").clicked() {
                modal.open();
            }

            if ui.button(egui_phosphor::regular::POWER).clicked() {
                if cfg!(target_arch ="aarch64") {
                    // TODO: shutdown requires sudo
                    // std::process::Command::new("systemctl")
                    //     .args(["poweroff"])
                    //     .spawn()
                    //     .unwrap();
                }
                else {
                    panic!("Shutdown");
                }

            }

            ui.label(Self::get_time());
            ui.label(Self::get_date());

            let username = whoami::username();
            let ip = local_ip_address::local_ip().unwrap();
            ui.label(format!("{}@{}", username, ip));

        });

    }


    fn ui_statemachine(&mut self, ui: &mut egui::Ui) {
        use egui::{vec2, Vec2, Pos2, pos2, Sense, Painter, Rect, Rounding, Stroke};

        let width:  f32 = ui.available_width();
        let height: f32 = 150.0;
        let (painter, center): (Painter, Pos2) =
        Self::ui_painting_setup(ui, width, height);

        let gap            = 30.0;                         // space between circles
        let segment_size   = width / (STATE_COUNT as f32); // +1 for extra space at the sides
        let radius         = (segment_size - gap) / 2.0;
        let offset         = (radius * 2.0) + gap;               // distance to next circle center from current circle center
        let offset_to_origin = width / 2.0 - segment_size / 2.0; // offset at the very left for the starting circle

        let mut font = egui::FontId::default();
        font.size = 15.0;
        // font.size = radius * 1.3; // NOTE: resizing will cause lag at first, because new font size is not cached yet

        // TODO: increase font step-wise
        // TODO: hover popup for descriptions


        let state = self.diag_state.clone() as usize;

        for i in 0..STATE_COUNT {

            let color_circle = if i == state {
                COLOR_ACTIVESTATE
            } else {
                COLOR_STATE
            };

            painter.circle_filled(
                center
                - vec2(offset_to_origin, 0.0)
                + vec2(i as f32 * offset, 0.0),
                radius + 1.5,
                Color32::BLACK
            );

            painter.circle_filled(
                center
                - vec2(offset_to_origin, 0.0)
                + vec2(i as f32 * offset, 0.0),
                radius,
                color_circle
            );

            painter.text(
                center
                - vec2(offset_to_origin, 0.0)
                + vec2(i as f32 * offset, 0.0),
                egui::Align2::CENTER_CENTER,
                format!("{i}"),
                font.clone(),
                Color32::BLACK
            );

            // Dont render arrow after the last state
            if i == STATE_COUNT - 1 {
                break;
            }

            painter.arrow(
                center
                - vec2(offset_to_origin - radius, 0.0)
                + vec2(i as f32 * offset, 0.0),
                vec2(gap, 0.0),
                Stroke::new(2.0, Color32::GRAY)
            );

        }

    }


    fn ui_diagnosis(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

        ui.heading("Diagnose");

        ui.collapsing("Legende", |ui| {
            ui.horizontal_wrapped(|ui| {

                let state = self.diag_state.clone() as usize;

                for i in 0..STATE_COUNT {

                    let color = if i == state {
                        COLOR_ACTIVESTATE
                    } else {
                        Color32::GRAY
                    };

                    ui.colored_label(color, format!("{i}"));
                    ui.colored_label(Color32::DARK_GRAY, "...");
                    ui.colored_label(color, STATE_LABELS[i]);
                    ui.end_row();
                }

            });
        });

        egui::containers::Frame::canvas(ui.style())
            .rounding(20.0)
            .outer_margin(10.0)
            // .stroke(egui::Stroke::new(1.0, COLOR_BACKGROUND))
            .fill(COLOR_BACKGROUND)
            .show(ui, |ui| {
                self.ui_statemachine(ui);
            });

        // let state_repr: &'static str = STATE_LABELS[state];
        // ui.label(format!("Status: ({}) {}", state, state_repr));

        let is_running = self.diag_state != DiagnosisState::Idle;

        let btn_start: egui::Response = ui.add_enabled(
            !is_running,
            egui::Button::new("Start")
        );

        if btn_start.clicked() {
            self.start_diagnosis();
        }

    }


    fn ui_dbmanager(&mut self, ui: &mut egui::Ui) {

        ui.heading("Database");
        ui.label("DB Verwaltung");

        if ui.button("add").clicked() {
            todo!("add");
        }

        ui.separator();

        // TODO: Modal for adding users

        // TODO: use try_lock() for making changes and show error popup
        // TODO: remove .unwrap() -> Error Popup

        /*
        let db = self.db.lock().unwrap();
        let modules: Vec<Module> = db.get_modules_all().unwrap();

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
        */

    }



    fn ui_serialmanager(&mut self, ui: &mut egui::Ui) {

        ui.heading("Serial");
        let mut text: &str = "123";
        ui.text_edit_singleline(&mut text);
        ui.label("Jetzige Seriennummber: 45");
        ui.button("Seriennummer Beschreiben");

        /*
        if ui.button("Seriennummer Lesen").clicked() {
            let s = self.eeprom.lock().unwrap().get_serial().unwrap();
            println!("{}", s);
        }
        */



    }




}






impl eframe::App for GuiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Config
        ctx.set_pixels_per_point(2.0);
        ctx.set_theme(egui::Theme::Dark);

        ctx.request_repaint(); // NOTE: egui only redraws UI when the position of the mouse cursor
                               // changes, therefore, to show the changing of states, we have to explicitly redraw the ui
                               // every frame

        // Receive new state from running diagnosis
        if let Ok(state) = self.diag_receiver.try_recv() {
            self.diag_state = state.clone();
        }


        egui::TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            self.ui_topbar(&ctx, ui);
        });


        egui::SidePanel::left("WindowList")
            .show_animated(ctx, self.show_windowlist, |ui| {
                ui.toggle_value(&mut self.show_dbmanager, PAGE_DBMANAGEMENT);
                ui.toggle_value(&mut self.show_diagnosis, PAGE_DIAGNOSIS);
                ui.toggle_value(&mut self.show_serialmanager, PAGE_SERIALMANAGER);
            });


        self.show_dbmanager =
            Self::new_window(ctx, self.show_dbmanager, PAGE_DBMANAGEMENT, |ui| {
                self.ui_dbmanager(ui);
            });


        self.show_serialmanager =
            Self::new_window(ctx, self.show_serialmanager, PAGE_SERIALMANAGER, |ui| {
                self.ui_serialmanager(ui);
            });




        // TODO: refactor this into a macro!
        // TODO: min_width()
        let mut active = self.show_diagnosis;

        egui::Window::new(PAGE_DIAGNOSIS)
            .fade_in(true)
            .fade_out(true)
            .open(&mut active)
            .enabled(true)
            .show(ctx, |ui| {
                self.ui_diagnosis(&ctx, ui);
            });

        self.show_diagnosis = active;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::containers::Frame::default().show(ui, |_ui| ());
        });


    }
}



fn frame_setup() -> eframe::NativeOptions {

    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
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
    diagnosis: Diagnosis
) -> eframe::Result {

    let options: eframe::NativeOptions = frame_setup();

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

            Ok(Box::new(GuiState::new(diagnosis)))

        }),
    )

}
