use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

// #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
// fn coords_at(index: usize) -> (i32, i32) {
//     (index as i32 % SCREEN_WIDTH, index as i32 / SCREEN_WIDTH)
// }

pub struct Map {
    map_width: i32,
    map_height: i32,
    pub tiles: Vec<TileType>,
}
impl Map {
    pub fn new(map_width: i32, map_height: i32) -> Self {
        let tiles_number = (map_width * map_height) as usize;

        Self {
            map_width,
            map_height,
            tiles: vec![TileType::Floor; tiles_number],
        }
    }

    pub fn is_traversable(&self, point: Point) -> bool {
        self.in_bounds(point.x, point.y)
            && *self
                .tiles
                .get(self.index(point.x, point.y))
                .expect("Invalid tile index")
                == TileType::Floor
    }

    pub fn index_of(&self, point: Point) -> Option<usize> {
        self.index_at(point.x, point.y)
    }

    pub fn index_at(&self, x: i32, y: i32) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some(self.index(x, y))
        } else {
            None
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                let index = self.index(x, y);
                match self.tiles.get(index).expect("Invalid tile index") {
                    TileType::Wall => ctx.set(x, y, LIGHT_GREEN, BLACK, to_cp437(WALL_GLYPH)),
                    TileType::Floor => ctx.set(x, y, LIGHT_SLATE, BLACK, to_cp437(FLOOR_GLYPH)),
                }
            }
        }
    }
    

    fn index(&self, x: i32, y: i32) -> usize {
        #![allow(clippy::cast_sign_loss)]
        ((y * self.map_width) + x) as usize
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0 <= x && x < self.map_width) && (0 <= y && y < self.map_height)
    }
}
