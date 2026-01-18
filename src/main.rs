use std::io;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;

use hyakki::Game;
use hyakki::ui::{Action, handle_key, render};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
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
