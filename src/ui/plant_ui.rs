use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{block::Title, Block, Borders, Gauge, Paragraph},
    Frame,
};
use ratatui::widgets::canvas::{Canvas, Points};

use crate::app::App;

pub fn draw_plant(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(Title::from(Line::from(format!(" Plant {} {} ", app.plant.stage.icon(), format!("{:?}", app.plant.stage).to_lowercase())).style(Style::default().fg(app.theme.blocks))).alignment(Alignment::Center))
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.blocks));
    f.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Length(3), Constraint::Min(3)])
        .margin(1)
        .split(area);

    // Canvas
    let canvas = Canvas::default()
        .block(Block::default())
        .x_bounds([-5.0, 5.0])
        .y_bounds([-1.0, 10.0])
        .paint(|ctx| {
            for &(x, y) in &app.plant.growth_points {
                ctx.draw(&Points {
                    coords: &[(x as f64, y as f64)],
                    color: app.theme.text,
                });
            }
        });
    f.render_widget(canvas, inner[0]);

    // Gauge
    let gauge = Gauge::default()
        .block(Block::default().title(Line::from("Stage Progress").style(Style::default().fg(app.theme.blocks))))
        .gauge_style(Style::default().fg(app.theme.gauge_running))
        .percent((app.plant.progress() * 100.0) as u16);
    f.render_widget(gauge, inner[1]);

    // Info
    let info = format!(
        "Sessions to next stage: {}\nTotal fully grown plants: {}",
        app.plant.sessions_to_next_stage(),
        app.garden.total_completed()
    );
    let para = Paragraph::new(info)
        .style(Style::default().fg(app.theme.text));
    f.render_widget(para, inner[2]);
}