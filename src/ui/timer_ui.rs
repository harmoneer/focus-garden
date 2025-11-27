use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{block::Title, Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::{app::App, timer::SessionType};

pub fn draw_timer(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(10)])
        .split(area);

    // Timer display
    let timer_block = Block::default()
        .title(Title::from(Line::from(" Focus Timer ").style(Style::default().fg(app.theme.blocks))).alignment(Alignment::Center))
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.blocks));
    let inner_area = timer_block.inner(chunks[0]);
    f.render_widget(timer_block, chunks[0]);

    let padded = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(11), Constraint::Length(0)])
        .split(inner_area);
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
        .split(padded[1]);

    // Big timer text
    let minutes = app.timer.remaining_seconds / 60;
    let seconds = app.timer.remaining_seconds % 60;
    let timer_text = format!("{:02}:{:02}", minutes, seconds);
    let big_text = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(app.theme.timer_text))
        .lines(vec![timer_text.into()])
        .alignment(Alignment::Center)
        .build();
    f.render_widget(big_text, inner[0]);

    // Status (session name)
    let session_name = match app.timer.session_type {
        crate::timer::SessionType::Focus => "Focus",
        crate::timer::SessionType::ShortBreak => "Short Break",
        crate::timer::SessionType::LongBreak => "Long Break",
    };
    let status = match app.timer.state {
        crate::timer::TimerState::Idle => "Idle".to_string(),
        crate::timer::TimerState::Running => session_name.to_string(),
        crate::timer::TimerState::Paused => format!("Paused: {}", session_name),
        crate::timer::TimerState::Finished => "Finished".to_string(),
    };
    let status_para = Paragraph::new(status)
        .style(Style::default().fg(app.theme.text))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(status_para, inner[1]);

    // Gauge
    let gauge_color = match app.timer.state {
        crate::timer::TimerState::Running => app.theme.gauge_running,
        crate::timer::TimerState::Paused => app.theme.gauge_paused,
        crate::timer::TimerState::Finished => app.theme.gauge_finished,
        _ => app.theme.text,
    };
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(Style::default().fg(gauge_color))
        .percent((app.timer.progress() * 100.0) as u16)
        .label(format!("{}%", (app.timer.progress() * 100.0) as u16));
    f.render_widget(gauge, inner[2]);

    // Bottom dual blocks
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Left: Session selection
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(1)])
        .split(bottom_chunks[0]);
    let sessions = vec!["Focus", "Short Break", "Long Break"];
    let session_items: Vec<ListItem> = sessions
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_selected = i == app.timer_selected_session && app.focus == crate::app::Focus::Left;
            let text = if is_selected { format!("→ {}", s) } else { format!("  {}", s) };
            let style = if is_selected {
                Style::default().fg(app.theme.highlight).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };
            ListItem::new(text).style(style)
        })
        .collect();
    let session_list = List::new(session_items)
        .block(Block::default().title(Title::from(Line::from(" Sessions ").style(Style::default().fg(app.theme.blocks))).alignment(Alignment::Center)).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)));
    f.render_widget(session_list, left_chunks[0]);
    let add_legend = Paragraph::new("Add to auto [Enter]")
        .style(Style::default().fg(app.theme.secondary_text))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(add_legend, left_chunks[1]);

    // Right: Auto-run set
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(1)])
        .split(bottom_chunks[1]);
    let auto_items: Vec<ListItem> = app.timer.auto_run
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let name = match s {
                SessionType::Focus => "Focus",
                SessionType::ShortBreak => "Short Break",
                SessionType::LongBreak => "Long Break",
            };
            let is_selected = i == app.timer_selected_auto && app.focus == crate::app::Focus::Right;
            let text = if is_selected { format!("→ {}", name) } else { format!("  {}", name) };
            let style = if is_selected {
                Style::default().fg(app.theme.highlight).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };
            ListItem::new(text).style(style)
        })
        .collect();
    let auto_list = List::new(auto_items)
        .block(Block::default().title(Title::from(Line::from(" Auto-Run Set ").style(Style::default().fg(app.theme.blocks))).alignment(Alignment::Center)).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)));
    f.render_widget(auto_list, right_chunks[0]);
    let remove_legend = Paragraph::new("Remove [Del]")
        .style(Style::default().fg(app.theme.secondary_text))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(remove_legend, right_chunks[1]);
}