extern crate rand;
use rand::thread_rng;
use rand::Rng;

use crate::cell_matrix::{Cell, CellKind, CellMatrix};
use crate::direction::Direction;
use crate::labyrinth_generator::LabyrinthGenerator;
use crate::room::Room;

use std::cmp;
use std::mem;

const PI_2: f32 = 3.141592 * 2f32;

pub struct Map {
    pub rooms: Vec<Room>,
    pub cell_matrix: CellMatrix
}

pub enum MapShape {
    Square,
    Circle,
}

pub struct GeneratorOptions {
    pub width: u16,
    pub height: u16,
    // (width, height)
    pub room_min: (u16, u16),
    // (width, height)
    pub room_max: (u16, u16),
    // pub wall_height: u8,
    pub iterations: u32,
    pub shape: MapShape,
    // Corridor options
    pub corridor_width: u8,
    pub corridor_height: u8,
    // top, right, bottom, left
    // These are the distance between rooms and corridors
    pub margins: (u8, u8),
    // 0-1
    pub corridor_errantness: f32,
}

pub struct Generator {
    options: GeneratorOptions,
}

impl Generator {
    pub fn new() -> Generator {
        return Generator {
            options: GeneratorOptions {
                width: 64,
                height: 32,
                room_min: (4, 4),
                room_max: (8, 8),
                iterations: 64,
                shape: MapShape::Square,
                corridor_width: 2,
                corridor_height: 2,
                corridor_errantness: 0.75,
                margins: (1, 3), // (x, y)
            },
        };
    }
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.options.width = width;
        self.options.height = height;
        return self;
    }
    pub fn iterations(mut self, iterations: u32) -> Self {
        self.options.iterations = iterations;
        return self;
    }
    pub fn room_size(mut self, min: (u16, u16), max: (u16, u16)) -> Self {
        self.options.room_min = min;
        self.options.room_max = max;
        return self;
    }
    pub fn margins(mut self, horizontal: u8, vertical: u8) -> Self {
        self.options.margins = (horizontal, vertical);
        return self;
    }
    pub fn corridor_size(mut self, width: u8, height: u8) -> Self {
        self.options.corridor_width = width;
        self.options.corridor_height = height;
        return self;
    }
    pub fn corridor_errantness(mut self, errantness: f32) -> Self {
        self.options.corridor_errantness = errantness;
        return self;
    }
    pub fn generate(self) -> Map {
        let options = self.options;
        let mut room_vector: Vec<Room> = vec![];
        let mut cell_matrix = CellMatrix::new(options.width, options.height, CellKind::Rock);

        // Generate all the rooms
        for _ in 0..options.iterations {
            let room_width = thread_rng().gen_range(options.room_min.0, options.room_max.0 + 1);
            let room_height = thread_rng().gen_range(options.room_min.1, options.room_max.1 + 1);

            let (x, y) = match options.shape {
                MapShape::Square => (
                    thread_rng().gen_range(0, options.width - room_width),
                    thread_rng().gen_range(0, options.height - room_height),
                ),
                MapShape::Circle => {
                    // TODO: why - 4?
                    let width = options.width - 4;
                    let height = options.height - 4;

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
            };
            if !room_vector
                .iter()
                .any(|r| r.collides_with(&room, options.margins))
            {
                room_vector.push(room);
            }
        }
        // Add all the newly created rooms to the cell vector
        for (idx, room) in room_vector.iter().enumerate() {
            for x in room.x..(room.x + room.width) {
                for y in room.y..(room.y + room.height) {
                    cell_matrix.set(
                        x,
                        y,
                        Cell {
                            kind: CellKind::Room(idx),
                        },
                    )
                }
            }
        }
        // Place walls
        // place_walls(&mut cell_matrix);
        let cell_matrix = LabyrinthGenerator::new(
            cell_matrix,
            options.corridor_width,
            options.corridor_height,
            options.corridor_errantness,
            options.margins,
        )
        .generate();
        // place_walls(&mut cell_matrix);
        return Map { rooms: room_vector, cell_matrix };
    }
}
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

pub fn print_map(cell_matrix: &CellMatrix) {
    // Print the map beautifully
    for (cell, x, y) in cell_matrix.iter_enumerate() {
        if x == 0 && y > 0 {
            print!("\n");
        }
        match cell.kind {
            CellKind::Room(_) => print!("R"),
            CellKind::Corridor => print!("C"),
            CellKind::Wall => print!("W"),
            _ => print!(" "),
        };
    }
}

fn place_walls(cell_matrix: &mut CellMatrix) {
    // Force place walls, don't care what's underneath really
    for y in 0..(cell_matrix.height - 1) {
        for x in 0..(cell_matrix.width - 1) {
            if (cell_matrix.get(x as i32, y as i32 + 1).kind.is_room()
                || cell_matrix.get(x as i32, y as i32 + 1).kind == CellKind::Corridor)
                && cell_matrix.get(x as i32, y as i32).kind == CellKind::Rock
            {
                cell_matrix.set(
                    x,
                    y,
                    Cell {
                        kind: CellKind::Wall,
                    },
                );
                if y as i32 - 1 > 0 {
                    cell_matrix.set(
                        x,
                        y - 1,
                        Cell {
                            kind: CellKind::Wall,
                        },
                    );
                }
            }
        }
    }
}
