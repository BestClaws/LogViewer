mod animal;
mod colors;
mod ext;
mod indexer;
mod log_loader;
mod log_viewer;
mod text_edit;

use crate::ext::string_ext::StringExt;
use crate::log_viewer::LogViewer;
use egui::{IconData, Stroke, Style, Theme, ViewportBuilder};
use std::collections::HashMap;
use std::fs::DirBuilder;
use std::hash::Hash;
use std::sync::Arc;
use tree_sitter::{Node, Parser};
use tree_sitter_loguage;
use crate::Val::LuceneQuery;

const APP_NAME: &'static str = "LogViewer";

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let async_runtime = tokio::runtime::Runtime::new().unwrap();
    // keep the reactor running
    let _enter = async_runtime.enter();

    play();

    // boot_logviewer();
}

fn play() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_loguage::LANGUAGE.into())
        .expect("Error loading Rust grammar");

    let source_code = "[search `hi`]";
    let mut tree = parser.parse(source_code, None).unwrap();
    let val = eval(source_code.as_bytes(), tree.root_node(), &mut HashMap::new());
    println!("val: {:?}", val);
}

#[derive(Debug)]
enum Val {
    Operation,
    LuceneQuery(String),
    Nil
}

fn eval(source: &[u8], node: Node, data: &HashMap<&str, Val>) -> Val {
    match node.kind() {
        "query" => {
            let cursor = &mut node.walk();
            let operations = node.children(cursor);
            let mut data: HashMap<&str, Val> = HashMap::new();
            for operation in operations {
                if !operation.is_named() { continue; }
                let result = eval(&source, operation, &data);
                data.insert("last_output", result);
            }

           let val = data.remove("last_output");
            val.unwrap_or(Val::Nil)
        }
        "operation" => {
            let operation_name = node.child(0).unwrap().utf8_text(source).unwrap();
            let operation_arguments =  node.child(1).unwrap();
            
            if operation_name == "search" {
                // this will be lucene node for sure
                let expression = operation_arguments.child(0).unwrap().utf8_text(source).unwrap();
                return LuceneQuery(expression.to_string())
            }
            Val::Nil
        }

        _ => {
            Val::Nil
        }
    }
}

fn boot_logviewer() -> eframe::Result {
    let icon = image::open("assets/icon.png")
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let icon_data = Arc::new(IconData {
        rgba: icon.into_raw(),
        width: icon_width,
        height: icon_height,
    });

    // prepare directories
    DirBuilder::new()
        .recursive(true)
        .create("data/indexes")
        .unwrap();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1280., 720.])
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_zoom_factor(1.2);
            cc.egui_ctx.set_theme(Theme::Light);
            egui_material_icons::initialize(&cc.egui_ctx);
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
