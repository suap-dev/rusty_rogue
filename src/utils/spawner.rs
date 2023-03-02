use crate::prelude::*;

pub fn spawn_enemy(world: &mut World, position: Point, glyph: FontCharType) {
    world.push((
        Enemy,
        position,
        Renderable {
            color: DEFAULT_COLOR,
            glyph,
        },
        MovingRandomly,
    ));
}

pub fn spawn_player(world: &mut World, position: Point) {
    world.push((
        Player,
        position,
        Renderable {
            color: DEFAULT_COLOR,
            glyph: PLAYER,
        },
    ));
}
