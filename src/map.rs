use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

// TODO: remove dead code

pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<TileType>,
}

impl Map {
    // // create a new map filled with Floor tiles
    // pub fn new(width: i32, height: i32) -> Self {
    //     #[allow(clippy::cast_sign_loss)]
    //     let tiles_number = (width * height) as usize;
    //     Self {
    //         width,
    //         height,
    //         tiles: vec![TileType::Floor; tiles_number],
    //     }
    // }

    // create a new map filled with tiles of given type
    pub fn filled(map_width: i32, map_height: i32, tile_type: TileType) -> Self {
        #[allow(clippy::cast_sign_loss)]
        let tiles_number = (map_width * map_height) as usize;

        Self {
            width: map_width,
            height: map_height,
            tiles: vec![tile_type; tiles_number],
        }
    }

    // create an empty map
    pub fn empty() -> Self {
        Self {
            width: 0,
            height: 0,
            tiles: vec![],
        }
    }

    // // fill the entire map with a tile of given type
    // pub fn fill(&mut self, tile_type: TileType) {
    //     self.tiles.iter_mut().for_each(|tile| {
    //         *tile = tile_type;
    //     });
    // }    

    pub fn set_tile(&mut self, tile: Point, tile_type: TileType) -> Result<(), String> {
        self.set_tile_at(tile.x, tile.y, tile_type)
    }

    pub fn set_tile_at(&mut self, x: i32, y: i32, tile_type: TileType) -> Result<(), String> {
        let index = self.index(x, y);

        if let Some(x) = self.tiles.get_mut(index) {
            *x = tile_type;
            Ok(())
        } else {
            Err("Tile out of map bounds".to_string())
        }
    }

    // check if entering a tile at given location is allowed
    pub fn is_traversable(&self, point: Point) -> bool {
        self.in_bounds(point.x, point.y)
            && *self
                .tiles
                .get(self.index(point.x, point.y))
                .expect("Invalid tile index")
                == TileType::Floor
    }

    // // TODO: implement Err?
    // pub fn index_of(&self, point: Point) -> Option<usize> {
    //     self.index_at(point.x, point.y)
    // }

    // TODO: implement Err?
    pub fn index_at(&self, x: i32, y: i32) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some(self.index(x, y))
        } else {
            None
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0 <= x && x < self.width) && (0 <= y && y < self.height)
    }

    #[allow(clippy::cast_sign_loss)]
    fn index(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    // // check if given index is valid
    // fn is_valid(&self, index: usize) -> bool {
    //     index < self.tiles.len()
    // }

    // // get coordinates, given the index at which the tile is located
    // #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    // fn coords(&self, index: usize) -> (i32, i32) {
    //     (index as i32 % self.width, index as i32 / self.height)
    // }

    pub fn tiles(&self) -> &[TileType] {
        self.tiles.as_ref()
    }
}
