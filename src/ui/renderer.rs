use crate::{game::Game, world::tile::TileType};
use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, game: &Game) {
    let mut lines: Vec<Line> = Vec::new();

    for y in 0..game.dungeon.height {
        let mut spans: Vec<Span> = Vec::new();
        for x in 0..game.dungeon.width {
            let (ch, style) =
                if game.player.position.x as usize == x && game.player.position.y as usize == y {
                    (game.player.to_char(), Style::default().fg(Color::Yellow))
                } else {
                    let tile = game.dungeon.tiles[y][x];
                    let ch = tile.to_char();
                    let color = match tile {
                        TileType::Wall => Color::Gray,
                        TileType::Floor => Color::DarkGray,
                        TileType::StairsDown => Color::Cyan,
                    };
                    (ch, Style::default().fg(color))
                };
            spans.push(Span::styled(ch.to_string(), style));
        }
        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Hyakki - 百鬼"),
    );

    frame.render_widget(paragraph, frame.area());
}
