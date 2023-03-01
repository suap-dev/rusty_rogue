use crate::prelude::*;

// Player,
// position: Point,
// Render:
//     color: ColorPair,
//     glyph: FontCharType,

// Enemy,
// position: Point,
// Render:
//     color: ColorPair,
//     glyph: FontCharType,

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collision(sub_world: &mut SubWorld, command_buffer: &mut CommandBuffer) {
    // we just get the first player position, since for now ther is one player. let's not overthink it.
    let player_position = *<&Point>::query()
        .filter(component::<Player>())
        .iter(sub_world)
        .next()
        .expect("No player exists in this query.");

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(sub_world)
        .filter(|(_, position)| **position == player_position)
        .for_each(|(entity, _)| command_buffer.remove(*entity));
}
