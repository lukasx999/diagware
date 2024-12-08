use crate::ui::{
    GuiState,
    COLOR_STATE, COLOR_BACKGROUND, COLOR_ACTIVESTATE,
    util,
};

use crate::diagnosis::{Diagnosis, DiagnosisState, STATE_COUNT, DIAGNOSIS_STATE_REPRS};

use eframe::egui::{self, Color32};




// Functions for directly rendering elements of the ui

impl GuiState {



    pub fn ui_topbar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

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

            ui.label(util::get_time());
            ui.label(util::get_date());

            let username = whoami::username();
            let ip = local_ip_address::local_ip().unwrap();
            ui.label(format!("{}@{}", username, ip));

        });

    }



    // TODO: refactor to helper function _(state, ui) {}
    fn canvas_statemachine(&mut self, ui: &mut egui::Ui) {
        use egui::{vec2, Vec2, Pos2, pos2, Painter, Rect, Stroke, Response};

        let width:  f32 = ui.available_width();
        let height: f32 = 150.0;
        let (response, painter, center): (Response, Painter, Pos2) = util::setup_canvas(ui, width, height);

        let gap            = 30.0;                         // space between circles
        let segment_size   = width / (STATE_COUNT as f32); // +1 for extra space at the sides
        let radius         = (segment_size - gap) / 2.0;
        let offset         = radius * 2.0 + gap;               // distance to next circle center from current circle center
        let offset_to_origin = width / 2.0 - segment_size / 2.0; // offset at the very left for the starting circle
        let outline_thickness = 1.5;

        let mut font = egui::FontId::default();
        font.size = 15.0;
        // font.size = radius * 1.3; // NOTE: resizing will cause lag at first, because new font size is not cached yet

        // TODO: increase font step-wise
        // TODO: hover popup for descriptions


        let state = self.diag_state.clone() as usize;

        for i in 0..STATE_COUNT {

            let mut color_circle = if i == state {
                COLOR_ACTIVESTATE
            } else {
                COLOR_STATE
            };



            let new_center = center
            - vec2(offset_to_origin, 0.0)
            + vec2(i as f32 * offset, 0.0);


            if let Some(pos) = response.hover_pos() {
                if pos.distance(new_center) < radius {
                    color_circle = color_circle.gamma_multiply(0.75);
                }
            }


            painter.circle_filled(
                new_center,
                radius + outline_thickness,
                Color32::BLACK
            );

            painter.circle_filled(
                new_center,
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



    pub fn ui_diagnosis(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

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
                    ui.colored_label(color, DIAGNOSIS_STATE_REPRS[i]);
                    ui.end_row();
                }

            });
        });

        egui::containers::Frame::canvas(ui.style())
            .rounding(10.0)
            .outer_margin(10.0)
            // .stroke(egui::Stroke::new(1.0, COLOR_BACKGROUND))
            .fill(COLOR_BACKGROUND)
            .show(ui, |ui| {
                self.canvas_statemachine(ui);
            });

        // let state_repr: &'static str = STATE_LABELS[state];
        // ui.label(format!("Status: ({}) {}", state, state_repr));

        let is_running = self.diag_state != DiagnosisState::Idle;

        let btn_start: egui::Response = ui.add_enabled(
            !is_running,
            egui::Button::new("Start")
        );

        if btn_start.clicked() {
            util::start_diagnosis(self.diagnosis.clone(), self.diag_sender.clone());
        }

    }



    pub fn ui_dbmanager(&mut self, ui: &mut egui::Ui) {

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



    pub fn ui_serialmanager(&mut self, ui: &mut egui::Ui) {

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






    fn canvas_pineditor(&mut self, ui: &mut egui::Ui) {
        use egui::{vec2, Vec2, Pos2, pos2, Sense, Painter, Rect, Rounding, Stroke, Response};

        const PIN_COUNT: u32 = 4; // TODO: refactor to config.rs

        let width:  f32 = ui.available_width();
        let height: f32 = 150.0;
        let (response, painter, center): (Response, Painter, Pos2) = util::setup_canvas(ui, width, height);


        let radius        = 10.0;
        let gap           = 5.0;
        let offset        = radius * 2.0 + gap;
        let offset_origin = ((offset * PIN_COUNT as f32) / 2.0) - radius;



        for x in 0..PIN_COUNT {
            for y in 0..PIN_COUNT {

                let circle_pos = center
                - Vec2::splat(offset_origin)
                + vec2(x as f32 * offset, y as f32 * offset);

                // TODO: this
                // if let Some(pos) = response.hover_pos() {
                //     if pos.distance(center) < radius {
                //         dbg!(pos);
                //     }
                // }

                painter.circle_filled(
                    circle_pos,
                    radius,
                    Color32::WHITE
                );

            }
        }

    }



    pub fn ui_pineditor(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

        ui.heading("Pin Editor");

        egui::containers::Frame::canvas(ui.style())
            .rounding(10.0)
            .outer_margin(10.0)
            // .stroke(egui::Stroke::new(1.0, COLOR_BACKGROUND))
            .fill(COLOR_BACKGROUND)
            .show(ui, |ui| {
                self.canvas_pineditor(ui);
            });

    }




}
