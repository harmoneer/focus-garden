use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{block::Title, Bar, BarChart, BarGroup, Block, Borders, Chart, Dataset, GraphType, List, ListItem, Padding, Paragraph, Widget},
    Frame,
};
use tui_piechart::{PieChart, PieSlice, symbols};
use std::env;

use crate::app::App;
use crate::storage::Data;
use chrono::{DateTime, Local};
use ratatui::text::Span;
use std::cmp::Reverse;
use tui_big_text::{BigText, PixelSize};

pub fn draw_stats(f: &mut Frame, app: &App, area: Rect) {
    let date_format = if env::var("LANG").unwrap_or_default().contains("US")
        || env::var("LC_TIME").unwrap_or_default().contains("US")
    {
        "%m/%d"
    } else {
        "%d/%m"
    };
    let max_minute_value = [
        &app.statistics.recent_minutes,
        &app.statistics.recent_focus_minutes,
        &app.statistics.recent_break_minutes,
    ].iter().flat_map(|v| v.iter().map(|(_, m)| *m)).max().unwrap_or(0);
    let max_y = ((max_minute_value as f64 / 10.0).ceil() * 10.0) as f64;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left: Stats categories and breakdown
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Min(5)])
        .split(chunks[0]);

    let today = Local::now().date_naive();
    let todays_sessions = app.statistics.recent_sessions.iter().find(|(d, _)| d.date_naive() == today).map(|(_, c)| *c).unwrap_or(0);
    let todays_minutes = app.statistics.recent_minutes.iter().find(|(d, _)| d.date_naive() == today).map(|(_, m)| *m).unwrap_or(0);
    let todays_focus_sessions = app.statistics.recent_focus_sessions.iter().find(|(d, _)| d.date_naive() == today).map(|(_, c)| *c).unwrap_or(0);
    let todays_focus_minutes = app.statistics.recent_focus_minutes.iter().find(|(d, _)| d.date_naive() == today).map(|(_, m)| *m).unwrap_or(0);
    let todays_break_sessions = app.statistics.recent_break_sessions.iter().find(|(d, _)| d.date_naive() == today).map(|(_, c)| *c).unwrap_or(0);
    let todays_break_minutes = app.statistics.recent_break_minutes.iter().find(|(d, _)| d.date_naive() == today).map(|(_, m)| *m).unwrap_or(0);
    let todays_plants = app.statistics.recent_plants.iter().find(|(d, _)| d.date_naive() == today).map(|(_, c)| *c).unwrap_or(0);
    let categories = vec![
        format!("Sessions: {}", todays_sessions),
        format!("Minutes: {}", todays_minutes),
        format!("Focus Sessions: {}", todays_focus_sessions),
        format!("Minutes Focused: {}", todays_focus_minutes),
        format!("Break Sessions: {}", todays_break_sessions),
        format!("Minutes Resting: {}", todays_break_minutes),
        format!("Grown Plants: {}", todays_plants),
        format!("Current Streak: {}", app.garden.current_streak),
        format!("Longest Streak: {}", app.garden.longest_streak),
    ];
    let items: Vec<ListItem> = categories
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let text = if i == app.stats_selected { format!("→ {}", s) } else { format!("  {}", s) };
            let style = if i == app.stats_selected {
                Style::default().fg(app.theme.highlight).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };
            ListItem::new(text).style(style)
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().title_top(Line::from(" Today's Stats ").style(Style::default().fg(app.theme.blocks)).centered()).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)));
    f.render_widget(list, left_chunks[0]);

    // Breakdown
    if app.stats_selected == 0 {
        let data = vec![
            PieSlice::new("Focus", app.statistics.total_focus_sessions as f64, app.theme.pine),
            PieSlice::new("Break", app.statistics.total_break_sessions as f64, app.theme.rose),
        ];
        let pie = PieChart::new(data).block(Block::default().title_top(Line::from(" Breakdown ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks))).pie_char(symbols::PIE_CHAR_BLOCK);
        f.render_widget(pie, left_chunks[1]);
    } else if app.stats_selected == 1 {
        let data = vec![
            PieSlice::new("Focus", app.statistics.total_focus_minutes as f64, app.theme.pine),
            PieSlice::new("Break", app.statistics.total_break_minutes as f64, app.theme.rose),
        ];
        let pie = PieChart::new(data).block(Block::default().title_top(Line::from(" Breakdown ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks))).pie_char(symbols::PIE_CHAR_BLOCK);
        f.render_widget(pie, left_chunks[1]);
    } else {
        let breakdown_content = match app.stats_selected {
            2 => {
                let logs = app.statistics.session_log.iter().rev().filter(|l| matches!(l.session_type, crate::timer::SessionType::Focus)).take(10).map(|l| format!("Focus session - {} mins - {}", l.duration, l.end_time.format("%Y-%m-%dT%H:%M:%S%.6f%z"))).collect::<Vec<_>>().join("\n");
                if logs.is_empty() { "No focus sessions".to_string() } else { logs }
            }
            3 => {
                let logs = app.statistics.session_log.iter().rev().filter(|l| matches!(l.session_type, crate::timer::SessionType::Focus)).take(10).map(|l| format!("Focus session - {} mins - {}", l.duration, l.end_time.format("%Y-%m-%dT%H:%M:%S%.6f%z"))).collect::<Vec<_>>().join("\n");
                if logs.is_empty() { "No focus sessions".to_string() } else { logs }
            }
            4 => {
                let logs = app.statistics.session_log.iter().rev().filter(|l| matches!(l.session_type, crate::timer::SessionType::ShortBreak | crate::timer::SessionType::LongBreak)).take(10).map(|l| {
                    let name = match l.session_type {
                        crate::timer::SessionType::ShortBreak => "Short break",
                        crate::timer::SessionType::LongBreak => "Long break",
                        _ => "Break",
                    };
                    format!("{} - {} mins - {}", name, l.duration, l.end_time.format("%Y-%m-%dT%H:%M:%S%.6f%z"))
                }).collect::<Vec<_>>().join("\n");
                if logs.is_empty() { "No break sessions".to_string() } else { logs }
            }
            5 => {
                let logs = app.statistics.session_log.iter().rev().filter(|l| matches!(l.session_type, crate::timer::SessionType::ShortBreak | crate::timer::SessionType::LongBreak)).take(10).map(|l| {
                    let name = match l.session_type {
                        crate::timer::SessionType::ShortBreak => "Short break",
                        crate::timer::SessionType::LongBreak => "Long break",
                        _ => "Break",
                    };
                    format!("{} - {} mins - {}", name, l.duration, l.end_time.format("%Y-%m-%dT%H:%M:%S%.6f%z"))
                }).collect::<Vec<_>>().join("\n");
                if logs.is_empty() { "No break sessions".to_string() } else { logs }
            }
            6 => {
                let logs = app.garden.completed_plants.iter().rev().take(10).map(|p| format!("Grown plant - {} - {}", p.plant.stage, p.completed_at.with_timezone(&Local).format("%Y-%m-%dT%H:%M:%S%.6f%z"))).collect::<Vec<_>>().join("\n");
                if logs.is_empty() { "No grown plants".to_string() } else { logs }
            }
            7 => {
                let dates = app.garden.current_streak_dates.iter().map(|d| d.format("%Y-%m-%d").to_string()).collect::<Vec<_>>().join("\n");
                format!("Streak Dates:\n{}", dates)
            },
            8 => {
                let dates = app.garden.longest_streak_dates.iter().map(|d| d.format("%Y-%m-%d").to_string()).collect::<Vec<_>>().join("\n");
                format!("Streak Dates:\n{}", dates)
            },
            _ => "Breakdown not available".to_string(),
        };
        let breakdown = Paragraph::new(breakdown_content)
            .block(Block::default().title_top(Line::from(" Breakdown ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
            .style(Style::default().fg(app.theme.text));
        f.render_widget(breakdown, left_chunks[1]);
    }

    // Right: Chart
    match app.stats_selected {
        0 => {
            // BarChart for Sessions
            let mut data: Vec<(DateTime<Local>, u32)> = app.statistics.recent_sessions.clone();
            let today = Local::now();
            if !data.iter().any(|(d, _)| d.date_naive() == today.date_naive()) {
                data.push((today, 0));
            }
            data.sort_by_key(|(d, _)| d.date_naive());
            let chart_data: Vec<(f64, f64)> = data.iter().enumerate().map(|(i, (_, v))| (i as f64, *v as f64)).collect();
            let x_labels = if data.len() >= 3 {
                vec![
                    Span::styled(data.last().unwrap().0.format(date_format).to_string(), Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(data[data.len() / 2].0.format(date_format).to_string()),
                    Span::styled(data[0].0.format(date_format).to_string(), Style::default().add_modifier(Modifier::BOLD)),
                ]
            } else if data.len() == 2 {
                vec![
                    Span::styled(data[1].0.format(date_format).to_string(), Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(data[0].0.format(date_format).to_string(), Style::default().add_modifier(Modifier::BOLD)),
                ]
            } else {
                vec![Span::styled(data[0].0.format(date_format).to_string(), Style::default().add_modifier(Modifier::BOLD))]
            };
            let max_val = data.iter().map(|(_, v)| *v as u64).max().unwrap_or(0);
            let max_y = ((max_val as f64 / 10.0).ceil() * 10.0) as u64;
            let bars: Vec<Bar> = data.iter().map(|(date, value)| {
                Bar::default()
                    .value(*value as u64)
                    .label(date.format(date_format).to_string().into())
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.pine).bg(app.theme.foam))
                    .style(Style::default().fg(app.theme.foam))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(format!(" Total Sessions: {} ", app.statistics.total_sessions)).style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 0, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(5)
                .bar_gap(1)
                .max(max_y);
            f.render_widget(barchart, chunks[1]);
        }
        1 => {
            // Horizontal BarChart for Minutes
            let mut data: Vec<(DateTime<Local>, u64)> = app.statistics.recent_minutes.clone();
            data.sort_by_key(|(d, _)| *d);
            let bars: Vec<Bar> = data.iter().map(|(date, value)| {
                Bar::default()
                    .value(*value)
                    .label(Line::from(date.format(date_format).to_string()))
                    .style(Style::default().fg(app.theme.rose))
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.love).bg(app.theme.rose))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(format!(" Total Minutes: {} ", app.statistics.total_minutes)).style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 1, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(1)
                .bar_gap(0)
                .direction(Direction::Horizontal)
                .max(max_y as u64);
            f.render_widget(barchart, chunks[1]);
        }
        2 => {
            // BarChart for Focus Sessions
            let mut data: Vec<(DateTime<Local>, u64)> = app.statistics.recent_focus_sessions.iter().map(|(d, v)| (*d, *v as u64)).collect();
            data.sort_by_key(|(d, _)| d.date_naive());
            let max_val = data.iter().map(|(_, v)| *v).max().unwrap_or(0);
            let max_y = ((max_val as f64 / 10.0).ceil() * 10.0) as u64;
            let bars: Vec<Bar> = data.iter().map(|(date, value)| {
                Bar::default()
                    .value(*value)
                    .label(date.format(date_format).to_string().into())
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.pine).bg(app.theme.foam))
                    .style(Style::default().fg(app.theme.foam))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(format!(" Total Focus Sessions: {} ", app.statistics.total_focus_sessions)).style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 0, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(5)
                .bar_gap(1)
                .max(max_y);
            f.render_widget(barchart, chunks[1]);
        }
        3 => {
            // Horizontal BarChart for Minutes Focused
            let mut data: Vec<(DateTime<Local>, u64)> = app.statistics.recent_focus_minutes.clone();
            data.sort_by_key(|(d, _)| *d);
            let bars: Vec<Bar> = data.iter().map(|(date, value)| {
                Bar::default()
                    .value(*value)
                    .label(Line::from(date.format(date_format).to_string()))
                    .style(Style::default().fg(app.theme.rose))
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.love).bg(app.theme.rose))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(format!(" Total Minutes Focused: {} ", app.statistics.total_focus_minutes)).style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 1, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(1)
                .bar_gap(0)
                .direction(Direction::Horizontal)
                .max(max_y as u64);
            f.render_widget(barchart, chunks[1]);
        }
        4 => {
            // BarChart for Break Sessions
            let mut data: Vec<(DateTime<Local>, u64)> = app.statistics.recent_break_sessions.iter().map(|(d, v)| (*d, *v as u64)).collect();
            data.sort_by_key(|(d, _)| d.date_naive());
            let max_val = data.iter().map(|(_, v)| *v).max().unwrap_or(0);
            let max_y = ((max_val as f64 / 10.0).ceil() * 10.0) as u64;
            let bars: Vec<Bar> = data.iter().map(|(date, value)| {
                Bar::default()
                    .value(*value)
                    .label(date.format(date_format).to_string().into())
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.pine).bg(app.theme.foam))
                    .style(Style::default().fg(app.theme.foam))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(format!(" Total Break Sessions: {} ", app.statistics.total_break_sessions)).style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 0, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(5)
                .bar_gap(1)
                .max(max_y);
            f.render_widget(barchart, chunks[1]);
        }
        5 => {
            // Horizontal BarChart for Minutes Resting
            let mut data: Vec<(DateTime<Local>, u64)> = app.statistics.recent_break_minutes.clone();
            data.sort_by_key(|(d, _)| *d);
            let bars: Vec<Bar> = data.iter().map(|(date, value)| {
                Bar::default()
                    .value(*value)
                    .label(Line::from(date.format(date_format).to_string()))
                    .style(Style::default().fg(app.theme.rose))
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.love).bg(app.theme.rose))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(format!(" Total Minutes Resting: {} ", app.statistics.total_break_minutes)).style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 1, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(1)
                .bar_gap(0)
                .direction(Direction::Horizontal)
                .max(max_y as u64);
            f.render_widget(barchart, chunks[1]);
        }
        6 => {
            // BarChart for Grown Plants
            let mut data: Vec<(DateTime<Local>, u64)> = app.statistics.recent_plants.iter().map(|(d, v)| (*d, *v as u64)).collect();
            data.sort_by_key(|(d, _)| d.date_naive());
            let max_val = data.iter().map(|(_, v)| *v).max().unwrap_or(0);
            let max_y = ((max_val as f64 / 10.0).ceil() * 10.0) as u64;
            let bars: Vec<Bar> = data.iter().map(|(date, value)| {
                Bar::default()
                    .value(*value)
                    .label(date.format(date_format).to_string().into())
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.vertical_value).bg(app.theme.foam))
                    .style(Style::default().fg(app.theme.foam))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(format!(" Total Grown Plants: {} ", app.statistics.completed_plants)).style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 0, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(5)
                .bar_gap(1)
                .max(max_y);
            f.render_widget(barchart, chunks[1]);
        }
        7 => {
            // Text display for Current Streak
            let date = if app.garden.current_streak == 0 {
                Local::now().format("%d %b %Y").to_string()
            } else {
                app.garden.current_streak_start_date.map(|d| d.with_timezone(&Local).format("%d %b %Y").to_string()).unwrap_or("N/A".to_string())
            };
            let display_streak = app.garden.current_streak;
            let big_streak = BigText::builder()
                .lines(vec![Line::from(format!("{:^3}", display_streak.to_string()))])
                .pixel_size(PixelSize::Quadrant)
                .alignment(Alignment::Center)
                .build();
            let block = Block::default()
                .title(Line::from(" Current Streak ").style(Style::default().fg(app.theme.blocks)))
                .borders(Borders::ALL)
                .style(Style::default().fg(app.theme.blocks))
                .padding(Padding::new(1, 0, 1, 0));
            let inner = block.inner(chunks[1]);
            f.render_widget(block, chunks[1]);
            let areas = Layout::vertical([
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]).split(inner);
            let date_block = Block::default()
                .title(Line::from(" Date Started ").alignment(Alignment::Center))
                .borders(Borders::ALL)
                .padding(Padding::new(1, 0, 0, 0));
            let block_areas = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(50),
                Constraint::Fill(1),
            ]).split(areas[0]);
            let date_inner = date_block.inner(block_areas[1]);
            f.render_widget(date_block, block_areas[1]);
            let big_date = BigText::builder()
                .lines(vec![Line::from(date.as_str())])
                .pixel_size(PixelSize::Quadrant)
                .alignment(Alignment::Center)
                .build();
            let date_areas = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
            ]).split(date_inner);
            f.render_widget(big_date, date_areas[1]);
            let counting_block = Block::default()
                .title(Line::from(" Counting ").alignment(Alignment::Center))
                .title_bottom(Line::from(" days ").alignment(Alignment::Center))
                .borders(Borders::ALL)
                .padding(Padding::new(0, 0, 0, 0));
            let counting_areas = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(50),
                Constraint::Fill(1),
            ]).split(areas[1]);
            let counting_inner = counting_block.inner(counting_areas[1]);
            f.render_widget(counting_block, counting_areas[1]);
            let counting_vertical = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
            ]).split(counting_inner);
            f.render_widget(big_streak, counting_vertical[1]);
            f.render_widget(
                Paragraph::new("")
                    .alignment(Alignment::Center),
                areas[2],
            );
            f.render_widget(
                Paragraph::new("")
                    .alignment(Alignment::Center),
                areas[3],
            );
        }
        8 => {
            // Text display for Longest Streak
            let date = app.garden.longest_streak_end_date.map(|d| d.with_timezone(&Local).format("%d %b %Y").to_string()).unwrap_or("N/A".to_string());
            let streak = app.garden.longest_streak;
            let big_streak = BigText::builder()
                .lines(vec![Line::from(format!("{:^3}", streak.to_string()))])
                .pixel_size(PixelSize::Quadrant)
                .alignment(Alignment::Center)
                .build();
            let block = Block::default().title(Line::from(" Longest Streak ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 1, 0));
            let inner = block.inner(chunks[1]);
            f.render_widget(block, chunks[1]);
            let areas = Layout::vertical([
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]).split(inner);
            let date_block = Block::default()
                .title(Line::from(" Date Ended ").alignment(Alignment::Center))
                .borders(Borders::ALL)
                .padding(Padding::new(1, 0, 0, 0));
            let block_areas = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(50),
                Constraint::Fill(1),
            ]).split(areas[0]);
            let date_inner = date_block.inner(block_areas[1]);
            f.render_widget(date_block, block_areas[1]);
            let big_date = BigText::builder()
                .lines(vec![Line::from(date.as_str())])
                .pixel_size(PixelSize::Quadrant)
                .alignment(Alignment::Center)
                .build();
            let date_areas = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
            ]).split(date_inner);
            f.render_widget(big_date, date_areas[1]);
            let counting_block = Block::default()
                .title(Line::from(" Counting ").alignment(Alignment::Center))
                .title_bottom(Line::from(" days ").alignment(Alignment::Center))
                .borders(Borders::ALL)
                .padding(Padding::new(0, 0, 0, 0));
            let counting_areas = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(50),
                Constraint::Fill(1),
            ]).split(areas[1]);
            let counting_inner = counting_block.inner(counting_areas[1]);
            f.render_widget(counting_block, counting_areas[1]);
            let counting_vertical = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
            ]).split(counting_inner);
            f.render_widget(big_streak, counting_vertical[1]);
            f.render_widget(
                Paragraph::new("")
                    .alignment(Alignment::Center),
                areas[2],
            );
            f.render_widget(
                Paragraph::new("")
                    .alignment(Alignment::Center),
                areas[3],
            );
        }

        _ => {}
    }
}