use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{block::Title, Block, Borders, Gauge, Padding, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_plant(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);

    // Left: Growing Plant
    let left_block = Block::default()
        .title_top(Line::from(format!(" Growing {} ", app.plant.stage)).style(Style::default().fg(app.theme.blocks)).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.blocks));
    f.render_widget(left_block, chunks[0]);

    let left_inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(5), Constraint::Length(1), Constraint::Min(3)])
        .split(chunks[0]);

    // Emoji subblock
    let emoji_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Fill(1),
        ])
        .split(left_inner[1]);
    let emoji_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.blocks));
    let emoji_inner = emoji_block.inner(emoji_layout[1]);
    f.render_widget(emoji_block, emoji_layout[1]);
    // Vertical centering inside the 3-line block
    let emoji_vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(emoji_inner);
    let emoji_para = Paragraph::new(app.plant.stage.icon())
        .style(Style::default().fg(app.theme.text))
        .alignment(Alignment::Center);
    f.render_widget(emoji_para, emoji_vert[1]);

    // Progress bar
    let progress = match app.plant.growth_points {
        0..=1 => (app.plant.growth_points as f64 / 2.0 * 100.0) as u16,
        2..=4 => ((app.plant.growth_points - 2) as f64 / 3.0 * 100.0) as u16,
        5..=9 => 0,
        _ => 100,
    };
    let gauge = Gauge::default()
        .block(Block::default().padding(Padding::horizontal(1)))
        .gauge_style(Style::default().fg(app.theme.gauge_running))
        .percent(progress);
    f.render_widget(gauge, left_inner[2]);

    // Sessions to next
    let next_stage = match app.plant.growth_points {
        0..=1 => "Sprout",
        2..=4 => "Seedling",
        5..=9 => "Young Plant",
        _ => "Complete",
    };
    let info = format!("{} sessions to {}", app.plant.sessions_to_next_stage(), next_stage);
    let para = Paragraph::new(info)
        .style(Style::default().fg(app.theme.text))
        .alignment(Alignment::Center);
    f.render_widget(para, left_inner[3]);

    // Right: Garden
    let right_block = Block::default()
        .title_top(Line::from(" Garden ").style(Style::default().fg(app.theme.blocks)).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.blocks));
    f.render_widget(right_block, chunks[1]);

    let right_inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .margin(1)
        .split(chunks[1]);

    // Plants
    let plants_text = "🪴".repeat(app.garden.total_completed() as usize);
    let plants_para = Paragraph::new(plants_text)
        .style(Style::default().fg(app.theme.text))
        .block(Block::default().padding(Padding::horizontal(1)));
    f.render_widget(plants_para, right_inner[0]);

    // Total
    let total = format!("Total fully grown plants: {}", app.garden.total_completed());
    let total_para = Paragraph::new(total)
        .style(Style::default().fg(app.theme.secondary_text))
        .block(Block::default().padding(Padding::horizontal(1)));
    f.render_widget(total_para, right_inner[1]);
}