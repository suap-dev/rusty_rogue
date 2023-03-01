use crate::prelude::*;

pub fn spawn_player(world: &mut World, pos: Point) {
    world.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(PLAYER_GLYPH),
        },
    ));
}
