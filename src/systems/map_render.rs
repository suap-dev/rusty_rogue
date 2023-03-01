use legion::system;

use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    // target(n) - n is the id of the console/context
    draw_batch.target(0);
    for y in camera.top()..camera.bottom() {
        for x in camera.left()..camera.right() {
            // if the index is correct, then match on the tile at the index
            if let Some(index) = map.index_at(x, y) {
                draw_batch.set(
                    // we target x, y point on screen coordinates,
                    // so we need to offset by the camera position
                    Point::new(x - camera.left(), y - camera.top()),
                    DEFAULT_COLOR,
                    match map.tiles()[index] {
                        TileType::Wall => WALL,
                        TileType::Floor => FLOOR,
                    },
                );
            }
        }
    }
    // submit(n) - n is the number in the drawing order
    // 0 means it's drawn first
    draw_batch.submit(0).expect("[Map] Batch submission failed");
}
