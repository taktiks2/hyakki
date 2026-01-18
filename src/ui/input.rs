use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Move { dx: i32, dy: i32 },
    Quit,
    None,
}

pub fn handle_key(key: KeyEvent) -> Action {
    match key.code {
        // Arrow keys
        KeyCode::Up => Action::Move { dx: 0, dy: -1 },
        KeyCode::Down => Action::Move { dx: 0, dy: 1 },
        KeyCode::Left => Action::Move { dx: -1, dy: 0 },
        KeyCode::Right => Action::Move { dx: 1, dy: 0 },
        // Vi-style keys
        KeyCode::Char('h') => Action::Move { dx: -1, dy: 0 },
        KeyCode::Char('j') => Action::Move { dx: 0, dy: 1 },
        KeyCode::Char('k') => Action::Move { dx: 0, dy: -1 },
        KeyCode::Char('l') => Action::Move { dx: 1, dy: 0 },
        // Quit
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Esc => Action::Quit,
        _ => Action::None,
    }
}
