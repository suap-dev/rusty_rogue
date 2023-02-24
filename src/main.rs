#![warn(clippy::all, clippy::pedantic)]

mod map;
mod player;
mod prelude {
    pub use bracket_lib::prelude::*;

    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;
    pub use crate::map::*;
    pub use crate::player::*;

    // TODO: package/enum/struct with glyphs
    pub const WALL_GLYPH: char = '#';
    pub const FLOOR_GLYPH: char = '.';
    pub const PLAYER_GLYPH: char = '@';
}

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Rogue")
        .build()?;

    main_loop(context, RustyRogue::new())
}

struct RustyRogue {
    map: Map,
    player: Player,
}
impl GameState for RustyRogue {
    fn tick(&mut self, ctx: &mut BTerm) {
        // update state
        self.player.update(ctx, &self.map);

        // render frame
        ctx.cls();
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
impl RustyRogue {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}
