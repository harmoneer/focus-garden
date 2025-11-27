use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{block::Title, Block, Borders, Paragraph, Sparkline},
    Frame,
};

use crate::app::App;

pub fn draw_stats(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(Title::from(Line::from(" Statistics ").style(Style::default().fg(app.theme.blocks))).alignment(Alignment::Center))
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.blocks));
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .margin(1)
        .split(area);

    // Text stats
    let stats_text = format!(
        "Total focus sessions: {}\nTotal minutes focused: {}\nFully grown plants: {}\nCurrent streak: {}\nLongest streak: {}",
        app.statistics.total_sessions,
        app.statistics.total_minutes,
        app.statistics.completed_plants,
        app.garden.current_streak,
        app.garden.longest_streak
    );
    let para = Paragraph::new(stats_text)
        .style(Style::default().fg(app.theme.text));
    f.render_widget(para, inner[0]);

    // Sparkline
    let data: Vec<u64> = app.statistics.recent_sessions.iter().map(|&x| x as u64).collect();
    let sparkline = Sparkline::default()
        .block(Block::default().title(Line::from("Recent Activity").style(Style::default().fg(app.theme.blocks))))
        .style(Style::default().fg(app.theme.sparkline))
        .data(&data);
    f.render_widget(sparkline, inner[1]);
}