mod log_viewer;
mod text_edit;
mod string_ext;
mod log_loader;
mod indexer;
mod ui_ext;

use std::fs::DirBuilder;
use egui::{Style, ViewportBuilder};
use crate::log_viewer::LogViewer;

const APP_NAME: &'static str = "LogViewer";


fn main() -> eframe::Result {
    
    // prepare directories
    DirBuilder::new().recursive(true).create("data/indexes").unwrap();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1280., 720.]),
        ..Default::default()
    };

    eframe::run_native(
       APP_NAME,
       native_options,
       Box::new(|cc| {
           cc.egui_ctx.set_style(Style {
               visuals: egui::Visuals::dark(),
               ..Default::default()
           });
           cc.egui_ctx.set_zoom_factor(1.2);
           Ok(Box::new(LogViewer::new()))
       })
    )
}

