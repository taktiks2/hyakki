use crate::entity::position::Position;
use crate::world::tile::TileType;

pub const DUNGEON_WIDTH: usize = 80;
pub const DUNGEON_HEIGHT: usize = 50;

pub struct Dungeon {
    pub tiles: Vec<Vec<TileType>>,
    pub width: usize,
    pub height: usize,
}

impl Dungeon {
    /// Creates a fixed test dungeon (for Phase 1)
    pub fn new_fixed() -> Self {
        let mut tiles = vec![vec![TileType::Wall; DUNGEON_WIDTH]; DUNGEON_HEIGHT];

        // Create a room in the center (10x10)
        for row in tiles.iter_mut().take(30).skip(20) {
            for tile in row.iter_mut().take(45).skip(35) {
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
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        let x = pos.x as usize;
        let y = pos.y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.tiles[y][x])
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
