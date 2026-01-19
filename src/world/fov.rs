//! Field of View (FOV) calculation module
//!
//! Uses recursive shadowcasting algorithm to calculate
//! which tiles are visible from the player's position.

use crate::entity::position::Position;
use crate::world::dungeon::Dungeon;
use std::collections::HashSet;

/// Manages field of view state
pub struct Fov {
    /// Currently visible tiles
    visible: HashSet<(i32, i32)>,
    /// Previously explored tiles (persist across moves)
    explored: HashSet<(i32, i32)>,
    /// FOV radius
    radius: i32,
}

impl Fov {
    /// Creates a new FOV with the given radius
    pub fn new(radius: i32) -> Self {
        Self {
            visible: HashSet::new(),
            explored: HashSet::new(),
            radius,
        }
    }

    /// Calculates visible tiles from the given origin
    pub fn calculate(&mut self, _origin: Position, _dungeon: &Dungeon) {
        // TODO: Implement shadowcasting
        self.visible.clear();
    }

    /// Returns true if the position is currently visible
    pub fn is_visible(&self, pos: Position) -> bool {
        self.visible.contains(&(pos.x, pos.y))
    }

    /// Returns true if the position has been explored
    pub fn is_explored(&self, pos: Position) -> bool {
        self.explored.contains(&(pos.x, pos.y))
    }

    /// Returns the number of currently visible tiles (for testing)
    pub fn visible_count(&self) -> usize {
        self.visible.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::tile::TileType;

    /// Creates a test dungeon from ASCII layout
    /// '#' = Wall, '.' = Floor, '>' = StairsDown
    fn create_test_dungeon(layout: &[&str]) -> Dungeon {
        let height = layout.len();
        let width = layout[0].len();

        let tiles: Vec<Vec<TileType>> = layout
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '#' => TileType::Wall,
                        '.' => TileType::Floor,
                        '>' => TileType::StairsDown,
                        _ => TileType::Wall,
                    })
                    .collect()
            })
            .collect();

        Dungeon {
            tiles,
            width,
            height,
            rooms: vec![],
            depth: 1,
            player_start: Position { x: 1, y: 1 },
            stairs_position: Position { x: 1, y: 1 },
        }
    }

    #[test]
    fn test_fov_includes_adjacent_tiles() {
        // 5x5 open room with player at center (2,2)
        let dungeon = create_test_dungeon(&[
            "#####",
            "#...#",
            "#...#",
            "#...#",
            "#####",
        ]);
        let mut fov = Fov::new(8);
        fov.calculate(Position { x: 2, y: 2 }, &dungeon);

        // Adjacent tiles should be visible
        assert!(fov.is_visible(Position { x: 2, y: 1 }), "Up should be visible");
        assert!(fov.is_visible(Position { x: 2, y: 3 }), "Down should be visible");
        assert!(fov.is_visible(Position { x: 1, y: 2 }), "Left should be visible");
        assert!(fov.is_visible(Position { x: 3, y: 2 }), "Right should be visible");
    }

    #[test]
    fn test_fov_blocked_by_walls() {
        // Room with wall in the middle
        let dungeon = create_test_dungeon(&[
            "#####",
            "#...#",
            "#.#.#",
            "#...#",
            "#####",
        ]);
        let mut fov = Fov::new(8);
        fov.calculate(Position { x: 1, y: 2 }, &dungeon);

        // Behind the wall should not be visible
        assert!(!fov.is_visible(Position { x: 3, y: 2 }), "Behind wall should not be visible");
        // Wall itself should be visible
        assert!(fov.is_visible(Position { x: 2, y: 2 }), "Wall should be visible");
    }

    #[test]
    fn test_fov_radius() {
        // Large open area (15x15)
        let layout: Vec<String> = (0..15)
            .map(|y| {
                (0..15)
                    .map(|x| {
                        if x == 0 || x == 14 || y == 0 || y == 14 {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect()
            })
            .collect();
        let layout_refs: Vec<&str> = layout.iter().map(|s| s.as_str()).collect();
        let dungeon = create_test_dungeon(&layout_refs);

        let mut fov = Fov::new(8);
        fov.calculate(Position { x: 7, y: 7 }, &dungeon);

        // Within radius 8 should be visible (distance 7 from center to edge of open area)
        assert!(fov.is_visible(Position { x: 7, y: 1 }), "Distance 6 should be visible");
        // The outer wall at distance 7 should be visible
        assert!(fov.is_visible(Position { x: 7, y: 0 }), "Wall at distance 7 should be visible");
    }

    #[test]
    fn test_explored_tiles_persist() {
        let dungeon = create_test_dungeon(&[
            "#######",
            "#.....#",
            "#.....#",
            "#.....#",
            "#######",
        ]);
        let mut fov = Fov::new(8);

        // Calculate FOV from first position
        fov.calculate(Position { x: 1, y: 2 }, &dungeon);
        assert!(fov.is_visible(Position { x: 1, y: 1 }));
        assert!(fov.is_explored(Position { x: 1, y: 1 }));

        // Move to different position and recalculate
        fov.calculate(Position { x: 5, y: 2 }, &dungeon);

        // Previous position should no longer be visible but still explored
        // (depending on room size, it might still be visible, so we test a corner case)
        // In this small room, the whole room is visible, so let's check explored persists
        assert!(fov.is_explored(Position { x: 1, y: 1 }), "Previously seen tile should remain explored");
    }

    #[test]
    fn test_player_position_always_visible() {
        let dungeon = create_test_dungeon(&[
            "###",
            "#.#",
            "###",
        ]);
        let mut fov = Fov::new(8);
        fov.calculate(Position { x: 1, y: 1 }, &dungeon);

        assert!(fov.is_visible(Position { x: 1, y: 1 }), "Player position should always be visible");
    }

    #[test]
    fn test_fov_diagonal_visibility() {
        let dungeon = create_test_dungeon(&[
            "#####",
            "#...#",
            "#...#",
            "#...#",
            "#####",
        ]);
        let mut fov = Fov::new(8);
        fov.calculate(Position { x: 2, y: 2 }, &dungeon);

        // Diagonal tiles should be visible
        assert!(fov.is_visible(Position { x: 1, y: 1 }), "Upper-left diagonal should be visible");
        assert!(fov.is_visible(Position { x: 3, y: 3 }), "Lower-right diagonal should be visible");
        assert!(fov.is_visible(Position { x: 1, y: 3 }), "Lower-left diagonal should be visible");
        assert!(fov.is_visible(Position { x: 3, y: 1 }), "Upper-right diagonal should be visible");
    }

    #[test]
    fn test_walls_visible_but_block_view() {
        let dungeon = create_test_dungeon(&[
            "#####",
            "#..##",
            "#...#",
            "#####",
            "#...#",
        ]);
        let mut fov = Fov::new(8);
        fov.calculate(Position { x: 1, y: 2 }, &dungeon);

        // Wall row should be visible
        assert!(fov.is_visible(Position { x: 0, y: 3 }), "Wall should be visible");
        // Behind wall row should not be visible
        assert!(!fov.is_visible(Position { x: 1, y: 4 }), "Behind wall should not be visible");
    }
}
