extern crate bmp;
extern crate daedalus;

use bmp::{Image, Pixel};
use daedalus::Cell;
use daedalus::Generator;

fn get_section_color(section: u16) -> Pixel {
    let section = section % 765;
    let r = match section {
        section if section < 255 => 255 - section,
        section if section > 510 => section - 510,
        _ => 0,
    };
    let g = match section {
        section if section < 255 => section,
        section if section > 510 => 0,
        _ => 255 - (section - 255),
    };
    let b = match section {
        section if section > 765 => 0,
        section if section > 510 => 255 - (section - 510),
        section if section < 255 => 0,
        _ => section - 255,
    };
    return Pixel::new(r as u8, g as u8, b as u8);
}

fn main() {
    let wallpixel: Pixel = Pixel::new(0, 0, 0);

    let map = Generator::new()
        .room_size((4, 4), (16, 16))
        .size(128, 128)
        .margins(1, 3)
        .corridor_size(3, 3)
        .iterations(64)
        .corridor_errantness(0.75)
        .generate();
    let mut img = Image::new(map.cell_matrix.width as u32, map.cell_matrix.height as u32);
    for (cell, x, y) in map.cell_matrix.iter_enumerate() {
        match cell {
            Cell::Room(idx) => img.set_pixel(
                x.into(),
                y.into(),
                get_section_color(map.rooms[idx].section.get_id() * 4),
            ),
            Cell::Corridor(idx) => img.set_pixel(
                x.into(),
                y.into(),
                get_section_color(map.corridors[idx].section.get_id() * 32),
            ),
            Cell::Rock(h, v) => img.set_pixel(
                x.into(),
                y.into(),
                Pixel::new(if h { 255 } else { 100 }, 100, if v { 255 } else { 100 }),
            ),
            _ => img.set_pixel(x.into(), y.into(), wallpixel),
        };
    }
    // Paint all sections
    for room in map.rooms.iter() {
        for connection in room.section.get_connections().iter() {
            let val = (connection.3 * 255f32) as u8;
            img.set_pixel(
                connection.0.into(),
                connection.1.into(),
                Pixel::new(val, val, val),
            );
        }
    }
    img.save("print.bmp").unwrap();
}
