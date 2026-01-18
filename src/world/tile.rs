#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
    StairsDown,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        matches!(self, TileType::Floor | TileType::StairsDown)
    }

    pub fn to_char(&self) -> char {
        match self {
            TileType::Wall => '#',
            TileType::Floor => '.',
            TileType::StairsDown => '>',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_not_walkable() {
        assert!(!TileType::Wall.is_walkable());
    }

    #[test]
    fn test_floor_walkable() {
        assert!(TileType::Floor.is_walkable());
    }

    #[test]
    fn test_stairs_walkable() {
        assert!(TileType::StairsDown.is_walkable());
    }

    #[test]
    fn test_tile_to_char() {
        assert_eq!(TileType::Wall.to_char(), '#');
        assert_eq!(TileType::Floor.to_char(), '.');
        assert_eq!(TileType::StairsDown.to_char(), '>');
    }
}
