use crate::prelude::*;

pub fn spawn_enemy(world: &mut World, pos: Point, glyph: FontCharType) {
    world.push((
        Enemy,
        pos,
        Render {
            color: DEFAULT_COLOR,
            glyph,
        },
    ));
}

pub fn spawn_player(world: &mut World, pos: Point) {
    world.push((
        Player,
        pos,
        Render {
            color: DEFAULT_COLOR,
            glyph: PLAYER,
        },
    ));
}
