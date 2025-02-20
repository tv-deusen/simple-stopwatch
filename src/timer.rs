use std::fmt::Display;
use std::time::{Duration, Instant};

use tracing::instrument;

#[derive(Debug)]
enum TimerState {
    Stopped,
    Paused,
    Running,
}

#[derive(Debug)]
pub struct Timer {
    state: TimerState,
    start_time: Option<Instant>,
    accumulated_time: Option<Duration>,
}

impl Timer {
    #[instrument(name = "new_timer")]
    pub fn new() -> Self {
        Timer {
            state: TimerState::Stopped,
            start_time: None,
            accumulated_time: None,
        }
    }

    #[instrument]
    pub fn start(&mut self) {
        self.state = TimerState::Running;
        self.start_time = Some(Instant::now());
        self.accumulated_time = Some(Duration::ZERO);
    }

    #[instrument]
    pub fn pause(&mut self) {
        self.state = TimerState::Paused;
        self.accumulated_time = Some(Instant::now().duration_since(self.start_time.unwrap()));
    }

    #[instrument]
    pub fn unpause(&mut self) {
        self.state = TimerState::Running;
        self.start_time = Some(Instant::now());
        self.accumulated_time = Some(Duration::new(0, 0));
    }

    #[instrument]
    pub fn stop(&mut self) {
        self.state = TimerState::Stopped;
        self.start_time = None;
        self.accumulated_time = None;
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elapsed = self.accumulated_time.unwrap()
            + Instant::now().duration_since(self.start_time.unwrap());
        // write!(f, "{}", elapsed.as_millis())
        let hours = elapsed.as_secs() / 3600;
        let minutes = elapsed.as_secs() / 60 % 60;
        let seconds = elapsed.as_secs() % 60;
        write!(f, "{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
