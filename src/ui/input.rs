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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventKind, KeyEventState, KeyModifiers};

    fn make_key_event(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn test_arrow_keys() {
        assert_eq!(
            handle_key(make_key_event(KeyCode::Up)),
            Action::Move { dx: 0, dy: -1 }
        );
        assert_eq!(
            handle_key(make_key_event(KeyCode::Down)),
            Action::Move { dx: 0, dy: 1 }
        );
        assert_eq!(
            handle_key(make_key_event(KeyCode::Left)),
            Action::Move { dx: -1, dy: 0 }
        );
        assert_eq!(
            handle_key(make_key_event(KeyCode::Right)),
            Action::Move { dx: 1, dy: 0 }
        );
    }

    #[test]
    fn test_vi_keys() {
        assert_eq!(
            handle_key(make_key_event(KeyCode::Char('h'))),
            Action::Move { dx: -1, dy: 0 }
        );
        assert_eq!(
            handle_key(make_key_event(KeyCode::Char('j'))),
            Action::Move { dx: 0, dy: 1 }
        );
        assert_eq!(
            handle_key(make_key_event(KeyCode::Char('k'))),
            Action::Move { dx: 0, dy: -1 }
        );
        assert_eq!(
            handle_key(make_key_event(KeyCode::Char('l'))),
            Action::Move { dx: 1, dy: 0 }
        );
    }

    #[test]
    fn test_quit_keys() {
        assert_eq!(handle_key(make_key_event(KeyCode::Char('q'))), Action::Quit);
        assert_eq!(handle_key(make_key_event(KeyCode::Esc)), Action::Quit);
    }

    #[test]
    fn test_unknown_keys() {
        assert_eq!(handle_key(make_key_event(KeyCode::Char('x'))), Action::None);
        assert_eq!(handle_key(make_key_event(KeyCode::Enter)), Action::None);
        assert_eq!(handle_key(make_key_event(KeyCode::Tab)), Action::None);
    }
}
