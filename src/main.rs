mod log_viewer;
mod text_edit;
mod string_ext;

use egui::ViewportBuilder;
use crate::log_viewer::LogViewer;

const APP_NAME: &'static str = "LogViewer";


fn main() -> eframe::Result {

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1280., 720.]),
        ..Default::default()
    };

    eframe::run_native(
       APP_NAME,
       native_options,
       Box::new(|cc| {
           Ok(Box::new(LogViewer::new()))
       })
    )
}

