#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod dialog;
mod manage_maps;
mod maps;
mod pref;

use eframe::{egui, NativeOptions};
use egui::{IconData, Vec2, ViewportBuilder};
use std::sync::Arc;

use crate::app::MapLoaderApp;
use crate::pref::Pref;

const WIDTH: f32 = 700.0;
const HEIGHT: f32 = 500.0;
const VEC2_SIZE: Vec2 = Vec2 {
    x: WIDTH,
    y: HEIGHT,
};
const TITLE_SUCCESS: &str = "✅ Success";
const TITLE_ERROR: &str = "⚠ Error";

fn main() -> eframe::Result {
    let prefs = pref::load_pref().unwrap_or_else(|e| {
        eprintln!("Error loading preferences : {}", e);
        Pref::default()
    });
    let app = MapLoaderApp::with_pref(prefs);

    let mode = dark_light::detect();

    //Include the icon directly into the binary
    let icon_bytes = include_bytes!("..\\media\\icon-128.png");
    let dark_icon_bytes = include_bytes!("..\\media\\dark-icon-128.png");
    let icon = match mode {
        dark_light::Mode::Light | dark_light::Mode::Default => image::load_from_memory(icon_bytes)
            .expect("Failed to load icon data")
            .to_rgba8(),
        dark_light::Mode::Dark => image::load_from_memory(dark_icon_bytes)
            .expect("Failed to load icon data")
            .to_rgba8(),
    };
    let (icon_width, icon_height) = icon.dimensions();

    let viewport = ViewportBuilder::default()
        .with_inner_size(VEC2_SIZE)
        .with_min_inner_size(VEC2_SIZE)
        .with_icon(Arc::new(IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }));

    let options = NativeOptions {
        viewport,
        follow_system_theme: true,
        ..NativeOptions::default()
    };
    eframe::run_native(
        "Rocket League Map Loader",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
