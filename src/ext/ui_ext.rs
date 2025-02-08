use crate::colors;
use crate::ext::string_ext::StringExt;
use egui::{Response, Ui};

pub(crate) trait UiExt {
    fn e_text_edit(&mut self, text_state: &mut String) -> Response;
}

impl UiExt for Ui {
    fn e_text_edit(&mut self, text_state: &mut String) -> Response {
        let text_edit = egui::widgets::TextEdit::multiline(text_state)
            .background_color(colors::SHADE3.hex_color())
            .hint_text(egui::RichText::new("Search here").color(colors::BG.hex_color()));
        self.add_sized([self.available_width(), 30.], text_edit)
    }
}
