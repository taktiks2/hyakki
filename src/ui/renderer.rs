use crate::{game::Game, world::tile::TileType};
use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, game: &Game) {
    let mut lines: Vec<Line> = Vec::new();
    let mut char_buf = [0u8; 4];

    for y in 0..game.dungeon.height {
        let mut spans: Vec<Span> = Vec::new();
        for x in 0..game.dungeon.width {
            let px = game.player.position.x;
            let py = game.player.position.y;
            let is_player_here = px >= 0 && py >= 0 && px as usize == x && py as usize == y;

            let (ch, style) = if is_player_here {
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
            // Use stack-allocated buffer to avoid heap allocation
            let s = ch.encode_utf8(&mut char_buf);
            spans.push(Span::styled(s.to_owned(), style));
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
