use crate::ui::components::prelude::*;

pub struct MyComponent;

impl Component for MyComponent {
    fn name(&self) -> &'static str {
        "My Component"
    }
    fn show(&mut self, ctx: &egui::Context, active: &mut bool) {
        egui::Window::new(self.name())
            .open(active)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl MyComponent {
    pub fn new() -> Self { Self }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("This is My Component!");
    }
}
