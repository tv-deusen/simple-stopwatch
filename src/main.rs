#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod timer;

use eframe::egui;
use timer::Timer;

fn main() -> eframe::Result {
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Stopwatch",
        options,
        Box::new(|cc| Box::<Content>::default()),
    )
}

#[derive(Default)]
struct Content {
    timer: Timer,
}

impl eframe::App for Content {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<eframe::ExitCode> {
        self.timer.update(ctx);
        None
    }
}
