mod app;
mod garden;
mod input;
mod plant;
mod storage;
mod theme;
mod timer;
mod ui;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::text::Line;

use crate::app::{App, Tab};
use crate::input::handle_key;
use crate::ui::{plant_ui, stats_ui, settings_ui, timer_ui};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
    let mut last_tick = Instant::now();

    loop {
        // Draw
        terminal.draw(|f| {
            let size = f.area();
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Length(3), // Header
                    ratatui::layout::Constraint::Min(21),   // Main
                    ratatui::layout::Constraint::Min(1),    // Footer
                ])
                .split(size);

            // Header: Tabs
            let tabs = ratatui::widgets::Tabs::new(vec![
                Line::from("⏳ Timer [1]").style(ratatui::style::Style::default().fg(app.theme.blocks)),
                Line::from("🌱 Plant [2]").style(ratatui::style::Style::default().fg(app.theme.blocks)),
                Line::from("📊 Stats [3]").style(ratatui::style::Style::default().fg(app.theme.blocks)),
                Line::from("⚙️ Settings [4]").style(ratatui::style::Style::default().fg(app.theme.blocks)),
            ])
            .select(match app.tab {
                Tab::Timer => 0,
                Tab::Plant => 1,
                Tab::Stats => 2,
                Tab::Settings => 3,
            })
            .style(ratatui::style::Style::default().fg(app.theme.tabs))
            .highlight_style(ratatui::style::Style::default().fg(app.theme.highlight).add_modifier(ratatui::style::Modifier::BOLD));
            f.render_widget(tabs, chunks[0]);

            // Main content
            match app.tab {
                Tab::Timer => timer_ui::draw_timer(f, &mut app, chunks[1]),
                Tab::Plant => plant_ui::draw_plant(f, &app, chunks[1]),
                Tab::Stats => stats_ui::draw_stats(f, &app, chunks[1]),
                Tab::Settings => settings_ui::draw_settings(f, &app, chunks[1]),
            }

            // Footer: Status and hints
            let footer_text = match app.tab {
                Tab::Timer => "Start/Pause [Space] | Stop [S] | Switch Blocks [←/→] | Select/Adjust [↑/↓] | Quit [Q]",
                Tab::Plant => "Quit [Q]",
                Tab::Stats => "Quit [Q]",
                Tab::Settings => "Switch Blocks [←/→] | Select/Adjust [↑/↓] | Quit [Q]",
            };
            let footer = ratatui::widgets::Paragraph::new(footer_text)
                .style(ratatui::style::Style::default().fg(app.theme.secondary_text));
            f.render_widget(footer, chunks[2]);
        })?;

        // Handle events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let Some(action) = handle_key(key) {
                    app.handle_input(action);
                }
            }
        }

        // Tick
        if last_tick.elapsed() >= Duration::from_millis(250) {
            app.tick();
            last_tick = Instant::now();
        }

                if app.should_quit {
                    app.save();
                    break;
                }
    }

    // Save data
    app.save();

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}