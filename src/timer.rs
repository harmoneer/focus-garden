use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SessionType {
    Focus,
    ShortBreak,
    LongBreak,
}

impl SessionType {
    pub fn duration_minutes(&self, settings: &crate::storage::Settings) -> u64 {
        match self {
            SessionType::Focus => settings.focus_duration,
            SessionType::ShortBreak => settings.short_break_duration,
            SessionType::LongBreak => settings.long_break_duration,
        }
    }
}

#[derive(Debug)]
pub struct Timer {
    pub state: TimerState,
    pub session_type: SessionType,
    pub duration_seconds: u64,
    pub remaining_seconds: u64,
    pub auto_run: Vec<SessionType>,
    pub auto_run_index: Option<usize>,
    pub last_tick: Option<Instant>,
}

impl Timer {
    pub fn new(settings: &crate::storage::Settings) -> Self {
        Timer {
            state: TimerState::Idle,
            session_type: SessionType::Focus,
            duration_seconds: SessionType::Focus.duration_minutes(settings) * 60,
            remaining_seconds: SessionType::Focus.duration_minutes(settings) * 60,
            auto_run: vec![],
            auto_run_index: None,
            last_tick: None,
        }
    }

    pub fn start(&mut self) {
        if self.state == TimerState::Idle || self.state == TimerState::Paused {
            self.state = TimerState::Running;
            self.last_tick = Some(Instant::now());
        }
    }

    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            self.state = TimerState::Paused;
            self.last_tick = None;
        }
    }

    pub fn resume(&mut self) {
        if self.state == TimerState::Paused {
            self.state = TimerState::Running;
            self.last_tick = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        self.state = TimerState::Idle;
        self.remaining_seconds = self.duration_seconds;
        self.last_tick = None;
    }

    pub fn reset(&mut self) {
        self.stop();
    }

    pub fn set_session(&mut self, session_type: SessionType, settings: &crate::storage::Settings) {
        self.session_type = session_type;
        self.duration_seconds = session_type.duration_minutes(settings) * 60;
        self.remaining_seconds = self.duration_seconds;
        self.state = TimerState::Idle;
    }

    pub fn switch_session(&mut self, session_type: SessionType, settings: &crate::storage::Settings) {
        self.session_type = session_type;
        self.duration_seconds = session_type.duration_minutes(settings) * 60;
        self.remaining_seconds = self.duration_seconds;
        self.state = TimerState::Idle;
    }

    pub fn tick(&mut self) -> bool { // returns true if session finished
        if self.state == TimerState::Running {
            if let Some(last) = self.last_tick {
                let elapsed = last.elapsed();
                if elapsed >= Duration::from_secs(1) {
                    if self.remaining_seconds > 0 {
                        self.remaining_seconds -= 1;
                    }
                    self.last_tick = Some(Instant::now());
                    if self.remaining_seconds == 0 {
                        self.state = TimerState::Finished;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn progress(&self) -> f64 {
        1.0 - (self.remaining_seconds as f64 / self.duration_seconds as f64)
    }

    pub fn add_to_auto_run(&mut self, session: SessionType) {
        self.auto_run.push(session);
    }

    pub fn remove_from_auto_run(&mut self, index: usize) {
        if index < self.auto_run.len() {
            self.auto_run.remove(index);
        }
    }

    pub fn next_auto_run(&mut self, settings: &crate::storage::Settings) {
        if let Some(next) = self.auto_run.first().cloned() {
            self.set_session(next, settings);
            self.auto_run.remove(0);
        }
    }
}