#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn translate(self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position { x: 5, y: 10 };
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_position_equality() {
        let pos1 = Position { x: 1, y: 2 };
        let pos2 = Position { x: 1, y: 2 };
        let pos3 = Position { x: 3, y: 4 };
        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_position_translate() {
        let pos = Position { x: 5, y: 10 };
        assert_eq!(pos.translate(1, 0), Position { x: 6, y: 10 });
        assert_eq!(pos.translate(-1, 0), Position { x: 4, y: 10 });
        assert_eq!(pos.translate(0, 1), Position { x: 5, y: 11 });
        assert_eq!(pos.translate(0, -1), Position { x: 5, y: 9 });
        assert_eq!(pos.translate(3, -2), Position { x: 8, y: 8 });
    }
}
