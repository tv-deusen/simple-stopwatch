use std::time::{Duration, Instant};

enum TimerState {
    Stopped,
    Paused,
    Running,
}

struct Timer {
    state: TimerState,
    start_time: Option<Instant>,
    accumulated: Duration,
    current_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            state: TimerState::Stopped,
            start_time: None,
            accumulated: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) {
        self.state = TimerState::Running;
        self.start_time = Some(Instant::now() + Instant::from(self.accumulated));
    }

    pub fn pause(&mut self) {
        self.state = TimerState::Paused;
        self.start_time = self.start_time.unwrap() + self.accumulated;
    }
}
