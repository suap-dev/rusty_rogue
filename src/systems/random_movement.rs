use crate::prelude::*;

const DIRECTION: [Point; 4] = [EAST, WEST, NORTH, SOUTH];

#[system]
#[write_component(Point)]
// #[read_component(MovingRandomly)]
pub fn random_movement(sub_world: &mut SubWorld, #[resource] map: &Map) {
    let mut rng = RandomNumberGenerator::new();
    <&mut Point>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(sub_world)
        .for_each(|position| {
            let destination = *position + *rng.random_slice_entry(&DIRECTION).unwrap();
            if map.is_traversable(destination) {
                *position = destination;
            }
        });
}
