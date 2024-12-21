
    pub fn ui_topbar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {

        let modal = egui_modal::Modal::new(ctx, "Login");

        // TODO: create another egui window for textedit

        modal.show(|ui| {

            modal.title(ui, "Login");

            modal.frame(ui, |ui| {
                modal.body(ui, "Passworteingabe");
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
                if cfg!(target_arch = "aarch64") {
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
