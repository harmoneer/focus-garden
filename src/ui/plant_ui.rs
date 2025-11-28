use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::app::App;

pub fn draw_plant(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left: Growing Plant
    let left_block = Block::default()
        .title_top(Line::from(format!(" Growing {} ", app.plant.stage)).style(Style::default().fg(app.theme.blocks)).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.blocks));
    f.render_widget(left_block, chunks[0]);

    let left_inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Length(2), Constraint::Min(3)])
        .margin(1)
        .split(chunks[0]);

    // Big emoji
    let big_text = BigText::builder()
        .lines(vec![app.plant.stage.icon().into()])
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(app.theme.text))
        .build();
    f.render_widget(big_text, left_inner[0]);

    // Progress bar
    let progress = match app.plant.growth_points {
        0..=2 => (app.plant.growth_points as f64 / 3.0 * 100.0) as u16,
        3..=5 => ((app.plant.growth_points - 3) as f64 / 3.0 * 100.0) as u16,
        6..=8 => ((app.plant.growth_points - 6) as f64 / 3.0 * 100.0) as u16,
        9 => 100,
        _ => 100,
    };
    let gauge = Gauge::default()
        .block(Block::default().title(Line::from("Stage Progress").style(Style::default().fg(app.theme.blocks))))
        .gauge_style(Style::default().fg(app.theme.gauge_running))
        .percent(progress);
    f.render_widget(gauge, left_inner[1]);

    // Sessions to next
    let next_stage = match app.plant.growth_points {
        0..=2 => "Sprout",
        3..=5 => "Seedling",
        6..=8 => "Young Plant",
        9 => "Full Grown Plant",
        _ => "Complete",
    };
    let info = format!("{} sessions to {}", app.plant.sessions_to_next_stage(), next_stage);
    let para = Paragraph::new(info)
        .style(Style::default().fg(app.theme.text))
        .alignment(Alignment::Center);
    f.render_widget(para, left_inner[2]);

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
        .style(Style::default().fg(app.theme.text));
    f.render_widget(plants_para, right_inner[0]);

    // Total
    let total = format!("Total fully grown plants: {}", app.garden.total_completed());
    let total_para = Paragraph::new(total)
        .style(Style::default().fg(app.theme.secondary_text));
    f.render_widget(total_para, right_inner[1]);
}