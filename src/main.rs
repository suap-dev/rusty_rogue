#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

const WINDOW_HEIGHT: i16 = 50;
const WINDOW_WIDTH: i16 = 80;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Rogue")
        .build()?;

    main_loop(context, RustyRogue {})
}

struct RustyRogue {}
impl GameState for RustyRogue {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(WINDOW_HEIGHT / 2, "Hello World!");
    }
}
