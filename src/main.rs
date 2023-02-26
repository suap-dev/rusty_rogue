#![warn(clippy::all, clippy::pedantic)]

mod map;
mod map_builder;
mod player;
mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;

    // TODO: package/enum/struct with glyphs
    pub const WALL_GLYPH: char = '#';
    pub const FLOOR_GLYPH: char = '.';
    pub const PLAYER_GLYPH: char = '@';
}

use prelude::*;

const SCREEN_HEIGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;

const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

const NUM_ROOMS: i32 = 20;

fn main() -> BError {
    // let context = BTermBuilder::simple80x50()
    let context = BTermBuilder::new()
        .with_title("Rusty Rogue")
        .with_fps_cap(30.0)
        // .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_tile_dimensions(16, 16)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        // .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "dungeonfont.png")
        // .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "dungeonfont.png")
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

        builder
            .carve_rooms(NUM_ROOMS)
            .carve_corridors()
            .default_player_spawn();

        Self {
            map: builder.consume_map(),
            player: Player::new(builder.get_player_spawn()),
        }
    }
}
