use crate::entity::position::Position;

/// Represents a rectangular room in the dungeon
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Room {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Room {
    /// Creates a new room with the given position and dimensions
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Room {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
        }
    }

    /// Returns the center position of the room
    pub fn center(&self) -> Position {
        Position {
            x: (self.x1 + self.x2) / 2,
            y: (self.y1 + self.y2) / 2,
        }
    }

    /// Checks if this room intersects with another room (with 1 tile margin)
    ///
    /// Returns true when rooms overlap or are within 1 tile of each other:
    ///
    /// ```text
    /// Case 1: Overlapping (true)     Case 2: Adjacent/1-tile gap (true)
    /// ┌─────────┐                    ┌─────┐ ┌─────┐
    /// │  Room1  │                    │  R1 │ │  R2 │
    /// │   ┌─────┼───┐                └─────┘ └─────┘
    /// │   │     │   │                   x2=5  x1=6  (5 >= 6-1 → true)
    /// └───┼─────┘   │
    ///     │  Room2  │                Case 3: 2+ tile gap (false)
    ///     └─────────┘                ┌─────┐   ┌─────┐
    ///                                │  R1 │   │  R2 │
    ///                                └─────┘   └─────┘
    ///                                   x2=5    x1=8  (5 >= 8-1=7 → false)
    /// ```
    ///
    /// The 1-tile margin ensures rooms don't spawn too close together.
    pub fn intersects(&self, other: &Room) -> bool {
        self.x1 <= other.x2 + 1
            && self.x2 >= other.x1 - 1
            && self.y1 <= other.y2 + 1
            && self.y2 >= other.y1 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_creation() {
        let room = Room::new(10, 20, 5, 4);
        assert_eq!(room.x1, 10);
        assert_eq!(room.y1, 20);
        assert_eq!(room.x2, 15);
        assert_eq!(room.y2, 24);
    }

    #[test]
    fn test_room_center() {
        let room = Room::new(10, 10, 10, 10);
        let center = room.center();
        assert_eq!(center.x, 15);
        assert_eq!(center.y, 15);
    }

    #[test]
    fn test_rooms_intersect() {
        let room1 = Room::new(0, 0, 10, 10);
        let room2 = Room::new(5, 5, 10, 10); // Overlaps
        assert!(room1.intersects(&room2));
    }

    #[test]
    fn test_rooms_do_not_intersect() {
        let room1 = Room::new(0, 0, 5, 5);
        let room2 = Room::new(10, 10, 5, 5); // Far apart
        assert!(!room1.intersects(&room2));
    }

    #[test]
    fn test_rooms_adjacent_intersect() {
        // Adjacent rooms (1 tile apart) should be considered intersecting
        // to prevent rooms from being too close
        let room1 = Room::new(0, 0, 5, 5);
        let room2 = Room::new(6, 0, 5, 5); // Only 1 tile gap
        assert!(room1.intersects(&room2));
    }
}
