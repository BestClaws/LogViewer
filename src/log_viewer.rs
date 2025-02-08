use crate::colors;
use crate::indexer::Indexer;
use crate::string_ext::StringExt;
use crate::ui_ext::UiExt;
use eframe::epaint::text::TextWrapMode;
use eframe::{App, Frame};
use egui::{Context, Style, TextStyle, Theme, Ui};
use egui_extras::{Column, TableBuilder};
use std::collections::HashMap;
use tokio::time::Instant;

pub struct LogViewer {
    t: Instant,
    search_query: String,
    status_bar_infos: Vec<String>,
    results: Vec<HashMap<String, String>>,
}

impl LogViewer {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            t: Instant::now(),
            search_query: "".to_string(),
            status_bar_infos: vec![String::from("indexing"), String::from("searching")],
            results: vec![],
        }
    }

    fn status_bar_ui(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.style_mut().visuals.override_text_color = Some(colors::SECONDARY_TEXT_STRONG.hex_color());
            ui.label(format!("fps: {}", 1000 / self.t.elapsed().as_millis()));
            self.t = Instant::now();
            for info in &self.status_bar_infos {
                let label_response = ui.label(info);
                ui.add(
                    egui::widgets::Spinner::new()
                        .size(label_response.intrinsic_size.unwrap().y)
                        .color(colors::YELLOW_ACCENT.hex_color()),
                );
            }
        });
    }

    fn search_widget_ui(&mut self, ui: &mut Ui) {
        // search widget

        let result = ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button("search").clicked() {
                let mut indexer = Indexer::new();
                self.results = indexer
                    .query(self.search_query.clone())
                    .into_iter()
                    .collect::<Vec<HashMap<String, String>>>();
            }

            if ui.button("Index").clicked() {
                let mut indexer = Indexer::new();
                indexer.index_logfile();
            }

            let search_widget = ui.e_text_edit(&mut self.search_query);

            if search_widget.has_focus() && ui.input::<bool>(|i| i.key_pressed(egui::Key::Enter)) {
                let mut indexer = Indexer::new();
                self.results = indexer.query(self.search_query.clone());
            }
        });
    }

    fn search_results_ui(ui: &mut Ui, results: &Vec<HashMap<String, String>>) {
        let frame = egui::frame::Frame {
            fill: colors::BG_CONTAINER.hex_color(),
            inner_margin: egui::Margin::same(4.),
            rounding: egui::Rounding::same(4.),
            ..Default::default()
        };
        frame.show(ui, |ui| {
            // search results
            egui::ScrollArea::both().show(ui, |ui| {
                let mut builder = TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center));

                if results.is_empty() {

                    ui.label("No results");
                    return;
                }
                let mut columns: Vec<&String> = results.get(0).unwrap().keys().collect();
                columns.sort();
                builder = builder.column(Column::remainder()).resizable(true);

                builder
                    .min_scrolled_height(0.0)
                    .header(20.0, |mut header| {
                        for &columnName in &columns {
                            header.col(|ui| {
                                ui.strong(columnName);
                            });
                        }
                    })
                    .body(|mut body| {
                        body.rows(20., results.len(), |mut table_row| {
                            let row_index = table_row.index();
                            let result_row = &results[row_index];
                            for &columnName in &columns {
                                table_row.col(|ui| {
                                    let val_row =
                                        egui::Label::new(result_row.get(columnName).unwrap())
                                            .wrap_mode(TextWrapMode::Wrap);
                                    ui.add(val_row);
                                });
                            }
                        });
                    });
            })
        });
    }
}

impl App for LogViewer {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::bottom("status_bar")
            .frame(egui::frame::Frame {
                fill: colors::FOOTER.hex_color(),
                // stroke: Stroke::from((0f32, "#555555".hex_color())),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    self.status_bar_ui(ctx, ui);
                });
            });
        // central panel.
        egui::CentralPanel::default()
            .frame(egui::frame::Frame {
                inner_margin: egui::Margin::symmetric(10., 10.),
                ..Default::default()
            })
            .show(ctx, |ui| {
                egui::Image::new(egui::include_image!("../assets/app_bg.png"))
                    .paint_at(ui, ui.ctx().screen_rect());
                ui.vertical(|ui| {
                    self.search_widget_ui(ui);
                    ui.add_space(10.0);
                    Self::search_results_ui(ui, &self.results);
                });
            });
    }
}
