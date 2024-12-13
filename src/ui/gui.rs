use crate::ui::{
    GuiState,
    logger,
    config,
    util,
};

use crate::diagnosis::{Diagnosis, DiagnosisState, DiagnosisMode, STATE_COUNT, DIAGNOSIS_STATE_REPRS};

use eframe::egui::{self, Color32};






// Functions for directly rendering elements of the ui

impl GuiState {

    // TODO: move to util.rs
    pub fn ui_error(ctx: &egui::Context, message: &str) -> egui_modal::Modal {

        let modal = egui_modal::Modal::new(ctx, message);

        modal.show(|ui| {

            modal.title(ui, "Error");

            modal.frame(ui, |ui| {
                modal.body(ui, "Ein Fehler ist aufgetreten");
                modal.body(ui, format!("Fehler: {}", message));
            });

            modal.buttons(ui, |ui| {
                if modal.button(ui, "Abbruch").clicked() {}
                if modal.button(ui, "Ok").clicked() {}
            });

        });

        modal

    }


    pub fn ui_topbar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

        let modal = Self::ui_error(ctx, "Datenbank Verbindung fehlgeschlagen");
        if ui.button("ERROR").clicked() {
            modal.open();
        }

        let modal = egui_modal::Modal::new(ctx, "Login");

        // TODO: create another egui window for textedit

        modal.show(|ui| {

            modal.title(ui, "Login");

            modal.frame(ui, |ui| {
                modal.body(ui, "Passworteingabe");
                // TODO: this

            });

            modal.buttons(ui, |ui| {
                if modal.button(ui, "Abbruch").clicked() {}
                if modal.button(ui, "Ok").clicked() {
                    self.is_expert_mode = true;
                }
            });

        });



