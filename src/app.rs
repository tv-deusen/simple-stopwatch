use crate::timer::Timer;
use eframe::egui;
use std::time::Duration;
use tokio::task;

#[derive(Default)]
pub struct StopwatchApp {
    timer: Timer,
    state: State,
}

impl StopwatchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

#[derive(Default, PartialEq, Eq)]
enum State {
    #[default]
    Stopped,
    Running,
    Paused,
}

impl eframe::App for StopwatchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Stopwatch");
            ui.vertical(|ui| {
                if self.state == State::Running {
                    let cc = ctx.clone();
                    task::spawn(async move {
                        tokio::time::sleep(Duration::new(1, 0)).await;
                        cc.request_repaint();
                    });
                }
                ui.label(self.timer.elapsed());
                ui.horizontal(|ui| match self.state {
                    State::Stopped => {
                        if ui.button("Start").clicked() {
                            self.timer.start();
                            self.state = State::Running;
                        }
                    }
                    State::Running => {
                        ui.horizontal(|ui| {
                            if ui.button("Pause").clicked() {
                                self.timer.pause();
                                self.state = State::Paused;
                            }
                            if ui.button("Stop").clicked() {
                                self.timer.reset();
                                self.state = State::Stopped;
                            }
                        });
                    }
                    State::Paused => {
                        if ui.button("Resume").clicked() {
                            self.timer.start();
                            self.state = State::Running;
                        }
                        if ui.button("Stop").clicked() {
                            self.timer.reset();
                            self.state = State::Stopped;
                        }
                    }
                });
            })
        });
    }
}
