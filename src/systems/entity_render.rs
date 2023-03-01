use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(sub_world: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    // 1 - console id
    draw_batch.target(1);

    let camera_top_left = Point::new(camera.left(), camera.top());
    <(&Point, &Render)>::query()
        .iter(sub_world)
        .for_each(|(position, render)| {
            draw_batch.set(*position - camera_top_left, render.color, render.glyph);
        });
    draw_batch
        .submit(usize::MAX)
        .expect("[Entity] Batch submission failed");
}