        ui.horizontal(|ui| {

            ui.toggle_value(&mut self.show_windowlist, "Windows");

            if self.is_expert_mode {

                if ui.button("Logout").clicked() {
                    self.is_expert_mode = false;
                }

            } else {

                if ui.button("Login").clicked() {
                    modal.open();
                }

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
        let (response, painter, center): (_, _, Pos2) =
            util::canvas_setup(ui, width, height);

        let gap               = 30.0;                         // space between circles
        let segment_size      = width / (STATE_COUNT as f32); // +1 for extra space at the sides
        let radius            = (segment_size - gap) / 2.0;
        let offset            = radius * 2.0 + gap;               // distance to next circle center from current circle center
        let offset_to_origin  = width / 2.0 - segment_size / 2.0; // offset at the very left for the starting circle
        let outline_thickness = 1.5;

        let mut font = egui::FontId::default();
        font.size = 15.0;
        // font.size = radius * 1.3; // NOTE: resizing will cause lag at first, because new font size is not cached yet

        // TODO: dont show anything of available_width is smaller than min size
        // TODO: increase font step-wise
        // TODO: hover popup for descriptions

        let state = self.diag_state as usize;

        for i in 0..STATE_COUNT {

            let mut color_circle = if i == state {
                config::COLOR_ACTIVESTATE
            } else {
                config::COLOR_STATE
            };


            let circle_center = center
            - vec2(offset_to_origin, 0.0)
            + vec2(i as f32 * offset, 0.0);


            if let Some(pos) = response.hover_pos() {
                if pos.distance(circle_center) < radius {
                    color_circle = color_circle.gamma_multiply(0.75);
                }
            }


            painter.circle_filled(
                circle_center,
                radius + outline_thickness,
                Color32::BLACK
            );

            painter.circle_filled(
                circle_center,
                radius,
                color_circle
            );

            painter.text(
                circle_center,
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
        use crate::diagnosis::DiagnosisResult;

        ui.heading("Diagnose");

        ui.collapsing("Legende", |ui| {
            ui.horizontal_wrapped(|ui| {

                let state = self.diag_state as usize;

                for i in 0..STATE_COUNT {

                    let color = if i == state {
                        config::COLOR_ACTIVESTATE
                    } else {
                        Color32::GRAY
                    };

                    ui.colored_label(color, format!("{i}"));
                    ui.colored_label(Color32::DARK_GRAY, "->");
                    ui.colored_label(color, DIAGNOSIS_STATE_REPRS[i]);
                    ui.end_row();
                }

            });
        });

        util::canvas_new(ui).show(ui, |ui| {
            self.canvas_statemachine(ui);
        });

        ui.label(format!("Status: {}", self.diag_state));

        // TODO: this
        /*
        let selected = &mut self.diagnosis.lock().unwrap().mode;

        egui::ComboBox::from_label("Modus")
        .selected_text(format!("{:?}", selected))
        .show_ui(ui, |ui| {
        ui.selectable_value(selected, DiagnosisMode::Automatic, "Automatisch");
        ui.selectable_value(selected, DiagnosisMode::Manual,    "Manuell");
        });
        */

        let is_running = self.diag_thread_handle.is_some();

        let btn_start: egui::Response = ui.add_enabled(
            !is_running,
            egui::Button::new("Start")
        );



        if btn_start.clicked() {
            assert!(self.diag_thread_handle.is_none(), "Diagnosis is already running");

            let diag = self.diagnosis.clone();

            let handle = std::thread::Builder::new()
                .name("diagnosis".to_owned())
                .spawn(move || {
                    diag.lock().unwrap().run_to_end()
                }).unwrap();

            self.diag_thread_handle = Some(handle);

        }

        // TODO: maybe switch to logging window
        let modal_error   = Self::ui_error(ctx, "ERROR");
        let modal_success = Self::ui_error(ctx, "SUCCESS");


        if let Some(h) = &self.diag_thread_handle {

            if h.is_finished() {
                let handle = Option::take(&mut self.diag_thread_handle).unwrap();
                let result: DiagnosisResult = handle.join().unwrap();

                match result {
                    Ok(value) => {
                        println!("Diagnosis was successful!");
                        modal_success.open();
                    }
                    Err(error) => {
                        println!("Diagnosis failed!");
                        modal_error.open();
                    }
                }

            }

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



    pub fn ui_serialmanager(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

        // TODO: this
        // text edit for writing serial to eeprom of module
        ui.heading("Verwaltung - Seriennummer");
        // ui.button("Seriennummer Beschreiben");

        let serial: String = if let Ok(diag) = self.diagnosis.try_lock() {
            diag.eeprom.get_serial().unwrap()
        } else {
            "Not Available".to_owned()
        };

        ui.label(format!("Serial: {}", serial));


        let modal = Self::ui_error(ctx, "Cannot access EEPROM while diagnosis is active");


        if ui.button("Seriennummer Lesen").clicked() {

            let serial: String = if let Ok(diag) = self.diagnosis.try_lock() {
                diag.eeprom.get_serial().unwrap()
            } else {
                modal.open();
                "Not Available".to_owned()
            };

            println!("{}", serial);

        }

    }






    fn canvas_pineditor(&mut self, ui: &mut egui::Ui) {
        use egui::{vec2, Vec2, Pos2, pos2, Sense, Painter, Rect, Rounding, Stroke, Response};

        const PIN_COUNT: u32 = 4; // TODO: refactor to config.rs

        let width:  f32 = ui.available_width();
        let height: f32 = 150.0;
        let (response, painter, center): (Response, Painter, Pos2) = util::canvas_setup(ui, width, height);


        let radius        = 10.0;
        let gap           = 5.0;
        let offset        = radius * 2.0 + gap;
        let offset_origin = ((offset * PIN_COUNT as f32) / 2.0) - radius;

        // TODO: scaling
        // TODO: keep track of selected pins, toggle selection with click


        for x in 0..PIN_COUNT {
            for y in 0..PIN_COUNT {

                let mut pin_color = config::COLOR_STATE; // TODO: config.rs

                let circle_center = center
                - Vec2::splat(offset_origin)
                + vec2(x as f32 * offset, y as f32 * offset);


                if let Some(pos) = response.hover_pos() {
                    if pos.distance(circle_center) < radius {
                        // TODO: refactor multiply factor to config.rs
                        pin_color = pin_color.gamma_multiply(0.75);
                    }
                }

                painter.circle_filled(
                    circle_center,
                    radius,
                    pin_color
                );

            }
        }

    }



    pub fn ui_pineditor(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

        ui.heading("Pin Editor");

        util::canvas_new(ui).show(ui, |ui| {
            self.canvas_pineditor(ui);
        });

    }

    pub fn ui_logging(&mut self, ui: &mut egui::Ui) {
        use logger::{LogMessage, LogLevel, Logger};

        ui.heading("Logging");

        // TODO: optionally write log to file

        if ui.button("Leeren").clicked() {
            self.logger.clear();
        }


        for msg in &self.logger.log {

            use LogLevel as L;
            let color = match msg.level {
                L::Info    => config::COLOR_LOG_INFO,
                L::Warning => config::COLOR_LOG_WARNING,
                L::Error   => config::COLOR_LOG_ERROR,
            };

            ui.horizontal(|ui| {
                ui.colored_label(Color32::DARK_GRAY, msg.timestamp.clone());

                ui.label(
                    egui::RichText::new(msg.level.to_string())
                        .background_color(color)
                        .strong()
                );

                ui.label(msg.message.clone());
                ui.end_row();
            });

        }


    }




}
