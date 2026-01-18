use crate::entity::position::Position;

pub struct Player {
    pub position: Position,
}

impl Player {
    pub fn new(position: Position) -> Self {
        Player { position }
    }

    pub fn to_char(&self) -> char {
        '@'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(Position { x: 5, y: 5 });
        assert_eq!(player.position, Position { x: 5, y: 5 });
    }

    #[test]
    fn test_player_symbol() {
        let player = Player::new(Position { x: 0, y: 0 });
        assert_eq!(player.to_char(), '@');
    }
}
