use chrono::Local;
use crate::garden::Garden;
use crate::input::InputAction;
use crate::plant::Plant;
use crate::storage::{load_data, save_data, Data, Settings, Statistics};
use crate::theme::{Theme, ThemeVariant};
use crate::timer::{SessionType, Timer};
use ratatui::widgets::{ListState, ScrollbarState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Timer,
    Plant,
    Stats,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Left,
    Right,
}

#[derive(Debug)]
pub struct App {
    pub tab: Tab,
    pub timer: Timer,
    pub plant: Plant,
    pub garden: Garden,
    pub settings: Settings,
    pub statistics: Statistics,
    pub theme: Theme,
    // UI state
    pub timer_selected_session: usize,
    pub timer_selected_auto: usize,
    pub settings_selected: usize,
    pub stats_selected: usize,
    pub focus: Focus,
    pub should_quit: bool,
    pub timer_auto_list_state: ListState,
    pub timer_auto_scrollbar_state: ScrollbarState,
}

impl App {
    pub fn new() -> Self {
        let data = load_data();
        let settings = data.settings.clone();
        let mut timer = Timer::new(&settings);
        timer.auto_run = data.auto_run;
        timer.auto_run_index = data.auto_run_index;
        let plant = Plant::from_stage(data.current_plant_stage, data.growth_points);
        let mut garden = Garden::new();
        garden.completed_plants = vec![]; // TODO: load from data if needed
        garden.current_streak = data.statistics.current_streak;
        garden.longest_streak = data.statistics.longest_streak;
        garden.current_streak_start_date = data.statistics.current_streak_start_date;
        garden.longest_streak_end_date = data.statistics.longest_streak_end_date;
        garden.current_streak_dates = data.statistics.current_streak_dates.clone();
        garden.longest_streak_dates = data.statistics.longest_streak_dates.clone();
        garden.completed_plants = data.completed_plants.clone();
        garden.update_streaks(&data.statistics.recent_sessions);
        let statistics = data.statistics;
        let theme = Theme::new(settings.theme);

        App {
            tab: Tab::Timer,
            timer,
            plant,
            garden,
            settings,
            statistics,
            theme,
             timer_selected_session: 0,
             timer_selected_auto: 0,
             settings_selected: 0,
             stats_selected: 0,
             focus: Focus::Left,
             should_quit: false,
             timer_auto_list_state: ListState::default(),
             timer_auto_scrollbar_state: ScrollbarState::new(1000),
        }
    }

    pub fn tick(&mut self) {
        if self.timer.tick() {
            // Session finished
            self.plant.add_growth();
            self.statistics.total_sessions += 1;
            let minutes = self.timer.session_type.duration_minutes(&self.settings);
            self.statistics.total_minutes += minutes;
            let today = Local::now();
            // Update recent_sessions
            if let Some((_, count)) = self.statistics.recent_sessions.iter_mut().find(|(d, _)| d.date_naive() == today.date_naive()) {
                *count += 1;
            } else {
                self.statistics.recent_sessions.push((today, 1));
            }
            // Update recent_minutes
            if let Some((_, mins)) = self.statistics.recent_minutes.iter_mut().find(|(d, _)| d.date_naive() == today.date_naive()) {
                *mins += minutes;
            } else {
                self.statistics.recent_minutes.push((today, minutes));
            }
            match self.timer.session_type {
                crate::timer::SessionType::Focus => {
                    self.statistics.total_focus_sessions += 1;
                    self.statistics.total_focus_minutes += minutes;
                    // Update recent_focus_sessions
                    if let Some((_, count)) = self.statistics.recent_focus_sessions.iter_mut().find(|(d, _)| d.date_naive() == today.date_naive()) {
                        *count += 1;
                    } else {
                        self.statistics.recent_focus_sessions.push((today, 1));
                    }
                    // Update recent_focus_minutes
                    if let Some((_, mins)) = self.statistics.recent_focus_minutes.iter_mut().find(|(d, _)| d.date_naive() == today.date_naive()) {
                        *mins += minutes;
                    } else {
                        self.statistics.recent_focus_minutes.push((today, minutes));
                    }
                }
                _ => {
                    self.statistics.total_break_sessions += 1;
                    self.statistics.total_break_minutes += minutes;
                    // Update recent_break_sessions
                    if let Some((_, count)) = self.statistics.recent_break_sessions.iter_mut().find(|(d, _)| d.date_naive() == today.date_naive()) {
                        *count += 1;
                    } else {
                        self.statistics.recent_break_sessions.push((today, 1));
                    }
                    // Update recent_break_minutes
                    if let Some((_, mins)) = self.statistics.recent_break_minutes.iter_mut().find(|(d, _)| d.date_naive() == today.date_naive()) {
                        *mins += minutes;
                    } else {
                        self.statistics.recent_break_minutes.push((today, minutes));
                    }
                }
            }
            if self.plant.is_complete() {
                self.garden.add_completed_plant(self.plant.clone());
                self.plant = Plant::new();
                self.statistics.completed_plants += 1;
                // Update recent_plants
                if let Some((_, count)) = self.statistics.recent_plants.iter_mut().find(|(d, _)| d.date_naive() == today.date_naive()) {
                    *count += 1;
                } else {
                    self.statistics.recent_plants.push((today, 1));
                }
            }
            self.garden.update_streaks(&self.statistics.recent_sessions);
            self.statistics.session_log.push(crate::storage::SessionLog {
                session_type: self.timer.session_type,
                duration: minutes,
                end_time: Local::now(),
            });
            if self.statistics.session_log.len() > 100 {
                self.statistics.session_log.remove(0);
            }
            // Auto run next
            if let Some(idx) = self.timer.auto_run_index {
                if idx + 1 < self.timer.auto_run.len() {
                    self.timer.auto_run_index = Some(idx + 1);
                    self.timer.switch_session(self.timer.auto_run[idx + 1], &self.settings);
                    self.timer.start();
                } else {
                    self.timer.auto_run_index = None;
                }
            }
        }
    }

