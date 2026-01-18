pub mod state;

use crate::{
    entity::{player::Player, position::Position},
    world::dungeon::{Dungeon, ROOM_CENTER_X, ROOM_CENTER_Y},
};
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
        let player = Player::new(Position {
            x: ROOM_CENTER_X,
            y: ROOM_CENTER_Y,
        });

        Game {
            state: GameState::default(),
            dungeon,
            player,
            running: true,
        }
    }

    pub fn try_move_player(&mut self, dx: i32, dy: i32) {
        let new_pos = self.player.position.translate(dx, dy);

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
        // Player starts at center of room (ROOM_CENTER_X, ROOM_CENTER_Y)

        // Move right (on Floor)
        let new_pos = Position {
            x: ROOM_CENTER_X + 1,
            y: ROOM_CENTER_Y,
        };
        assert!(game.dungeon.is_walkable(new_pos));
        game.try_move_player(1, 0);
        assert_eq!(game.player.position, new_pos);
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
