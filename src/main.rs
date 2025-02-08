mod animal;
mod colors;
mod ext;
mod indexer;
mod log_loader;
mod log_viewer;
mod text_edit;

use crate::ext::string_ext::StringExt;
use crate::log_viewer::LogViewer;
use egui::{Stroke, Style, Theme, ViewportBuilder};
use std::fs::DirBuilder;

const APP_NAME: &'static str = "LogViewer";

fn main() -> eframe::Result {
    let async_runtime = tokio::runtime::Runtime::new().unwrap();

    // keep the reactor running
    let _enter = async_runtime.enter();

    // prepare directories
    DirBuilder::new()
        .recursive(true)
        .create("data/indexes")
        .unwrap();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1280., 720.]),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_zoom_factor(1.2);
            cc.egui_ctx.set_theme(Theme::Light);
            setup_custom_style(&cc.egui_ctx);
            Ok(Box::new(LogViewer::new(cc)))
        }),
    )
}

fn setup_custom_style(ctx: &egui::Context) {
    ctx.style_mut_of(Theme::Light, use_endfield_theme);
}

fn use_endfield_theme(style: &mut Style) {
    style.visuals.override_text_color = Some(colors::PRIMARY_TEXT.hex_color());
    style.visuals.extreme_bg_color = colors::BG.hex_color();
    style.visuals.widgets.hovered.weak_bg_fill = colors::YELLOW_ACCENT.hex_color();
    style.visuals.widgets.inactive.bg_stroke = Stroke {
        width: 1.,
        color: colors::SHADE2.hex_color(),
    };

    style.visuals.selection.stroke = Stroke {
        width: 1.,
        color: colors::SHADE2.hex_color(),
    }
}
