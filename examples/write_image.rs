extern crate daedalus;
extern crate bmp;

use daedalus::Generator;
use daedalus::CellKind;
use bmp::{Image, Pixel};


fn main() {
    let room_pixel: Pixel = Pixel::new(149,184,209);
    let corridor_pixel: Pixel = Pixel::new(102,106,134);
    let wallpixel: Pixel = Pixel::new(51,51,51);

    let map = Generator::new().room_size((4,4), (16,16)).size(128, 128).iterations(512).generate();
    let mut img = Image::new(map.cell_matrix.width as u32, map.cell_matrix.height as u32);
    for (cell, x, y) in map.cell_matrix.iter_enumerate() {
        match cell.kind {
            CellKind::Room => img.set_pixel(x.into(), y.into(), room_pixel),
            CellKind::Corridor => img.set_pixel(x.into(), y.into(), corridor_pixel),
            CellKind::Wall => img.set_pixel(x.into(), y.into(), wallpixel),
            _ => img.set_pixel(x.into(), y.into(), wallpixel),
        };
    }
    img.save("print.bmp").unwrap();
}
