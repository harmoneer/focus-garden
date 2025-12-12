use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy)]
pub enum InputAction {
    Tab(u8), // 1-4...
    Left,
    Right,
    Up,
    Down,
    Space,
    Pause,
    Stop,
    Quit,
    Enter,
    Delete,
}

pub fn handle_key(key: KeyEvent) -> Option<InputAction> {
    match key.code {
        KeyCode::Char('1') => Some(InputAction::Tab(1)),
        KeyCode::Char('2') => Some(InputAction::Tab(2)),
        KeyCode::Char('3') => Some(InputAction::Tab(3)),
        KeyCode::Char('4') => Some(InputAction::Tab(4)),
        KeyCode::Left => Some(InputAction::Left),
        KeyCode::Right => Some(InputAction::Right),
        KeyCode::Up => Some(InputAction::Up),
        KeyCode::Down => Some(InputAction::Down),
        KeyCode::Char(' ') => Some(InputAction::Space),
        KeyCode::Char('p') => Some(InputAction::Pause),
        KeyCode::Char('s') | KeyCode::Char('S') => Some(InputAction::Stop),
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(InputAction::Quit),
        KeyCode::Enter => Some(InputAction::Enter),
        KeyCode::Delete => Some(InputAction::Delete),
        KeyCode::Backspace => Some(InputAction::Delete),
        _ => None,
    }
}