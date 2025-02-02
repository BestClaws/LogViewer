use eframe::{App, Frame};
use egui::{Color32, Context};
use egui_extras::{Column, TableBuilder};
use crate::string_ext::StringExt;

pub struct LogViewer {
    search_query: String,
}

impl LogViewer {
    pub fn new() -> Self {
        Self {
            search_query: "".to_string(),
        }
    }
}

impl App for LogViewer {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        
        ctx.set_zoom_factor(1.2);
        // central panel.
        egui::CentralPanel::default().frame(egui::frame::Frame {
            fill: "#2a2a2a".hex_color(),
            inner_margin: egui::Margin::symmetric(5.0, 5.0),
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0, },
            ..Default::default()
        }).show(ctx, |ui| {
           
            ui.vertical(|ui|{
                // search widget
                let search_widget = egui::widgets::TextEdit::multiline(&mut self.search_query)
                    .background_color("#3a3a3a".hex_color());
                ui.add_sized([ui.available_width(), 50.], search_widget);
              
                // search results
                egui::ScrollArea::both().show(ui, |ui| {
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

                    table.header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Timestamp");
                        });
                        header.col(|ui| {
                            ui.strong("Search Results");
                        });

                    })
                        
                        
                    
                        .body(|mut body| {
                            for i in 1..100 {
                                body.row(20., |mut row| {
                                    row.col(|ui| {
                                        ui.label("00:00:00");
                                    });
                                    row.col(|ui| {
                                        ui.label("hello how are you and what are you doing");
                                    });
                                });
                            }
                        });
                });
                
            });
            
            
        });
    }
}
