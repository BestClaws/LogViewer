use crate::string_ext::StringExt;
use egui::{Response, Stroke, Ui};

pub(crate) trait UiExt {
    fn e_text_edit(&mut self, text_state: &mut String) -> Response;
}

impl UiExt for Ui {
    fn e_text_edit(&mut self, text_state: &mut String) -> Response {
        let frame = egui::frame::Frame {
            stroke: Stroke::from((2f32, "#555555".hex_color())),
            rounding: egui::Rounding::same(4.),
            ..Default::default()
        };

        let mut response = None;

        frame.show(self, |ui| {
            let text_edit = egui::widgets::TextEdit::multiline(text_state)
                .background_color("#3b3b3b".hex_color())
                .hint_text(egui::RichText::new("Search here").color("#2c2c2c".hex_color()));
            let res = ui.add_sized([ui.available_width(), 30.], text_edit);
            response = Some(res)
        });
        response.unwrap()
    }
}
