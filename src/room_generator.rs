use crate::cell_matrix::{Cell, Map};
use crate::map_generator::MapShape;
use crate::room::Room;
use crate::sections::Section;

use rand::thread_rng;
use rand::Rng;

use std::cmp;

const PI_2: f32 = 3.141592 * 2f32;

fn is_within_circle_shape(a: &Room, radius: u16) -> bool {
    let dx = cmp::max(
        radius as i16 - a.x as i16,
        (a.x + a.width) as i16 - radius as i16,
    ) as i32;
    let dy = cmp::max(
        radius as i16 - a.y as i16,
        (a.y + a.height) as i16 - radius as i16,
    ) as i32;
    return radius as i32 * radius as i32 >= dx * dx + dy * dy;
}

pub fn generate_rooms(
    map: &mut Map,
    room_min: (u16, u16),
    room_max: (u16, u16),
    margins: (u8, u8),
    iterations: u32,
    shape: MapShape,
) {
    for _ in 0..iterations {
        let room_width = thread_rng().gen_range(room_min.0, room_max.0 + 1);
        let room_height = thread_rng().gen_range(room_min.1, room_max.1 + 1);

        let (x, y) = match shape {
            MapShape::Square => (
                thread_rng().gen_range(0, map.width - room_width),
                thread_rng().gen_range(0, map.height - room_height),
            ),
            MapShape::Circle => {
                // TODO: why - 4?
                let width = map.width - 4;
                let height = map.height - 4;

                let angle = thread_rng().gen::<f32>() * PI_2;
                let mut rng = thread_rng();
                let r_x = rng.gen::<f32>() * ((width - room_width) as f32 / 2f32);
                let r_y = rng.gen::<f32>() * ((height - room_height) as f32 / 2f32);
                (
                    (width as f32 / 2f32 + r_x * (angle.cos())).floor() as u16,
                    (height as f32 / 2f32 + r_y * (angle.sin())).floor() as u16,
                )
            }
        };

        let room = Room {
            width: room_width,
            height: room_height,
            x,
            y,
            section_id: map.new_section(),
        };
        if !map.iter_rooms().any(|r| r.collides_with(&room, margins)) {
            let idx = map.push_room(room);
            for x_pos in x..(x + room_width) {
                for y_pos in y..(y + room_height) {
                    map.set(x_pos, y_pos, Cell::Room(idx))
                }
            }
        }
    }
}
