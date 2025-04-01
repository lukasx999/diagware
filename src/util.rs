use egui::Color32;

// Utility and helper functions

pub fn get_time() -> String {
    chrono::Local::now()
        .time()
        .format("%H:%M:%S")
        .to_string()
}

pub fn get_date() -> String {
    chrono::Local::now()
        .date_naive()
        .format("%d.%m.%Y")
        .to_string()
}


pub fn canvas_setup(
    ui: &mut egui::Ui,
    width: f32,
    height: f32
) -> (egui::Response, egui::Painter, egui::Pos2) {
    use egui::{vec2, Sense, Painter, Rect, Response};

    let (response, painter): (Response, Painter) = ui.allocate_painter(
        vec2(width, height),
        Sense::click_and_drag()
    );

    let rect: Rect = ui.allocate_at_least(
        vec2(0.0, 0.0),
        Sense::click()
    ).0;

    let center = rect.center()
    - vec2(0.0, height/2.0)
    + vec2(width/2.0, 0.0);

    (response, painter, center)

}

pub fn canvas_new(ui: &egui::Ui) -> egui::containers::Frame {
    egui::containers::Frame::canvas(ui.style())
        .corner_radius(10.0)
        .outer_margin(10.0)
        .fill(Color32::from_rgb(35, 35, 35))
}
