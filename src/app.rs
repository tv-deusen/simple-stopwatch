use crate::timer::{Timer, TimerState};
use eframe::egui;

#[derive(Default)]
pub struct StopwatchApp {
    timer: Timer,
}

impl StopwatchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for StopwatchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Stopwatch");
            ui.vertical(|ui| {
                ui.label(format!("{}", self.timer.elapsed()));
                ui.horizontal(|ui| match self.timer.state {
                    TimerState::Stopped => {
                        if ui.button("Start").clicked() {
                            self.timer.start();
                        }
                    }
                    TimerState::Running => {
                        ui.horizontal(|ui| {
                            if ui.button("Pause").clicked() {
                                self.timer.pause();
                            }
                            if ui.button("Stop").clicked() {
                                self.timer.stop();
                            }
                        });
                    }
                    TimerState::Paused => {
                        if ui.button("Resume").clicked() {
                            self.timer.resume();
                        }
                    }
                });
            })
        });
    }
}
