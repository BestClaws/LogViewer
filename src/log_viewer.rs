use crate::animal::EatSpit;
use crate::ext::string_ext::StringExt;
use crate::ext::ui_ext::UiExt;
use crate::indexer::Indexer;
use crate::loguage::exec::Loguage;
use crate::{animal, colors};
use eframe::epaint::text::TextWrapMode;
use eframe::{App, Frame};
use egui::{remap, Color32, Context, Pos2, Ui};
use egui_extras::{Column, TableBuilder};
use egui_material_icons::icons;
use egui_plot::{Bar, BarChart, Legend, Line, Plot, PlotPoints};
use log::{error, info, log};
use std::collections::HashMap;
use tokio::time::Instant;
use crate::loguage::interpreter::Val;

pub struct LogViewer {
    t: Instant,
    search_query: String,
    status_bar_infos: Vec<String>,
    results: EatSpit<Vec<HashMap<String, String>>>,
}

impl LogViewer {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let lv = Self {
            t: Instant::now(),
            search_query: "".to_string(),
            status_bar_infos: vec![String::from("indexing"), String::from("searching")],
            results: EatSpit::new(vec![]),
        };
        log::info!("done creating logviewer");
        lv
    }

    fn status_bar_ui(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.style_mut().visuals.override_text_color =
                Some(colors::SECONDARY_TEXT_STRONG.hex_color());
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
            let tx = self.results.mouth();
            if ui
                .button("search ".to_owned() + icons::ICON_SEARCH)
                .clicked()
            {
                let query = self.search_query.clone();
                tokio::spawn(async move {
                    if let Val::SearchResults(result) = Loguage::new().exec(&query) {
                        match tx.send(result).await {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error sending search result: {:?}", e);
                            }
                        }
                    } else {
                        error!("query failed");
                    }
                });
            }

            if ui
                .button("Index ".to_owned() + icons::ICON_MANAGE_SEARCH)
                .clicked()
            {
                let mut indexer = Indexer::new();
                indexer.index_logfile();
            }
            ui.e_query_text_edit(&mut self.search_query);
        });
    }


    fn search_results_ui(ui: &mut Ui, res: &mut EatSpit<Vec<HashMap<String, String>>>) {

        let results = res.spit();
        
        let bin_size_ms = 1000; // 1 second binning
        let time_count = process_data(results, bin_size_ms);

        // let mut chart = BarChart::new(
        //     (-395..=395)
        //         .step_by(10)
        //         .map(|x| x as f64 * 0.01)
        //         .map(|x| {
        //             (
        //                 x,
        //                 (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
        //             )
        //         })
        //         // The 10 factor here is purely for a nice 1:1 aspect ratio
        //         .map(|(x, f)| Bar::new(x, f * 10.0).width(0.1))
        //         .collect(),
        // )
        //     .color(Color32::LIGHT_BLUE);
        //
        //
        //
        // Plot::new("search_distribution")
        //     .clamp_grid(true)
        //     .allow_zoom(false)
        //     .allow_drag(false)
        //     .allow_scroll(false)
        //     .show_grid(false)
        //     .height(80.)
        //     .show(ui, |plot_ui| plot_ui.bar_chart(chart));


        let bars: Vec<Bar> = time_count.into_iter()
            .map(|(time, count)| Bar::new(time, count).width(0.1)) // Adjust width as needed
            .collect();

        let chart = BarChart::new(bars).color(Color32::LIGHT_BLUE);

        Plot::new("time_distribution")
            .clamp_grid(true)
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .show_grid(false)
            .height(80.0)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart));

        let frame = egui::frame::Frame {
            fill: colors::BG_CONTAINER.hex_color(),
            inner_margin: egui::Margin::same(4),
            corner_radius: egui::CornerRadius::same(4),
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
                for i in 0..columns.len() {
                    builder = builder.column(Column::remainder());
                }
                builder
                    .resizable(true)
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
                inner_margin: egui::Margin::symmetric(10, 10),
                ..Default::default()
            })
            .show(ctx, |ui| {
                egui::Image::new(egui::include_image!("../assets/app_bg.png"))
                    .paint_at(ui, ui.ctx().screen_rect());
                ui.vertical(|ui| {
                    self.search_widget_ui(ui);
                    ui.add_space(10.0);
                    Self::search_results_ui(ui, &mut self.results);
                });
            });
    }
}




fn process_data(data: &mut Vec<HashMap<String, String>>, bin_size_ms: u64) -> Vec<(f64, f64)> {
    let mut counts: HashMap<u64, u64> = HashMap::new();

    // Count occurrences of each time within the bin size
    for entry in data {
        if let Some(time_str) = entry.get("_time") {
            // Parse the _time string to a u64 (epoch time in milliseconds)
            if let Ok(time_millis) = time_str.parse::<u64>() {
                // Floor the time to the nearest bin
                let bin = time_millis / bin_size_ms;
                *counts.entry(bin).or_insert(0) += 1;
            }
        }
    }

    // Convert to Vec of (time, count) pairs, scale the time to seconds
    let mut time_count: Vec<(f64, f64)> = counts.into_iter()
        .map(|(bin, count)| (bin as f64 * (bin_size_ms as f64 / 1000.0), count as f64)) // Convert time to seconds
        .collect();

    // Sort by time (ascending)
    time_count.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    time_count
}
