use std::time::{Duration, Instant};

#[derive(Default, Debug)]
pub struct Timer {
    start_time: Option<Instant>,
    elapsed_time: Duration,
}

impl Timer {
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn pause(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed_time += start.elapsed();
            self.start_time = None;
        }
    }

    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed_time = Duration::ZERO;
    }

    pub fn elapsed(&self) -> String {
        let cum_time = match self.start_time {
            Some(start) => self.elapsed_time + start.elapsed(),
            None => self.elapsed_time,
        };
        let hours = cum_time.as_secs() / 3600;
        let minutes = cum_time.as_secs() / 60 % 60;
        let seconds = cum_time.as_secs() % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