    pub fn handle_input(&mut self, action: InputAction) {
        match action {
            InputAction::Tab(n) => {
                match n {
                    1 => self.tab = Tab::Timer,
                    2 => self.tab = Tab::Plant,
                    3 => self.tab = Tab::Stats,
                    4 => self.tab = Tab::Settings,
                    _ => {}
                }
            }
            InputAction::Left => {
                self.focus = Focus::Left;
            }
            InputAction::Right => {
                self.focus = Focus::Right;
            }
            InputAction::Up => {
                self.handle_up();
            }
            InputAction::Down => {
                self.handle_down();
            }
            InputAction::Space => {
                if self.tab == Tab::Timer && self.focus == Focus::Left {
                    let sessions = [SessionType::Focus, SessionType::ShortBreak, SessionType::LongBreak];
                    let selected_session = sessions[self.timer_selected_session];
                    if self.timer.session_type == selected_session {
                        match self.timer.state {
                            crate::timer::TimerState::Idle => self.timer.start(),
                            crate::timer::TimerState::Running => self.timer.pause(),
                            crate::timer::TimerState::Paused => self.timer.resume(),
                            _ => {}
                        }
                    } else {
                        self.timer.switch_session(selected_session, &self.settings);
                        self.timer.start();
                    }
             } else {
                 // Logic for right focus - start selected auto-run session
                 if !self.timer.auto_run.is_empty() && self.timer_selected_auto < self.timer.auto_run.len() {
                     let selected_session = self.timer.auto_run[self.timer_selected_auto];
                     if self.timer.session_type == selected_session {
                         match self.timer.state {
                             crate::timer::TimerState::Idle => {
                                 self.timer.start();
                                 self.timer.auto_run_index = Some(self.timer_selected_auto);
                             }
                             crate::timer::TimerState::Running => self.timer.pause(),
                             crate::timer::TimerState::Paused => self.timer.resume(),
                             _ => {}
                         }
                     } else {
                         self.timer.switch_session(selected_session, &self.settings);
                         self.timer.start();
                         self.timer.auto_run_index = Some(self.timer_selected_auto);
                     }
                 }
             }
             self.timer_auto_list_state.select(Some(self.timer_selected_auto));
            }
            InputAction::Pause => {
                if self.tab == Tab::Timer {
                    match self.timer.state {
                        crate::timer::TimerState::Running => self.timer.pause(),
                        crate::timer::TimerState::Paused => self.timer.resume(),
                        _ => {}
                    }
                }
            }
            InputAction::Stop => {
                if self.tab == Tab::Timer {
                    self.timer.stop();
                }
            }
            InputAction::Quit => self.should_quit = true,
             InputAction::Enter => {
                 if self.tab == Tab::Timer && self.focus == Focus::Left {
                     let sessions = [SessionType::Focus, SessionType::ShortBreak, SessionType::LongBreak];
                     if self.timer_selected_session < sessions.len() {
                         self.timer.add_to_auto_run(sessions[self.timer_selected_session]);
                         self.timer_selected_auto = self.timer.auto_run.len() - 1;
                         self.timer_auto_list_state.select(Some(self.timer_selected_auto));
                     }
                 }
             }
             InputAction::Delete => {
                 if self.tab == Tab::Timer && self.focus == Focus::Right && !self.timer.auto_run.is_empty() && self.timer_selected_auto < self.timer.auto_run.len() {
                     self.timer.auto_run.remove(self.timer_selected_auto);
                     if let Some(idx) = self.timer.auto_run_index {
                         if idx == self.timer_selected_auto {
                             self.timer.auto_run_index = None;
                         } else if idx > self.timer_selected_auto {
                             self.timer.auto_run_index = Some(idx - 1);
                         }
                     }
                     if self.timer_selected_auto >= self.timer.auto_run.len() && self.timer_selected_auto > 0 {
                         self.timer_selected_auto -= 1;
                     }
                     self.timer_auto_list_state.select(Some(self.timer_selected_auto));
                 }
             }
        }
    }

