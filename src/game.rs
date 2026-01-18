pub mod state;

use crate::entity::player::Player;
use crate::entity::position::Position;
use crate::world::dungeon::Dungeon;
use state::GameState;

pub struct Game {
    pub state: GameState,
    pub dungeon: Dungeon,
    pub player: Player,
    pub running: bool,
}

impl Game {
    pub fn new() -> Self {
        let dungeon = Dungeon::new_fixed();
        // Place player in the center of the room
        let player = Player::new(Position { x: 40, y: 25 });

        Game {
            state: GameState::default(),
            dungeon,
            player,
            running: true,
        }
    }

    pub fn try_move_player(&mut self, dx: i32, dy: i32) {
        let new_pos = Position {
            x: self.player.position.x + dx,
            y: self.player.position.y + dy,
        };

        if self.dungeon.is_walkable(new_pos) {
            self.player.position = new_pos;
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_movement_valid() {
        let mut game = Game::new();
        // Player is at center of room (40, 25)
        game.player.position = Position { x: 40, y: 25 };

        // Move right (on Floor)
        let new_pos = Position { x: 41, y: 25 };
        assert!(game.dungeon.is_walkable(new_pos));
        game.try_move_player(1, 0);
        assert_eq!(game.player.position, Position { x: 41, y: 25 });
    }

    #[test]
    fn test_player_blocked_by_wall() {
        let mut game = Game::new();
        // Place player next to wall
        game.player.position = Position { x: 35, y: 25 };

        // Try to move left (into wall)
        let wall_pos = Position { x: 34, y: 25 };
        assert!(!game.dungeon.is_walkable(wall_pos));
        game.try_move_player(-1, 0);
        // Position should not change
        assert_eq!(game.player.position, Position { x: 35, y: 25 });
    }

    #[test]
    fn test_player_cannot_move_outside_dungeon() {
        let mut game = Game::new();
        game.player.position = Position { x: 0, y: 0 };

        // Try to move outside boundary
        game.try_move_player(-1, 0);
        assert_eq!(game.player.position, Position { x: 0, y: 0 });
    }
}
