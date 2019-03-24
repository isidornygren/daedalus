use crate::cell_matrix::{Cell, CellMatrix};
use crate::labyrinth_generator::LabyrinthGenerator;
use crate::room::{Corridor, Room};
use crate::room_generator::generate_rooms;
use crate::sections::SectionMerger;

pub struct Map {
    pub rooms: Vec<Room>,
    pub corridors: Vec<Corridor>,
    pub cell_matrix: CellMatrix,
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
        assert!(horizontal > 0, "Horizontal margin must be greater than 0");
        assert!(vertical > 0, "Vertical margin must be greater than 0");
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
        let mut cell_matrix =
            CellMatrix::new(options.width, options.height, Cell::Rock(false, false));

        let room_vector = generate_rooms(
            &mut cell_matrix,
            options.room_min,
            options.room_max,
            options.margins,
            options.iterations,
            options.shape,
        );
        let (cell_matrix, corridor_vector) = LabyrinthGenerator::new(
            cell_matrix,
            options.corridor_width,
            options.corridor_height,
            options.corridor_errantness,
            options.margins,
        )
        .generate();

        // place_walls(&mut cell_matrix);
        let map = Map {
            rooms: room_vector,
            corridors: corridor_vector,
            cell_matrix,
        };
        return SectionMerger::new(
            map,
            options.margins,
            (options.corridor_width, options.corridor_height),
        )
        .generate();
    }
}
