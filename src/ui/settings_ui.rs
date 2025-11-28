use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

pub fn draw_settings(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left: Settings list
    let settings = vec![
        format!("Focus Duration: {} min", app.settings.focus_duration),
        format!("Short Break: {} min", app.settings.short_break_duration),
        format!("Long Break: {} min", app.settings.long_break_duration),
        format!("Theme: {:?}", app.settings.theme),
    ];
    let items: Vec<ListItem> = settings
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_selected = i == app.settings_selected && app.focus == crate::app::Focus::Left;
            let text = if is_selected { format!("→ {}", s) } else { format!("  {}", s) };
            let style = if is_selected {
                Style::default().fg(app.theme.highlight).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };
            ListItem::new(text).style(style)
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().title_top(Line::from(" Settings ").style(Style::default().fg(app.theme.blocks)).centered()).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)));
    f.render_widget(list, chunks[0]);

    // Right: Adjustment
    let is_focused = app.focus == crate::app::Focus::Right;
    let right_items: Vec<ListItem> = match app.settings_selected {
        0 => {
            let mut style = Style::default().fg(app.theme.text);
            if is_focused {
                style = style.add_modifier(Modifier::BOLD);
            }
            vec![ListItem::new(Line::from(vec![
                Span::styled("  ", style),
                Span::styled(format!("{}", app.settings.focus_duration), Style::default().fg(app.theme.highlight)),
                Span::styled(" min", style),
            ])).style(style)]
        }
        1 => {
            let mut style = Style::default().fg(app.theme.text);
            if is_focused {
                style = style.add_modifier(Modifier::BOLD);
            }
            vec![ListItem::new(Line::from(vec![
                Span::styled("  ", style),
                Span::styled(format!("{}", app.settings.short_break_duration), Style::default().fg(app.theme.highlight)),
                Span::styled(" min", style),
            ])).style(style)]
        }
        2 => {
            let mut style = Style::default().fg(app.theme.text);
            if is_focused {
                style = style.add_modifier(Modifier::BOLD);
            }
            vec![ListItem::new(Line::from(vec![
                Span::styled("  ", style),
                Span::styled(format!("{}", app.settings.long_break_duration), Style::default().fg(app.theme.highlight)),
                Span::styled(" min", style),
            ])).style(style)]
        }
        3 => {
            let themes = vec!["System", "Rose Pine Light", "Rose Pine Dark"];
            let current_index = match app.settings.theme {
                crate::theme::ThemeVariant::System => 0,
                crate::theme::ThemeVariant::RosePineLight => 1,
                crate::theme::ThemeVariant::RosePineDark => 2,
            };
            themes.iter().enumerate().map(|(i, &theme)| {
                let prefix = if i == current_index { "→ " } else { "  " };
                let style = if i == current_index && is_focused {
                    Style::default().fg(app.theme.highlight).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.text)
                };
                ListItem::new(format!("{}{}", prefix, theme)).style(style)
            }).collect()
        }
        _ => vec![],
    };
    let right_list = List::new(right_items)
        .block(Block::default().title_top(Line::from(" Adjust ").style(Style::default().fg(app.theme.blocks)).centered()).borders(Borders::ALL).style(Style::default().fg(app.theme.blocks)));
    f.render_widget(right_list, chunks[1]);
}