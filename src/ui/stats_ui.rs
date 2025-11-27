use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{block::Title, Bar, BarChart, BarGroup, Block, Borders, Chart, Dataset, GraphType, List, ListItem, Padding, Paragraph},
    Frame,
};
use std::env;

use crate::app::App;
use chrono::{DateTime, Local};
use ratatui::text::Span;
use std::cmp::Reverse;

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

    let categories = vec![
        format!("Sessions: {}", app.statistics.total_sessions),
        format!("Minutes: {}", app.statistics.total_minutes),
        format!("Focus Sessions: {}", app.statistics.total_focus_sessions),
        format!("Minutes Focused: {}", app.statistics.total_focus_minutes),
        format!("Break Sessions: {}", app.statistics.total_break_sessions),
        format!("Minutes Resting: {}", app.statistics.total_break_minutes),
        format!("Grown Plants: {}", app.statistics.completed_plants),
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
        .block(Block::default().title(Title::from(Line::from(" Today's Stats ").style(Style::default().fg(app.theme.blocks))).alignment(Alignment::Center)).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)));
    f.render_widget(list, left_chunks[0]);

    // Breakdown
    let breakdown_content = match app.stats_selected {
        0 => format!("Focus: {}, Break: {}", app.statistics.total_focus_sessions, app.statistics.total_break_sessions),
        1 => format!("Focused: {}, Break: {}", app.statistics.total_focus_minutes, app.statistics.total_break_minutes),
        2 => {
            if app.statistics.recent_sessions.is_empty() {
                "No session data".to_string()
            } else {
                format!("Session distribution: {:?}", app.statistics.recent_sessions)
            }
        }
        3 => "Minutes focused distribution: N/A".to_string(),
        4 => "Break sessions distribution: N/A".to_string(),
        5 => "Minutes resting distribution: N/A".to_string(),
        6 => "Grown plants distribution: N/A".to_string(),
        7 => format!("Sessions distribution: {:?}", app.statistics.recent_sessions),
        8 => "Minutes distribution: N/A".to_string(),
        _ => "Breakdown not available".to_string(),
    };
    let breakdown = Paragraph::new(breakdown_content)
        .block(Block::default().title(Title::from(Line::from(" Breakdown ").style(Style::default().fg(app.theme.blocks))).alignment(Alignment::Center)).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
        .style(Style::default().fg(app.theme.text));
    f.render_widget(breakdown, left_chunks[1]);

    // Right: Chart
    match app.stats_selected {
        0 => {
            // BarChart for Sessions
            let mut data: Vec<(DateTime<Local>, u32)> = app.statistics.recent_sessions.clone();
            let today = Local::now();
            if !data.iter().any(|(d, _)| d.date_naive() == today.date_naive()) {
                data.push((today, 0));
            }
            data.sort_by_key(|(d, _)| Reverse(d.date_naive()));
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
            let max_y = data.iter().map(|(_, v)| *v as u64).max().unwrap_or(0).max(10);
            let bars: Vec<Bar> = data.iter().map(|(date, value)| {
                Bar::default()
                    .value(*value as u64)
                    .label(date.format(date_format).to_string().into())
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.pine).bg(app.theme.foam))
                    .style(Style::default().fg(app.theme.foam))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(" Total Sessions ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 0, 0)))
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
                .block(Block::default().title(Line::from(" Total Minutes ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 1, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(1)
                .bar_gap(0)
                .direction(Direction::Horizontal)
                .max(max_y as u64);
            f.render_widget(barchart, chunks[1]);
        }
        2 => {
            // BarChart for Focus Sessions
            let mut data: Vec<(String, u64)> = app.statistics.recent_focus_sessions.iter().map(|(d, v)| (d.format(date_format).to_string(), *v as u64)).collect();
            data.sort_by_key(|(date, _)| date.clone());
            let max_y = data.iter().map(|(_, v)| *v).max().unwrap_or(0).max(10);
            let bars: Vec<Bar> = data.iter().map(|(label, value)| {
                Bar::default()
                    .value(*value)
                    .label(label.clone().into())
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.pine).bg(app.theme.foam))
                    .style(Style::default().fg(app.theme.foam))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(" Total Focus Sessions ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 0, 0)))
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
                .block(Block::default().title(Line::from(" Total Minutes Focused ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 1, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(1)
                .bar_gap(0)
                .direction(Direction::Horizontal)
                .max(max_y as u64);
            f.render_widget(barchart, chunks[1]);
        }
        4 => {
            // BarChart for Break Sessions
            let mut data: Vec<(String, u64)> = app.statistics.recent_break_sessions.iter().map(|(d, v)| (d.format(date_format).to_string(), *v as u64)).collect();
            data.sort_by_key(|(date, _)| date.clone());
            let max_y = data.iter().map(|(_, v)| *v).max().unwrap_or(0).max(10);
            let bars: Vec<Bar> = data.iter().map(|(label, value)| {
                Bar::default()
                    .value(*value)
                    .label(label.clone().into())
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.pine).bg(app.theme.foam))
                    .style(Style::default().fg(app.theme.foam))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(" Total Break Sessions ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 0, 0)))
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
                .block(Block::default().title(Line::from(" Total Minutes Resting ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 1, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(1)
                .bar_gap(0)
                .direction(Direction::Horizontal)
                .max(max_y as u64);
            f.render_widget(barchart, chunks[1]);
        }
        6 => {
            // BarChart for Grown Plants
            let mut data: Vec<(String, u64)> = app.statistics.recent_plants.iter().map(|(d, v)| (d.format(date_format).to_string(), *v as u64)).collect();
            data.sort_by_key(|(date, _)| date.clone());
            let max_y = data.iter().map(|(_, v)| *v).max().unwrap_or(0).max(10);
            let bars: Vec<Bar> = data.iter().map(|(label, value)| {
                Bar::default()
                    .value(*value)
                    .label(label.clone().into())
                    .text_value(format!("{:^5}", value))
                    .value_style(Style::default().fg(app.theme.vertical_value).bg(app.theme.highlight))
                    .style(Style::default().fg(app.theme.foam))
            }).collect();
            let barchart = BarChart::default()
                .block(Block::default().title(Line::from(" Total Grown Plants ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)).padding(Padding::new(1, 0, 0, 0)))
                .data(BarGroup::default().bars(&bars))
                .bar_width(5)
                .bar_gap(1)
                .max(max_y);
            f.render_widget(barchart, chunks[1]);
        }
        7 => {
            // Chart for Current Streak
            let data = vec![(0.0, 0.0), (1.0, app.garden.current_streak as f64)];
            let dataset = Dataset::default()
                .data(&data)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(app.theme.highlight)); // accent
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title(Line::from(" Current Streak ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .x_axis(ratatui::widgets::Axis::default().bounds([0.0, 1.0]))
                .y_axis(ratatui::widgets::Axis::default().bounds([0.0, (app.garden.current_streak as f64).max(5.0)]));
            f.render_widget(chart, chunks[1]);
        }
        8 => {
            // Paragraph for Longest Streak
            let text = format!("Longest Streak: {}", app.garden.longest_streak);
            let para = Paragraph::new(text)
                .block(Block::default().title(Line::from(" Longest Streak ").style(Style::default().fg(app.theme.blocks))).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)))
                .style(Style::default().fg(app.theme.text));
            f.render_widget(para, chunks[1]);
        }

        _ => {}
    }
}