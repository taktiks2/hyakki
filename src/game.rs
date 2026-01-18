pub mod config;
pub mod state;

use crate::{
    entity::player::Player,
    world::{dungeon::Dungeon, tile::TileType},
};
use config::MAX_DEPTH;
use state::GameState;

pub struct Game {
    pub state: GameState,
    pub dungeon: Dungeon,
    pub player: Player,
    pub running: bool,
}

impl Game {
    pub fn new() -> Self {
        let dungeon = Dungeon::new_random(1);
        let player = Player::new(dungeon.player_start);

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

    /// Attempts to descend to the next dungeon level if standing on stairs
    pub fn try_descend(&mut self) {
        // Check if player is standing on stairs
        if self.dungeon.get_tile(self.player.position) == Some(TileType::StairsDown) {
            let new_depth = self.dungeon.depth + 1;
            if new_depth <= MAX_DEPTH {
                self.dungeon = Dungeon::new_random(new_depth);
                self.player.position = self.dungeon.player_start;
            }
        }
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
    use crate::entity::position::Position;

    #[test]
    fn test_player_starts_at_dungeon_player_start() {
        let game = Game::new();
        assert_eq!(game.player.position, game.dungeon.player_start);
    }

    #[test]
    fn test_player_movement_valid() {
        let mut game = Game::new();
        let start_pos = game.player.position;

        // Try moving in each direction until we find a walkable tile
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = start_pos.translate(dx, dy);
            if game.dungeon.is_walkable(new_pos) {
                game.try_move_player(dx, dy);
                assert_eq!(game.player.position, new_pos);
                return;
            }
        }
        // Player should be able to move at least one direction in a room
        panic!("Player couldn't move in any direction from start position");
    }

    #[test]
    fn test_player_blocked_by_wall() {
        let mut game = Game::new();
        // Move player to a wall position (0,0 is always a wall)
        let wall_adjacent = Position { x: 1, y: 1 };
        game.player.position = wall_adjacent;

        // The corner (0,0) is always a wall
        let wall_pos = Position { x: 0, y: 0 };
        assert!(!game.dungeon.is_walkable(wall_pos));

        // Try to move into the wall
        game.try_move_player(-1, -1);
        // Position should not change (diagonal move not supported, but blocked anyway)
        assert_eq!(game.player.position, wall_adjacent);
    }

    #[test]
    fn test_player_cannot_move_outside_dungeon() {
        let mut game = Game::new();
        game.player.position = Position { x: 0, y: 0 };

        // Try to move outside boundary
        game.try_move_player(-1, 0);
        assert_eq!(game.player.position, Position { x: 0, y: 0 });
    }

    #[test]
    fn test_descend_on_stairs() {
        let mut game = Game::new();
        let initial_depth = game.dungeon.depth;

        // Move player to stairs
        game.player.position = game.dungeon.stairs_position;

        // Descend
        game.try_descend();

        // Depth should increase
        assert_eq!(game.dungeon.depth, initial_depth + 1);
        // Player should be at new dungeon's start position
        assert_eq!(game.player.position, game.dungeon.player_start);
    }

    #[test]
    fn test_descend_not_on_stairs() {
        let mut game = Game::new();
        let initial_depth = game.dungeon.depth;

        // Player is at start position (not on stairs)
        let start_pos = game.player.position;

        // Try to descend (should fail)
        game.try_descend();

        // Depth should not change
        assert_eq!(game.dungeon.depth, initial_depth);
        // Position should not change
        assert_eq!(game.player.position, start_pos);
    }

    #[test]
    fn test_cannot_descend_past_max_depth() {
        use crate::game::config::MAX_DEPTH;

        let mut game = Game::new();
        // Set dungeon to max depth
        game.dungeon = Dungeon::new_random(MAX_DEPTH);
        game.player.position = game.dungeon.stairs_position;

        // Try to descend (should fail since we're at max depth)
        game.try_descend();

        // Depth should still be MAX_DEPTH
        assert_eq!(game.dungeon.depth, MAX_DEPTH);
    }
}
