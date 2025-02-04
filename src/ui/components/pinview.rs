use crate::ui::components::prelude::*;
use crate::db::DB;



pub struct Pinview {
    db: DB,
}

impl Component for Pinview {
    fn name(&self) -> &'static str {
        config::PAGE_PINVIEW
    }
    fn show(&mut self, ctx: &egui::Context, active: &mut bool) {
        egui::Window::new(self.name())
            .fade_in(true)
            .fade_out(true)
            .open(active)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}


impl Pinview {

    pub fn new() -> Self {
        Self {
            db: DB::new().unwrap(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {

        // TODO: highlight legend when hovering pin
        // multiple highlights for multiple pins
        // TODO: how do we get to current module from running diagnosis?

        // let module = ...

        util::canvas_new(ui).show(ui, |ui| {
            self.canvas_pineditor(ui);
        });

        self.ui_legend(ui);

    }

    fn ui_legend(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Legend", |ui| {
            ui.horizontal_wrapped(|ui| {

                let legend = [
                    ("GND",  Color32::GRAY),
                    ("V+",   Color32::RED),
                    ("V-",   Color32::BLUE),
                    ("DDS1", Color32::YELLOW),
                    ("DDS2", Color32::ORANGE),
                    ("DDS3", Color32::LIGHT_YELLOW),
                    ("ADC1", Color32::GREEN),
                    ("ADC2", Color32::DARK_GREEN),
                ];

                for (index, pair) in legend.iter().enumerate() {
                    ui.colored_label(Color32::WHITE, format!("{index}"));
                    ui.colored_label(Color32::DARK_GRAY, "->");
                    ui.colored_label(pair.1, pair.0);
                    ui.end_row();
                }

            });
        });
    }



    fn canvas_pineditor(&mut self, ui: &mut egui::Ui) {
        use egui::{vec2, Vec2, Pos2, Painter, Response};

        const PIN_COUNT: u32 = 4; // TODO: refactor to config.rs

        let width:  f32 = ui.available_width();
        let height: f32 = 150.0;
        let (response, painter, center): (Response, Painter, Pos2) = util::canvas_setup(ui, width, height);

        let radius        = 10.0;
        let gap           = 5.0;
        let offset        = radius * 2.0 + gap;
        let offset_origin = ((offset * PIN_COUNT as f32) / 2.0) - radius;


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
