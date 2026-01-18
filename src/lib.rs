pub mod entity;
pub mod game;
pub mod ui;
pub mod world;

pub use game::Game;

use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;

use ui::{Action, handle_key, render};

pub fn run() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = run_game_loop(&mut terminal);
    ratatui::restore();
    result
}

fn run_game_loop(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut game = Game::new();

    while game.running {
        terminal.draw(|frame| render(frame, &game))?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match handle_key(key) {
                Action::Move { dx, dy } => game.try_move_player(dx, dy),
                Action::Quit => game.quit(),
                Action::None => {}
            }
        }
    }

    Ok(())
}
