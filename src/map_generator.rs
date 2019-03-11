extern crate rand;
use rand::Rng;
use rand::thread_rng;

use crate::cell_matrix::{Cell, CellKind, CellMatrix};
use crate::direction::Direction;
use crate::room::Room;

use std::cmp;

const PI_2: f32 = 3.141592 * 2f32;

pub struct Map {
    pub rooms: Vec<Room>,
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
    pub margins: (u8, u8, u8, u8),
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
                height: 64,
                room_min: (4, 4),
                room_max: (8, 8),
                iterations: 64,
                shape: MapShape::Square,
                corridor_width: 2,
                corridor_height: 2,
                corridor_errantness: 0.5,
                margins: (2, 1, 1, 1)
                // TODO: Add a corridor_margin(u8, u8, u8, u8)
            },
        };
    }
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.options.width = width;
        self.options.height = height;
        return self;
    }
    pub fn room_size(mut self, min: (u16, u16), max: (u16, u16)) -> Self {
        self.options.room_min = min;
        self.options.room_max = max;
        return self;
    }
    pub fn margins(mut self, top: u8, right: u8, bottom: u8, left: u8) -> Self {
        self.options.margins = (top, right, bottom, left);
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
            let room_width = thread_rng().gen_range(options.room_min.0, options.room_max.0);
            let room_height = thread_rng().gen_range(options.room_min.1, options.room_max.1);

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
            if !room_vector.iter().any(|r| r.collides_with(&room, options.margins)) {
                room_vector.push(room);
            }
        }
        // Add all the newly created rooms to the cell vector
        for room in room_vector.iter() {
            for x in room.x..(room.x + room.width) {
                for y in room.y..(room.y + room.height) {
                    cell_matrix.set(
                        x,
                        y,
                        Cell {
                            kind: CellKind::Room,
                        },
                    )
                }
            }
        }
        // Place walls
        // place_walls(&mut cell_matrix);
        'suitable: loop {
            match found_suitable_corridor_location(
                &cell_matrix,
                options.corridor_width + options.margins.0 + options.margins.2,
                options.corridor_height + options.margins.1 + options.margins.3,
            ) {
                Ok((x, y)) => {
                    traverse_corridor(
                        &mut cell_matrix,
                        x,
                        y,
                        options.corridor_width,
                        options.corridor_height,
                        options.margins,
                        Direction::rand(),
                    );
                }
                Err(_) => break 'suitable,
            };
        }

        // place_walls(&mut cell_matrix);
        print_map(cell_matrix);
        return Map { rooms: room_vector };
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

pub fn print_map(cell_matrix: CellMatrix) {
    println!("Printing map");
    // Print the map beautifully
    for (cell, x, _) in cell_matrix.iter_enumerate() {
        match cell.kind {
            CellKind::Room => print!("R"),
            CellKind::Corridor => print!("C"),
            CellKind::Wall => print!("W"),
            _ => print!(" "),
        };
        if (x % cell_matrix.width) == 0 {
            print!("\n");
        }
    }
}

fn is_suitable_corridor_location(
    cell_matrix: &CellMatrix,
    x: i32,
    y: i32,
    width: u8,
    height: u8,
) -> bool {
    let cell_rect = cell_matrix.get_rect(x, y, width as u16, height as u16);
    if !cell_rect
        .cell_vector
        .iter()
        .any(|c| c.kind != CellKind::Rock)
    {
        return true;
    } else {
        return false;
    }
}

fn found_suitable_corridor_location(
    cell_matrix: &CellMatrix,
    width: u8,
    height: u8,
) -> Result<(u16, u16), String> {
    let start_x = thread_rng().gen_range(0, cell_matrix.width);
    let start_y = thread_rng().gen_range(0, cell_matrix.height);

    for y in 0..(cell_matrix.height - (1 + height) as u16) {
        for x in 0..(cell_matrix.width - (1 + width) as u16) {
            let x_pos = (x + start_x) % cell_matrix.width;
            let y_pos = (y + start_y) % cell_matrix.height;

            if is_suitable_corridor_location(cell_matrix, x_pos as i32, y_pos as i32, width, height) {
                return Ok((x_pos, y_pos));
            }
        }
    }
    return Err(String::from(
        "Could not find suitable corridor on cellmatrix",
    ));
}

fn place_walls(cell_matrix: &mut CellMatrix) {
    // Force place walls, don't care what's underneath really
    for y in 0..(cell_matrix.height - 1) {
        for x in 0..(cell_matrix.width - 1) {
            if (cell_matrix.get(x as i32, y as i32 + 1).kind == CellKind::Room
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

/**
 * Recursive corridor logic
 */
fn traverse_corridor(
    cell_matrix: &mut CellMatrix,
    x: u16,
    y: u16,
    corridor_width: u8,
    corridor_height: u8,
    // top, right, bottom, left
    margins: (u8, u8, u8, u8),
    direction: Direction,
) {
    let mut direction = match thread_rng().gen::<f32>() {
        x if x > 0.75 => Direction::rand(),
        _ => direction,
    };
    // let mut direction = direction;
    // Start the labyrinth algorithm here
    // This just sets a corridor piece at the starting location
    cell_matrix.set_rect(
        Cell {
            kind: CellKind::Corridor,
        },
        x + margins.0 as u16,
        y + margins.3 as u16,
        corridor_width as u16,
        corridor_height as u16,
    );
    for _ in 0..3 {
        match direction {
            Direction::N => {
                if is_suitable_corridor_location(
                    cell_matrix,
                    x as i32,
                    y as i32 - 1,
                    corridor_width + margins.1 + margins.3,
                    2,
                ) {
                    traverse_corridor(
                        cell_matrix,
                        x,
                        y - 1,
                        corridor_width,
                        corridor_height,
                        margins,
                        direction.clone(),
                    );
                }
            }
            Direction::E => {
                let x_new = x as i32 + (corridor_width + margins.1 + margins.3) as i32 - 1;
                if is_suitable_corridor_location(
                    cell_matrix,
                    x_new,
                    y as i32,
                    2,
                    corridor_height + margins.0 + margins.2,
                ) {
                    traverse_corridor(
                        cell_matrix,
                        x + 1,
                        y,
                        corridor_width,
                        corridor_height,
                        margins,
                        direction.clone(),
                    );
                }
            }
            Direction::S => {
                let y_new = y as i32
                    + (corridor_height + margins.0 + margins.2) as i32
                    - 1;
                if is_suitable_corridor_location(
                    cell_matrix,
                    x as i32,
                    y_new,
                    corridor_width + margins.1 + margins.3,
                    2,
                ) {
                    traverse_corridor(
                        cell_matrix,
                        x,
                        y + 1,
                        corridor_width,
                        corridor_height,
                        margins,
                        direction.clone(),
                    );
                }
            }
            Direction::W => {
                if is_suitable_corridor_location(
                    cell_matrix,
                    x as i32 - 1,
                    y as i32,
                    2,
                    corridor_height + margins.0 + margins.2,
                ) {
                    traverse_corridor(
                        cell_matrix,
                        x - 1,
                        y,
                        corridor_width,
                        corridor_height,
                        margins,
                        direction.clone(),
                    );
                }
            }
        }
        direction = direction.turn_clockwise();
    }
}
