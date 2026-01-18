use crate::{entity::position::Position, world::tile::TileType};

pub const DUNGEON_WIDTH: usize = 80;
pub const DUNGEON_HEIGHT: usize = 50;

// Fixed test room constants (Phase 1)
const ROOM_Y_START: usize = 20;
const ROOM_Y_END: usize = 30;
const ROOM_X_START: usize = 35;
const ROOM_X_END: usize = 45;

/// Center of the fixed test room (for player spawn)
pub const ROOM_CENTER_X: i32 = (ROOM_X_START + ROOM_X_END) as i32 / 2;
pub const ROOM_CENTER_Y: i32 = (ROOM_Y_START + ROOM_Y_END) as i32 / 2;

pub struct Dungeon {
    pub tiles: Vec<Vec<TileType>>,
    pub width: usize,
    pub height: usize,
}

impl Dungeon {
    /// Creates a fixed test dungeon (for Phase 1)
    pub fn new_fixed() -> Self {
        let mut tiles = vec![vec![TileType::Wall; DUNGEON_WIDTH]; DUNGEON_HEIGHT];

        // Create a 10x10 room (rows 20-29, columns 35-44)
        for row in tiles.iter_mut().take(ROOM_Y_END).skip(ROOM_Y_START) {
            for tile in row.iter_mut().take(ROOM_X_END).skip(ROOM_X_START) {
                *tile = TileType::Floor;
            }
        }

        Dungeon {
            tiles,
            width: DUNGEON_WIDTH,
            height: DUNGEON_HEIGHT,
        }
    }

    pub fn get_tile(&self, pos: Position) -> Option<TileType> {
        let x: usize = pos.x.try_into().ok()?;
        let y: usize = pos.y.try_into().ok()?;
        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn is_walkable(&self, pos: Position) -> bool {
        self.get_tile(pos).is_some_and(|t| t.is_walkable())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dungeon_dimensions() {
        let dungeon = Dungeon::new_fixed();
        assert_eq!(dungeon.width, DUNGEON_WIDTH);
        assert_eq!(dungeon.height, DUNGEON_HEIGHT);
    }

    #[test]
    fn test_dungeon_boundaries_negative() {
        let dungeon = Dungeon::new_fixed();
        assert!(dungeon.get_tile(Position { x: -1, y: 0 }).is_none());
        assert!(dungeon.get_tile(Position { x: 0, y: -1 }).is_none());
    }

    #[test]
    fn test_dungeon_boundaries_overflow() {
        let dungeon = Dungeon::new_fixed();
        assert!(dungeon.get_tile(Position { x: 80, y: 0 }).is_none());
        assert!(dungeon.get_tile(Position { x: 0, y: 50 }).is_none());
    }

    #[test]
    fn test_dungeon_has_floor() {
        let dungeon = Dungeon::new_fixed();
        // Confirm the center room has Floor
        let center = Position { x: 40, y: 25 };
        assert_eq!(dungeon.get_tile(center), Some(TileType::Floor));
    }

    #[test]
    fn test_dungeon_corners_are_walls() {
        let dungeon = Dungeon::new_fixed();
        assert_eq!(
            dungeon.get_tile(Position { x: 0, y: 0 }),
            Some(TileType::Wall)
        );
        assert_eq!(
            dungeon.get_tile(Position { x: 79, y: 49 }),
            Some(TileType::Wall)
        );
    }
}
