use crate::prelude::*;
const NUM_ROOMS: usize = 20;

struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_spawn: Point,
}