use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub focus_duration: u64, // in minutes
    pub short_break_duration: u64,
    pub long_break_duration: u64,
    pub theme: crate::theme::ThemeVariant,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            focus_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
            theme: crate::theme::ThemeVariant::System,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Statistics {
    pub total_sessions: u32,
    pub total_focus_sessions: u32,
    pub total_break_sessions: u32,
    pub total_minutes: u64,
    pub total_focus_minutes: u64,
    pub total_break_minutes: u64,
    pub completed_plants: u32,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub recent_sessions: Vec<(DateTime<Local>, u32)>, // (date, count) for daily totals
    pub recent_focus_sessions: Vec<(DateTime<Local>, u32)>,
    pub recent_break_sessions: Vec<(DateTime<Local>, u32)>,
    pub recent_plants: Vec<(DateTime<Local>, u32)>,
    pub recent_minutes: Vec<(DateTime<Local>, u64)>,
    pub recent_focus_minutes: Vec<(DateTime<Local>, u64)>,
    pub recent_break_minutes: Vec<(DateTime<Local>, u64)>,
}

impl Default for Statistics {
    fn default() -> Self {
        Statistics {
            total_sessions: 0,
            total_focus_sessions: 0,
            total_break_sessions: 0,
            total_minutes: 0,
            total_focus_minutes: 0,
            total_break_minutes: 0,
            completed_plants: 0,
            current_streak: 0,
            longest_streak: 0,
            recent_sessions: vec![],
            recent_focus_sessions: vec![],
            recent_break_sessions: vec![],
            recent_plants: vec![],
            recent_minutes: vec![],
            recent_focus_minutes: vec![],
            recent_break_minutes: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub current_plant_stage: u32,
    pub growth_points: Vec<(u16, u16)>,
    pub settings: Settings,
    pub statistics: Statistics,
    pub auto_run: Vec<crate::timer::SessionType>,
    pub auto_run_index: Option<usize>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            current_plant_stage: 0,
            growth_points: vec![],
            settings: Settings::default(),
            statistics: Statistics::default(),
            auto_run: vec![],
            auto_run_index: None,
        }
    }
}

pub fn get_data_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("focus-garden");
    fs::create_dir_all(&path).unwrap();
    path.push("data.json");
    path
}

pub fn load_data() -> Data {
    let path = get_data_path();
    if path.exists() {
        let contents = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        Data::default()
    }
}

pub fn save_data(data: &Data) {
    let path = get_data_path();
    let json = serde_json::to_string_pretty(data).unwrap();
    fs::write(&path, json).unwrap();
}