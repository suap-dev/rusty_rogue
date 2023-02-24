use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const WALL: char = '#';
const PLAYER: char = '@';
const FLOOR: char = '.';

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Wall, // #
    Floor,
    // @ - player
}

pub fn get_index(x: i32, y: i32) -> usize {
    #![allow(clippy::cast_sign_loss)]
    ((y * SCREEN_WIDTH) + x) as usize
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn get_coordinates(index: usize) -> (i32, i32) {
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

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = get_index(x, y);
                match self.tiles.get(index).expect("Invalid tile index") {
                    Tile::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437(WALL)),
                    Tile::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437(FLOOR)),
                }
            }
        }
    }
}
