#![warn(clippy::all, clippy::pedantic)]

mod map;
mod map_builder;
mod player;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;

    // TODO: package/enum/struct with glyphs
    pub const WALL_GLYPH: char = '#';
    pub const FLOOR_GLYPH: char = '.';
    pub const PLAYER_GLYPH: char = '@';
}

use prelude::*;

const SCREEN_HEIGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;
const NUM_ROOMS: i32 = 20;

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
        // let mut rng = RandomNumberGenerator::new();
        let mut builder = MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        builder.fill(TileType::Wall);
        builder.carve_rooms(NUM_ROOMS);
        builder.carve_corridors();
        builder.player_spawn = builder.rooms.first().expect("No first room?").center();
        Self {
            map: builder.map,
            player: Player::new(builder.player_spawn),
        }

        // Self {
        //     map: Map::new(),
        //     player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        // }
    }
}
