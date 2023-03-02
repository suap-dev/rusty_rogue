use crate::prelude::*;

#[system]
#[write_component(Point)]
// #[read_component(Player)]
#[allow(clippy::trivially_copy_pass_by_ref)] // regarding this: #[resource] key_pressed: &Option<VirtualKeyCode>,
pub fn player_input(
    sub_world: &mut SubWorld,
    #[resource] map: &Map,
    // so Legion searches for the resource by TYPE ?
    #[resource] key_pressed: &Option<VirtualKeyCode>, // TODO: research if this is doable in a way that clippy doesn't complain
    #[resource] camera: &mut Camera,
) {
    if let Some(key_code) = key_pressed {
        let direction = match key_code {
            VirtualKeyCode::Up => NORTH,
            VirtualKeyCode::Down => SOUTH,
            VirtualKeyCode::Right => EAST,
            VirtualKeyCode::Left => WEST,
            _other => Point::zero(),
        };

        if direction != Point::zero() {
            let mut players = <&mut Point>::query().filter(component::<Player>());

            players.iter_mut(sub_world).for_each(|position| {
                let destination = *position + direction;
                if map.is_traversable(destination) {
                    *position = destination;
                    camera.center_at(*position);
                }
            });
        }
    }
}
