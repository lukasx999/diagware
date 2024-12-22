use crate::ui::Component;


pub struct Pineditor {
}

impl Component for Pineditor {
    fn name(&self) -> &'static str {
        "Pineditor"
    }
    fn show(&mut self, ctx: &egui::Context, active: &mut bool) {
        egui::Window::new(self.name())
            .fade_in(true)
            .fade_out(true)
            .open(active)
            .enabled(true)
            .vscroll(true)
            .hscroll(true)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}


impl Pineditor {

    pub fn new() -> Self {
        Self {}
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        use crate::ui::util;

        /*
        TODO:
        combobox for selecting Pin (fetch from DB)
        click on pin to apply pin selection
        also color pins accordingly
        */


        util::canvas_new(ui).show(ui, |ui| {
            self.canvas_pineditor(ui);
        });

    }


    fn canvas_pineditor(&mut self, ui: &mut egui::Ui) {
        use egui::{vec2, Vec2, Pos2, Painter, Response};
        use crate::ui::{config, util};

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

                let mut pin_color = config::COLOR_CIRCLE; // TODO: config.rs

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
}
