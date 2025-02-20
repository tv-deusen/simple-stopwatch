#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod timer;

use crate::app::StopwatchApp;
use eframe::egui;

fn main() -> eframe::Result {
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Stopwatch",
        options,
        Box::new(|cc| Ok(Box::new(crate::StopwatchApp::new(cc)))),
    )
}