    fn handle_up(&mut self) {
        match self.tab {
            Tab::Timer => {
                if self.focus == Focus::Left {
                    if self.timer_selected_session > 0 {
                        self.timer_selected_session -= 1;
                    }
                } else if self.timer_selected_auto > 0 {
                    self.timer_selected_auto -= 1;
                    self.timer_auto_list_state.select(Some(self.timer_selected_auto));
                }
            }
            Tab::Settings => {
                if self.focus == Focus::Left {
                    if self.settings_selected > 0 {
                        self.settings_selected -= 1;
                    }
                } else {
                    self.adjust_setting(1);
                }
            }
            Tab::Stats => {
                if self.stats_selected > 0 {
                    self.stats_selected -= 1;
                }
            }
            _ => {}
        }
    }

    fn handle_down(&mut self) {
        match self.tab {
            Tab::Timer => {
                if self.focus == Focus::Left {
                    let max = 2; // Focus, Short, Long
                    if self.timer_selected_session < max {
                        self.timer_selected_session += 1;
                    }
                } else if self.timer_selected_auto < self.timer.auto_run.len().saturating_sub(1) {
                    self.timer_selected_auto += 1;
                    self.timer_auto_list_state.select(Some(self.timer_selected_auto));
                }
            }
            Tab::Settings => {
                if self.focus == Focus::Left {
                    let max = 3; // Focus, Short, Long, Theme
                    if self.settings_selected < max {
                        self.settings_selected += 1;
                    }
                } else {
                    self.adjust_setting(-1);
                }
            }
            Tab::Stats => {
                let max = 8; // 9 categories
                if self.stats_selected < max {
                    self.stats_selected += 1;
                }
            }
            _ => {}
        }
    }

    fn adjust_setting(&mut self, delta: i64) {
        match self.settings_selected {
            0 => { // Focus
                self.settings.focus_duration = (self.settings.focus_duration as i64 + delta).clamp(1, 60) as u64;
                if self.timer.session_type == SessionType::Focus {
                    self.timer.set_session(SessionType::Focus, &self.settings);
                }
            }
            1 => { // Short break
                self.settings.short_break_duration = (self.settings.short_break_duration as i64 + delta).clamp(1, 60) as u64;
                if self.timer.session_type == SessionType::ShortBreak {
                    self.timer.set_session(SessionType::ShortBreak, &self.settings);
                }
            }
            2 => { // Long break
                self.settings.long_break_duration = (self.settings.long_break_duration as i64 + delta).clamp(1, 60) as u64;
                if self.timer.session_type == SessionType::LongBreak {
                    self.timer.set_session(SessionType::LongBreak, &self.settings);
                }
            }
            3 => { // Theme
                let themes = [ThemeVariant::System, ThemeVariant::RosePineDawn, ThemeVariant::RosePine, ThemeVariant::GruvboxDark, ThemeVariant::GruvboxLight, ThemeVariant::SolarizedDark, ThemeVariant::SolarizedLight, ThemeVariant::Nord, ThemeVariant::TokyoNight, ThemeVariant::Monokai, ThemeVariant::Vesper, ThemeVariant::Everforest, ThemeVariant::CatppuccinLatte, ThemeVariant::CatppuccinFrappe, ThemeVariant::CatppuccinMacchiato, ThemeVariant::CatppuccinMocha];
                let current = themes.iter().position(|&t| t == self.settings.theme).unwrap_or(0);
                let new_index = (current as i64 - delta).rem_euclid(themes.len() as i64) as usize;
                self.settings.theme = themes[new_index];
                self.theme = Theme::new(self.settings.theme);
            }
            _ => {}
        }
    }

    pub fn save(&self) {
        let mut statistics = self.statistics.clone();
        statistics.current_streak = self.garden.current_streak;
        statistics.longest_streak = self.garden.longest_streak;
        statistics.current_streak_start_date = self.garden.current_streak_start_date;
        statistics.longest_streak_end_date = self.garden.longest_streak_end_date;
        statistics.current_streak_dates = self.garden.current_streak_dates.clone();
        statistics.longest_streak_dates = self.garden.longest_streak_dates.clone();
        let data = Data {
            current_plant_stage: self.plant.stage.to_u32(),
            growth_points: self.plant.growth_points,
            settings: self.settings.clone(),
            statistics,
            completed_plants: self.garden.completed_plants.clone(),
            auto_run: self.timer.auto_run.clone(),
            auto_run_index: self.timer.auto_run_index,
        };
        save_data(&data);
    }
}