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
    let wall_pixel: Pixel = Pixel::new(125, 125, 125);
    let connection_pixel: Pixel = Pixel::new(64, 255, 64);
    let removed_pixel: Pixel = Pixel::new(64, 0, 0);

    let map = Generator::new()
        .room_size((4, 4), (16, 16))
        .size(64, 64)
        .margins(1, 1)
        .corridor_size(2, 2)
        .iterations(0)
        .corridor_errantness(0.75)
        .generate();
    let mut img = Image::new(map.width as u32, map.height as u32);

    for (cell, x, y) in map.iter_enumerate() {
        match cell {
            Cell::Room(idx) => img.set_pixel(
                x.into(),
                y.into(),
                get_section_color(
                    map.section_vec[map.get_room(idx).section_id].get_id() as u16 * 4,
                ),
            ),
            Cell::Corridor(idx) => img.set_pixel(
                x.into(),
                y.into(),
                get_section_color(
                    map.section_vec[map.get_corridor(idx).section_id].get_id() as u16 * 32,
                ),
            ),
            Cell::Removed => img.set_pixel(x.into(), y.into(), removed_pixel),
            Cell::Connection => img.set_pixel(x.into(), y.into(), connection_pixel),
            _ => img.set_pixel(x.into(), y.into(), wall_pixel),
        };
    }
    img.save("print.bmp").unwrap();
}
