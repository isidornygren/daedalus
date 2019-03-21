use crate::cell_matrix::{Cell, CellKind, CellMatrix};
use crate::map_generator::print_map;

use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::Rng;

use crate::direction::Direction;

pub struct LabyrinthGenerator {
    cell_matrix: CellMatrix,
    corridor_width: u8,
    corridor_height: u8,
    corridor_errantness: f32,
    margins: (u8, u8),
    rng: ThreadRng,
}

impl LabyrinthGenerator {
    pub fn new(
        cell_matrix: CellMatrix,
        corridor_width: u8,
        corridor_height: u8,
        corridor_errantness: f32,
        margins: (u8, u8),
    ) -> LabyrinthGenerator {
        return LabyrinthGenerator {
            cell_matrix,
            corridor_width,
            corridor_height,
            corridor_errantness,
            margins,
            rng: thread_rng(),
        };
    }
    pub fn generate(mut self) -> CellMatrix {
        'suitable: loop {
            match self.find_suitable_corridor_location() {
                Ok((x, y)) => {
                    self.traverse_corridor(x, y, Direction::rand());
                }
                Err(_) => break 'suitable,
            };
        }
        return self.cell_matrix;
    }

    fn find_suitable_corridor_location(&mut self) -> Result<(u16, u16), String> {
        let start_x = self.rng.gen_range(0, self.cell_matrix.width);
        let start_y = self.rng.gen_range(0, self.cell_matrix.height);

        for y in 0..(self.cell_matrix.height - (1 + self.corridor_height) as u16) {
            for x in 0..(self.cell_matrix.width - (1 + self.corridor_width) as u16) {
                let x_pos = (x + start_x) % self.cell_matrix.width;
                let y_pos = (y + start_y) % self.cell_matrix.height;

                if is_suitable_corridor_location(
                    &self.cell_matrix,
                    x_pos as i32,
                    y_pos as i32,
                    self.corridor_width,
                    self.corridor_width,
                    (
                        self.margins.0,
                        self.margins.1,
                        self.margins.0,
                        self.margins.1,
                    ),
                ) {
                    return Ok((x_pos, y_pos));
                }
            }
        }
        return Err(String::from(
            "Could not find suitable corridor on cellmatrix",
        ));
    }

    /**
     * Recursive corridor logic
     */
    fn traverse_corridor(
        &mut self,
        x: u16,
        y: u16,
        // horizontal / vertical
        direction: Direction,
    ) {
        let mut direction = match thread_rng().gen::<f32>() {
            x if x > self.corridor_errantness => Direction::rand(),
            _ => direction,
        };
        // Start the labyrinth algorithm here
        // This just sets a corridor piece at the starting location
        self.cell_matrix.set_rect(
            Cell {
                kind: CellKind::Corridor,
            },
            x as u16,
            y as u16,
            self.corridor_width as u16,
            self.corridor_height as u16,
        );

        let mut direction_pool = vec![Direction::E, Direction::N, Direction::S, Direction::W];
        for _ in 0..3 {
            match direction {
                Direction::N => {
                    if is_suitable_corridor_location(
                        &self.cell_matrix,
                        x as i32,
                        y as i32 - 1,
                        self.corridor_width,
                        1,
                        (self.margins.0, self.margins.1, self.margins.0, 0),
                    ) {
                        self.traverse_corridor(x, y - 1, direction.clone());
                    }
                }
                Direction::E => {
                    let x_new = x + self.corridor_width as u16;
                    if is_suitable_corridor_location(
                        &self.cell_matrix,
                        x_new as i32,
                        y as i32,
                        1,
                        self.corridor_height,
                        (0, self.margins.1, self.margins.0, self.margins.1),
                    ) {
                        self.traverse_corridor(x + 1, y, direction.clone());
                    }
                }
                Direction::S => {
                    let y_new = y + self.corridor_height as u16;
                    if is_suitable_corridor_location(
                        &self.cell_matrix,
                        x as i32,
                        y_new as i32,
                        self.corridor_width,
                        1,
                        (self.margins.0, 0, self.margins.0, self.margins.1),
                    ) {
                        self.traverse_corridor(x, y + 1, direction.clone());
                    }
                }
                Direction::W => {
                    if is_suitable_corridor_location(
                        &self.cell_matrix,
                        x as i32 - 1,
                        y as i32,
                        1,
                        self.corridor_height,
                        (self.margins.0, self.margins.1, 0, self.margins.1),
                    ) {
                        self.traverse_corridor(x - 1, y, direction.clone());
                    }
                }
            }
            let idx = direction_pool.iter().position(|d| *d == direction).unwrap();
            direction_pool.remove(idx);
            if (direction_pool.len() == 1) {
                direction = direction_pool[0];
            } else {
                direction = direction_pool[self.rng.gen_range(0, direction_pool.len()) as usize];
            }
        }
    }
}

fn is_suitable_corridor_location(
    cell_matrix: &CellMatrix,
    x: i32,
    y: i32,
    width: u8,
    height: u8,
    // Margins doesn't check for position outside of map
    // left, top, right, bottom
    margins: (u8, u8, u8, u8),
) -> bool {
    let margin_rect = cell_matrix.get_rect(
        x - margins.0 as i32,
        y - margins.1 as i32,
        width as u16 + (margins.2 + margins.0) as u16,
        height as u16 + (margins.3 + margins.1) as u16,
    );
    if margin_rect
        .cell_vector
        .iter()
        .any(|c| c.kind != CellKind::SolidRock && c.kind != CellKind::Rock)
    {
        return false;
    }
    // Check the inner rectangle as well
    let cell_rect = cell_matrix.get_rect(x, y, width as u16, height as u16);
    if cell_rect
        .cell_vector
        .iter()
        .any(|c| c.kind != CellKind::Rock)
    {
        return false;
    }
    return true;
}
