use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_spawn: Point,
}
impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut builder = Self {
            map: Map::new(),
            rooms: Vec::new(),
            player_spawn: Point::zero(),
        };

        builder.fill(TileType::Wall);
        builder.carve_random_rooms(rng);
        builder.carve_corridors(rng);
        builder.player_spawn = builder.rooms.first().expect("No first room?").center();

        builder
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn carve_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        const MAX_ROOM_WIDTH: i32 = 10;
        const MAX_ROOM_HEIGHT: i32 = 10;

        // generate rooms until you have NUM_ROOMS of them
        while self.rooms.len() < NUM_ROOMS {
            // generate a room
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - MAX_ROOM_WIDTH),
                rng.range(1, SCREEN_HEIGHT - MAX_ROOM_HEIGHT),
                rng.range(2, MAX_ROOM_WIDTH),
                rng.range(2, MAX_ROOM_HEIGHT),
            );

            // check if it overlaps with previously generated rooms
            let mut overlap = false;
            for r in &self.rooms {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            // if not - add it to rooms list
            if !overlap {
                room.for_each(|p| {
                    if 0 < p.x && p.x < SCREEN_WIDTH && 0 < p.y && p.y < SCREEN_HEIGHT {
                        let index = get_index(p.x, p.y);
                        self.map.tiles[index] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn carve_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        // I feel like this is a hack, not a clean solution
        // we clone it to avoid borrow checker making a scene
        // we actualy carve the tunnels in the actual room list
        // this is just used as a map of them
        let mut rooms = self.rooms.clone();

        // sort by the order of the x coordinate of the rooms centers
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev_center = rooms[i - 1].center();
            let this_center = room.center();

            // coin flip
            if rng.range(0, 2) == 1 {
                self.carve_horizontal_tunnel(prev_center.x, this_center.x, prev_center.y);
                self.carve_vertical_tunnel(prev_center.y, this_center.y, this_center.x);
            } else {
                self.carve_vertical_tunnel(prev_center.y, this_center.y, prev_center.x);
                self.carve_horizontal_tunnel(prev_center.x, this_center.x, this_center.y);
            }
        }
    }

    // vertical tunnel between 2 points
    fn carve_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};

        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = self.map.index_at(x, y) {
                self.map.tiles[index] = TileType::Floor;
            }
        }
    }

    // horizontal tunnel between 2 points
    fn carve_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};

        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = self.map.index_at(x, y) {
                self.map.tiles[index] = TileType::Floor;
            }
        }
    }
}
