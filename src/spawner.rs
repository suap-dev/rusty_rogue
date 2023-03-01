use crate::prelude::*;

const ENEMY_GLYPHS: [FontCharType; 4] = [glyph::ETTIN, glyph::OGRE, glyph::ORC, glyph::GOBLIN];
pub fn spawn_monster(world: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    world.push((
        Enemy,
        pos,
        Render {
            color: DEFAULT_COLOR,
            glyph: ENEMY_GLYPHS[rng.range(0, 4)],
        },
    ));
}

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
