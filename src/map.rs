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
    width: i32,
    height: i32,
    pub tiles: Vec<TileType>,
}
impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let tiles_number = (width * height) as usize;

        Self {
            width,
            height,
            tiles: vec![TileType::Floor; tiles_number],
        }
    }
    
    pub fn fill(map_width: i32, map_height: i32, tile_type: TileType) -> Self {
        let tiles_number = (map_width * map_height) as usize;

        Self {
            width: map_width,
            height: map_height,
            tiles: vec![tile_type; tiles_number],
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

    // TODO: implement Err
    pub fn index_of(&self, point: Point) -> Option<usize> {
        self.index_at(point.x, point.y)
    }

    // TODO: implement Err
    pub fn index_at(&self, x: i32, y: i32) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some(self.index(x, y))
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, tile: Point, tile_type: TileType) -> Result<(), String> {
        self.set_tile_at(tile.x, tile.y, tile_type)
    }

    pub fn set_tile_at(&mut self, x: i32, y: i32, tile_type: TileType) -> Result<(), String> {
        let index = self.index(x,y);
        
        if let Some(x) = self.tiles.get_mut(index) {
            *x = tile_type;
            Ok(())
        } else {
            Err("Tile out of map bounds".to_string())
        }
    }

    pub fn width(&self) -> i32{
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..self.height {
            for x in 0..self.width {
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
        ((y * self.width) + x) as usize
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0 <= x && x < self.width) && (0 <= y && y < self.height)
    }
}
