use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
    world.push((
        Player,
        pos,
        Render {
            color: DEFAULT_COLOR,
            glyph: glyph::PLAYER,
        },
    ));
}
