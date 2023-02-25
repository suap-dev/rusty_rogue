use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

fn get_index(x: i32, y: i32) -> usize {
    #![allow(clippy::cast_sign_loss)]
    ((y * SCREEN_WIDTH) + x) as usize
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn get_coordinates(index: usize) -> (i32, i32) {
    (index as i32 % SCREEN_WIDTH, index as i32 / SCREEN_WIDTH)
}

pub struct Map {
    pub tiles: Vec<Tile>,
}
impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![Tile::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        (0 <= point.x && point.x < SCREEN_WIDTH) && (0 <= point.y && point.y < SCREEN_HEIGHT)
    }

    pub fn is_traversable(&self, point: Point) -> bool {
        self.in_bounds(point)
            && *self
                .tiles
                .get(get_index(point.x, point.y))
                .expect("Invalid tile index")
                == Tile::Floor
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(get_index(point.x, point.y))
        } else {
            None
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = get_index(x, y);
                match self.tiles.get(index).expect("Invalid tile index") {
                    Tile::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437(WALL_GLYPH)),
                    Tile::Floor => ctx.set(x, y, LIGHT_SLATE, BLACK, to_cp437(FLOOR_GLYPH)),
                }
            }
        }
    }
}
