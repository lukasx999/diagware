use crate::ui::components::prelude::*;


pub struct Topbar {
    show_windowlist: Rc<RefCell<bool>>,
    is_expertmode:   Rc<RefCell<bool>>,
    logger:          Rc<RefCell<Logger>>,

    modal_open: bool,
    modal_current_password: String,
}

impl Component for Topbar {
    fn name(&self) -> &'static str {
        "Topbar"
    }
    fn show(&mut self, ctx: &egui::Context, _active: &mut bool) {
        egui::TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

impl Topbar {

    pub fn new(
        logger:          Rc<RefCell<Logger>>,
        show_windowlist: Rc<RefCell<bool>>,
        is_expertmode:   Rc<RefCell<bool>>,
    ) -> Self {
        Self {
            logger,
            show_windowlist,
            is_expertmode,
            modal_open: false,
            modal_current_password: String::new(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        if self.modal_open {
            self.login_modal(ui);
        }

        ui.horizontal(|ui| {

            ui.toggle_value(
                &mut self.show_windowlist.borrow_mut(),
                format!("🗖 Windows")
            );

            let icon_lock      = egui_phosphor::regular::LOCK_SIMPLE;
            let icon_lock_open = egui_phosphor::regular::LOCK_SIMPLE_OPEN;

            if *self.is_expertmode.borrow() {

                if ui.button(format!("{icon_lock} Logout")).clicked() {
                    *self.is_expertmode.borrow_mut() = false;
                }

            } else {

                if ui.button(format!("{icon_lock_open} Login")).clicked() {
                    self.modal_open = true;
                }

            }



            if ui.button(egui_phosphor::regular::POWER).clicked() {

                if cfg!(all(target_arch = "aarch64", not(debug_assertions))) {
                    std::process::Command::new("systemctl")
                        .args(["poweroff"])
                        .spawn()
                        .unwrap();
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

    fn login_modal(&mut self, ui: &mut egui::Ui) {
        use egui::containers::Modal;
        use egui::Id;

        let modal = Modal::new(Id::new("Login")).show(ui.ctx(), |ui| {

            ui.heading("Login");

            let response = egui::TextEdit::singleline(&mut self.modal_current_password)
                .password(true)
                .show(ui).response;

            response.request_focus();

            let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
            if enter_pressed {
                self.login();
            }


            ui.separator();

            egui::Sides::new().show( ui, |_ui| (), |ui| {
                if ui.button("Cancel").clicked() {
                    self.modal_current_password.clear();
                    self.modal_open = false;
                }
                if ui.button("Login").clicked() {
                    self.login();
                }
            },
            );

        });

        if modal.should_close() {
            self.modal_open = false;
        }

    }

    fn login(&mut self) {
        let mut logger = self.logger.borrow_mut();

        if self.modal_current_password == config::EXPERT_PASSWORD {
            *self.is_expertmode.borrow_mut() = true;
            logger.append(LogLevel::Info, "Logged in as Expert");
        } else {
            logger.append(LogLevel::Error, "Password is incorrect");
        }

        self.modal_current_password.clear();
        self.modal_open = false;

    }


}
