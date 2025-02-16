use crate::colors;
use crate::ext::string_ext::StringExt;
use egui::{Color32, FontId, Response, TextFormat, TextStyle, Ui};
use egui::text::LayoutJob;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};
use tree_sitter_loguage;

pub(crate) trait UiExt {
    fn e_query_text_edit(&mut self, text_state: &mut String) -> Response;
}

impl UiExt for Ui {
    fn e_query_text_edit(&mut self, text_state: &mut String) -> Response {
        let mut layouter = |ui: &Ui, string: &str, _wrap_width: f32| {
            let job = highlight_text(string);
            ui.fonts(|f| f.layout_job(job))
        };
        
        let text_edit = egui::widgets::TextEdit::multiline(text_state)
            .font(egui::TextStyle::Monospace)
            .background_color(colors::SHADE3.hex_color())
            .hint_text(egui::RichText::new("Search here").color(colors::BG.hex_color()))
            .layouter(&mut layouter);
        self.add_sized([self.available_width(), 30.], text_edit)
    }
}

fn highlight_text(input: &str) -> LayoutJob {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_loguage::LANGUAGE.into()).unwrap();

    let tree = parser.parse(input, None).unwrap();
    let query = get_highlight_query();
    let mut cursor = QueryCursor::new();

    let mut matches = cursor.matches(&query, tree.root_node(), input.as_bytes());

    let mut job = LayoutJob::default();
    let mut last_end = 0;

    while let Some(m) = matches.next() {
        for cap in m.captures {
            let start = cap.node.start_byte();
            let end = cap.node.end_byte();
            let text = &input[start..end];



            if last_end < start {
                job.append(
                    &input[last_end..start],
                    0.0,
                    TextFormat::simple(FontId::monospace(12.0), Color32::BLACK),
                );
            }

            let color = match query.capture_names()[cap.index as usize] {
                "operation" => Color32::DARK_BLUE,
                "lucene_query" => Color32::DARK_GREEN,
                "term" => Color32::DARK_GREEN,
                _ => Color32::BLACK,
            };

            job.append(text, 0.0, TextFormat::simple(FontId::monospace(12.0), color));
            last_end = end;
        }
    }

    if last_end < input.len() {
        job.append(
            &input[last_end..],
            0.0,
            TextFormat::simple(FontId::monospace(12.0), Color32::BLACK),
        );
    }

    job
}

// Load Tree-sitter grammar for "loguage"
fn get_highlight_query() -> Query {
    const LOGUAGE_HIGHLIGHT_QUERY: &str = r#"
    (operation_name) @operation
    (term) @term
    (lucene_query) @lucene_query
    
"#;

    
    Query::new(&tree_sitter_loguage::LANGUAGE.into(), LOGUAGE_HIGHLIGHT_QUERY).unwrap()
}
