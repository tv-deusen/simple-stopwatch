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
            let available_height = ui.available_height();

            // Proportional spacing
            let timer_height = available_height * 0.60;
            let button_height = available_height * 0.40;

            // Timer display
            ui.add_space(timer_height * 0.5);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if self.state == State::Running {
                    let cc = ctx.clone();
                    task::spawn(async move {
                        tokio::time::sleep(Duration::new(1, 0)).await;
                        cc.request_repaint();
                    });
                }
                ui.label(
                    egui::RichText::new(self.timer.elapsed().to_string())
                        .heading()
                        .strong()
                        .size(50.0),
                );
            });

            // Buttons
            ui.add_space(button_height * 0.25);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal_centered(|ui| {
                    match self.state {
                        State::Stopped => {
                            if ui
                                .add_sized([200.0, 60.0], egui::Button::new("Start"))
                                .clicked()
                            {
                                self.timer.start();
                                self.state = State::Running;
                            }
                        }
                        State::Running => {
                            if ui
                                .add_sized([150.0, 50.0], egui::Button::new("Pause"))
                                .clicked()
                            {
                                self.timer.pause();
                                self.state = State::Paused;
                            }
                            ui.add_space(20.0); // Space between buttons
                            if ui
                                .add_sized([150.0, 50.0], egui::Button::new("Stop"))
                                .clicked()
                            {
                                self.timer.reset();
                                self.state = State::Stopped;
                            }
                        }
                        State::Paused => {
                            if ui
                                .add_sized([150.0, 50.0], egui::Button::new("Resume"))
                                .clicked()
                            {
                                self.timer.start();
                                self.state = State::Running;
                            }
                            ui.add_space(20.0); // Space between buttons
                            if ui
                                .add_sized([150.0, 50.0], egui::Button::new("Stop"))
                                .clicked()
                            {
                                self.timer.reset();
                                self.state = State::Stopped;
                            }
                        }
                    }
                });
            });

            ui.add_space(button_height * 0.25);
        });
    }
}
