use crate::indexer::Indexer;
use crate::string_ext::StringExt;
use eframe::epaint::text::TextWrapMode;
use eframe::{App, Frame};
use egui::{Context, Stroke, Ui};
use egui_extras::{Column, TableBuilder};
use tokio::time::Instant;
use crate::ui_ext::UiExt;


pub struct LogViewer {
    t: Instant,
    search_query: String,
    status_bar_infos: Vec<String>,
    results: Vec<String>,
}

impl LogViewer {
    pub fn new() -> Self {
        println!("no");
        Self {
            t: Instant::now(),
            search_query: "".to_string(),
            status_bar_infos: vec![String::from("indexing"), String::from("searching")],
            results: vec![],
        }
    }

    fn status_bar_ui(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("fps: {}", 1000 / self.t.elapsed().as_millis()));
            self.t = Instant::now();
            for info in &self.status_bar_infos {
                let label_response = ui.label(info);
                ui.add(
                    egui::widgets::Spinner::new()
                        .size(label_response.intrinsic_size.unwrap().y)
                        .color("#827156".hex_color()),
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
                    .collect::<Vec<String>>();
            }

            if ui.button("Index").clicked() {
                let mut indexer = Indexer::new();
                indexer.index_logfile();
            }

            let search_widget = ui.e_text_edit(&mut self.search_query);

            if search_widget.has_focus() && ui.input::<bool>(|i| {
                i.key_pressed(egui::Key::Enter)
            }) {

                let mut indexer = Indexer::new();
                self.results = indexer
                    .query(self.search_query.clone())
                    .collect::<Vec<String>>();
            }


        });
    }

    fn search_results_ui(ui: &mut Ui, results: &Vec<String>) {
        let frame = egui::frame::Frame {
            fill: "#3b3b3b".hex_color(),
            inner_margin: egui::Margin::same(4.),
            rounding: egui::Rounding::same(4.),
            ..Default::default()
        };
        frame.show(ui, |ui| {
            // search results
            egui::ScrollArea::both().show(ui, |ui| {
                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .resizable(true)
                    .column(Column::remainder().clip(false).resizable(true))
                    .auto_shrink(true)
                    .min_scrolled_height(0.0);

                table
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Timestamp");
                        });
                        header.col(|ui| {
                            ui.strong("Search Results");
                        });
                    })
                    .body(|mut body| {
                        body.rows(20., results.len(), |mut row| {
                            let row_index = row.index();
                            let val = &results[row_index];
                            row.col(|ui| {
                                ui.label("00:00:00");
                            });
                            let (a, b) = row.col(|ui| {
                                let val_row = egui::Label::new(val).wrap_mode(TextWrapMode::Wrap);
                                ui.add(val_row);
                            });
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
                fill: "#2c2c2c".hex_color(),
                stroke: Stroke::from((0f32, "#555555".hex_color())),
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
                fill: "#2c2c2c".hex_color(),
                inner_margin: egui::Margin::symmetric(10., 10.),
                rounding: egui::Rounding {
                    nw: 1.0,
                    ne: 1.0,
                    sw: 1.0,
                    se: 1.0,
                },
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.search_widget_ui(ui);
                    ui.add_space(10.0);
                    Self::search_results_ui(ui, &self.results);
                });
            });
    }
}
