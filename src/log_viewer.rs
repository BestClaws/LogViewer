use crate::indexer::Indexer;
use crate::string_ext::StringExt;
use eframe::{App, Frame};
use egui::{Context, Ui};
use egui_extras::{Column, TableBuilder};
use tokio::time::Instant;

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
                        .color("#a1a1a1".hex_color()),
                );
            }
        });
    }

    fn search_widget_ui(&mut self, ui: &mut Ui) {
        // search widget
        ui.horizontal(|ui| {
            let search_widget = egui::widgets::TextEdit::multiline(&mut self.search_query)
                .background_color("#aeaeae".hex_color());
            // .background_color("#fefefe".hex_color());
            ui.add(search_widget);
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
        });
    }

    fn search_results_ui(ui: &mut Ui, results: &Vec<String>) {
        // search results
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(false)
                    .resizable(true),
            )
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
                       row.col(|ui| {
                           ui.label(val);
                       });
                       
                   });
                    
            
            });
    }
}

impl App for LogViewer {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.status_bar_ui(ctx, ui);
            });
        });
        // central panel.
        egui::CentralPanel::default()
            .frame(egui::frame::Frame {
                fill: "#c1c1c1".hex_color(),
                inner_margin: egui::Margin::symmetric(5.0, 5.0),
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
                    Self::search_results_ui(ui, &self.results);
                });
            });
    }
}
