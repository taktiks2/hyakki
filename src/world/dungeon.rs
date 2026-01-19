use crate::{
    entity::position::Position,
    game::config::{
        DUNGEON_HEIGHT, DUNGEON_WIDTH, MAX_ROOM_SIZE, MAX_ROOMS, MIN_ROOM_SIZE, MIN_ROOMS,
    },
    world::{generator::Room, tile::TileType},
};
use rand::Rng;

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
    pub rooms: Vec<Room>,
    pub depth: u32,
    pub player_start: Position,
    pub stairs_position: Position,
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
            rooms: vec![],
            depth: 1,
            player_start: Position {
                x: ROOM_CENTER_X,
                y: ROOM_CENTER_Y,
            },
            stairs_position: Position {
                x: ROOM_CENTER_X,
                y: ROOM_CENTER_Y,
            },
        }
    }

    /// Creates a randomly generated dungeon
    pub fn new_random(depth: u32) -> Self {
        let mut rng = rand::thread_rng();
        Self::new_random_with_rng(depth, &mut rng)
    }

    /// Creates a randomly generated dungeon with a provided RNG (for testing)
    pub fn new_random_with_rng<R: Rng>(depth: u32, rng: &mut R) -> Self {
        // Initialize all tiles as walls
        let mut tiles = vec![vec![TileType::Wall; DUNGEON_WIDTH]; DUNGEON_HEIGHT];
        let mut rooms = Vec::new();

        // Determine desired number of rooms (may not be reached due to MAX_ATTEMPTS)
        let desired_room_count = rng.gen_range(MIN_ROOMS..=MAX_ROOMS);
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 200;

        // Try to place rooms
        while rooms.len() < desired_room_count && attempts < MAX_ATTEMPTS {
            let width = rng.gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);
            let height = rng.gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);
            let x = rng.gen_range(1..(DUNGEON_WIDTH as i32 - width - 1));
            let y = rng.gen_range(1..(DUNGEON_HEIGHT as i32 - height - 1));

            let new_room = Room::new(x, y, width, height);

            // Check if this room intersects with any existing room
            let intersects = rooms.iter().any(|r: &Room| r.intersects(&new_room));

            if !intersects {
                // Carve out the room
                Self::carve_room(&mut tiles, &new_room);

                // Connect to previous room with corridor
                if !rooms.is_empty() {
                    let prev_center = rooms.last().unwrap().center();
                    let new_center = new_room.center();
                    Self::carve_corridor(&mut tiles, prev_center, new_center, rng);
                }

                rooms.push(new_room);
            }

            attempts += 1;
        }

        // Determine player start and stairs positions
        // Panic if no rooms were generated - this indicates a bug in the generation algorithm
        let player_start = rooms
            .first()
            .map(|r| r.center())
            .expect("Dungeon generation failed: no rooms were placed");
        let stairs_position = rooms
            .last()
            .map(|r| r.center())
            .expect("Dungeon generation failed: no rooms were placed");

        // Place stairs
        tiles[stairs_position.y as usize][stairs_position.x as usize] = TileType::StairsDown;

        Dungeon {
            tiles,
            width: DUNGEON_WIDTH,
            height: DUNGEON_HEIGHT,
            rooms,
            depth,
            player_start,
            stairs_position,
        }
    }

    /// Carves out a room (fills with Floor tiles)
    ///
    /// "Carve" means to sculpt/dig out space from solid walls, like carving stone.
    /// This dungeon uses the "carving approach":
    ///
    /// ```text
    /// 1. Start: Fill everything with walls
    ///    ████████████
    ///    ████████████
    ///
    /// 2. Carve rooms: Remove walls to create floor
    ///    ████████████
    ///    ███┌──┐█████
    ///    ███│  │█████  ← Walls become Floor
    ///    ███└──┘█████
    /// ```
    ///
    /// # Why `&mut [Vec<TileType>]` instead of `&mut [&mut [TileType]]`?
    ///
    /// - `Vec<Vec<T>>` auto-converts to `&mut [Vec<T>]` (Deref coercion)
    /// - `Vec<Vec<T>>` does NOT auto-convert to `&mut [&mut [T]]`
    /// - Using `&mut [&mut [T]]` would require manual conversion at call site
    fn carve_room(tiles: &mut [Vec<TileType>], room: &Room) {
        for y in room.y1..room.y2 {
            for x in room.x1..room.x2 {
                tiles[y as usize][x as usize] = TileType::Floor;
            }
        }
    }

    /// Carves an L-shaped corridor between two points
    ///
    /// # Arguments
    /// * `tiles` - The dungeon tile grid to modify
    /// * `start` - Starting position (typically center of previous room)
    /// * `end` - Ending position (typically center of new room)
    /// * `rng` - Random number generator for choosing corridor direction
    ///
    /// # Why generic `R: Rng` instead of concrete type like `ThreadRng`?
    ///
    /// - Production: uses `rand::thread_rng()` (ThreadRng)
    /// - Testing: uses `StdRng::seed_from_u64(seed)` for reproducible results
    /// - Generic allows both, with static dispatch (no runtime cost)
    fn carve_corridor<R: Rng>(
        tiles: &mut [Vec<TileType>],
        start: Position,
        end: Position,
        rng: &mut R,
    ) {
        // Randomly choose horizontal-first or vertical-first
        if rng.gen_bool(0.5) {
            // Horizontal then vertical
            Self::carve_horizontal_tunnel(tiles, start.x, end.x, start.y);
            Self::carve_vertical_tunnel(tiles, start.y, end.y, end.x);
        } else {
            // Vertical then horizontal
            Self::carve_vertical_tunnel(tiles, start.y, end.y, start.x);
            Self::carve_horizontal_tunnel(tiles, start.x, end.x, end.y);
        }
    }

    /// Carves a horizontal tunnel at fixed Y coordinate
    ///
    /// # Arguments
    /// * `tiles` - The dungeon tile grid to modify
    /// * `x1`, `x2` - X coordinates to connect (order doesn't matter)
    /// * `y` - Fixed Y coordinate for the tunnel
    fn carve_horizontal_tunnel(tiles: &mut [Vec<TileType>], x1: i32, x2: i32, y: i32) {
        let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in min_x..=max_x {
            tiles[y as usize][x as usize] = TileType::Floor;
        }
    }

    /// Carves a vertical tunnel at fixed X coordinate
    ///
    /// # Arguments
    /// * `tiles` - The dungeon tile grid to modify
    /// * `y1`, `y2` - Y coordinates to connect (order doesn't matter)
    /// * `x` - Fixed X coordinate for the tunnel
    fn carve_vertical_tunnel(tiles: &mut [Vec<TileType>], y1: i32, y2: i32, x: i32) {
        let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in min_y..=max_y {
            tiles[y as usize][x as usize] = TileType::Floor;
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

    /// Returns true if the tile at the position allows light to pass through.
    /// Walls block light, while floors and stairs are transparent.
    pub fn is_transparent(&self, pos: Position) -> bool {
        self.get_tile(pos)
            .is_some_and(|t| matches!(t, TileType::Floor | TileType::StairsDown))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Fixed dungeon tests (Phase 1) =====

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

    // ===== Random dungeon tests (Phase 2) =====

    #[test]
    fn test_dungeon_has_rooms() {
        let dungeon = Dungeon::new_random(1);
        assert!(!dungeon.rooms.is_empty());
        assert!(dungeon.rooms.len() >= MIN_ROOMS);
        assert!(dungeon.rooms.len() <= MAX_ROOMS);
    }

    #[test]
    fn test_rooms_do_not_overlap() {
        let dungeon = Dungeon::new_random(1);
        for (i, room1) in dungeon.rooms.iter().enumerate() {
            for room2 in dungeon.rooms.iter().skip(i + 1) {
                // Check actual overlap (not just adjacency)
                let overlaps = room1.x1 < room2.x2
                    && room1.x2 > room2.x1
                    && room1.y1 < room2.y2
                    && room1.y2 > room2.y1;
                assert!(!overlaps, "Rooms {:?} and {:?} overlap", room1, room2);
            }
        }
    }

    #[test]
    fn test_dungeon_has_stairs() {
        let dungeon = Dungeon::new_random(1);
        let stairs_count = dungeon
            .tiles
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&tile| tile == TileType::StairsDown)
            .count();
        assert_eq!(stairs_count, 1);
    }

    #[test]
    fn test_stairs_in_last_room() {
        let dungeon = Dungeon::new_random(1);
        let last_room = dungeon.rooms.last().unwrap();
        let stairs = dungeon.stairs_position;
        assert!(
            stairs.x >= last_room.x1
                && stairs.x < last_room.x2
                && stairs.y >= last_room.y1
                && stairs.y < last_room.y2,
            "Stairs should be in last room"
        );
    }

    #[test]
    fn test_player_start_in_first_room() {
        let dungeon = Dungeon::new_random(1);
        let first_room = &dungeon.rooms[0];
        let start = dungeon.player_start;
        assert!(
            start.x >= first_room.x1
                && start.x < first_room.x2
                && start.y >= first_room.y1
                && start.y < first_room.y2,
            "Player should start in first room"
        );
    }

    #[test]
    fn test_all_rooms_connected() {
        use std::collections::VecDeque;

        let dungeon = Dungeon::new_random(1);
        let start = dungeon.player_start;

        let mut visited = vec![vec![false; dungeon.width]; dungeon.height];
        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited[start.y as usize][start.x as usize] = true;

        while let Some(pos) = queue.pop_front() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next = pos.translate(dx, dy);
                if next.x >= 0
                    && next.x < dungeon.width as i32
                    && next.y >= 0
                    && next.y < dungeon.height as i32
                    && !visited[next.y as usize][next.x as usize]
                    && dungeon.is_walkable(next)
                {
                    visited[next.y as usize][next.x as usize] = true;
                    queue.push_back(next);
                }
            }
        }

        // Check all room centers are reachable
        for room in &dungeon.rooms {
            let center = room.center();
            assert!(
                visited[center.y as usize][center.x as usize],
                "Room center {:?} is not reachable",
                center
            );
        }
    }

    #[test]
    fn test_dungeon_depth() {
        let dungeon = Dungeon::new_random(5);
        assert_eq!(dungeon.depth, 5);
    }

    #[test]
    fn test_rooms_within_bounds() {
        let dungeon = Dungeon::new_random(1);
        for room in &dungeon.rooms {
            assert!(room.x1 >= 1, "Room x1 should be >= 1");
            assert!(room.y1 >= 1, "Room y1 should be >= 1");
            assert!(
                (room.x2 as usize) < DUNGEON_WIDTH - 1,
                "Room x2 should be < width - 1"
            );
            assert!(
                (room.y2 as usize) < DUNGEON_HEIGHT - 1,
                "Room y2 should be < height - 1"
            );
        }
    }

    #[test]
    fn test_player_start_is_walkable() {
        let dungeon = Dungeon::new_random(1);
        assert!(
            dungeon.is_walkable(dungeon.player_start),
            "Player start position should be walkable"
        );
    }

    #[test]
    fn test_stairs_is_walkable() {
        let dungeon = Dungeon::new_random(1);
        assert!(
            dungeon.is_walkable(dungeon.stairs_position),
            "Stairs position should be walkable"
        );
    }

    // ===== Transparency tests (Phase 3 - FOV) =====

    #[test]
    fn test_wall_not_transparent() {
        let dungeon = Dungeon::new_fixed();
        // Corner (0,0) is always a wall
        assert!(
            !dungeon.is_transparent(Position { x: 0, y: 0 }),
            "Wall should not be transparent"
        );
    }

    #[test]
    fn test_floor_transparent() {
        let dungeon = Dungeon::new_fixed();
        // Player start is always floor
        assert!(
            dungeon.is_transparent(dungeon.player_start),
            "Floor should be transparent"
        );
    }

    #[test]
    fn test_stairs_transparent() {
        let dungeon = Dungeon::new_random(1);
        assert!(
            dungeon.is_transparent(dungeon.stairs_position),
            "Stairs should be transparent"
        );
    }

    #[test]
    fn test_outside_bounds_not_transparent() {
        let dungeon = Dungeon::new_fixed();
        assert!(
            !dungeon.is_transparent(Position { x: -1, y: 0 }),
            "Outside bounds should not be transparent"
        );
    }
}
