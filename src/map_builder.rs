use crate::prelude::*;

const MAX_ROOM_WIDTH: i32 = 10;
const MAX_ROOM_HEIGHT: i32 = 10;

pub struct MapBuilder {
    pub map: Map,
    rooms: Vec<Rect>,
    player_spawn: Point,
    rng: RandomNumberGenerator,
}
impl MapBuilder {
    pub fn new(map_width: i32, map_height: i32) -> Self {
        Self {
            map: Map::filled(map_width, map_height, TileType::Wall),
            rooms: Vec::new(),
            player_spawn: Point::zero(),
            rng: RandomNumberGenerator::new(),
        }
    }

    pub fn fill(&mut self, tile_type: TileType) {
        self.map.fill(tile_type);
    }

    pub fn carve_rooms(&mut self, rooms_number: i32) {
        // generate rooms until you have NUM_ROOMS of them

        #[allow(clippy::cast_sign_loss)]
        while self.rooms.len() < rooms_number as usize {
            // generate a room
            let room = Rect::with_size(
                self.rng.range(1, self.map.width() - MAX_ROOM_WIDTH),
                self.rng.range(1, self.map.height() - MAX_ROOM_HEIGHT),
                self.rng.range(2, MAX_ROOM_WIDTH),
                self.rng.range(2, MAX_ROOM_HEIGHT),
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
                room.for_each(|tile| {
                    if 0 < tile.x
                        && tile.x < self.map.width()
                        && 0 < tile.y
                        && tile.y < self.map.height()
                    {
                        self.map
                            .set_tile(tile, TileType::Floor)
                            .expect("Can't set tile");
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    pub fn carve_corridors(&mut self) {
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
            if self.rng.range(0, 2) == 1 {
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
            self.map
                .set_tile_at(x, y, TileType::Floor)
                .expect("Can't set tile");
            // self.map.tiles[index] = TileType::Floor;
        }
    }

    // horizontal tunnel between 2 points
    fn carve_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};

        for x in min(x1, x2)..=max(x1, x2) {
            self.map
                .set_tile_at(x, y, TileType::Floor)
                .expect("Can't set tile");
            // if let Some(index) = self.map.index_at(x, y) {
            //     self.map.tiles[index] = TileType::Floor;
            // }
        }
    }

    pub fn default_player_spawn(&mut self) {
        self.player_spawn = self.rooms.first().expect("No first room?").center();
    }

    // pub fn map(&self) -> Map {
    //     self.map
    // }

    pub fn get_player_spawn(&self) -> Point {
        self.player_spawn
    }

    pub fn consume_map(&mut self) -> Map {
        std::mem::replace(&mut self.map, Map::empty())
    }
}
