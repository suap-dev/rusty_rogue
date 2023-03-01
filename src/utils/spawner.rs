use crate::prelude::*;

pub fn spawn_enemy(world: &mut World, position: Point, glyph: FontCharType) {
    world.push((
        Enemy,
        position,
        Render {
            color: DEFAULT_COLOR,
            glyph,
        },
    ));
}

pub fn spawn_player(world: &mut World, position: Point) {
    world.push((
        Player,
        position,
        Render {
            color: DEFAULT_COLOR,
            glyph: PLAYER,
        },
    ));
}
