#![warn(clippy::all, clippy::pedantic)]

mod map;
mod prelude {
    pub use bracket_lib::prelude::*;

    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;
    pub use crate::map::*;
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
}
impl GameState for RustyRogue {
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx.print_centered(SCREEN_HEIGHT / 2, "Hello World!");
        ctx.cls();
        self.map.render(ctx);
    }
}
impl RustyRogue {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}
