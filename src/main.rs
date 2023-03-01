#![warn(clippy::all, clippy::pedantic)]

mod camera;
mod components;
pub mod glyph;
mod map;
mod map_builder;
mod spawner;
mod systems;
// mod player;
mod prelude {
    pub use crate::{camera::*, components::*, map::*, map_builder::*, spawner::*, systems::*, *};
    pub use bracket_lib::prelude::*;
    pub use legion::{systems::CommandBuffer, world::SubWorld, *};

    pub const DEFAULT_COLOR: ColorPair = ColorPair {
        // WHITE
        fg: RGBA {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        // BLACK
        bg: RGBA {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    };
}

use prelude::*;

const SCREEN_HEIGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;

const CAMERA_WIDTH: i32 = SCREEN_WIDTH / 2;
const CAMERA_HEIGHT: i32 = SCREEN_HEIGHT / 2;

const NUM_ROOMS: i32 = 20;

struct RustyRogue {
    world: World,
    resources: Resources,
    schedule: Schedule,
}
impl GameState for RustyRogue {
    fn tick(&mut self, ctx: &mut BTerm) {
        // render frame
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.resources.insert(ctx.key);
        self.schedule.execute(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render failed");
    }
}
impl RustyRogue {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut builder = MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        builder
            .carve_rooms(NUM_ROOMS)
            .carve_corridors()
            .default_player_spawn();

        spawn_player(&mut ecs, builder.get_player_spawn());

        resources.insert(builder.consume_map());
        resources.insert(Camera::new(
            builder.get_player_spawn(),
            CAMERA_WIDTH,
            CAMERA_HEIGHT,
        ));

        Self {
            world: ecs,
            resources,
            schedule: schedule(),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Rusty Rogue")
        .with_fps_cap(30.0)
        .with_dimensions(CAMERA_WIDTH, CAMERA_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(CAMERA_WIDTH, CAMERA_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(CAMERA_WIDTH, CAMERA_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, RustyRogue::new())
}
