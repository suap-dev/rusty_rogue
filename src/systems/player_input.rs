use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    sub_world: &mut SubWorld,
    #[resource] map: &Map,
    // so Legion searches for the resource by TYPE ?
    #[resource] key_pressed: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key_code) = key_pressed {
        let delta = match key_code {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _other => Point::zero(),
        };

        // if both x and y are none-zero
        // TODO: can I implement it with delta != Point::zero() ?
        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());

            players.iter_mut(sub_world).for_each(|position| {
                let destination = *position + delta;
                if map.is_traversable(destination) {
                    *position = destination;
                    camera.center_at(*position);
                }
            });
        }
    }
}
