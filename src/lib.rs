pub mod entity;
pub mod game;
pub mod ui;
pub mod world;

pub use game::Game;

use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use ui::{Action, handle_key, render};

/// RAII guard to ensure terminal restoration on drop (including panics)
struct TerminalGuard {
    terminal: DefaultTerminal,
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

pub fn run() -> Result<()> {
    let mut guard = TerminalGuard {
        terminal: ratatui::init(),
    };
    run_game_loop(&mut guard.terminal)
}

fn run_game_loop(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut game = Game::new();

    while game.running {
        terminal.draw(|frame| render(frame, &game))?;

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match handle_key(key) {
                Action::Move { dx, dy } => game.try_move_player(dx, dy),
                Action::Quit => game.quit(),
                Action::None => {}
            },
            Event::Resize(_, _) => {
                // Terminal resized, the next draw will handle it automatically
            }
            _ => {}
        }
    }

    Ok(())
}
